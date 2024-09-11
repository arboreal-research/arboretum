use std::{array::TryFromSliceError, num::TryFromIntError, sync::PoisonError};

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

    InvalidPrefix,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RkivSerializationError(_) => f.write_str("RkivSerializationError"),
            Error::IoError(_) => f.write_str("IoError"),
            Error::SledError(_) => f.write_str("SledError"),
            Error::SledDbMissingNextNode => f.write_str("SledDbMissingNextNode"),
            Error::SledDbMissingNextEdge => f.write_str("SledDbMissingNextEdge"),
            Error::SliceBug(_) => f.write_str("SliceBug"),
            Error::MalformedSubgraphConfig(_) => f.write_str("MalformedSubgraphConfig"),
            Error::MalformedRootGraphNextSubgraphId => f.write_str("MalformedRootGraphNextSubgraphId"),
            Error::LockPoison => f.write_str("LockPoison"),
            Error::InvalidPrefix => f.write_str("InvalidPrefix"),
        }
    }
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

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Error::InvalidPrefix
    }
}
