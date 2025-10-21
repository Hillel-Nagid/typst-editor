//! LSP request management

use std::collections::HashMap;
use std::time::{ Duration, Instant };

/// Priority level for requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// Pending request
#[derive(Debug)]
pub struct PendingRequest {
    pub id: i64,
    pub method: String,
    pub priority: Priority,
    pub sent_at: Instant,
    pub timeout: Duration,
}

/// Request manager
pub struct RequestManager {
    pending: HashMap<i64, PendingRequest>,
    next_id: i64,
}

impl RequestManager {
    pub fn new() -> Self {
        Self {
            pending: HashMap::new(),
            next_id: 1,
        }
    }

    /// Create a new request
    pub fn create_request(&mut self, method: String, priority: Priority) -> i64 {
        let id = self.next_id;
        self.next_id += 1;

        let request = PendingRequest {
            id,
            method,
            priority,
            sent_at: Instant::now(),
            timeout: Duration::from_secs(5),
        };

        self.pending.insert(id, request);
        id
    }

    /// Mark request as completed
    pub fn complete_request(&mut self, id: i64) -> Option<PendingRequest> {
        self.pending.remove(&id)
    }

    /// Cancel a request
    pub fn cancel_request(&mut self, id: i64) -> bool {
        self.pending.remove(&id).is_some()
    }

    /// Get pending request
    pub fn get_pending(&self, id: i64) -> Option<&PendingRequest> {
        self.pending.get(&id)
    }

    /// Check for timed out requests
    pub fn check_timeouts(&mut self) -> Vec<i64> {
        let now = Instant::now();
        let mut timed_out = Vec::new();

        self.pending.retain(|id, req| {
            if now.duration_since(req.sent_at) > req.timeout {
                timed_out.push(*id);
                false
            } else {
                true
            }
        });

        timed_out
    }

    /// Get count of pending requests
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

impl Default for RequestManager {
    fn default() -> Self {
        Self::new()
    }
}
