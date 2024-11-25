use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PathNode {
    NodeHole { name: Option<String> },
    SubgraphHole { name: Option<String> },
    Id(u64),
    Name(String),
}

impl PathNode {
    pub fn nameless_node() -> Self {
        PathNode::NodeHole { name: None }
    }

    pub fn nameless_subgraph() -> Self {
        PathNode::SubgraphHole { name: None }
    }
}

impl From<u64> for PathNode {
    fn from(value: u64) -> Self {
        PathNode::Id(value)
    }
}

impl From<&'_ str> for PathNode {
    fn from(value: &'_ str) -> Self {
        PathNode::Name(value.to_string())
    }
}

impl From<String> for PathNode {
    fn from(value: String) -> Self {
        PathNode::Name(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Path(Vec<PathNode>);

impl Path {
    pub fn new() -> Self {
        Path(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_trivial(&self) -> bool {
        self.0.len() == 1
    }

    pub fn is_query(&self) -> bool {
        self.0.iter().any(|e| match e {
            PathNode::NodeHole { .. } => true,
            PathNode::SubgraphHole { .. } => true,
            _ => false,
        })
    }

    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, PathNode> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, PathNode> {
        self.0.iter_mut()
    }
}

impl FromIterator<PathNode> for Path {
    fn from_iter<T: IntoIterator<Item = PathNode>>(iter: T) -> Self {
        Path(Vec::from_iter(iter))
    }
}

mod test {
    use super::*;

    #[test]
    pub fn test_is_empty() {
        let path = Path::new();
        assert!(path.is_empty());

        let path = Path::from_iter([1.into()]);
        assert!(!path.is_empty());

        let path = Path::from_iter([1.into(), "/foo/bar/".into(), 3.into()]);
        assert!(!path.is_empty());
    }

    #[test]
    pub fn test_is_trivial() {
        let path = Path::new();
        assert!(!path.is_trivial());

        let path = Path::from_iter([1.into()]);
        assert!(path.is_trivial());

        let path = Path::from_iter([1.into(), "/foo/bar/".into(), 3.into()]);
        assert!(!path.is_trivial());
    }

    #[test]
    pub fn test_is_query() {
        let path = Path::new();
        assert!(!path.is_query());

        let path = Path::from_iter([1.into()]);
        assert!(!path.is_query());

        let path = Path::from_iter([1.into(), "/foo/bar/".into(), 3.into()]);
        assert!(!path.is_query());

        let path = Path::from_iter([1.into(), PathNode::nameless_node(), 3.into()]);
        assert!(path.is_query());

        let path = Path::from_iter([1.into(), PathNode::nameless_subgraph(), 3.into()]);
        assert!(path.is_query());
    }
}
