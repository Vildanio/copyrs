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
#[path = "platform/wayland.rs"]
mod platform;
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
#[path = "platform/x11.rs"]
mod platform;

pub use platform::*;
mod common;
pub use common::*;
