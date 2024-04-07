# Copyrs

Copyrs — is a rust library, which provides crossplatform API for system clipboard.

**It provides implementation for:**
- Windows
- Macos
- Linux
    - X11
    - Wayland

**Please note that the library does not currently support:**
- Linux's "primary" clipboard
- Formats other than text (however it can be easily solved)
Additionally, the macOS and Windows implementations have not undergone thorough testing, and it's uncertain whether they compile successfully.

### Example

```rust
use copyrs::{x11::X11Clipboard, Clipboard, ClipboardContent};

fn main() {
    let mut clipboard = X11Clipboard::new().unwrap();

    let content = ClipboardContent::from_plain_str("Hello, world!");

    clipboard.set_content(content).unwrap();

    let content = clipboard.get_content().unwrap();

    println!("{0}", content);
}
```

### License
The code is distributed under the **MIT License**, allowing for flexible usage and modification. Feel free to contribute to the project to improve its functionality and reliability across different platforms. Your contributions are valuable in making Copyrs a robust solution for handling clipboard operations in Rust.