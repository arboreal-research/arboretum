use std::{collections::BTreeMap, path::PathBuf};

use memmap2::Mmap;
use num::{ToPrimitive, Zero};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    mmap::{builder::MmapGraphArchive, EdgeOrder},
    IdType, PropsType,
};

#[derive(Debug)]
pub(super) struct MmapGraphInner<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: std::fmt::Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: std::fmt::Debug,
{
    pub(super) path: PathBuf,

    // This is actually used, but due to unsafe the compiler doesn't know 🤫🤫🤫
    #[allow(dead_code)]
    pub(super) mmap: Mmap,

    // This is the data is available in the mmap. It's read only, so it's safe to share between threads.
    pub(super) archive: &'static MmapGraphArchive<Id, NodeProps, EdgeProps>,

    // Indexes. We currently need to deserialize these because ArchivedBTreeMap doesn't support range queries (yet).
    pub(super) spo_index: BTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,
    pub(super) pos_index: BTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,
    pub(super) osp_index: BTreeMap<(Id::Archived, Id::Archived, Id::Archived), Id::Archived>,
}

impl<Id, NodeProps, EdgeProps> MmapGraphInner<Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: std::fmt::Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: std::fmt::Debug,
{
    // Returns an edge by index in the order specified by edge_order.
    pub(crate) fn get_ordered_edge(
        &self,
        edge_order: &EdgeOrder,
        idx: usize,
    ) -> Option<(
        &'static Id::Archived,
        &'static Id::Archived,
        &'static Id::Archived,
        Option<&'static EdgeProps::Archived>,
    )> {
        match edge_order {
            EdgeOrder::SPO => self.archive.spo.get(idx),
            EdgeOrder::POS => self.archive.pos.get(idx),
            EdgeOrder::OSP => self.archive.osp.get(idx),
        }
        .map(|(a, b, c, e)| {
            (
                a,
                b,
                c,
                if e.is_zero() {
                    None
                } else {
                    self.archive.edge_props.get(e.to_usize().unwrap() - 1)
                },
            )
        })
    }

    // Returns an edge by index in the order specified by edge_order, but with the result fields reordered into SPO.
    pub(crate) fn get_edge(
        &self,
        edge_order: &EdgeOrder,
        idx: usize,
    ) -> Option<(
        &'static Id::Archived,
        &'static Id::Archived,
        &'static Id::Archived,
        Option<&'static EdgeProps::Archived>,
    )> {
        match edge_order {
            EdgeOrder::SPO => self.archive.spo.get(idx).map(|(s, p, o, e)| (s, p, o, e)),
            EdgeOrder::POS => self.archive.pos.get(idx).map(|(p, o, s, e)| (s, p, o, e)),
            EdgeOrder::OSP => self.archive.osp.get(idx).map(|(o, s, p, e)| (s, p, o, e)),
        }
        .map(|(s, p, o, e)| {
            (
                s,
                p,
                o,
                if e.is_zero() {
                    None
                } else {
                    self.archive.edge_props.get(e.to_usize().unwrap() - 1)
                },
            )
        })
    }

    pub(crate) fn par_iter_range_spo(
        &self,
        start_idx: usize,
        end_idx: usize,
    ) -> impl rayon::iter::ParallelIterator<
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
        self.archive.spo[start_idx..end_idx]
            .par_iter()
            .map(|(s, p, o, e)| {
                (
                    s,
                    p,
                    o,
                    if e.is_zero() {
                        None
                    } else {
                        self.archive.edge_props.get(e.to_usize().unwrap() - 1)
                    },
                )
            })
    }

    pub(crate) fn par_iter_range_pos(
        &self,
        start_idx: usize,
        end_idx: usize,
    ) -> impl rayon::iter::ParallelIterator<
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
        self.archive.spo[start_idx..end_idx]
            .par_iter()
            .map(|(p, o, s, e)| (s, p, o, e))
            .map(|(s, p, o, e)| {
                (
                    s,
                    p,
                    o,
                    if e.is_zero() {
                        None
                    } else {
                        self.archive.edge_props.get(e.to_usize().unwrap() - 1)
                    },
                )
            })
    }

    pub(crate) fn par_iter_range_osp(
        &self,
        start_idx: usize,
        end_idx: usize,
    ) -> impl rayon::iter::ParallelIterator<
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
        self.archive.spo[start_idx..end_idx]
            .par_iter()
            .map(|(o, s, p, e)| (s, p, o, e))
            .map(|(s, p, o, e)| {
                (
                    s,
                    p,
                    o,
                    if e.is_zero() {
                        None
                    } else {
                        self.archive.edge_props.get(e.to_usize().unwrap() - 1)
                    },
                )
            })
    }
}
