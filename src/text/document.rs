use std::io::Read;
use std::path::{Path, PathBuf};

use lazy_init::Lazy;

use text::{LineMap, Error};

/// Represents a Document, which is a wrapper around a source code
/// file loaded into memory for compilation.
///
/// A `Document` owns the text it contains, so cloning it means copying
/// all the text contained within it.
pub struct Document {
    path: PathBuf,
    content: String,
    line_map: Lazy<LineMap>,
}

impl Document {
    /// Construct a new `Document`
    pub fn new<P: Into<PathBuf>, C: Into<String>>(path: P, content: C) -> Document {
        Document {
            path: path.into(),
            content: content.into(),
            line_map: Lazy::new(),
        }
    }

    /// Reads all the text from the specified reader into a new `Document`
    pub fn read<P: Into<PathBuf>, R: Read>(path: P, reader: &mut R) -> Result<Document, Error> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        Ok(Document::new(path, text))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get a `LineMap` representing the lines in the document
    ///
    /// The first call to this will cause the document to be parsed
    /// to find the line endings
    pub fn line_map(&self) -> &LineMap {
        self.line_map
            .get_or_create(|| LineMap::parse(&self.content))
    }
}