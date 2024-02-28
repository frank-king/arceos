use core::{alloc::Layout, ptr::NonNull};

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

use talc::{ErrOnOom, Span, Talc};

pub struct TalcByteAllocator {
    inner: Talc<ErrOnOom>,
    mem_span: Span,
    used_bytes: usize,
}

impl TalcByteAllocator {
    pub const fn new() -> Self {
        let inner = Talc::new(ErrOnOom);
        Self {
            inner,
            mem_span: Span::empty(),
            used_bytes: 0,
        }
    }
}

impl BaseAllocator for TalcByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        let span = Span::from_base_size(start as *mut u8, size);
        if let Ok(span) = unsafe { self.inner.claim(span) } {
            self.mem_span = span;
        }
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        let span = self
            .mem_span
            .fit_over(Span::from_base_size(start as *mut u8, size));
        self.mem_span = unsafe { self.inner.extend(self.mem_span, span) };
        Ok(())
    }
}

impl ByteAllocator for TalcByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        unsafe {
            self.inner
                .malloc(layout)
                .map_err(|()| AllocError::NoMemory)
                .inspect(|_| self.used_bytes += layout.size())
        }
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unsafe { self.inner.free(pos, layout) };
        self.used_bytes -= layout.size();
    }

    fn total_bytes(&self) -> usize {
        self.mem_span.size()
    }
    fn used_bytes(&self) -> usize {
        self.used_bytes
    }
    fn available_bytes(&self) -> usize {
        self.total_bytes() - self.used_bytes()
    }
}
