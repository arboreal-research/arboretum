use std::{
    collections::{BinaryHeap, HashMap},
    path::PathBuf,
    sync::Arc,
    time::Instant,
};

use crate::subgraph_entry::SubgraphEntry;

/// The strategy and parameters used to cache subgraphs.
#[derive(Clone, Debug)]
pub enum SubgraphCacheStrategy {
    /// Least-Recently-Used
    LRU { max_usage_bytes: usize },
}

pub(crate) trait SubgraphCache: Send + Sync {
    // Marks entry as having been used at std::time::Instant::now().
    fn notify_used(&mut self, entry: SubgraphEntry) -> anyhow::Result<()>;

    // Returns the list of SubgraphEntries which are in use.
    // fn entries_in_use(&mut self) -> Vec<SubgraphEntry>;
}

/// A [SubgraphEntry] along with the last moment when it was used.
#[derive(PartialEq, Eq)]
struct LruSubgraphCacheEntry {
    age: Instant,
    entry: SubgraphEntry,
}

// Inverted comparison orders to get min-heap.
impl PartialOrd for LruSubgraphCacheEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.age.partial_cmp(&self.age)
    }
}

impl Ord for LruSubgraphCacheEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.age.cmp(&self.age)
    }
}

impl LruSubgraphCacheEntry {
    pub fn new(entry: SubgraphEntry) -> Self {
        Self {
            age: Instant::now(),
            entry,
        }
    }
}

/// An LRU cache of subgraphs
pub(crate) struct LruSubgraphCache {
    // The sum of memory used by all active subgraph entries.
    cur_usage: usize,

    // The maximum allowed sum of memory used by all active subgraph entries.
    max_usage: usize,

    // A lookup table which associates a [SubgraphEntry] to the [LruSubgraphCacheEntry]
    // which maintains the last time it was accessed.
    entries: HashMap<SubgraphEntry, Arc<LruSubgraphCacheEntry>>,

    // The heap which is used to determine the least recently used, active subgraph.
    cache: BinaryHeap<Arc<LruSubgraphCacheEntry>>,

    // The path to the directory of subgraphs.
    subgraphs_path: PathBuf,
}

impl LruSubgraphCache {
    pub fn new(subgraphs_path: PathBuf, max_usage: usize) -> Self {
        Self {
            cur_usage: 0,
            max_usage,
            entries: HashMap::new(),
            cache: BinaryHeap::new(),
            subgraphs_path,
        }
    }
}

impl SubgraphCache for LruSubgraphCache {
    /// Notifies the cache that a particular subgraph entry has been used by some operation
    ///
    /// The cache may respond by evicting other entries to maintain the configured amount of available system memory.
    fn notify_used(&mut self, entry: SubgraphEntry) -> anyhow::Result<()> {
        let entry_size = entry.size(self.subgraphs_path.as_path())?;

        let mut new_used_size = self.cur_usage + entry_size;

        // Evict until we have enough buffer space and we're under the maximum usage.
        while new_used_size > self.max_usage {
            let to_evict = self.cache.pop();
            if let Some(to_evict) = to_evict {
                // Perform the eviction.
                to_evict.entry.evict()?;

                // Update the condition.
                new_used_size =
                    new_used_size - to_evict.entry.size(self.subgraphs_path.as_path())?;
            } else {
                // This shouldn't happen, but if it does then we break and proceed to inevitable OOM DOOM below.
                break;
            }
        }

        self.cur_usage = new_used_size;

        let new_entry = Arc::new(LruSubgraphCacheEntry::new(entry.clone()));
        let existing_entry = self.entries.insert(entry, new_entry.clone());
        if let Some(existing_entry) = existing_entry {
            self.cache.retain(|e| *e != existing_entry);
        }
        self.cache.push(new_entry);

        Ok(())
    }
}
