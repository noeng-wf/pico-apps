/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdbool.h>
#include <stdint.h>

#include "rust-lib/rust-lib.h"

// --------------------------------------------------------------------------------------

typedef volatile uint32_t io_rw_32;
typedef const volatile uint32_t io_ro_32;
typedef volatile uint32_t io_wo_32;

#define IO_BANK0_BASE 0x40014000u
#define PADS_BANK0_BASE 0x4001c000u
#define SIO_BASE 0xd0000000u
#define TIMER_BASE 0x40054000u

#define PADS_BANK0_GPIO0_IE_BITS 0x00000040u
#define PADS_BANK0_GPIO0_OD_BITS 0x00000080u
#define IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB 0u

// --------------------------------------------------------------------------------------

#define IO_BANK0_GPIO0_STATUS_OFFSET 0x00000000

typedef struct {
    io_rw_32 status;
    io_rw_32 ctrl;
} iobank0_hw_io_t;

static iobank0_hw_io_t *const iobank0_io = (iobank0_hw_io_t*)(IO_BANK0_BASE + IO_BANK0_GPIO0_STATUS_OFFSET);

#define PADS_BANK0_GPIO0_OFFSET 0x00000004

static io_rw_32 *const padsbank0_io = (io_rw_32*)(PADS_BANK0_BASE + PADS_BANK0_GPIO0_OFFSET);

#define SIO_GPIO_OUT_SET_OFFSET 0x00000014
#define SIO_GPIO_OUT_CLR_OFFSET 0x00000018
#define SIO_GPIO_OE_SET_OFFSET 0x00000024

static io_rw_32 *const sio_gpio_out_set = (io_rw_32*)(SIO_BASE + SIO_GPIO_OUT_SET_OFFSET);
static io_rw_32 *const sio_gpio_out_clr = (io_rw_32*)(SIO_BASE + SIO_GPIO_OUT_CLR_OFFSET);
static io_rw_32 *const sio_gpio_oe_set = (io_rw_32*)(SIO_BASE + SIO_GPIO_OE_SET_OFFSET);

#define TIMER_TIMERAWL_OFFSET 0x00000028

static io_ro_32 *const timer_timerawl = (io_ro_32*)(TIMER_BASE + TIMER_TIMERAWL_OFFSET);

// --------------------------------------------------------------------------------------

#define GPIO_FUNC_SIO 5

// --------------------------------------------------------------------------------------

#define MY_PICO_DEFAULT_LED_PIN 25

void gpio_init_as_sio_output(uint32_t gpio) {
    // Set input enable on, output disable off
    padsbank0_io[gpio] = (padsbank0_io[gpio] & (PADS_BANK0_GPIO0_IE_BITS | PADS_BANK0_GPIO0_OD_BITS)) | PADS_BANK0_GPIO0_IE_BITS;

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    iobank0_io[gpio].ctrl = GPIO_FUNC_SIO << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB;

    // Setup GPIO as output
    *sio_gpio_out_clr = 1ul << gpio;
    *sio_gpio_oe_set = 1ul << gpio;
}

void my_gpio_put(uint32_t gpio, bool value) {
    uint32_t mask = 1ul << gpio;
    if (value)
        *sio_gpio_out_set = mask;
    else
        *sio_gpio_out_clr = mask;
}

void my_sleep_ms(uint32_t delay_ms) {
    uint32_t base = *timer_timerawl;
    while ((*timer_timerawl - base) < (delay_ms * 1000)) {
    }
}

int main() {
    const uint32_t DELAY_MS = get_delay_value(); // Call Rust code

    const uint32_t LED_PIN = MY_PICO_DEFAULT_LED_PIN;
    gpio_init_as_sio_output(LED_PIN);
    while (true) {
        my_gpio_put(LED_PIN, 1);
        my_sleep_ms(DELAY_MS);
        my_gpio_put(LED_PIN, 0);
        my_sleep_ms(DELAY_MS);
    }
}
