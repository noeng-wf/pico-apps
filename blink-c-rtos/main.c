/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "pico/stdlib.h"

#include "FreeRTOS.h"
#include "task.h"

void led_task(void *param) {
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    while (true) {
        gpio_put(LED_PIN, 1);
        vTaskDelay(250);
        gpio_put(LED_PIN, 0);
        vTaskDelay(250);
    }
}

int main() {
    TaskHandle_t led_task_handle = NULL;
    uint32_t status = xTaskCreate(
        led_task,
        "led_task",
        1024,
        NULL,
        tskIDLE_PRIORITY,
        &led_task_handle);

    vTaskStartScheduler();

    // Should never be reached
    while (true) {}
}
