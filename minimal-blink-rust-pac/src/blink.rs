use rp2040_pac::Peripherals;

const PICO_DEFAULT_LED_PIN: usize = 25;

fn gpio_init_as_sio_output(p: &mut Peripherals, gpio: usize) {
    // Set input enable on, output disable off
    p.PADS_BANK0.gpio[gpio].modify(|_r, w| w.ie().set_bit().od().clear_bit());

    // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
    // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
    p.IO_BANK0.gpio[gpio]
        .gpio_ctrl
        .write(|w| w.funcsel().sio_0());

    // Setup GPIO as output
    p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(1 << gpio) });
    p.SIO.gpio_oe_set.write(|w| unsafe { w.bits(1 << gpio) });
}

fn gpio_put(p: &mut Peripherals, gpio: usize, value: bool) {
    let mask = 1 << gpio;

    if value {
        p.SIO.gpio_out_set.write(|w| unsafe { w.bits(mask) });
    } else {
        p.SIO.gpio_out_clr.write(|w| unsafe { w.bits(mask) });
    }
}

fn sleep_ms(p: &Peripherals, delay_ms: u32) {
    let base = p.TIMER.timerawl.read().bits();
    while (p.TIMER.timerawl.read().bits() - base) < (delay_ms * 1000) {}
}

pub fn blink_loop(p: &mut Peripherals) -> ! {
    const LED_PIN: usize = PICO_DEFAULT_LED_PIN;

    gpio_init_as_sio_output(p, LED_PIN);

    loop {
        gpio_put(p, LED_PIN, true);
        sleep_ms(p, 500);
        gpio_put(p, LED_PIN, false);
        sleep_ms(p, 500);
    }
}
