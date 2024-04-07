use std::io::{self, Read};

pub use wl_clipboard_rs;
use wl_clipboard_rs::{
    copy::{self, Options, ServeRequests},
    paste,
};

use crate::{Clipboard, ClipboardContent, Result};

pub struct WaylandClipboard;

impl Clipboard for WaylandClipboard {
    fn set_content(&mut self, content: crate::ClipboardContent) -> Result<()> {
        if let ClipboardContent::Text { text, .. } = content {
            let bytes = text.as_bytes();
            let mut options = Options::new();

            options
                .seat(copy::Seat::All)
                .trim_newline(false)
                .foreground(false)
                .serve_requests(ServeRequests::Unlimited);

            options
                .copy(copy::Source::Bytes(bytes.into()), copy::MimeType::Text)
                .map_err(Into::into)
        } else {
            Err("Binary format not supported".into())
        }
    }

    fn get_content(&self) -> Result<ClipboardContent> {
        let mut reader = match paste::get_contents(
            paste::ClipboardType::Regular,
            paste::Seat::Unspecified,
            paste::MimeType::Text,
        ) {
            Ok((reader, _)) => reader,
            Err(
                paste::Error::NoSeats | paste::Error::ClipboardEmpty | paste::Error::NoMimeType,
            ) => return Ok(ClipboardContent::EMPTY_TEXT),
            Err(e) => return Err(e.into()),
        };

        let string = read_into_string(&mut reader).map_err(Box::new)?;

        Ok(ClipboardContent::from_plain_string(string))
    }
}

fn read_into_string<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}
