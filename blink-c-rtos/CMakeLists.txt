cmake_minimum_required(VERSION 3.13)

# Pull in SDK (must be before project)
include($ENV{PICO_SDK_PATH}/pico_sdk_init.cmake)

project(blink-c-rtos)

# Initialize the SDK
pico_sdk_init()

add_executable(blink-c-rtos
        main.c
)

# FreeRTOS
FILE(GLOB FreeRTOS_src FreeRTOS-Kernel/*.c)
add_library(FreeRTOS STATIC
        ${FreeRTOS_src}
        FreeRTOS-Kernel/portable/GCC/ARM_CM0/port.c
        FreeRTOS-Kernel/portable/MemMang/heap_4.c
)
target_include_directories(FreeRTOS PUBLIC
        .
        FreeRTOS-Kernel/include
        FreeRTOS-Kernel/portable/GCC/ARM_CM0
)

# Pull in our pico_stdlib which pulls in commonly used features
target_link_libraries(blink-c-rtos
        pico_stdlib
        FreeRTOS
)

# create map/bin/hex file etc.
pico_add_extra_outputs(blink-c-rtos)
