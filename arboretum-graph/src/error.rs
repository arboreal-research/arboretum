use std::{array::TryFromSliceError, sync::PoisonError};

#[derive(Debug)]
pub enum Error {
    RkivSerializationError(
        rkyv::ser::serializers::CompositeSerializerError<
            std::convert::Infallible,
            rkyv::ser::serializers::AllocScratchError,
            rkyv::ser::serializers::SharedSerializeMapError,
        >,
    ),

    IoError(std::io::Error),

    SledError(sled::Error),
    SledDbMissingNextNode,
    SledDbMissingNextEdge,

    SliceBug(TryFromSliceError),

    MalformedSubgraphConfig(u32),
    MalformedRootGraphNextSubgraphId,

    LockPoison,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl
    From<
        rkyv::ser::serializers::CompositeSerializerError<
            std::convert::Infallible,
            rkyv::ser::serializers::AllocScratchError,
            rkyv::ser::serializers::SharedSerializeMapError,
        >,
    > for Error
{
    fn from(
        e: rkyv::ser::serializers::CompositeSerializerError<
            std::convert::Infallible,
            rkyv::ser::serializers::AllocScratchError,
            rkyv::ser::serializers::SharedSerializeMapError,
        >,
    ) -> Self {
        Error::RkivSerializationError(e)
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Error::SledError(e)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Error::SliceBug(e)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::LockPoison
    }
}
