use copyrs::{clipboard, Clipboard, ClipboardContent};

fn main() {
    let mut clipboard = clipboard().unwrap();

    let content = ClipboardContent::from_plain_str("Hello, world!");

    clipboard.set_content(content).unwrap();

    let content = clipboard.get_content().unwrap();

    println!("{0}", content);
}
