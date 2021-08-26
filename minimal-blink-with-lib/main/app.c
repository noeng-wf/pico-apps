#include "hw.h"

void run_app() {
    hw_led_init();
    while (true) {
        hw_led_set(true);
        hw_sleep_ms(500);
        hw_led_set(false);
        hw_sleep_ms(500);
    }
}
