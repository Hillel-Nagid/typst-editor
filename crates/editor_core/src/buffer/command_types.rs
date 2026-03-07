use crate::buffer::command::{BufferSnapshot, EditCommand};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct SnapshotCommand {
    pub before: BufferSnapshot,
    pub after: BufferSnapshot,
    pub merge_key: Option<String>,
    pub timestamp: Instant,
}

impl SnapshotCommand {
    pub fn new(
        before: BufferSnapshot,
        after: BufferSnapshot,
        merge_key: Option<String>,
        timestamp: Instant,
    ) -> Self {
        Self {
            before,
            after,
            merge_key,
            timestamp,
        }
    }
}

impl EditCommand for SnapshotCommand {
    fn before(&self) -> &BufferSnapshot {
        &self.before
    }

    fn after(&self) -> &BufferSnapshot {
        &self.after
    }

    fn merge_key(&self) -> Option<&str> {
        self.merge_key.as_deref()
    }

    fn timestamp(&self) -> Instant {
        self.timestamp
    }

    fn update_after(&mut self, snapshot: BufferSnapshot, timestamp: Instant) {
        self.after = snapshot;
        self.timestamp = timestamp;
    }
}
