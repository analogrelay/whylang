use std::path::{Path, PathBuf};

use lazy_init::Lazy;

use text::LineMap;

/// Represents an owned Document, which is a wrapper around a source code
/// file loaded into memory for compilation.
pub struct Document {
    path: PathBuf,
    content: String,
    line_map: Lazy<LineMap>,
}

impl Document {
    pub fn new<P: Into<PathBuf>, C: Into<String>>(path: P, content: C) -> Document {
        Document {
            path: path.into(),
            content: content.into(),
            line_map: Lazy::new(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn line_map(&self) -> &LineMap {
        self.line_map
            .get_or_create(|| LineMap::parse(&self.content))
    }
}
