use std::{borrow::Cow, error::Error};

/// Provides access to the system clipboard.
pub trait Clipboard {
    /// Sets the clipboard content.
    fn set_content(&mut self, content: ClipboardContent) -> Result<()>;

    /// Gets the clipboard content.
    fn get_content(&self) -> Result<Option<ClipboardContent>>;
}

/// Result of the [`Clipboard`] operations.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Possible content of the clipboard.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClipboardContent<'a> {
    /// Text.
    Text {
        text: Cow<'a, str>,
        kind: ClipboardTextKind,
    },
    /// Binary data.
    Binary {
        data: Cow<'a, [u8]>,
        kind: ClipboardBinaryKind,
    },
}

impl<'a> ClipboardContent<'a> {
    /// Creates [`ClipboardContent::Text`] with
    /// the given `text` and [`ClipboardTextKind::UTF8`]
    pub fn from_plain_str(text: &'a str) -> Self {
        ClipboardContent::Text {
            text: Cow::Borrowed(text),
            kind: ClipboardTextKind::UTF8,
        }
    }

    /// Creates [`ClipboardContent::Text`] with
    /// the given `text` and [`ClipboardTextKind::UTF8`]
    pub fn from_plain_string(text: String) -> Self {
        ClipboardContent::Text {
            text: Cow::Owned(text),
            kind: ClipboardTextKind::UTF8,
        }
    }

    /// Returns content as bytes.
    pub fn get_bytes(&self) -> &[u8] {
        match self {
            ClipboardContent::Text { text, .. } => match text {
                Cow::Borrowed(str) => str.as_bytes(),
                Cow::Owned(string) => string.as_bytes(),
            },
            ClipboardContent::Binary { data, .. } => match data {
                Cow::Borrowed(data) => data,
                Cow::Owned(data) => data,
            },
        }
    }
}

/// Kind of [`ClipboardContent::Binary`] data.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClipboardBinaryKind {
    /// The data is a bitmap.
    Image,
}

/// Kind of [`ClipboardContent::Text`] data.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClipboardTextKind {
    /// The data is utf-8 encoded plain text.
    UTF8,
}
