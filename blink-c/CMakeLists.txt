cmake_minimum_required(VERSION 3.13)

# Pull in SDK (must be before project)
include($ENV{PICO_SDK_PATH}/pico_sdk_init.cmake)

project(blink-c)

# Initialize the SDK
pico_sdk_init()

add_executable(blink-c
        blink.c
        )

# Pull in our pico_stdlib which pulls in commonly used features
target_link_libraries(blink-c pico_stdlib)

# create map/bin/hex file etc.
pico_add_extra_outputs(blink-c)
