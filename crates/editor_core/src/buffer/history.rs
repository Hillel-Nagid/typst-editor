use crate::buffer::command::EditCommand;
use crate::buffer::command_types::SnapshotCommand;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct History {
    undo_stack: Vec<SnapshotCommand>,
    redo_stack: Vec<SnapshotCommand>,
    max_size: usize,
    last_save_index: Option<usize>,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: max_size.max(1),
            last_save_index: Some(0),
        }
    }

    pub fn record(&mut self, command: SnapshotCommand) {
        let merged = if let Some(last) = self.undo_stack.last_mut() {
            let can_merge = last.merge_key() == command.merge_key()
                && last.merge_key().is_some()
                && command.timestamp().duration_since(last.timestamp()) <= Duration::from_secs(1);
            if can_merge {
                last.update_after(command.after.clone(), command.timestamp());
                true
            } else {
                false
            }
        } else {
            false
        };

        if !merged {
            self.undo_stack.push(command);
            if self.undo_stack.len() > self.max_size {
                self.undo_stack.remove(0);
                self.last_save_index = self.last_save_index.and_then(|idx| idx.checked_sub(1));
            }
        }

        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<SnapshotCommand> {
        let command = self.undo_stack.pop()?;
        self.redo_stack.push(command.clone());
        Some(command)
    }

    pub fn redo(&mut self) -> Option<SnapshotCommand> {
        let command = self.redo_stack.pop()?;
        self.undo_stack.push(command.clone());
        Some(command)
    }

    pub fn mark_saved(&mut self) {
        self.last_save_index = Some(self.undo_stack.len());
    }

    pub fn is_at_save_point(&self) -> bool {
        self.last_save_index == Some(self.undo_stack.len())
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.last_save_index = Some(0);
    }

    pub fn now() -> Instant {
        Instant::now()
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new(1000)
    }
}
