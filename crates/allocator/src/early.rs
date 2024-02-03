use core::{alloc::Layout, ptr::NonNull};

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    bytes_pos: usize,
    pages_pos: usize,
    bytes_count: usize,
    pages_count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        assert!(PAGE_SIZE.is_power_of_two());
        EarlyAllocator {
            start: 0,
            end: 0,
            bytes_pos: 0,
            pages_pos: 0,
            bytes_count: 0,
            pages_count: 0,
        }
    }

    pub fn init(&mut self, start_vaddr: usize, size: usize) {
        self.start = start_vaddr;
        self.end = start_vaddr + size;
        debug_assert_ne!(self.start, 0);
        self.bytes_pos = self.start;
        self.pages_pos = self.end;
    }
    pub fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        let end = start + size;
        if start < self.end || self.start < end {
            return Err(AllocError::MemoryOverlap);
        }
        if start == self.end {
            self.end = end;
            if self.pages_pos == start {
                debug_assert_eq!(self.pages_count, 0);
                self.pages_pos = self.end;
            }
        } else if end == self.start {
            self.start = start;
            if self.bytes_pos == end {
                debug_assert_eq!(self.bytes_count, 0);
                self.bytes_pos = self.start;
            }
        }
        self.check_wf();
        Ok(())
    }

    pub fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if layout.size() == 0 {
            return Ok(unsafe { NonNull::new_unchecked(layout.align() as *mut u8) });
        }
        let start = crate::align_up(self.bytes_pos, layout.align());
        let end = start
            .checked_add(layout.size())
            .filter(|&end| end <= self.pages_pos)
            .ok_or(AllocError::NoMemory)?;
        self.bytes_pos = end;
        self.bytes_count += 1;
        self.check_wf();
        Ok(unsafe { NonNull::new_unchecked(start as *mut u8) })
    }
    pub fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        let start = pos.as_ptr() as usize;
        let end = start + layout.size();
        debug_assert!(start >= self.start && end <= self.bytes_pos);
        if end == self.bytes_pos {
            self.bytes_pos = start;
        }
        self.bytes_count -= 1;
        if self.bytes_count == 0 {
            self.bytes_pos = self.start;
        }
        self.check_wf();
    }

    pub fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        let end = crate::align_down(self.pages_pos, align_pow2);
        let size = num_pages.checked_mul(PAGE_SIZE);
        let start = size
            .and_then(|size| end.checked_sub(size))
            .filter(|&start| start >= self.bytes_pos)
            .ok_or(AllocError::NoMemory)?;
        self.pages_pos = start;
        self.pages_count += 1;
        self.check_wf();
        Ok(start)
    }
    pub fn dealloc_pages(&mut self, start: usize, num_pages: usize) {
        let size = num_pages * PAGE_SIZE;
        let end = start + size;
        debug_assert!(start >= self.pages_pos && end <= self.end);
        if start == self.pages_pos {
            self.pages_pos = end;
        }
        self.pages_count -= 1;
        if self.pages_count == 0 {
            self.pages_pos = self.end;
        }
        self.check_wf();
    }

    pub const fn total_pages(&self) -> usize {
        self.total_bytes() / PAGE_SIZE
    }
    pub const fn used_pages(&self) -> usize {
        (self.end - self.pages_pos) / PAGE_SIZE
    }
    pub const fn available_pages(&self) -> usize {
        self.available_bytes() / PAGE_SIZE
    }

    pub const fn total_bytes(&self) -> usize {
        self.end - self.start
    }
    pub const fn used_bytes(&self) -> usize {
        self.bytes_pos - self.start
    }
    pub const fn available_bytes(&self) -> usize {
        self.pages_pos - self.bytes_pos
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn check_wf(&self) {}

    #[cfg(debug_assertions)]
    #[inline(never)]
    fn check_wf(&self) {
        assert!(self.start <= self.bytes_pos);
        assert!(self.bytes_count == 0 || self.start < self.bytes_pos);
        assert!(self.bytes_pos <= self.pages_pos);
        assert!(self.pages_pos <= self.end);
        assert!(self.pages_count == 0 || self.pages_pos < self.end);
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.init(start, size);
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        self.add_memory(start, size)
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        self.alloc_pages(num_pages, align_pow2)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        self.dealloc_pages(pos, num_pages);
    }

    fn total_pages(&self) -> usize {
        self.total_pages()
    }

    fn used_pages(&self) -> usize {
        self.used_pages()
    }

    fn available_pages(&self) -> usize {
        self.available_pages()
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.alloc(layout)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.dealloc(pos, layout);
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes()
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes()
    }

    fn available_bytes(&self) -> usize {
        self.available_bytes()
    }
}
