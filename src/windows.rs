use crate::{Clipboard, ClipboardBinaryKind, ClipboardContent, ClipboardTextKind, Result};

pub use clipboard_win::Clipboard as WindowsClipboard;

impl Clipboard for WindowsClipboard {
    fn set_content(&mut self, content: ClipboardContent) -> Result<()> {
        match content {
            ClipboardContent::Text { text, kind } => match kind {
                ClipboardTextKind::Unicode => {
                    let bytes: &[u16] = &text.encode_utf16().collect();

                    // UTF-16 text format
                    self.set_data(13, &bytes)
                }
            },
            ClipboardContent::Binary { data, kind } => match kind {
                ClipboardBinaryKind::Image => {
                    let data: &[u8] = data.into();

                    // Bitmap format
                    self.set_data(2, &data)
                }
            },
        }
    }

    fn get_content(&self) -> Result<ClipboardContent> {
        let mut string = String::new();

        self.get_string(&mut string)?;

        Ok(string)
    }
}
