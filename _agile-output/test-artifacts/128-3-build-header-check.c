#include "irq-registers.h"
_Static_assert(IRQ_PENDING_OFFSET == 0 && IRQ_ENABLE_OFFSET == 4, "local offsets");
_Static_assert(IRQ_TEST_OFFSET == 8 && IRQ_RAW_OFFSET == 12, "local offsets");
_Static_assert(IRQ_PENDING_MASK == 31 && IRQ_ENABLE_MASK == 31 && IRQ_TEST_MASK == 31 && IRQ_RAW_MASK == 31, "five sources");
_Static_assert(IRQ_PENDING_TIMER_MASK == 1 && IRQ_PENDING_UART_RX_MASK == 2 && IRQ_PENDING_UART_TX_MASK == 4 && IRQ_PENDING_UART_ERROR_MASK == 8 && IRQ_PENDING_GPIO_MASK == 16, "fixed source order");
int main(void) { return 0; }
