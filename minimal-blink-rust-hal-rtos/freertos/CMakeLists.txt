cmake_minimum_required(VERSION 3.13)

set(CMAKE_TOOLCHAIN_FILE arm_toolchain.cmake)

project(freertos_if)

# FreeRTOS
set(FREE_RTOS_CONFIG_H_PATH ${CMAKE_CURRENT_SOURCE_DIR})
add_subdirectory(../../freertos-lib freertos-lib)

# Required for Cargo
install(TARGETS freertos DESTINATION .)
