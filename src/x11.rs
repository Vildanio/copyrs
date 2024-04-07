use crate::{Clipboard, ClipboardContent, Result};
use std::time::Duration;

pub use x11_clipboard::Clipboard as X11Clipboard;

impl Clipboard for X11Clipboard {
    fn set_content(&mut self, content: crate::ClipboardContent) -> crate::Result<()> {
        let bytes = content.get_bytes();

        Ok(self.store(
            self.setter.atoms.clipboard,
            self.setter.atoms.utf8_string,
            bytes,
        )?)
    }

    fn get_content(&self) -> Result<ClipboardContent> {
        let bytes = self.load(
            self.setter.atoms.clipboard,
            self.getter.atoms.utf8_string,
            self.getter.atoms.property,
            Duration::from_secs(3),
        )?;

        let string = String::from_utf8(bytes)?;

        Ok(ClipboardContent::from_plain_string(string))
    }
}
