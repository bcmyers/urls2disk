use std::path::{Path, PathBuf};

use url::Url;

use document::Document;

/// `SimpleDocument` is a model struct implementing the `Document` trait.
/// Although you can certainly use this struct, you may want to consider writing
/// your own simple struct implementing `Document` in order to provide more
/// customized behavior.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct SimpleDocument {
    bytes: Option<Vec<u8>>,
    path: PathBuf,
    url: Url,
    wkhtmltopdf: bool,
}

impl SimpleDocument {
    /// Creates a new `SimpleDocument`
    pub fn new(path: PathBuf, url: Url, wkhtmltopdf: bool) -> Self {
        SimpleDocument {
            bytes: None,
            path,
            url,
            wkhtmltopdf,
        }
    }
    /// If `SimpleDocument` has already been downloaded by `Client`, will
    /// return `Some(bytes)`; otherwise will return `None`
    pub fn bytes(&self) -> Option<&[u8]> {
        match self.bytes {
            Some(ref v) => Some(v),
            None => None,
        }
    }
}

impl Document for SimpleDocument {
    fn path(&self) -> &Path {
        &self.path
    }
    fn url(&self) -> &Url {
        &self.url
    }
    fn wkhtmltopdf(&self) -> bool {
        self.wkhtmltopdf
    }
    fn set_bytes(&mut self, bytes: Option<Vec<u8>>) {
        self.bytes = bytes
    }
}
