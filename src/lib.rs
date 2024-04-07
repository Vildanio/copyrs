mod common;
pub use common::*;
#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(feature = "wayland")]
pub mod wayland;
#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(feature = "x11")]
pub mod x11;

#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(feature = "wayland")]
pub fn clipboard() -> impl Clipboard {
    wayland::WaylandClipboard
}

#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten"
    ))
))]
#[cfg(all(feature = "x11", not(feature = "wayland")))]
pub fn clipboard() -> Result<impl Clipboard> {
    x11::X11Clipboard::new().map_err(Into::into)
}
