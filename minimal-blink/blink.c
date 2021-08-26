/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "hardware/gpio.h"
#include "hardware/timer.h"

#define PICO_DEFAULT_LED_PIN 25

void my_sleep_ms(uint delay_ms) {
    uint64_t base = time_us_64();
    while ((int64_t)(time_us_64() - base) < (delay_ms * 1000)) {
    }
}

int main() {
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    while (true) {
        gpio_put(LED_PIN, 1);
        my_sleep_ms(700);
        gpio_put(LED_PIN, 0);
        my_sleep_ms(300);
    }
}
