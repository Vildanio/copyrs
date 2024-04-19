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
