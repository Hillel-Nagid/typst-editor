use crate::selection::MultiCursor;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub text: String,
    pub cursors: MultiCursor,
    pub dirty: bool,
}

pub trait EditCommand {
    fn before(&self) -> &BufferSnapshot;
    fn after(&self) -> &BufferSnapshot;
    fn merge_key(&self) -> Option<&str>;
    fn timestamp(&self) -> Instant;
    fn update_after(&mut self, snapshot: BufferSnapshot, timestamp: Instant);
}
