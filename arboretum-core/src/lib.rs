pub mod constant;
mod domain;
mod graph_buffer;
mod map_id;
mod prefix;
mod types;
mod value;

pub use domain::Domain;
pub use graph_buffer::GraphBuffer;
pub use prefix::Prefix;
pub use types::{IdType, PropsType};
pub use value::{ArchivedValue, Value, ValueResolver};

pub fn split_u64(value: u64) -> (u32, u32) {
    let high = (value >> 32) as u32;
    let low = value as u32;
    (high, low)
}

pub fn merge_u64(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) + (low as u64)
}
