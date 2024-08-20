use crate::error::Error;
use crate::{IdType, PropsType};
use memmap2::{Mmap, MmapOptions};
use num::{Bounded, CheckedAdd, Integer, NumCast, ToPrimitive, Zero};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rkyv::Archive;
use std::fmt::Debug;
use std::ops::Bound::{Excluded, Unbounded};
use std::{collections::BTreeMap, fs::File, path::Path, sync::Arc};

mod builder;
pub use builder::{MmapGraphBuilder, MmapGraphBuilderOptions};

mod iter;
pub use iter::MmapGraphRangeIter;

mod inner;
use inner::*;

#[derive(Debug)]
enum EdgeOrder {
    SPO,
    POS,
    OSP,
}

/// A graph which can be loaded via mmap in a zero-copy fashion.
pub struct MmapGraph<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: Debug,
{
    inner: Arc<MmapGraphInner<Id, NodeProps, EdgeProps>>,
}

impl<Id, NodeProps, EdgeProps> Clone for MmapGraph<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: Debug,
{
    fn clone(&self) -> Self {
        MmapGraph {
            inner: self.inner.clone(),
        }
    }
}

impl<Id, NodeProps, EdgeProps> Debug for MmapGraph<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nMmapGraph{\n")?;
        f.write_fmt(format_args!(
            "  path: \"{}\"\n",
            self.inner.path.to_string_lossy()
        ))?;
        f.write_fmt(format_args!(
            "  # node props: \"{}\"\n",
            self.inner.archive.node_props.len()
        ))?;
        f.write_fmt(format_args!(
            "  # edge props: \"{}\"\n",
            self.inner.archive.edge_props.len()
        ))?;
        f.write_fmt(format_args!(
            "  # spo edges: \"{}\"\n",
            self.inner.archive.spo.len()
        ))?;
        f.write_fmt(format_args!(
            "  # pos edges: \"{}\"\n",
            self.inner.archive.pos.len()
        ))?;
        f.write_fmt(format_args!(
            "  # osp edges: \"{}\"\n",
            self.inner.archive.osp.len()
        ))?;
        f.write_fmt(format_args!(
            "  # spo index: \"{}\"\n",
            self.inner.archive.spo_index.len()
        ))?;
        f.write_fmt(format_args!(
            "  # pos index: \"{}\"\n",
            self.inner.archive.pos_index.len()
        ))?;
        f.write_fmt(format_args!(
            "  # osp index: \"{}\"\n",
            self.inner.archive.osp_index.len()
        ))?;
        f.write_str("}\n")
    }
}

impl<Id, NodeProps, EdgeProps> MmapGraph<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: Debug,
{
    pub fn min_value(&self) -> Id {
        Id::min_value()
    }

    pub fn max_value(&self) -> Id {
        Id::max_value()
    }

    pub fn get_memory_usage(&self) -> Result<usize, Error> {
        Ok(self.inner.mmap.len())
    }

