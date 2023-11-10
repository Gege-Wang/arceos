//! Early memory allocation
//! 
//! TODO: decrease

use core::{ptr::NonNull, num};

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    //inner:,
    used_pages: usize,
    page_pos: usize,
    used_bytes: usize,
    byte_pos: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Creates a new empty [`EarlyAllocator`].
    pub const fn new() ->Self {
        Self {
            //inner: Tlsf::new(),
            used_pages: 0,
            page_pos: 0,
            used_bytes: 0,
            byte_pos: 0

        }

    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        assert!(PAGE_SIZE.is_power_of_two());
        let end = super::align_down(start + size, PAGE_SIZE);
        let start = super::align_up(start, PAGE_SIZE);
        self.page_pos = end;
        self.byte_pos = start;
    }
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}


impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let number: usize = self.available_pages();
        if number >= num_pages {
            self.page_pos -= num_pages * PAGE_SIZE;
            self.used_pages += num_pages;
            Ok(self.page_pos)
        } else {
            Err(AllocError::NoMemory)
        }
        
    }
    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / PAGE_SIZE
        
    }
    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        if self.available_bytes() < PAGE_SIZE {
            self.page_pos += self.used_pages * PAGE_SIZE;
         } 
        
    }
    fn total_pages(&self) -> usize {
        unimplemented!()
        
    }
    fn used_pages(&self) -> usize {
        self.used_pages 
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: core::alloc::Layout) -> AllocResult<core::ptr::NonNull<u8>> {
        let size = self.page_pos - self.byte_pos;
        if size >= layout.size() {
            self.used_bytes += layout.size();
            self.byte_pos += layout.size();
            // Return a non-null pointer to the allocated memory
            let ptr = unsafe { NonNull::new_unchecked((self.byte_pos - layout.size()) as *mut u8) };
            Ok(ptr)
        } else {
            Err(AllocError::NoMemory)
        }
    }
    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }
    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
     if self.available_bytes() < PAGE_SIZE {
        self.byte_pos -= self.used_bytes;
     } 
    }
    fn total_bytes(&self) -> usize {
     unimplemented!()   
    }
    fn used_bytes(&self) -> usize {
        self.used_bytes 
    }
}