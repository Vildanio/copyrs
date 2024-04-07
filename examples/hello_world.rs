use copyrs::{x11::X11Clipboard, Clipboard, ClipboardContent};

fn main() {
    let mut clipboard = X11Clipboard::new().unwrap();

    let content = ClipboardContent::from_plain_str("Hello, world!");

    clipboard.set_content(content).unwrap();

    let content = clipboard.get_content().unwrap();

    println!("{0}", content);
}