    fn make_range_iter(
        &self,
        edge_order: EdgeOrder,
        index: &BTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,
        default_end: Id::Archived,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> MmapGraphRangeIter<'static, Id, NodeProps, EdgeProps> {
        let (start_bound, end_bound) = {
            let (pred, obj) = prefix
                .1
                .map(|p| (Some(p.0), p.1))
                .unwrap_or_else(|| (None, None));
            (
                (
                    prefix.0.clone(),
                    pred.clone().unwrap_or(Id::Archived::min_value()),
                    obj.clone().unwrap_or(Id::Archived::min_value()),
                ),
                (
                    prefix.0,
                    pred.unwrap_or(Id::Archived::max_value()),
                    obj.unwrap_or(Id::Archived::max_value()),
                ),
            )
        };

        // Use the index to find a rough start position for start_cur.
        let mut start_cur = {
            let mut chunks = index.range((
                Unbounded::<(Id::Archived, Id::Archived, Id::Archived)>,
                Excluded(start_bound.clone()),
            ));
            chunks
                .next_back()
                .map(|(_, idx)| idx.clone())
                .unwrap_or(Id::Archived::zero())
        };

        // Use the index to find a rough end position for end_cur.
        let mut end_cur = {
            let mut chunks = index.range((
                Excluded(end_bound.clone()),
                Unbounded::<(Id::Archived, Id::Archived, Id::Archived)>,
            ));
            chunks
                .next()
                .map(|(_, idx)| {
                    idx.clone()
                        .checked_add(&<Id::Archived as NumCast>::from(1).unwrap())
                        .unwrap()
                })
                .unwrap_or(default_end.clone())
        };

        // Walk the index forward util we find the first included key.
        {
            if start_cur < default_end {
                let mut cur = self
                    .inner
                    .get_ordered_edge(&edge_order, start_cur.to_usize().unwrap())
                    .unwrap();
                while start_cur < default_end
                    && start_bound > (cur.0.clone(), cur.1.clone(), cur.2.clone())
                {
                    start_cur.inc();
                    cur = self
                        .inner
                        .get_ordered_edge(&edge_order, start_cur.to_usize().unwrap())
                        .unwrap();
                }
            }
        }

        // Walk the index backward until we find the last included key.
        {
            if end_cur > Id::Archived::zero() {
                let mut cur = self
                    .inner
                    .get_ordered_edge(&edge_order, end_cur.to_usize().unwrap() - 1)
                    .unwrap();
                while end_cur > Id::Archived::zero()
                    && end_bound < (cur.0.clone(), cur.1.clone(), cur.2.clone())
                {
                    end_cur.dec();
                    if end_cur == Id::Archived::zero() {
                        break;
                    }

                    cur = self
                        .inner
                        .get_ordered_edge(&edge_order, end_cur.to_usize().unwrap() - 1)
                        .unwrap();
                }
            }
        }

        MmapGraphRangeIter {
            inner: self.inner.clone(),
            edge_order,
            start_cur: start_cur.to_usize().unwrap(),
            end_cur: end_cur.to_usize().unwrap(),
            phantom: std::marker::PhantomData,
        }
    }

    /// Invokes lock on the underlying memmap2::Mmap to lock the pages of the data into RAM.
    pub fn lock(&self) -> std::io::Result<()> {
        self.inner.mmap.lock()
    }

    /// Invokes unlock on the underlying memmap2::Mmap to lock the pages of the data into RAM.
    pub fn unlock(&self) -> std::io::Result<()> {
        self.inner.mmap.unlock()
    }

    /// Get the properties assciated with the given node, if any.
    pub fn get_node_props(&self, node: Id::Archived) -> Option<&NodeProps::Archived> {
        self.inner.archive.node_props.get(&node)
    }

    /// Iterate through all nodes which have associated properties.
    pub fn iter_node_props(
        &self,
    ) -> rkyv::collections::btree_map::Iter<Id::Archived, <NodeProps as Archive>::Archived> {
        self.inner.archive.node_props.iter()
    }

    /// Iterator through all edges in SPO order.
    pub fn iter_edges_spo(
        &self,
    ) -> impl Iterator<
        Item = (
            Id::Archived,
            Id::Archived,
            Id::Archived,
            Option<&EdgeProps::Archived>,
        ),
    > {
        self.inner.archive.spo.iter().map(|(s, p, o, edge_idx)| {
            (
                s.clone(),
                p.clone(),
                o.clone(),
                if edge_idx.is_zero() {
                    None
                } else {
                    self.inner
                        .archive
                        .edge_props
                        .get(edge_idx.to_usize().unwrap() - 1)
                },
            )
        })
    }

    // Parallel iterator through all edges in SPO order.
    pub fn par_iter_edges_spo(
        &self,
    ) -> impl ParallelIterator<
        Item = (
            Id::Archived,
            Id::Archived,
            Id::Archived,
            Option<&EdgeProps::Archived>,
        ),
    >
    where
        NodeProps::Archived: Send + Sync,
        EdgeProps::Archived: Send + Sync,
    {
        self.inner
            .archive
            .spo
            .par_iter()
            .map(|(s, p, o, edge_idx)| {
                (
                    s.clone(),
                    p.clone(),
                    o.clone(),
                    if edge_idx.is_zero() {
                        None
                    } else {
                        self.inner
                            .archive
                            .edge_props
                            .get(edge_idx.to_usize().unwrap() - 1)
                    },
                )
            })
    }

