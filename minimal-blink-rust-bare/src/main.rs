#![no_std]
#![no_main]

//
// Note:
// The resulting ELF file after the build must be converted into a UF2 file,
// e.g. with the elf2uf2 tool from the Pico SDK.
//
// See: https://github.com/raspberrypi/pico-sdk/tree/master/tools/elf2uf2
// (this sub directory can be built separately with CMake)
//

mod blink;

// Second stage boot loader
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// Reset vector, points to the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14 + 32] = [
    Vector { handler: DefaultExceptionHandler },    // 2: NMI
    Vector { handler: DefaultExceptionHandler },    // 3: HardFault
    Vector { reserved: 0 },                         // 4
    Vector { reserved: 0 },                         // 5
    Vector { reserved: 0 },                         // 6
    Vector { reserved: 0 },                         // 7
    Vector { reserved: 0 },                         // 8
    Vector { reserved: 0 },                         // 9
    Vector { reserved: 0 },                         // 10
    Vector { handler: DefaultExceptionHandler },    // 11: SVCall
    Vector { reserved: 0 },                         // 12
    Vector { reserved: 0 },                         // 13
    Vector { handler: DefaultExceptionHandler },    // 14: PendSV
    Vector { handler: DefaultExceptionHandler },    // 15: SysTick

    Vector { handler: DefaultExceptionHandler },    // 16: TIMER_IRQ_0
    Vector { handler: DefaultExceptionHandler },    // 17: TIMER_IRQ_1
    Vector { handler: DefaultExceptionHandler },    // 18: TIMER_IRQ_2
    Vector { handler: DefaultExceptionHandler },    // 19: TIMER_IRQ_3
    Vector { handler: DefaultExceptionHandler },    // 20: PWM_IRQ_WRAP
    Vector { handler: DefaultExceptionHandler },    // 21: USBCTRL_IRQ
    Vector { handler: DefaultExceptionHandler },    // 22: XIP_IRQ
    Vector { handler: DefaultExceptionHandler },    // 23: PIO0_IRQ_0
    Vector { handler: DefaultExceptionHandler },    // 24: PIO0_IRQ_1
    Vector { handler: DefaultExceptionHandler },    // 25: PIO1_IRQ_0
    Vector { handler: DefaultExceptionHandler },    // 26: PIO1_IRQ_1
    Vector { handler: DefaultExceptionHandler },    // 27: DMA_IRQ_0
    Vector { handler: DefaultExceptionHandler },    // 28: DMA_IRQ_1
    Vector { handler: DefaultExceptionHandler },    // 29: IO_IRQ_BANK0
    Vector { handler: DefaultExceptionHandler },    // 30: IO_IRQ_QSPI
    Vector { handler: DefaultExceptionHandler },    // 31: SIO_IRQ_PROC0
    Vector { handler: DefaultExceptionHandler },    // 32: SIO_IRQ_PROC1
    Vector { handler: DefaultExceptionHandler },    // 33: CLOCKS_IRQ
    Vector { handler: DefaultExceptionHandler },    // 34: SPI0_IRQ
    Vector { handler: DefaultExceptionHandler },    // 35: SPI1_IRQ
    Vector { handler: DefaultExceptionHandler },    // 36: UART0_IRQ
    Vector { handler: DefaultExceptionHandler },    // 37: UART1_IRQ
    Vector { handler: DefaultExceptionHandler },    // 38: ADC_IRQ_FIFO
    Vector { handler: DefaultExceptionHandler },    // 39: I2C0_IRQ
    Vector { handler: DefaultExceptionHandler },    // 40: I2C1_IRQ
    Vector { handler: DefaultExceptionHandler },    // 41: RTC_IRQ
    Vector { handler: DefaultExceptionHandler },    // 42: unused
    Vector { handler: DefaultExceptionHandler },    // 43: unused
    Vector { handler: DefaultExceptionHandler },    // 44: unused
    Vector { handler: DefaultExceptionHandler },    // 45: unused
    Vector { handler: DefaultExceptionHandler },    // 46: unused
    Vector { handler: DefaultExceptionHandler },    // 47: unused
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    blink::blink_loop();
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
