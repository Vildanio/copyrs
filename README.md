# Copyrs

Copyrs — is a tiny rust library, which provides crossplatform API for system clipboard.

**It provides implementation for:**
- Windows
- Linux
    - X11
    - Wayland
- ~~IOS~~
- ~~Macos~~
- ~~Android~~

**Please note that the library does not currently support:**
- Linux's "primary" clipboard.
- Formats other than text (however it can be easily solved).

### Example

```rust
use copyrs::{clipboard, Clipboard, ClipboardContentKind};

fn main() {
    let mut clipboard = clipboard().unwrap();

    let content = std::borrow::Cow::Borrowed("Hello, world!".as_bytes());

    clipboard
        .set_content(content, ClipboardContentKind::Text)
        .unwrap();

    let content = clipboard.get_content().unwrap();

    println!("{0}", content);
}
```

### License
The code is distributed under the **MIT License**, allowing for flexible usage and modification. Feel free to contribute to the project to improve its functionality and reliability across different platforms. Your contributions are valuable in making Copyrs a robust solution for handling clipboard operations in Rust.