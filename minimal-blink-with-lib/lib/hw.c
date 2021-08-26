#include <stdbool.h>
#include <stdint.h>

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

typedef struct {
    io_rw_32 inte[4];
    io_rw_32 intf[4];
    io_rw_32 ints[4];
} io_irq_ctrl_hw_t;

typedef struct {
    struct {
        io_rw_32 status;
        io_rw_32 ctrl;
    } io[30];
    io_rw_32 intr[4];
    io_irq_ctrl_hw_t proc0_irq_ctrl;
    io_irq_ctrl_hw_t proc1_irq_ctrl;
    io_irq_ctrl_hw_t dormant_wake_irq_ctrl;
} iobank0_hw_t;

#define iobank0_hw ((iobank0_hw_t *const)IO_BANK0_BASE)

typedef struct {
    io_rw_32 voltage_select;
    io_rw_32 io[30];
} padsbank0_hw_t;

#define padsbank0_hw ((padsbank0_hw_t *)PADS_BANK0_BASE)

typedef struct {
    io_rw_32 accum[2];
    io_rw_32 base[3];
    io_ro_32 pop[3];
    io_ro_32 peek[3];
    io_rw_32 ctrl[2];
    io_rw_32 add_raw[2];
    io_wo_32 base01;
} interp_hw_t;

typedef struct {
    io_ro_32 cpuid;
    io_ro_32 gpio_in;
    io_ro_32 gpio_hi_in;
    uint32_t _pad;

    io_rw_32 gpio_out;
    io_wo_32 gpio_set;
    io_wo_32 gpio_clr;
    io_wo_32 gpio_togl;

    io_wo_32 gpio_oe;
    io_wo_32 gpio_oe_set;
    io_wo_32 gpio_oe_clr;
    io_wo_32 gpio_oe_togl;

    io_rw_32 gpio_hi_out;
    io_wo_32 gpio_hi_set;
    io_wo_32 gpio_hi_clr;
    io_wo_32 gpio_hi_togl;

    io_wo_32 gpio_hi_oe;
    io_wo_32 gpio_hi_oe_set;
    io_wo_32 gpio_hi_oe_clr;
    io_wo_32 gpio_hi_oe_togl;

    io_rw_32 fifo_st;
    io_wo_32 fifo_wr;
    io_ro_32 fifo_rd;
    io_ro_32 spinlock_st;

    io_rw_32 div_udividend;
    io_rw_32 div_udivisor;
    io_rw_32 div_sdividend;
    io_rw_32 div_sdivisor;

    io_rw_32 div_quotient;
    io_rw_32 div_remainder;
    io_rw_32 div_csr;

    uint32_t _pad2;

    interp_hw_t interp[2];
} sio_hw_t;

#define sio_hw ((sio_hw_t *)SIO_BASE)

#define NUM_TIMERS 4

typedef struct {
    io_wo_32 timehw;
    io_wo_32 timelw;
    io_ro_32 timehr;
    io_ro_32 timelr;
    io_rw_32 alarm[NUM_TIMERS];
    io_rw_32 armed;
    io_ro_32 timerawh;
    io_ro_32 timerawl;
    io_rw_32 dbgpause;
    io_rw_32 pause;
    io_rw_32 intr;
    io_rw_32 inte;
    io_rw_32 intf;
    io_ro_32 ints;
} timer_hw_t;

#define timer_hw ((timer_hw_t *const)TIMER_BASE)

// --------------------------------------------------------------------------------------

#define MY_GPIO_FUNC_SIO 5

// --------------------------------------------------------------------------------------

#define MY_PICO_DEFAULT_LED_PIN 25

static void my_gpio_set_function(uint32_t gpio, uint32_t fn) {
    // Set input enable on, output disable off
    padsbank0_hw->io[gpio] = (padsbank0_hw->io[gpio] & (PADS_BANK0_GPIO0_IE_BITS | PADS_BANK0_GPIO0_OD_BITS)) | PADS_BANK0_GPIO0_IE_BITS;

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    iobank0_hw->io[gpio].ctrl = fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB;
}

static void my_gpio_init(uint32_t gpio) {
    sio_hw->gpio_oe_clr = 1ul << gpio;
    sio_hw->gpio_clr = 1ul << gpio;
    my_gpio_set_function(gpio, MY_GPIO_FUNC_SIO);
}

static void my_gpio_set_dir_out(uint32_t gpio) {
    sio_hw->gpio_oe_set = 1ul << gpio;
}

static void my_gpio_put(uint32_t gpio, bool value) {
    uint32_t mask = 1ul << gpio;
    if (value)
        sio_hw->gpio_set = mask;
    else
        sio_hw->gpio_clr = mask;
}

static uint32_t my_time_us_32() {
    return timer_hw->timerawl;
}

static void my_sleep_ms(uint32_t delay_ms) {
    uint32_t base = my_time_us_32();
    while ((int64_t)(my_time_us_32() - base) < (delay_ms * 1000)) {
    }
}

void hw_led_init() {
    my_gpio_init(MY_PICO_DEFAULT_LED_PIN);
    my_gpio_set_dir_out(MY_PICO_DEFAULT_LED_PIN);
}

void hw_led_set(bool state) {
    my_gpio_put(MY_PICO_DEFAULT_LED_PIN, state ? 1 : 0);
}

void hw_sleep_ms(uint32_t delay_ms) {
    my_sleep_ms(delay_ms);
}
