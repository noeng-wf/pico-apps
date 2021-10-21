const NUMBER_OF_GPIOS: usize = 30;

const RESETS_BASE: usize = 0x4000c000;
const IO_BANK0_BASE: usize = 0x40014000;
const PADS_BANK0_BASE: usize = 0x4001c000;
const SIO_BASE: usize = 0xd0000000;
const TIMER_BASE: usize = 0x40054000;

const RESETS_RESET_OFFSET: usize = 0x00000000;
const RESETS_RESET_DONE_OFFSET: usize = 0x00000008;
const IO_BANK0_GPIO0_STATUS_OFFSET: usize = 0x00000000;
const PADS_BANK0_GPIO0_OFFSET: usize = 0x00000004;
const SIO_GPIO_OUT_SET_OFFSET: usize = 0x00000014;
const SIO_GPIO_OUT_CLR_OFFSET: usize = 0x00000018;
const SIO_GPIO_OE_SET_OFFSET: usize = 0x00000024;
const TIMER_TIMERAWL_OFFSET: usize = 0x00000028;

//const RESETS_RESET_USBCTRL_BITS: u32 = 0x01000000;
//const RESETS_RESET_UART1_BITS: u32 = 0x00800000;
//const RESETS_RESET_UART0_BITS: u32 = 0x00400000;
const RESETS_RESET_TIMER_BITS: u32 = 0x00200000;
//const RESETS_RESET_TBMAN_BITS: u32 = 0x00100000;
//const RESETS_RESET_SYSINFO_BITS: u32 = 0x00080000;
//const RESETS_RESET_SYSCFG_BITS: u32 = 0x00040000;
//const RESETS_RESET_SPI1_BITS: u32 = 0x00020000;
//const RESETS_RESET_SPI0_BITS: u32 = 0x00010000;
//const RESETS_RESET_RTC_BITS: u32 = 0x00008000;
//const RESETS_RESET_PWM_BITS: u32 = 0x00004000;
//const RESETS_RESET_PLL_USB_BITS: u32 = 0x00002000;
//const RESETS_RESET_PLL_SYS_BITS: u32 = 0x00001000;
//const RESETS_RESET_PIO1_BITS: u32 = 0x00000800;
const RESETS_RESET_PIO0_BITS: u32 = 0x00000400;
//const RESETS_RESET_PADS_QSPI_BITS: u32 = 0x00000200;
const RESETS_RESET_PADS_BANK0_BITS: u32 = 0x00000100;
//const RESETS_RESET_JTAG_BITS: u32 = 0x00000080;
//const RESETS_RESET_IO_QSPI_BITS: u32 = 0x00000040;
const RESETS_RESET_IO_BANK0_BITS: u32 = 0x00000020;
//const RESETS_RESET_I2C1_BITS: u32 = 0x00000010;
//const RESETS_RESET_I2C0_BITS: u32 = 0x00000008;
//const RESETS_RESET_DMA_BITS: u32 = 0x00000004;
//const RESETS_RESET_BUSCTRL_BITS: u32 = 0x00000002;
//const RESETS_RESET_ADC_BITS: u32 = 0x00000001;

const PADS_BANK0_GPIO0_IE_BITS: u32 = 0x00000040;
const PADS_BANK0_GPIO0_OD_BITS: u32 = 0x00000080;
const IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB: u32 = 0;

const GPIO_FUNC_SIO: u32 = 5;

const PICO_DEFAULT_LED_PIN: usize = 25;

#[repr(C)]
struct IoBank0HwIo {
    status: u32,
    ctrl: u32,
}

struct Peripherals {
    resets_reset: *mut u32,
    resets_reset_done: *const u32,
    iobank0_io: *mut [IoBank0HwIo; NUMBER_OF_GPIOS],
    padsbank0_io: *mut [u32; NUMBER_OF_GPIOS],
    sio_gpio_out_set: *mut u32,
    sio_gpio_out_clr: *mut u32,
    sio_gpio_oe_set: *mut u32,
    timer_timerawl: *const u32,
}

