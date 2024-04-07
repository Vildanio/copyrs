use objc::rc::autoreleasepool;
use objc::runtime::{Class, Object, BOOL, NO, YES};
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::{INSArray, INSString};
use objc_foundation::{NSArray, NSString};
use objc_id::Id;

use crate::{Clipboard, ClipboardContent, Result};

pub struct OSXClipboardContext {
    pasteboard: Id<Object>,
}

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSPasteboardTypeFileURL: *mut Object;
    pub static NSPasteboardTypeString: *mut Object;
}

impl OSXClipboardContext {
    pub fn new() -> Result<OSXClipboardContext> {
        let cls = Class::get("NSPasteboard").ok_or("Class::get(\"NSPasteboard\")")?;
        let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
        if pasteboard.is_null() {
            return Err("NSPasteboard#generalPasteboard returned null".into());
        }
        let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };
        Ok(OSXClipboardContext { pasteboard })
    }
}

impl Clipboard for OSXClipboardContext {
    fn set_content(&mut self, content: ClipboardContent) -> Result<()> {
        if let ClipboardContent::Text { text, .. } = content {
            let string_array = NSArray::from_vec(vec![NSString::from_str(&text)]);
            let _: usize = unsafe { msg_send![self.pasteboard, clearContents] };
            let success: bool = unsafe { msg_send![self.pasteboard, writeObjects: string_array] };

            if success {
                Ok(())
            } else {
                Err("NSPasteboard#writeObjects: returned false".into())
            }
        } else {
            Err("Binary format not supported".into())
        }
    }

    fn get_content(&self) -> Result<Option<ClipboardContent>> {
        autoreleasepool(|| unsafe {
            let types: *mut NSArray<*mut NSString> = msg_send![self.pasteboard, types];
            let has_file: BOOL = msg_send![types, containsObject: NSPasteboardTypeFileURL];
            let has_str: BOOL = msg_send![types, containsObject: NSPasteboardTypeString];

            if has_str == NO {
                return Err("NSPasteboard#types doesn't contain NSPasteboardTypeString".into());
            }

            let text = if has_file == YES {
                let file_url_string: *mut NSString =
                    msg_send![self.pasteboard, stringForType: NSPasteboardTypeFileURL];
                let file_url: *mut Object =
                    msg_send![class!(NSURL), URLWithString: file_url_string];
                let text: *mut NSString = msg_send![file_url, path];
                text
            } else {
                let text: *mut NSString =
                    msg_send![self.pasteboard, stringForType: NSPasteboardTypeString];
                text
            };

            if text.is_null() {
                return Err(("NSPasteboard#stringForType returned null").into());
            }

            Ok((*text).as_str().to_owned())
        })
        .map(|string| Some(ClipboardContent::from_plain_string(string)))
    }
}
