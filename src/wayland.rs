pub use smithay_clipboard::Clipboard as WaylandClipboard;

use crate::{Clipboard, ClipboardContent, Result};

impl Clipboard for WaylandClipboard {
    fn set_content(&mut self, content: crate::ClipboardContent) -> Result<()> {
        if let ClipboardContent::Text { text, .. } = content {
            self.store(text);

            return Ok(());
        }

        Err("Binary format not supported".into())
    }

    fn get_content(&self) -> Result<Option<crate::ClipboardContent>> {
        let string = self.load()?;

        Ok(Some(ClipboardContent::from_plain_string(string)))
    }
}
