use header::Tag;

#[derive(Debug)]
pub struct MemoryMapTag {
    inner: *const MemoryMapTagInner,
    entries: usize,
}

pub fn memory_map_tag(tag: &Tag) -> MemoryMapTag {
    assert_eq!(6, tag.typ);
    let inner = unsafe { (tag as *const _).offset(1) } as *const _;
    MemoryMapTag {
        inner: inner,
        entries: (tag.size / unsafe { &*inner }.entry_size) as usize,
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct MemoryMapTagInner {
    entry_size: u32,
    entry_version: u32,
}

impl MemoryMapTag {
    pub fn memory_areas(&self) -> MemoryAreaIter {
        MemoryAreaIter {
            current_area: self.first_area(),
            remaining_entries: self.entries,
            entry_size: self.get().entry_size,
        }
    }

    fn first_area(&self) -> *const u8 {
        (unsafe { self.inner.offset(1) }) as *const _
    }

    fn get(&self) -> &MemoryMapTagInner {
        unsafe { &*self.inner }
    }
}

#[derive(Clone, Debug)]
pub struct MemoryAreaIter {
    current_area: *const u8,
    remaining_entries: usize,
    entry_size: u32,
}

impl Iterator for MemoryAreaIter {
    type Item = MemoryArea;

    fn next(&mut self) -> Option<MemoryArea> {
        while self.remaining_entries != 0 {
            let area = MemoryArea {
                inner: self.current_area,
                entry_size: self.entry_size,
            };

            self.current_area = unsafe { self.current_area.offset(self.entry_size as isize) };
            self.remaining_entries -= 1;

            if area.get().typ == 1 {
                return Some(area);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct MemoryArea {
    inner: *const u8,
    entry_size: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct MemoryAreaInner {
    base_addr: u64,
    length: u64,
    typ: u32,
    _reserved: u32,
}

impl MemoryArea {
    pub fn start_address(&self) -> u64 {
        self.get().base_addr
    }

    pub fn end_address(&self) -> u64 {
        (self.get().base_addr + self.get().length)
    }

    pub fn size(&self) -> u64 {
        self.get().length
    }

    fn get(&self) -> &MemoryAreaInner {
        assert_eq!(24, self.entry_size);
        unsafe { &*(self.inner as *const MemoryAreaInner) }
    }
}
