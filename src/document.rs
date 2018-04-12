use std::path::Path;

use url::Url;

/// `Document` is a trait for representing objects that can be downloaded and
/// written to disk using the `Client` struct.  If an object implementing
/// `Document` returns `true` from its `wkhtmltopdf()` method, it will
/// be converted to PDF before it is written to disk.
pub trait Document {
    /// Returns a `&Path` representing the location on disk to write the
    /// document to
    fn path(&self) -> &Path;

    /// Returns a `&Url` representing the location of the document on the
    /// interwebs :)
    fn url(&self) -> &Url;

    /// Returns a `bool` representing whether or not the document should be
    /// converted to pdf using `wkhtmltopdf` before being written to disk.
    /// `true` means you would like the document to be converted using
    /// `wkhtmltopdf` before being written to disk. `false` means you would
    /// like to write raw bytes only.
    fn wkhtmltopdf(&self) -> bool;

    /// Enables setting raw bytes of the object after they have been downloaded.
    fn set_bytes(&mut self, bytes: Option<Vec<u8>>);
}
