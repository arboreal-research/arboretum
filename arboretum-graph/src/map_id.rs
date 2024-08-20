pub(crate) trait MapId {
    fn map_id<F>(self, f: &F) -> Self
    where
        F: Fn(u64) -> u64 + Send + Sync;
}

impl MapId for (u64, u64, u64) {
    fn map_id<F>(self, f: &F) -> Self
    where
        F: Fn(u64) -> u64 + Send + Sync,
    {
        (f(self.0), f(self.1), f(self.2))
    }
}
