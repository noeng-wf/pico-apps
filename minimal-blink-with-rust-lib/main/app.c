#include "hw.h"

#include "../lib/app.h"

void run_app() {
    hw_led_init();
    while (true) {
        uint32_t dim_value = get_next_dim_value();

        for (uint32_t i = 0; i < 10; i++) {
            hw_led_set(true);
            hw_sleep_ms(dim_value);
            hw_led_set(false);
            hw_sleep_ms(10 - dim_value);
        }
    }
}
