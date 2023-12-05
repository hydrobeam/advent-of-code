use core::alloc::GlobalAlloc;
use core::cell::Cell;

macro_rules! try_null {
    ( $e:expr ) => {
        match $e {
            None => return core::ptr::null_mut(),
            Some(e) => e,
        }
    };
}

/// Standard Bump allocator
///
/// Bumps downwards. Based on: https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html
#[derive(Debug)]
pub struct BumpAllocator {
    ptr: Cell<*mut u8>,
    start: *mut u8,
    end: *mut u8,
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        assert!(align > 0);
        assert!(align.is_power_of_two());

        let ptr = self.ptr.get() as usize;
        let new_ptr = try_null!(ptr.checked_sub(size));

        // Round down to the requested alignment.
        let new_ptr = new_ptr & !(align - 1);

        let start = self.start as usize;

        if new_ptr < start {
            // Didn't have enough capacity!
            return core::ptr::null_mut();
        }

        self.ptr.set(new_ptr as *mut u8);

        new_ptr as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // no-op
    }
}

impl BumpAllocator {
    pub const fn empty() -> Self {
        Self {
            ptr: Cell::new(core::ptr::null_mut()),
            start: core::ptr::null_mut(),
            end: core::ptr::null_mut(),
        }
    }

    /// Initialize the allocator with a start/end location in memory.
    pub fn init(&mut self, heap_bottom: *mut u8, len: usize) {
        self.end = match (heap_bottom as usize).checked_add(len) {
            Some(end) => end as *mut u8,
            None => core::intrinsics::abort(),
        };

        self.start = heap_bottom as *mut u8;
        self.ptr.set(self.end);
    }
}
