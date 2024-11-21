use rkyv::Archive;

/// Data which can be stored as props for a node or an edge.
///
/// See also [ArchivedValue]
#[derive(
    Clone, Debug, Archive, rkyv::Serialize, rkyv::Deserialize, serde::Serialize, serde::Deserialize,
)]
#[archive_attr(derive(Debug))]
pub enum Value {
    Unsigned(u64),
    Signed(i64),
    Double(f64),
    String(String),
}
