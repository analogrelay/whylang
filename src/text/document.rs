use std::path::PathBuf;

use text::LineMap;

/// Represents an owned Document, which is a wrapper around a source code
/// file loaded into memory for compilation.
pub struct Document {
    path: PathBuf,
    content: String,
    line_breaks: Options<LineMap>,
}
