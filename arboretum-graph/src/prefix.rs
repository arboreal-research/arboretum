use std::{fmt::Debug, num::TryFromIntError};

use crate::IdType;

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
