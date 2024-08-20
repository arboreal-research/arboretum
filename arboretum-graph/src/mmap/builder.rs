use std::{
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
    hash::Hash,
    path::Path,
};

use memmap2::MmapMut;
use num::{Integer, Unsigned};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rkyv::{
    collections::{btree_map::BTreeMapResolver, ArchivedBTreeMap},
    out_field,
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    Archive, Fallible, Resolver, Serialize,
};

use crate::{error::Error, mmap::MmapGraph, IdType, PropsType};

/// Options which are used by [MmapGraphBuilder].
///
/// Use [Default::default()] for reasonable defaults.
pub struct MmapGraphBuilderOptions {
    // The size of the pages in bytes (should be 4K on linux by default).
    pub page_size: usize,

    // How many keys to index per page.
    pub splits_per_page: usize,
}

impl Default for MmapGraphBuilderOptions {
    fn default() -> Self {
        Self {
            page_size: 4096,
            splits_per_page: 4,
        }
    }
}

/// Builder for an [MmapGraph].
pub struct MmapGraphBuilder<Id, NodeProps, EdgeProps>
where
    Id: Clone + Sized + Unsigned + Integer + Hash,
{
    pub(super) opts: MmapGraphBuilderOptions,
    pub(super) node_props: BTreeMap<Id, NodeProps>,
    pub(super) next_node_id: Id,
    pub(super) edge_props: Vec<EdgeProps>,
    pub(super) edges: HashMap<(Id, Id, Id), Id>,
}

impl<Id, NodeProps, EdgeProps> MmapGraphBuilder<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: std::fmt::Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: std::fmt::Debug,
{
    pub fn new(opts: MmapGraphBuilderOptions) -> Self {
        Self {
            opts,
            node_props: BTreeMap::new(),
            next_node_id: Id::zero(),
            edge_props: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self) -> Id {
        let node_id = self.next_node_id.clone();
        self.next_node_id.inc();
        node_id
    }

    pub fn add_node_with_props(&mut self, props: NodeProps) -> Id {
        let node_id = self.add_node();
        self.node_props.insert(node_id.clone(), props);
        node_id
    }

    pub fn set_node_with_props(&mut self, id: Id, props: NodeProps) {
        self.node_props.insert(id, props);
    }

    pub fn add_edge(&mut self, triple: (Id, Id, Id)) {
        self.edges.insert(triple, Id::zero());
    }

    pub fn add_edge_with_props(&mut self, triple: (Id, Id, Id), props: EdgeProps) {
        self.edge_props.push(props);
        self.edges
            .insert(triple, Id::from(self.edge_props.len()).unwrap());
    }

    pub fn build<P: AsRef<Path>>(
        self,
        path: P,
    ) -> Result<MmapGraph<Id, NodeProps, EdgeProps>, Error> {
        let bytes = rkyv::to_bytes::<_, 1024>(&self)?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())?;

        file.set_len(bytes.len() as u64)?;

        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        mmap[..].copy_from_slice(&bytes);

        Ok(MmapGraph::from_mmap(path, mmap.make_read_only()?)?)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MmapGraphArchive<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    EdgeProps: PropsType,
{
    pub(crate) node_props: ArchivedBTreeMap<Id::Archived, NodeProps::Archived>,

    pub(crate) edge_props: ArchivedVec<EdgeProps::Archived>,

    pub(crate) spo: ArchivedVec<(Id::Archived, Id::Archived, Id::Archived, Id::Archived)>,
    pub(crate) spo_index:
        ArchivedBTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,

    pub(crate) pos: ArchivedVec<(Id::Archived, Id::Archived, Id::Archived, Id::Archived)>,
    pub(crate) pos_index:
        ArchivedBTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,

    pub(crate) osp: ArchivedVec<(Id::Archived, Id::Archived, Id::Archived, Id::Archived)>,
    pub(crate) osp_index:
        ArchivedBTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,
}

#[repr(C)]
pub struct MmapGraphResolver<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    EdgeProps: PropsType,
{
    pub(crate) node_props: Resolver<BTreeMap<Id, NodeProps>>,
    pub(crate) edge_props: Resolver<Vec<EdgeProps>>,
    pub(crate) spo: VecResolver,
    pub(crate) spo_index: BTreeMapResolver,
    pub(crate) pos: VecResolver,
    pub(crate) pos_index: BTreeMapResolver,
    pub(crate) osp: VecResolver,
    pub(crate) osp_index: BTreeMapResolver,

    pub(crate) spo_edge_data: Vec<(Id, Id, Id, Id)>,
    pub(crate) spo_index_data: BTreeMap<(Id, Id, Id), Id>,
    pub(crate) pos_edge_data: Vec<(Id, Id, Id, Id)>,
    pub(crate) pos_index_data: BTreeMap<(Id, Id, Id), Id>,
    pub(crate) osp_edge_data: Vec<(Id, Id, Id, Id)>,
    pub(crate) osp_index_data: BTreeMap<(Id, Id, Id), Id>,
}

impl<Id, NodeProps, EdgeProps> Archive for MmapGraphBuilder<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    EdgeProps: PropsType,
{
    type Archived = MmapGraphArchive<Id, NodeProps, EdgeProps>;
    type Resolver = MmapGraphResolver<Id, NodeProps, EdgeProps>;

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.node_props);
        Archive::resolve(&self.node_props, pos + fp, resolver.node_props, fo);

        let (fp, fo) = out_field!(out.edge_props);
        Archive::resolve(&self.edge_props, pos + fp, resolver.edge_props, fo);

        // SPO
        let (fp, fo) = out_field!(out.spo);
        Archive::resolve(&resolver.spo_edge_data, pos + fp, resolver.spo, fo);

        let (fp, fo) = out_field!(out.spo_index);
        Archive::resolve(&resolver.spo_index_data, pos + fp, resolver.spo_index, fo);

        // POS
        let (fp, fo) = out_field!(out.pos);
        Archive::resolve(&resolver.pos_edge_data, pos + fp, resolver.pos, fo);

        let (fp, fo) = out_field!(out.pos_index);
        Archive::resolve(&resolver.pos_index_data, pos + fp, resolver.pos_index, fo);

        // OSP
        let (fp, fo) = out_field!(out.osp);
        Archive::resolve(&resolver.osp_edge_data, pos + fp, resolver.osp, fo);

        let (fp, fo) = out_field!(out.osp_index);
        Archive::resolve(&resolver.osp_index_data, pos + fp, resolver.osp_index, fo);
    }
}

