use std::{fmt::Debug, num::TryFromIntError};

use crate::IdType;

/// Ordered edge components.
///
/// These can be in SPO, POS, or OSP order.
///
/// For example:
///   * When querying edges in Subject-Predicate-Object order,
///     then Two(3, 4) is an edge where the subject is 3 and the predicate is 4.
///
///   * When querying edges in Object-Subject-Predicate order,
///     then Three(7, 8, 9) is an edge where the subject is 8, the predicate is 9, and the object is 7.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Prefix<Id: IdType> {
    One(Id),
    Two(Id, Id),
    Three(Id, Id, Id),
}

impl<Id: IdType> AsRef<Prefix<Id>> for &Prefix<Id> {
    fn as_ref(&self) -> &Prefix<Id> {
        self
    }
}

impl TryFrom<Prefix<u64>> for Prefix<u16> {
    type Error = TryFromIntError;

    fn try_from(value: Prefix<u64>) -> Result<Self, Self::Error> {
        match value {
            Prefix::One(a) => Ok(Prefix::One(a.try_into()?)),
            Prefix::Two(a, b) => Ok(Prefix::Two(a.try_into()?, b.try_into()?)),
            Prefix::Three(a, b, c) => {
                Ok(Prefix::Three(a.try_into()?, b.try_into()?, c.try_into()?))
            }
        }
    }
}

impl TryFrom<Prefix<u64>> for Prefix<u32> {
    type Error = TryFromIntError;

    fn try_from(value: Prefix<u64>) -> Result<Self, Self::Error> {
        match value {
            Prefix::One(a) => Ok(Prefix::One(a.try_into()?)),
            Prefix::Two(a, b) => Ok(Prefix::Two(a.try_into()?, b.try_into()?)),
            Prefix::Three(a, b, c) => {
                Ok(Prefix::Three(a.try_into()?, b.try_into()?, c.try_into()?))
            }
        }
    }
}
