#[cfg(any(feature = "x11", feature = "wayland"))]
use crate::{Clipboard, ClipboardContent, ClipboardContentKind, Result};
#[cfg(any(feature = "x11", feature = "wayland"))]
use std::borrow::Cow;

#[cfg(feature = "x11")]
use crate::NotSupportedError;
#[cfg(feature = "x11")]
use std::time::Duration;

#[cfg(feature = "x11")]
pub fn clipboard() -> Result<impl Clipboard> {
    x11_clipboard()
}

#[cfg(feature = "x11")]
pub fn x11_clipboard() -> Result<impl Clipboard> {
    X11Clipboard::new().map_err(Into::into)
}

#[cfg(feature = "x11")]
pub use x11_clipboard::Clipboard as X11Clipboard;

#[cfg(feature = "x11")]
impl Clipboard for X11Clipboard {
    // Supports only utf-8 strings

    fn get_content(&self) -> Result<ClipboardContent> {
        let bytes = self.load(
            self.setter.atoms.clipboard,
            self.getter.atoms.utf8_string,
            self.getter.atoms.property,
            Duration::from_secs(3),
        )?;

        let string = String::from_utf8(bytes)?;

        Ok(string.into())
    }

    fn set_content(&mut self, data: Cow<[u8]>, kind: ClipboardContentKind) -> Result<()> {
        if let ClipboardContentKind::Text = kind {
            self.store(self.setter.atoms.clipboard, self.setter.atoms.incr, data)
                .map_err(Into::into)
        } else {
            Err(Box::new(NotSupportedError))
        }
    }
}

#[cfg(feature = "wayland")]
use std::io::Read;

#[cfg(feature = "wayland")]
use wl_clipboard_rs::{
    copy::{self, Options, ServeRequests},
    paste,
};

#[cfg(all(feature = "wayland", not(feature = "x11")))]
pub fn clipboard() -> Result<WaylandClipboard> {
    Ok(wayland_clipboard())
}

#[cfg(feature = "wayland")]
pub fn wayland_clipboard() -> WaylandClipboard {
    WaylandClipboard
}

#[cfg(feature = "wayland")]
pub struct WaylandClipboard;

#[cfg(feature = "wayland")]
impl Clipboard for WaylandClipboard {
    fn set_content(&mut self, data: Cow<[u8]>, kind: ClipboardContentKind) -> Result<()> {
        if let ClipboardContentKind::Text = kind {
            let mut options = Options::new();

            options
                .seat(copy::Seat::All)
                .trim_newline(false)
                .foreground(false)
                .serve_requests(ServeRequests::Unlimited);

            options
                .copy(copy::Source::Bytes(data.into()), copy::MimeType::Text)
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
            ) => return Ok(ClipboardContent::from_string(String::new())),
            Err(e) => return Err(e.into()),
        };

        let mut string = String::new();

        reader.read_to_string(&mut string).map_err(Box::new)?;

        Ok(string.into())
    }
}