impl<S: Fallible + ?Sized + Serializer + ScratchSpace, Id, NodeProps, EdgeProps> Serialize<S>
    for MmapGraphBuilder<Id, NodeProps, EdgeProps>
where
    Id: IdType + Serialize<S>,
    Id::Archived: IdType + Serialize<S>,
    NodeProps: PropsType + Serialize<S>,
    EdgeProps: PropsType + Serialize<S>,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let edges_per_page =
            self.opts.page_size / self.opts.splits_per_page / (4 * size_of::<Id>());

        let work: [Box<
            dyn Fn() -> (BTreeMap<(Id, Id, Id), Id>, Vec<(Id, Id, Id, Id)>) + Send + Sync,
        >; 3] = [
            Box::new(|| {
                let mut spo_edge_data = self
                    .edges
                    .par_iter()
                    .map(|((sub, pred, obj), edge_idx)| {
                        (sub.clone(), pred.clone(), obj.clone(), edge_idx.clone())
                    })
                    .collect::<Vec<_>>();
                spo_edge_data.sort();

                let mut spo_index_data = BTreeMap::<(Id, Id, Id), Id>::new();
                for (i, edge) in spo_edge_data.iter().enumerate().step_by(edges_per_page) {
                    spo_index_data.insert(
                        (edge.0.clone(), edge.1.clone(), edge.2.clone()),
                        Id::from(i).unwrap(),
                    );
                }
                (spo_index_data, spo_edge_data)
            }),
            Box::new(|| {
                let mut pos_index_data = BTreeMap::<(Id, Id, Id), Id>::new();
                let mut pos_edge_data = self
                    .edges
                    .par_iter()
                    .map(|((s, p, o), e)| (p.clone(), o.clone(), s.clone(), e.clone()))
                    .collect::<Vec<_>>();
                pos_edge_data.sort();
                for (i, edge) in pos_edge_data.iter().enumerate().step_by(edges_per_page) {
                    pos_index_data.insert(
                        (edge.0.clone(), edge.1.clone(), edge.2.clone()),
                        Id::from(i).unwrap(),
                    );
                }
                (pos_index_data, pos_edge_data)
            }),
            Box::new(|| {
                let mut osp_index_data = BTreeMap::<(Id, Id, Id), Id>::new();
                let mut osp_edge_data = self
                    .edges
                    .par_iter()
                    .map(|((s, p, o), e)| (o.clone(), s.clone(), p.clone(), e.clone()))
                    .collect::<Vec<_>>();
                osp_edge_data.sort();

                for (i, edge) in osp_edge_data.iter().enumerate().step_by(edges_per_page) {
                    osp_index_data.insert(
                        (edge.0.clone(), edge.1.clone(), edge.2.clone()),
                        Id::from(i).unwrap(),
                    );
                }
                (osp_index_data, osp_edge_data)
            }),
        ];

        let mut work = work
            .into_par_iter()
            .map(|f| f())
            .collect::<Vec<_>>()
            .into_iter();

        let (spo_index_data, spo_edge_data) = work.next().unwrap();
        let (pos_index_data, pos_edge_data) = work.next().unwrap();
        let (osp_index_data, osp_edge_data) = work.next().unwrap();

        Ok(MmapGraphResolver {
            node_props: Serialize::<S>::serialize(&self.node_props, serializer)?,
            edge_props: Serialize::<S>::serialize(&self.edge_props, serializer)?,

            spo: Serialize::<S>::serialize(&spo_edge_data, serializer)?,
            spo_index: Serialize::<S>::serialize(&spo_index_data, serializer)?,

            pos: Serialize::<S>::serialize(&pos_edge_data, serializer)?,
            pos_index: Serialize::<S>::serialize(&pos_index_data, serializer)?,

            osp: Serialize::<S>::serialize(&osp_edge_data, serializer)?,
            osp_index: Serialize::<S>::serialize(&osp_index_data, serializer)?,

            spo_edge_data,
            pos_edge_data,
            osp_edge_data,

            spo_index_data,
            pos_index_data,
            osp_index_data,
        })
    }
}
