use num::{
    traits::{FromBytes, ToBytes},
    Bounded, CheckedAdd, Integer, NumCast, Unsigned,
};
use rkyv::{
    ser::serializers::{
        AlignedSerializer, AllocScratch, CompositeSerializer, FallbackScratch, HeapScratch,
        SharedSerializeMap,
    },
    AlignedVec, Archive, Serialize,
};

/// Trait for types which can be used as NodeProps or EdgeProps.
pub trait PropsType:
    Archive
    + std::fmt::Debug
    + Serialize<
        CompositeSerializer<
            AlignedSerializer<AlignedVec>,
            FallbackScratch<HeapScratch<1024>, AllocScratch>,
            SharedSerializeMap,
        >,
    > + Send
    + Sync
    + 'static
{
}

impl<T> PropsType for T where
    T: Archive
        + std::fmt::Debug
        + Serialize<
            CompositeSerializer<
                AlignedSerializer<AlignedVec>,
                FallbackScratch<HeapScratch<1024>, AllocScratch>,
                SharedSerializeMap,
            >,
        > + Send
        + Sync
        + 'static
{
}

/// Trait for types which can be used as identifiers inside the graph.
pub trait IdType:
    Clone
    + std::fmt::Debug
    + FromBytes
    + ToBytes
    + NumCast
    + Sized
    + Unsigned
    + Integer
    + Archive
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + CheckedAdd
    + Bounded
    + Serialize<
        CompositeSerializer<
            AlignedSerializer<AlignedVec>,
            FallbackScratch<HeapScratch<1024>, AllocScratch>,
            SharedSerializeMap,
        >,
    > + Send
    + Sync
    + std::hash::Hash
    + 'static
{
}
impl<T> IdType for T
where
    T: Clone
        + std::fmt::Debug
        + FromBytes
        + ToBytes
        + NumCast
        + Sized
        + Unsigned
        + Integer
        + Archive
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + CheckedAdd
        + Bounded
        + Serialize<
            CompositeSerializer<
                AlignedSerializer<AlignedVec>,
                FallbackScratch<HeapScratch<1024>, AllocScratch>,
                SharedSerializeMap,
            >,
        > + Send
        + Sync
        + std::hash::Hash
        + 'static,
    T::Archived: Clone
        + FromBytes
        + ToBytes
        + NumCast
        + Sized
        + Unsigned
        + Integer
        + Archive
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + CheckedAdd
        + Bounded
        + Serialize<
            CompositeSerializer<
                AlignedSerializer<AlignedVec>,
                FallbackScratch<HeapScratch<1024>, AllocScratch>,
                SharedSerializeMap,
            >,
        > + 'static,
{
}
