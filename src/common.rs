use std::{borrow::Cow, error::Error, fmt::Display};

/// Provides access to the system clipboard.
pub trait Clipboard {
    /// Gets the clipboard content.
    fn get_content(&self) -> Result<ClipboardContent>;

    /// Sets the clipboard content.
    fn set_content(&mut self, data: Cow<[u8]>, kind: ClipboardContentKind) -> Result<()>;
}

/// Result of the [`Clipboard`] operations.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Possible kinds of content, that
/// can be stored in clipboard.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ClipboardContentKind {
    /// UTF-8 encoded text.
    Text,
    /// Bitmap.
    Image,
}

/// Possible content of the clipboard.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClipboardContent {
    // This should be valid utf-8 string
    // if kind is ClipboardContentKind::Text
    pub data: Vec<u8>,
    pub kind: ClipboardContentKind,
}

impl ClipboardContent {
    pub fn new(data: Vec<u8>, kind: ClipboardContentKind) -> Self {
        Self { data, kind }
    }

    pub fn from_string(string: String) -> Self {
        Self::new(string.into_bytes(), ClipboardContentKind::Text)
    }
}

impl Display for ClipboardContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ClipboardContentKind::Text => {
                write!(f, "{0}", std::str::from_utf8(&self.data).unwrap())
            }
            ClipboardContentKind::Image => write!(f, "image"),
        }
    }
}

impl From<String> for ClipboardContent {
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotImplementedError;

impl Display for NotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation is not implemented")
    }
}

impl Error for NotImplementedError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotSupportedError;

impl Display for NotSupportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation is not supported")
    }
}

impl Error for NotSupportedError {}
