use crate::{Clipboard, ClipboardContent, ClipboardContentKind, Result};
pub use clipboard_win::{image::Image, Clipboard as WindowsClipboard};
use std::borrow::Cow;

pub fn clipboard() -> Result<impl Clipboard> {
    WindowsClipboard::new().map_err(Into::into)
}

impl Clipboard for WindowsClipboard {
    fn get_content(&self) -> Result<ClipboardContent> {
        let mut string = String::new();

        self.get_string(&mut string)?;

        Ok(string.into())
    }

    fn set_content(&mut self, data: Cow<[u8]>, kind: ClipboardContentKind) -> Result<()> {
        match kind {
            ClipboardContentKind::Text => self
                .set_string(std::str::from_utf8(&data).map_err(Box::new)?)
                .map_err(Into::into),
            ClipboardContentKind::Image => {
                let image = Image {
                    // TODO: Optimize this or use another crate
                    bytes: data.to_vec(),
                };

                self.set_bitmap(&image).map_err(Into::into)
            }
        }
    }
}
