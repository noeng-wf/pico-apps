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
set(FREE_RTOS_CONFIG_H_PATH ${CMAKE_CURRENT_SOURCE_DIR})
add_subdirectory(../freertos-lib freertos-lib)

# Pull in our pico_stdlib which pulls in commonly used features
target_link_libraries(blink-c-rtos
        pico_stdlib
        freertos
)

# create map/bin/hex file etc.
pico_add_extra_outputs(blink-c-rtos)
