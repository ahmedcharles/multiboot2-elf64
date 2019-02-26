use header::Tag;
use core::{mem, str, slice};

#[derive(Debug)]
pub struct BootLoaderNameTag {
    string: &'static str,
}

pub fn boot_loader_name_tag(tag: &Tag) -> BootLoaderNameTag {
    assert_eq!(2, tag.typ);
    let string = {
        let addr = unsafe { (tag as *const _).offset(1) } as *const u8;
        let size = tag.size as usize - mem::size_of::<Tag>() - 1; // zero-terminated string.
        let data = unsafe { slice::from_raw_parts(addr, size) };
        unsafe { str::from_utf8_unchecked(data) } // Multiboot requires UTF-8.
    };
    BootLoaderNameTag {
        string: string,
    }
}

impl BootLoaderNameTag {
    pub fn name(&self) -> &str {
        &self.string
    }
}
