use std::{fmt::Debug, num::TryFromIntError};

use crate::{split_u64, types::IdType};

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

impl<Id: IdType> Prefix<Id> {
    pub fn apply<F: FnMut(Id)>(&self, mut f: F) {
        match self {
            Prefix::One(a) => f(a.clone()),
            Prefix::Two(a, b) => {
                f(a.clone());
                f(b.clone());
            }
            Prefix::Three(a, b, c) => {
                f(a.clone());
                f(b.clone());
                f(c.clone());
            }
        }
    }
}

impl<Id: IdType> AsRef<Prefix<Id>> for &Prefix<Id> {
    fn as_ref(&self) -> &Prefix<Id> {
        self
    }
}

impl From<Prefix<u64>> for Prefix<u16> {
    fn from(value: Prefix<u64>) -> Self {
        match value {
            Prefix::One(a) => Prefix::One(split_u64(a).1 as u16),
            Prefix::Two(a, b) => Prefix::Two(split_u64(a).1 as u16, split_u64(b).1 as u16),
            Prefix::Three(a, b, c) => Prefix::Three(
                split_u64(a).1 as u16,
                split_u64(b).1 as u16,
                split_u64(c).1 as u16,
            ),
        }
    }
}

impl From<Prefix<u64>> for Prefix<u32> {
    fn from(value: Prefix<u64>) -> Self {
        match value {
            Prefix::One(a) => Prefix::One(split_u64(a).1 as u32),
            Prefix::Two(a, b) => Prefix::Two(split_u64(a).1 as u32, split_u64(b).1 as u32),
            Prefix::Three(a, b, c) => Prefix::Three(
                split_u64(a).1 as u32,
                split_u64(b).1 as u32,
                split_u64(c).1 as u32,
            ),
        }
    }
}
