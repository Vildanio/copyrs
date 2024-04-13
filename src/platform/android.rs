use crate::{Clipboard, NotImplementedError, Result};

pub fn clipboard() -> Result<impl Clipboard> {
    Ok(AndroidClipboard)
}

pub struct AndroidClipboard;

impl Clipboard for AndroidClipboard {
    fn get_content(&self) -> Result<crate::ClipboardContent> {
        Err(Box::new(NotImplementedError))
    }

    fn set_content(&mut self, _content: crate::ClipboardContent) -> Result<()> {
        Err(Box::new(NotImplementedError))
    }
}
