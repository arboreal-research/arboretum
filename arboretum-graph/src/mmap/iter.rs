use std::sync::Arc;

use crate::{
    mmap::{EdgeOrder, MmapGraphInner},
    IdType, PropsType,
};

/// Range Iterator for edges in a [MmapGraph][crate::MmapGraph].
pub struct MmapGraphRangeIter<'a, Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: std::fmt::Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: std::fmt::Debug,
{
    pub(super) inner: Arc<MmapGraphInner<Id, NodeProps, EdgeProps>>,
    pub(super) edge_order: EdgeOrder,
    pub(super) start_cur: usize,
    pub(super) end_cur: usize,
    pub(super) phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, Id, NodeProps, EdgeProps> Iterator for MmapGraphRangeIter<'a, Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: std::fmt::Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: std::fmt::Debug,
{
    type Item = (
        &'a Id::Archived,
        &'a Id::Archived,
        &'a Id::Archived,
        Option<&'a EdgeProps::Archived>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_cur < self.end_cur {
            let result = self.inner.get_edge(&self.edge_order, self.start_cur);
            self.start_cur += 1;
            result
        } else {
            None
        }
    }
}

impl<'a, Id, NodeProps, EdgeProps> DoubleEndedIterator
    for MmapGraphRangeIter<'a, Id, NodeProps, EdgeProps>
where
    Id: IdType,
    Id::Archived: IdType,
    NodeProps: PropsType,
    NodeProps::Archived: std::fmt::Debug,
    EdgeProps: PropsType,
    EdgeProps::Archived: std::fmt::Debug,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end_cur > self.start_cur {
            self.end_cur -= 1;
            let result = self.inner.get_edge(&self.edge_order, self.end_cur);
            result
        } else {
            None
        }
    }
}
