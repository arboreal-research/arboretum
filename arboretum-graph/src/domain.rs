use crate::map_id::MapId;

/// The set of subgraph ids which are present within a particular subgraph.
///
/// Each subgraph within the overall graph can only contain edges which are part of its domain.
///
/// For example,
///   * If the domain of graph #3 is Single(3), then it can only contain edges between nodes
///     in graph #3 using edges in graph #3.
///
///   * If the domain of graph #3 is Double(4, 5), then it can only contain edges between nodes
///     in graph #4 or #5 using edges in graph #4 or #5.
///
///   * If the domain is Triple(6, 7, 8), then it can only contain edges between nodes in graph
///     #6, #7, or #8 using edges in graph #6, #7, #8.
///
/// In practice the edges used will belong to a graph which represents a schema, so the typical
/// setup is:
///    * Graph #N:   Single(N) Contains only nodes
///    * Graph #N+1: Double(S, N) Contains edges between nodes in N using schema edges in S.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Domain {
    Single(u32),
    Double(u32, u32),
    Triple(u32, u32, u32),
}

impl Domain {
    pub fn from_iter<I>(parts: I) -> Option<Domain>
    where
        I: IntoIterator<Item = u32>,
    {
        let mut parts = parts.into_iter();
        let a = parts.next();
        let b = parts.next();
        let c = parts.next();

        if parts.next().is_some() {
            return None;
        }

        fn handle_double(q: u32, r: u32) -> Domain {
            if q < r {
                Domain::Double(q, r)
            } else if r < q {
                Domain::Double(r, q)
            } else {
                Domain::Single(q)
            }
        }

        Some(match (a, b, c) {
            (Some(a), None, None) => Domain::Single(a),
            (Some(a), Some(b), None) => handle_double(a, b),
            (Some(mut a), Some(mut b), Some(mut c)) => {
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }
                if b > c {
                    std::mem::swap(&mut b, &mut c);
                }
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }

                match (a == b, b == c) {
                    (true, true) => Domain::Single(a),
                    (true, false) => Domain::Double(a, c),
                    (false, true) => Domain::Double(a, b),
                    (false, false) => Domain::Triple(a, b, c),
                }
            }
            _ => return None,
        })
    }
}

impl MapId for Domain {
    fn map_id<F>(self, f: &F) -> Self
    where
        F: Fn(u64) -> u64 + Send + Sync,
    {
        match self {
            Domain::Single(a) => Domain::Single((f((a as u64) << 32) >> 32) as u32),
            Domain::Double(a, b) => Domain::from_iter([
                (f((a as u64) << 32) >> 32) as u32,
                (f((b as u64) << 32) >> 32) as u32,
            ])
            .unwrap(),
            Domain::Triple(a, b, c) => Domain::from_iter([
                (f((a as u64) << 32) >> 32) as u32,
                (f((b as u64) << 32) >> 32) as u32,
                (f((c as u64) << 32) >> 32) as u32,
            ])
            .unwrap(),
        }
    }
}
