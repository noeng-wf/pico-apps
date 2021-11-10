cmake_minimum_required(VERSION 3.13)

# Inspired by: https://kubasejdak.com/how-to-cross-compile-for-embedded-with-cmake-like-a-champ

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR arm)

# Skip compilation check
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

find_program(CMAKE_ASM_COMPILER arm-none-eabi-gcc)
find_program(CMAKE_C_COMPILER arm-none-eabi-gcc)
find_program(CMAKE_CXX_COMPILER arm-none-eabi-g++)

set(CMAKE_ASM_FLAGS_INIT " -mcpu=cortex-m0plus -mthumb")
set(CMAKE_C_FLAGS_INIT " -mcpu=cortex-m0plus -mthumb")
set(CMAKE_CXX_FLAGS_INIT " -mcpu=cortex-m0plus -mthumb")
