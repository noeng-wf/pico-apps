use super::native;

use core::ptr::NonNull;

// FFI
use core::ffi::c_void;

// Interrupts
use cortex_m::interrupt;

/// Minimal Box implementation that stores an object on the FreeRTOS heap.
/// The content cannot be accessed directly (thus it is opaque) to keep
/// the implementation simple and to be able to ignore alignment on the
/// heap.
pub struct OpaqueBox<T> {
    /// The pointer to the heap block:
    /// - Is 'None' if it has already been unboxed.
    /// - Contains a valid pointer if the size of T is greater than 0.
    /// - Contains a dangling pointer (NonNull::dangling()) if the size of T is 0.
    heap_ptr: Option<NonNull<T>>,
}

impl<T> OpaqueBox<T> {
    pub fn new(value: T) -> Self {
        // Allocate on heap.
        let heap_ptr = OpaqueBox::<T>::allocate();

        // Move value to heap.
        unsafe { OpaqueBox::<T>::write(heap_ptr, value) };

        Self {
            heap_ptr: Some(heap_ptr),
        }
    }

    pub fn unbox(mut self) -> T {
        // Use 'take' to leave the box in an empty state (no destruction required anymore).
        let heap_ptr = self.heap_ptr.take().unwrap();
        let value = unsafe { OpaqueBox::<T>::read(heap_ptr) };

        // Deallocate on heap.
        unsafe { OpaqueBox::<T>::deallocate(heap_ptr) };

        value
    }

    /// Marked as unsafe because the caller is responsible to provide a pointer
    /// that came from 'OpaqueBox::into_raw' and not to use it elsewhere.
    pub unsafe fn from_raw(raw_ptr: *mut T) -> Self {
        let heap_ptr = NonNull::new(raw_ptr).unwrap(); // must be non-null (otherwise panic)

        Self {
            heap_ptr: Some(heap_ptr),
        }
    }

    pub fn into_raw(mut self) -> *mut T {
        // Use 'take' to leave the box in an empty state (no destruction required anymore).
        self.heap_ptr.take().unwrap().as_ptr()
    }

    fn allocate() -> NonNull<T> {
        let value_size = core::mem::size_of::<T>();
        // Note: Depending on the type the size may be 0 so a work around is needed in that case.
        if value_size > 0 {
            let mut raw_ptr = core::ptr::null_mut();

            // Precaution:
            // Do memory allocation with disabled interrupts to allow using it in an
            // interrupt handler.
            interrupt::free(|_| {
                // Allocate block on heap (not respecting the alignment!).
                raw_ptr = unsafe { native::pvPortMalloc(value_size) } as *mut T;
            });

            // Assumption: There's always enough heap memory (raw_ptr non-null otherwise panic).
            NonNull::new(raw_ptr).unwrap()
        } else {
            // Some non-null pointer required for read/write operations even if T has size of 0.
            NonNull::dangling()
        }
    }

    /// Marked as unsafe because the caller is responsible to provide a valid heap pointer
    /// if size of T is not 0.
    unsafe fn deallocate(heap_ptr: NonNull<T>) {
        if core::mem::size_of::<T>() > 0 {
            // Precaution:
            // Do memory deallocation with disabled interrupts to allow using it in an
            // interrupt handler.
            interrupt::free(|_| {
                unsafe { native::vPortFree(heap_ptr.as_ptr() as *mut c_void) };
            });
        }
    }

    /// Marked as unsafe because the caller is responsible to provide a valid heap pointer
    /// if size of T is not 0.
    unsafe fn read(heap_ptr: NonNull<T>) -> T {
        core::ptr::read_unaligned(heap_ptr.as_ptr())
    }

    /// Marked as unsafe because the caller is responsible to provide a valid heap pointer
    /// if size of T is not 0.
    unsafe fn write(heap_ptr: NonNull<T>, value: T) {
        core::ptr::write_unaligned(heap_ptr.as_ptr(), value);
    }
}

impl<T> Drop for OpaqueBox<T> {
    fn drop(&mut self) {
        if let Some(heap_ptr) = self.heap_ptr.take() {
            // Call any destructor on value.
            core::mem::drop(unsafe { OpaqueBox::<T>::read(heap_ptr) });

            // Deallocate on heap.
            unsafe { OpaqueBox::<T>::deallocate(heap_ptr) };
        }
    }
}

/// Should be safe to send to another thread because no one else has the pointer.
unsafe impl<T> Send for OpaqueBox<T> where T: Send {}
/// Also the sync trait can be implemented for the same reason.
unsafe impl<T> Sync for OpaqueBox<T> where T: Sync {}
// See also: https://doc.rust-lang.org/nomicon/send-and-sync.html
