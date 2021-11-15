cmake_minimum_required(VERSION 3.13)

if(NOT EXISTS "${FREE_RTOS_CONFIG_H_PATH}/FreeRTOSConfig.h")
	message(FATAL_ERROR "Couldn't find FreeRTOSConfig.h in directory '${FREE_RTOS_CONFIG_H_PATH}'. Please set the variable FREE_RTOS_CONFIG_H_PATH correctly.")
endif()

FILE(GLOB freertos_src FreeRTOS-Kernel/*.c)

add_library(freertos STATIC
        ${freertos_src}
        FreeRTOS-Kernel/portable/GCC/ARM_CM0/port.c
        FreeRTOS-Kernel/portable/MemMang/heap_4.c
        freertos_helper.c
)

target_include_directories(freertos PUBLIC
        ${FREE_RTOS_CONFIG_H_PATH}
        FreeRTOS-Kernel/include
        FreeRTOS-Kernel/portable/GCC/ARM_CM0
)