    /// Iterator through edges in the specified range in SPO order.
    pub fn prefix_edges_spo(
        &self,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> MmapGraphRangeIter<'static, Id, NodeProps, EdgeProps> {
        self.make_range_iter(
            EdgeOrder::SPO,
            &self.inner.spo_index,
            <Id::Archived as NumCast>::from(self.inner.archive.spo.len()).unwrap(),
            prefix,
        )
    }

    /// Parallel iterator through all edges in the specified range in SPO order.
    pub fn par_prefix_edges_spo(
        &self,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> impl ParallelIterator<
        Item = (
            &'static Id::Archived,
            &'static Id::Archived,
            &'static Id::Archived,
            Option<&'static EdgeProps::Archived>,
        ),
    >
    where
        NodeProps::Archived: Send + Sync,
        EdgeProps::Archived: Send + Sync,
    {
        let range_iter = self.make_range_iter(
            EdgeOrder::SPO,
            &self.inner.spo_index,
            <Id::Archived as NumCast>::from(self.inner.archive.spo.len()).unwrap(),
            prefix,
        );

        self.inner
            .par_iter_range_spo(range_iter.start_cur, range_iter.end_cur)
    }

    /// Iterator through all edges in POS order.
    pub fn iter_edges_pos(
        &self,
    ) -> impl Iterator<
        Item = (
            Id::Archived,
            Id::Archived,
            Id::Archived,
            Option<&EdgeProps::Archived>,
        ),
    > {
        self.inner.archive.pos.iter().map(|(p, o, s, edge_idx)| {
            (
                s.clone(),
                p.clone(),
                o.clone(),
                if edge_idx.is_zero() {
                    None
                } else {
                    self.inner
                        .archive
                        .edge_props
                        .get(edge_idx.to_usize().unwrap() - 1)
                },
            )
        })
    }

    /// Parallel iterator through all edges in POS order.
    pub fn par_iter_edges_pos(
        &self,
    ) -> impl ParallelIterator<
        Item = (
            Id::Archived,
            Id::Archived,
            Id::Archived,
            Option<&EdgeProps::Archived>,
        ),
    >
    where
        NodeProps::Archived: Send + Sync,
        EdgeProps::Archived: Send + Sync,
    {
        self.inner
            .archive
            .pos
            .par_iter()
            .map(|(p, o, s, edge_idx)| {
                (
                    s.clone(),
                    p.clone(),
                    o.clone(),
                    if edge_idx.is_zero() {
                        None
                    } else {
                        self.inner
                            .archive
                            .edge_props
                            .get(edge_idx.to_usize().unwrap() - 1)
                    },
                )
            })
    }

    /// Iterator through all edges in the specified range in POS order.
    pub fn prefix_edges_pos(
        &self,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> MmapGraphRangeIter<'_, Id, NodeProps, EdgeProps> {
        self.make_range_iter(
            EdgeOrder::POS,
            &self.inner.pos_index,
            <Id::Archived as NumCast>::from(self.inner.archive.pos.len()).unwrap(),
            prefix,
        )
    }

    /// Parallel iterator through all edges in the specified range in POS order.
    pub fn par_prefix_edges_pos(
        &self,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> impl ParallelIterator<
        Item = (
            &'static Id::Archived,
            &'static Id::Archived,
            &'static Id::Archived,
            Option<&'static EdgeProps::Archived>,
        ),
    >
    where
        NodeProps::Archived: Send + Sync,
        EdgeProps::Archived: Send + Sync,
    {
        let range_iter = self.make_range_iter(
            EdgeOrder::POS,
            &self.inner.pos_index,
            <Id::Archived as NumCast>::from(self.inner.archive.pos.len()).unwrap(),
            prefix,
        );

        self.inner
            .par_iter_range_pos(range_iter.start_cur, range_iter.end_cur)
    }

    /// Iterator through all edges in OSP order.
    pub fn iter_edges_osp(
        &self,
    ) -> impl Iterator<
        Item = (
            Id::Archived,
            Id::Archived,
            Id::Archived,
            Option<&EdgeProps::Archived>,
        ),
    > {
        self.inner.archive.osp.iter().map(|(o, s, p, edge_idx)| {
            (
                s.clone(),
                p.clone(),
                o.clone(),
                if edge_idx.is_zero() {
                    None
                } else {
                    self.inner
                        .archive
                        .edge_props
                        .get(edge_idx.to_usize().unwrap() - 1)
                },
            )
        })
    }

    /// Parallel iterator through all edges in OSP order.
    pub fn par_iter_edges_osp(
        &self,
    ) -> impl ParallelIterator<
        Item = (
            Id::Archived,
            Id::Archived,
            Id::Archived,
            Option<&EdgeProps::Archived>,
        ),
    >
    where
        NodeProps::Archived: Send + Sync,
        EdgeProps::Archived: Send + Sync,
    {
        self.inner
            .archive
            .osp
            .par_iter()
            .map(|(o, s, p, edge_idx)| {
                (
                    s.clone(),
                    p.clone(),
                    o.clone(),
                    if edge_idx.is_zero() {
                        None
                    } else {
                        self.inner
                            .archive
                            .edge_props
                            .get(edge_idx.to_usize().unwrap() - 1)
                    },
                )
            })
    }

    /// Iterator through all edges in the specified range in OSP order.
    pub fn prefix_edges_osp(
        &self,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> MmapGraphRangeIter<Id, NodeProps, EdgeProps> {
        self.make_range_iter(
            EdgeOrder::OSP,
            &self.inner.osp_index,
            <Id::Archived as NumCast>::from(self.inner.archive.osp.len()).unwrap(),
            prefix,
        )
    }

    /// Parallel iterator through all edges in the specified range in OSP order.
    pub fn par_prefix_edges_osp(
        &self,
        prefix: (Id::Archived, Option<(Id::Archived, Option<Id::Archived>)>),
    ) -> impl ParallelIterator<
        Item = (
            &'static Id::Archived,
            &'static Id::Archived,
            &'static Id::Archived,
            Option<&'static EdgeProps::Archived>,
        ),
    >
    where
        NodeProps::Archived: Send + Sync,
        EdgeProps::Archived: Send + Sync,
    {
        let range_iter = self.make_range_iter(
            EdgeOrder::OSP,
            &self.inner.osp_index,
            <Id::Archived as NumCast>::from(self.inner.archive.osp.len()).unwrap(),
            prefix,
        );

        self.inner
            .par_iter_range_osp(range_iter.start_cur, range_iter.end_cur)
    }

    /// Load an MmapGraph from a specified file on disk.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Self::from_mmap(path, mmap)
    }

    /// Load an MmapGraph from an already existing [memmap2::Mmap]
    pub fn from_mmap<P: AsRef<Path>>(path: P, mmap: Mmap) -> Result<Self, Error> {
        let archive_ptr: *const [u8] = &mmap[..];

        let archive = unsafe {
            rkyv::archived_root::<MmapGraphBuilder<Id, NodeProps, EdgeProps>>(&*archive_ptr)
        };

        Ok(Self {
            inner: Arc::new(MmapGraphInner {
                path: path.as_ref().into(),
                mmap,
                archive: archive,
                spo_index: archive
                    .spo_index
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<BTreeMap<_, _>>(),
                pos_index: archive
                    .pos_index
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<BTreeMap<_, _>>(),
                osp_index: archive
                    .osp_index
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<BTreeMap<_, _>>(),
            }),
        })
    }
}

#[cfg(test)]
mod test {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use crate::{par_query, query};

    use super::{MmapGraph, MmapGraphBuilder};

    fn build_basic(dir: &tempdir::TempDir) -> MmapGraph<u16, u32, String> {
        let mut builder: MmapGraphBuilder<u16, u32, String> =
            MmapGraphBuilder::new(super::MmapGraphBuilderOptions {
                page_size: 4096,
                splits_per_page: 1,
            });

        builder.add_node();
        builder.add_node_with_props(1);
        builder.add_node_with_props(2);
        builder.add_node_with_props(3);

        builder.add_edge((0, 4, 8));
        builder.add_edge_with_props((1, 5, 6), "foo".to_string());
        builder.add_edge((2, 3, 7));

        builder.build(&dir.path().join("db")).unwrap()
    }

    fn build_generated(dir: &tempdir::TempDir) -> MmapGraph<u32, u32, u32> {
        let mut builder: MmapGraphBuilder<u32, u32, u32> =
            MmapGraphBuilder::new(super::MmapGraphBuilderOptions {
                page_size: 4096,
                splits_per_page: 8,
            });

        // Add the nodes
        for _ in 0..3000001 {
            builder.add_node();
        }

        // Add the edges with ranges which don't advance in lock step, but follow a pattern.
        //
        //   (1, 20000, 25001)
        //   (2, 19999, 25002)
        //   ...
        //   (10000, 10001, 25000)
        //
        for ((s, p), o) in (1..4000001)
            .zip((4000001..8000001).rev())
            .zip((8500001..12000001).chain(8000001..8500001))
        {
            builder.add_edge((s, p, o));
        }

        builder.build(&dir.path().join("db")).unwrap()
    }

    #[test]
    fn test_create() {
        let dir = tempdir::TempDir::new("test_create").unwrap();
        let mmap = build_basic(&dir);

        assert_eq!(
            mmap.iter_node_props()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<_>>(),
            [(1, 1), (2, 2), (3, 3)].to_vec()
        );

        // SPO
        assert_eq!(
            mmap.iter_edges_spo()
                .map(|(s, p, o, e)| (s, p, o, e.map(|e| e.to_string())))
                .collect::<Vec<_>>(),
            [
                (0, 4, 8, None),
                (1, 5, 6, Some("foo".to_string())),
                (2, 3, 7, None)
            ]
            .to_vec()
        );

        // POS
        assert_eq!(
            mmap.iter_edges_pos()
                .map(|(s, p, o, e)| (s, p, o, e.map(|e| e.to_string())))
                .collect::<Vec<_>>(),
            [
                (2, 3, 7, None),
                (0, 4, 8, None),
                (1, 5, 6, Some("foo".to_string())),
            ]
            .to_vec()
        );

        // OSP
        assert_eq!(
            mmap.iter_edges_osp()
                .map(|(s, p, o, e)| (s, p, o, e.map(|e| e.to_string())))
                .collect::<Vec<_>>(),
            [
                (1, 5, 6, Some("foo".to_string())),
                (2, 3, 7, None),
                (0, 4, 8, None)
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_query() {
        let dir = tempdir::TempDir::new("test_query").unwrap();

        let before_build = std::time::Instant::now();

        let mmap = build_generated(&dir);

        let after_build = std::time::Instant::now();

        let _ = (1..100000)
            .into_par_iter()
            .map(|sub| (sub, query!(mmap, sub -?-> ?)))
            .map(|(sub, edges)| (sub, edges.collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let after_query1 = std::time::Instant::now();

        let _ = (1..100000)
            .into_par_iter()
            .map(|sub| (sub, query!(mmap, sub -?-> ?)))
            .map(|(sub, edges)| (sub, edges.collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let after_query2 = std::time::Instant::now();

        let _: Vec<(u32, Vec<(&u32, &u32, &u32, Option<&u32>)>)> = (100001..200000)
            .into_par_iter()
            .map(|sub| (sub, par_query!(mmap, sub -?-> ?)))
            .map(|(sub, edges)| (sub, edges.collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let after_par_query1 = std::time::Instant::now();

        let _ = (100001..200000)
            .into_par_iter()
            .map(|sub| (sub, par_query!(mmap, sub -?-> ?).collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        let after_par_query2 = std::time::Instant::now();

        println!("Build Time: {:#?}", (after_build - before_build));
        println!("Query Time #1: {:#?}", (after_query1 - after_build));
        println!("Query Time #2: {:#?}", (after_query2 - after_query1));
        println!("ParQuery Time #1: {:#?}", (after_par_query1 - after_query2));
        println!(
            "ParQuery Time #2: {:#?}",
            (after_par_query2 - after_par_query1)
        );
    }
}
