use crate::{Clipboard, ClipboardContentKind, NotImplementedError, Result};
use std::borrow::Cow;

pub fn clipboard() -> Result<impl Clipboard> {
    Ok(DummyClipboard)
}

pub struct DummyClipboard;

impl Clipboard for DummyClipboard {
    fn get_content(&self) -> Result<crate::ClipboardContent> {
        Err(Box::new(NotImplementedError))
    }

    fn set_content(&mut self, _data: Cow<[u8]>, _kind: ClipboardContentKind) -> Result<()> {
        Err(Box::new(NotImplementedError))
    }
}
