mod native;
pub mod opaque_box;

use opaque_box::OpaqueBox;

// FFI
use core::ffi::c_void;

use embedded_time::duration::Milliseconds;

pub struct TaskParameters<'a> {
    pub name: &'a str,
    pub stack_depth: u16,
    pub priority: u32,
}

pub fn create_task<F: FnOnce() + Send + 'static>(task_func: F, params: &TaskParameters) {
    const MAX_NAME_LEN: usize = 15;

    unsafe {
        assert!(native::freertos_sizeof_size_t() == core::mem::size_of::<usize>() as u8);
        assert!(
            native::freertos_sizeof_configSTACK_DEPTH_TYPE()
                == core::mem::size_of_val(&params.stack_depth)
        );
        assert!(native::freertos_sizeof_BaseType_t() == core::mem::size_of_val(&params.priority));
        assert!(native::freertos_configMAX_TASK_NAME_LEN() == MAX_NAME_LEN + 1);

        // Backup closure to the heap.
        let task_func_on_heap = OpaqueBox::new(task_func);

        // TODO:
        // A more efficient solution would be to use the closure directly on the heap (to avoid
        // moving it back and forth) using the alloc library (Box).
        // But then a fully blown GlobalAlloc heap implementation would be necessary based on the
        // FreeRTOS heap. This in turn requires a nightly toolchain because of the unstable
        // 'alloc_error_handler' attribute.

        // Prepare null-terminated task name (assuming configMAX_TASK_NAME_LEN is 16)
        let mut name: [u8; MAX_NAME_LEN + 1] = [0; MAX_NAME_LEN + 1];
        let name_len = if params.name.len() < MAX_NAME_LEN {
            params.name.len()
        } else {
            MAX_NAME_LEN
        };
        name[0..name_len].clone_from_slice(&params.name.as_bytes()[0..name_len]);

        // Create task
        let mut task_handle = core::ptr::null_mut(); // not used
        let status = native::xTaskCreate(
            task_entry::<F>,
            name.as_ptr(),
            params.stack_depth,
            task_func_on_heap.into_raw() as *mut c_void,
            params.priority,
            &mut task_handle,
        );
        assert!(status == 1); // Assuming that it always succeeds (pdPASS is 1)

        extern "C" fn task_entry<F: FnOnce()>(param: *mut c_void) {
            let task_func = unsafe { OpaqueBox::from_raw(param as *mut F).unbox() };
            task_func();
        }
    }
}

pub fn delay(duration: Milliseconds) {
    unsafe {
        assert!(native::freertos_sizeof_size_t() == core::mem::size_of::<usize>() as u8);
        assert!(native::freertos_sizeof_TickType_t() == core::mem::size_of::<u32>());

        native::vTaskDelay(duration.0);
    }
}

pub fn start_scheduler() -> ! {
    unsafe {
        native::vTaskStartScheduler();
    }

    // Should not be reached (except if there's not enough heap memory left)
    panic!("Not enough heap memory");
}
