#include "uart-csr-registers.h"
#include <string.h>
_Static_assert(UARTCSR_CTRL_OFFSET == 0 && UARTCSR_CTRL_MASK == 1, "ctrl");
_Static_assert(UARTCSR_BAUD_DIV_OFFSET == 4 && UARTCSR_BAUD_DIV_MASK == UINT32_MAX, "divider");
_Static_assert(UARTCSR_STATUS_OFFSET == 8 && UARTCSR_STATUS_MASK == 15, "status");
_Static_assert(UARTCSR_TX_DATA_OFFSET == 12 && UARTCSR_TX_DATA_MASK == 255, "tx");
_Static_assert(UARTCSR_RX_DATA_OFFSET == 16 && UARTCSR_RX_DATA_MASK == 255, "rx");
_Static_assert(UARTCSR_EVENT_OFFSET == 20 && UARTCSR_EVENT_MASK == 15, "events");
int main(void) { return strcmp(UARTCSR_EVENT_ACCESS, "W1C"); }
