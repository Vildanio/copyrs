use crate::{Clipboard, NotImplementedError, Result};

pub fn clipboard() -> Result<impl Clipboard> {
    Ok(WindowsClipboard)
}

pub struct WindowsClipboard;

impl Clipboard for WindowsClipboard {
    fn get_content(&self) -> Result<crate::ClipboardContent> {
        Err(Box::new(NotImplementedError))
    }

    fn set_content(&mut self, _content: crate::ClipboardContent) -> Result<()> {
        Err(Box::new(NotImplementedError))
    }
}
