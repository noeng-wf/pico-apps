#include "hw.h"

uint32_t get_next_dim_value() {
    static uint32_t values[] = { 0, 0, 1, 2, 4, 10, 10, 4, 2, 1 };
    static uint32_t index = 0;

    uint32_t value = values[index];
    index = (index + 1) % (sizeof(values) / sizeof(values[0]));
 
    return value;
}

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
