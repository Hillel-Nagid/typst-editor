//! Source-preview synchronization

use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::path::PathBuf;

/// Position in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

/// Position in preview (page and coordinates)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PreviewPosition {
    pub page: usize,
    pub x: f32,
    pub y: f32,
}

/// Source mapping between source and preview
#[derive(Clone)]
pub struct SourceMapping {
    /// Map from source positions to preview positions
    source_to_preview: HashMap<(PathBuf, SourcePosition), Vec<PreviewPosition>>,
    /// Map from preview positions to source positions
    preview_to_source: HashMap<(usize, (u32, u32)), Vec<(PathBuf, SourcePosition)>>,
}

impl SourceMapping {
    pub fn new() -> Self {
        Self {
            source_to_preview: HashMap::new(),
            preview_to_source: HashMap::new(),
        }
    }

    /// Add a mapping
    pub fn add_mapping(
        &mut self,
        file: PathBuf,
        source_pos: SourcePosition,
        preview_pos: PreviewPosition
    ) {
        self.source_to_preview
            .entry((file.clone(), source_pos))
            .or_insert_with(Vec::new)
            .push(preview_pos);

        let grid_pos = (((preview_pos.x as u32) / 10) * 10, ((preview_pos.y as u32) / 10) * 10);
        self.preview_to_source
            .entry((preview_pos.page, grid_pos))
            .or_insert_with(Vec::new)
            .push((file, source_pos));
    }

    /// Find preview positions for a source position
    pub fn source_to_preview_lookup(
        &self,
        file: &PathBuf,
        pos: SourcePosition
    ) -> Option<&[PreviewPosition]> {
        self.source_to_preview.get(&(file.clone(), pos)).map(|v| v.as_slice())
    }

    /// Find source positions for a preview position
    pub fn preview_to_source_lookup(
        &self,
        page: usize,
        x: f32,
        y: f32
    ) -> Option<&[(PathBuf, SourcePosition)]> {
        let grid_pos = (((x as u32) / 10) * 10, ((y as u32) / 10) * 10);
        self.preview_to_source.get(&(page, grid_pos)).map(|v| v.as_slice())
    }

    /// Clear all mappings
    pub fn clear(&mut self) {
        self.source_to_preview.clear();
        self.preview_to_source.clear();
    }
}

impl Default for SourceMapping {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages synchronization between source and preview
pub struct SyncManager {
    mapping: SourceMapping,
}

impl SyncManager {
    pub fn new() -> Self {
        Self {
            mapping: SourceMapping::new(),
        }
    }

    /// Update mappings from compilation result
    pub fn update_mapping(&mut self, mapping: SourceMapping) {
        self.mapping = mapping;
    }

    /// Sync from source to preview
    pub fn sync_to_preview(&self, file: &PathBuf, pos: SourcePosition) -> Option<PreviewPosition> {
        self.mapping
            .source_to_preview_lookup(file, pos)
            .and_then(|positions| positions.first())
            .copied()
    }

    /// Sync from preview to source
    pub fn sync_to_source(&self, page: usize, x: f32, y: f32) -> Option<(PathBuf, SourcePosition)> {
        self.mapping
            .preview_to_source_lookup(page, x, y)
            .and_then(|positions| positions.first())
            .cloned()
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}
