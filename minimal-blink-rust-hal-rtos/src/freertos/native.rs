// FFI
use core::ffi::c_void;

// Dummy placeholder types for better type safety
pub enum TaskControlBlock {}
pub type TaskHandle = *mut TaskControlBlock;

#[link(name = "freertos", kind = "static")]
extern "C" {
    // Expose FreeRTOS internal dynamic memory allocation
    // (as a helper to deal with closures in the Task abstraction)
    pub fn pvPortMalloc(wanted_size: usize) -> *mut c_void;

    // Should be 32 bit, except if configUSE_16_BIT_TICKS is set to 1
    pub fn vTaskDelay(xTicksToDelay: u32);

    pub fn vTaskStartScheduler();

    pub fn xTaskCreate(
        task_func: extern "C" fn(*mut c_void),
        name: *const u8,  // see also configMAX_TASK_NAME_LEN
        stack_depth: u16, // type from configSTACK_DEPTH_TYPE
        task_param: *mut c_void,
        priority: u32,
        task_handle: *mut TaskHandle,
    ) -> i32;
}
