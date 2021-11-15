// Helper functions for Rust to access FreeRTOS header file constants.

#include <stddef.h> // for size_t
#include <stdint.h>

#include "FreeRTOS.h"

uint8_t freertos_sizeof_size_t(void) {
    return sizeof(size_t);
}

size_t freertos_sizeof_BaseType_t(void) {
    return sizeof(BaseType_t);
}

size_t freertos_sizeof_TickType_t(void) {
    return sizeof(TickType_t);
}

size_t freertos_sizeof_configSTACK_DEPTH_TYPE(void) {
    return sizeof(configSTACK_DEPTH_TYPE);
}

size_t freertos_configMAX_TASK_NAME_LEN(void) {
    return configMAX_TASK_NAME_LEN;
}
