/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "hardware/structs/iobank0.h"
#include "hardware/structs/padsbank0.h"
#include "hardware/structs/sio.h"
#include "hardware/structs/timer.h"

#define MY_PICO_DEFAULT_LED_PIN 25

#define MY_GPIO_FUNC_SIO 5

void my_gpio_set_function(uint gpio, uint fn) {
    // Set input enable on, output disable off
    padsbank0_hw->io[gpio] = (padsbank0_hw->io[gpio] & (PADS_BANK0_GPIO0_IE_BITS | PADS_BANK0_GPIO0_OD_BITS)) | PADS_BANK0_GPIO0_IE_BITS;

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    iobank0_hw->io[gpio].ctrl = fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB;
}

void my_gpio_init(uint gpio) {
    sio_hw->gpio_oe_clr = 1ul << gpio;
    sio_hw->gpio_clr = 1ul << gpio;
    my_gpio_set_function(gpio, MY_GPIO_FUNC_SIO);
}

void my_gpio_set_dir_out(uint gpio) {
    sio_hw->gpio_oe_set = 1ul << gpio;
}

void my_gpio_put(uint gpio, bool value) {
    uint32_t mask = 1ul << gpio;
    if (value)
        sio_hw->gpio_set = mask;
    else
        sio_hw->gpio_clr = mask;
}

uint32_t my_time_us_32() {
    return timer_hw->timerawl;
}

void my_sleep_ms(uint delay_ms) {
    uint32_t base = my_time_us_32();
    while ((int64_t)(my_time_us_32() - base) < (delay_ms * 1000)) {
    }
}

int main() {
    const uint LED_PIN = MY_PICO_DEFAULT_LED_PIN;
    my_gpio_init(LED_PIN);
    my_gpio_set_dir_out(LED_PIN);
    while (true) {
        my_gpio_put(LED_PIN, 1);
        my_sleep_ms(900);
        my_gpio_put(LED_PIN, 0);
        my_sleep_ms(100);
    }
}
