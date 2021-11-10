mod native;

// FFI
use core::ffi::c_void;

use embedded_time::duration::Milliseconds;

pub struct TaskParameters<'a> {
    pub name: &'a str,
    pub stack_depth: u16,
    pub priority: u32,
}

pub fn create_task<F: FnOnce() + Send + 'static>(
    task_func: F,
    params: &TaskParameters
) {
    unsafe {
        let raw_size = core::mem::size_of::<F>();

        let alloc_size;
        if raw_size > 0 {
            alloc_size = raw_size;
        } else {
            // Workaround:
            // In some cases that the closure size is zero bytes (e.g. when using the module
            // rp2040_hal::gpio::pin where all pin information is encoded into the type).
            // The native::pvPortMalloc function doesn't support this. So at least 1 byte is
            // allocated.
            alloc_size = 1;
        }

        // Allocate memory for the closure (task_func) on the FreeRTOS heap. Won't be deallocated anymore.
        let heap_ptr = native::pvPortMalloc(alloc_size) as *mut F;
        assert!(!heap_ptr.is_null()); // Assuming that always enough heap memory

        // TODO:
        // Find a more efficient solution (move closure to heap memory and use it directly from there)
        // than the backup and restore approach.
        // The problem so far has been the compiler error when calling a FnOnce type via a raw pointer
        // (compiler error: cannot move out of xxx which is behind a raw pointer). 
        // Using FnMut instead of FnOnce is probably not a good workaround.

        // Backup closure to the heap.
        core::ptr::copy_nonoverlapping(&task_func, heap_ptr, 1);
        core::mem::forget(task_func);

        // Prepare null-terminated task name (assuming configMAX_TASK_NAME_LEN is 16)
        const MAX_NAME_LEN: usize = 15;
        let mut name: [u8; MAX_NAME_LEN + 1] = [0; MAX_NAME_LEN + 1];
        let name_len = if params.name.len() < MAX_NAME_LEN { params.name.len() } else { MAX_NAME_LEN };
        name[0..name_len].clone_from_slice(&params.name.as_bytes()[0..name_len]);

        // Create task
        let mut task_handle = 0 as native::TaskHandle; // not used
        let status = native::xTaskCreate(
            task_entry::<F>,
            name.as_ptr(),
            params.stack_depth,
            heap_ptr as *mut c_void,
            params.priority,
            &mut task_handle
        );
        assert!(status == 1); // Assuming that it always succeeds (pdPASS is 1)

        extern "C" fn task_entry<F: FnOnce()>(param: *mut c_void) {
            let heap_ptr = param as *mut F;
            unsafe {
                // Restore closure from the heap.
                let mut task_func = core::mem::MaybeUninit::<F>::uninit();
                core::ptr::copy_nonoverlapping(heap_ptr, task_func.as_mut_ptr(), 1);

                // Call it
                task_func.assume_init()();
            }
        }
    }
}

pub fn delay(duration: Milliseconds) {
    unsafe {
        native::vTaskDelay(duration.0);
    }
}

pub fn start_scheduler() -> ! {
    unsafe {
        native::vTaskStartScheduler();
    }

    // Should not be reached (except if there's not enough heap memory left)
    panic!();
}
