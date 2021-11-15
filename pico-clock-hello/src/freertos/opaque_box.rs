use super::native;

// FFI
use core::ffi::c_void;

// Interrupts
use cortex_m::interrupt;

/// Minimal Box implementation that stores an object on the FreeRTOS heap.
/// The content cannot be accessed directly (thus it is opaque) to keep
/// the implementation simple and to be able to ignore alignment on the
/// heap.
pub struct OpaqueBox<T> {
    /// The pointer to the heap block.
    /// Is null if either T is zero-sized otherwise non-null.
    heap_ptr: *mut T,
}

impl<T> OpaqueBox<T> {
    pub fn new(value: T) -> Self {
        let value_size = core::mem::size_of::<T>();
        let mut heap_ptr = core::ptr::null_mut();
        // Note: Depending on the type the size may be 0 so a work around is needed in that case.
        if value_size > 0 {
            // Precaution:
            // Do memory allocation with disabled interrupts to allow using it in an
            // interrupt handler.
            interrupt::free(|_| {
                // Allocate block on heap (not respecting the alignment!).
                heap_ptr = unsafe { native::pvPortMalloc(value_size) } as *mut T;
            });
            assert!(!heap_ptr.is_null()); // Assuming that always enough heap memory

            // Move value to heap.
            unsafe { core::ptr::write_unaligned(heap_ptr, value) };
        }

        Self { heap_ptr }
    }

    pub fn unbox(self) -> T {
        if core::mem::size_of::<T>() > 0 {
            assert!(!self.heap_ptr.is_null());
            unsafe { core::ptr::read_unaligned(self.heap_ptr) }
        } else {
            assert!(self.heap_ptr.is_null());
            // Dummy operation to get a zero sized value
            unsafe { core::ptr::read(core::ptr::NonNull::<T>::dangling().as_ptr()) }
        }

        // The destructor of 'self' will be called after leaving this scope.
    }

    /// Marked as unsafe because the caller is responsible to provide a pointer
    /// that came from 'OpaqueBox::into_raw' and not to use it elsewhere.
    pub unsafe fn from_raw(heap_ptr: *mut T) -> Self {
        assert!(
            (core::mem::size_of::<T>() > 0 && !heap_ptr.is_null())
                || (core::mem::size_of::<T>() == 0 && heap_ptr.is_null())
        );
        Self { heap_ptr }
    }

    pub fn into_raw(self) -> *mut T {
        assert!(
            (core::mem::size_of::<T>() > 0 && !self.heap_ptr.is_null())
                || (core::mem::size_of::<T>() == 0 && self.heap_ptr.is_null())
        );

        let heap_ptr = self.heap_ptr;
        core::mem::forget(self); // Avoids freeing the heap memory in the destructor.
        heap_ptr
    }
}

impl<T> Drop for OpaqueBox<T> {
    fn drop(&mut self) {
        if core::mem::size_of::<T>() > 0 {
            assert!(!self.heap_ptr.is_null());

            // Precaution:
            // Do memory deallocation with disabled interrupts to allow using it in an
            // interrupt handler.
            interrupt::free(|_| {
                unsafe { native::vPortFree(self.heap_ptr as *mut c_void) };
            });
        } else {
            assert!(self.heap_ptr.is_null());
        }
    }
}

/// Should be safe to send to another thread because no one else has the pointer.
unsafe impl<T> Send for OpaqueBox<T> where T: Send {}
/// Also the sync trait can be implemented for the same reason.
unsafe impl<T> Sync for OpaqueBox<T> where T: Sync {}
// See also: https://doc.rust-lang.org/nomicon/send-and-sync.html