impl Peripherals {
    fn new() -> Self {
        Self {
            resets_reset: (RESETS_BASE + RESETS_RESET_OFFSET) as *mut u32,
            resets_reset_done: (RESETS_BASE + RESETS_RESET_DONE_OFFSET) as *const u32,
            iobank0_io: (IO_BANK0_BASE + IO_BANK0_GPIO0_STATUS_OFFSET)
                as *mut [IoBank0HwIo; NUMBER_OF_GPIOS],
            padsbank0_io: (PADS_BANK0_BASE + PADS_BANK0_GPIO0_OFFSET)
                as *mut [u32; NUMBER_OF_GPIOS],
            sio_gpio_out_set: (SIO_BASE + SIO_GPIO_OUT_SET_OFFSET) as *mut u32,
            sio_gpio_out_clr: (SIO_BASE + SIO_GPIO_OUT_CLR_OFFSET) as *mut u32,
            sio_gpio_oe_set: (SIO_BASE + SIO_GPIO_OE_SET_OFFSET) as *mut u32,
            timer_timerawl: (TIMER_BASE + TIMER_TIMERAWL_OFFSET) as *const u32,
        }
    }
}

fn start_subsystems(p: &Peripherals) {
    // Subsystems that will be used
    let mask = RESETS_RESET_TIMER_BITS | RESETS_RESET_PIO0_BITS | RESETS_RESET_PADS_BANK0_BITS | RESETS_RESET_IO_BANK0_BITS;

    unsafe {
        // Take subsystems out of reset state
        core::ptr::write_volatile(p.resets_reset, core::ptr::read_volatile(p.resets_reset) & !mask);

        // Wait until reset state has been left
        while (core::ptr::read_volatile(p.resets_reset_done) & mask) != mask {}
    };
}

fn gpio_init_as_sio_output(p: &Peripherals, gpio: usize) {
    // Set input enable on, output disable off
    unsafe {
        core::ptr::write_volatile(
            &mut (*p.padsbank0_io)[gpio],
            (core::ptr::read_volatile(&(*p.padsbank0_io)[gpio])
                & (PADS_BANK0_GPIO0_IE_BITS | PADS_BANK0_GPIO0_OD_BITS))
                | PADS_BANK0_GPIO0_IE_BITS,
        )
    };

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    unsafe {
        core::ptr::write_volatile(
            &mut (*p.iobank0_io)[gpio].ctrl,
            GPIO_FUNC_SIO << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB,
        )
    };

    // Setup GPIO as output
    unsafe { core::ptr::write_volatile(p.sio_gpio_out_clr, 1 << gpio) };
    unsafe { core::ptr::write_volatile(p.sio_gpio_oe_set, 1 << gpio) };
}

fn gpio_put(p: &Peripherals, gpio: usize, value: bool) {
    let mask = 1 << gpio;
    if value {
        unsafe { core::ptr::write_volatile(p.sio_gpio_out_set, mask) };
    } else {
        unsafe { core::ptr::write_volatile(p.sio_gpio_out_clr, mask) };
    }
}

fn sleep_ms(p: &Peripherals, delay_ms: u32) {
    let base = unsafe { core::ptr::read_volatile(p.timer_timerawl) };
    while (unsafe { core::ptr::read_volatile(p.timer_timerawl) } - base) < (delay_ms * 1000) {}
}

pub fn blink_loop() -> ! {
    let p = Peripherals::new();
    const LED_PIN: usize = PICO_DEFAULT_LED_PIN;

    start_subsystems(&p);
    gpio_init_as_sio_output(&p, LED_PIN);
    loop {
        gpio_put(&p, LED_PIN, true);
        sleep_ms(&p, 500);
        gpio_put(&p, LED_PIN, false);
        sleep_ms(&p, 500);
    }
}
