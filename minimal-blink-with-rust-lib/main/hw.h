#include <stdbool.h>
#include <stdint.h>

void hw_led_init();
void hw_led_set(bool state);
void hw_sleep_ms(uint32_t delay_ms);
