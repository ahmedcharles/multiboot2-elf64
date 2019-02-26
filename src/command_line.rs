use header::Tag;
use core::{mem, str, slice};

#[derive(Debug)]
pub struct CommandLineTag {
    string: &'static str,
}

pub fn command_line_tag(tag: &Tag) -> CommandLineTag {
    assert_eq!(1, tag.typ);
    let string = {
        let addr = unsafe { (tag as *const _).offset(1) } as *const u8;
        let size = tag.size as usize - mem::size_of::<Tag>() - 1; // zero-terminated string.
        let data = unsafe { slice::from_raw_parts(addr, size) };
        unsafe { str::from_utf8_unchecked(data) } // Multiboot requires UTF-8.
    };
    CommandLineTag {
        string: string,
    }
}

impl CommandLineTag {
    pub fn command_line(&self) -> &str {
        &self.string
    }
}
