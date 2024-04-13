use crate::{Clipboard, ClipboardBinaryKind, ClipboardContent, ClipboardTextKind, Result};
use clipboard_win::image::Image;
pub use clipboard_win::Clipboard as WindowsClipboard;

pub fn clipboard() -> Result<impl Clipboard> {
    WindowsClipboard::new().map_err(Into::into)
}

impl Clipboard for WindowsClipboard {
    fn set_content(&mut self, content: ClipboardContent) -> Result<()> {
        match content {
            ClipboardContent::Text { text, kind } => match kind {
                ClipboardTextKind::UTF8 => self.set_string(&text).map_err(Into::into),
            },
            ClipboardContent::Binary { data, kind } => match kind {
                ClipboardBinaryKind::Image => {
                    let image = Image {
                        // TODO: Optimize this or use another crate
                        bytes: data.to_vec(),
                    };

                    self.set_bitmap(&image).map_err(Into::into)
                }
            },
        }
    }

    fn get_content(&self) -> Result<ClipboardContent> {
        let mut string = String::new();

        self.get_string(&mut string)?;

        let content = ClipboardContent::from_plain_string(string);

        Ok(content)
    }
}
