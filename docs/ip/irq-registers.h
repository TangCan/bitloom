/* Local byte offsets: caller supplies the base address. */
#ifndef BITLOOM_IRQ_CSR_H
#define BITLOOM_IRQ_CSR_H
#include <stdint.h>

#ifdef IRQ_PENDING_OFFSET
#error "Bitloom CSR macro collision: IRQ_PENDING_OFFSET"
#endif
#ifdef IRQ_PENDING_MASK
#error "Bitloom CSR macro collision: IRQ_PENDING_MASK"
#endif
#ifdef IRQ_PENDING_RESET
#error "Bitloom CSR macro collision: IRQ_PENDING_RESET"
#endif
#ifdef IRQ_PENDING_ACCESS
#error "Bitloom CSR macro collision: IRQ_PENDING_ACCESS"
#endif
#ifdef IRQ_PENDING_TIMER_MASK
#error "Bitloom CSR macro collision: IRQ_PENDING_TIMER_MASK"
#endif
#ifdef IRQ_PENDING_TIMER_RESET
#error "Bitloom CSR macro collision: IRQ_PENDING_TIMER_RESET"
#endif
#ifdef IRQ_PENDING_TIMER_ACCESS
#error "Bitloom CSR macro collision: IRQ_PENDING_TIMER_ACCESS"
#endif
#ifdef IRQ_PENDING_UART_RX_MASK
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_RX_MASK"
#endif
#ifdef IRQ_PENDING_UART_RX_RESET
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_RX_RESET"
#endif
#ifdef IRQ_PENDING_UART_RX_ACCESS
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_RX_ACCESS"
#endif
#ifdef IRQ_PENDING_UART_TX_MASK
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_TX_MASK"
#endif
#ifdef IRQ_PENDING_UART_TX_RESET
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_TX_RESET"
#endif
#ifdef IRQ_PENDING_UART_TX_ACCESS
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_TX_ACCESS"
#endif
#ifdef IRQ_PENDING_UART_ERROR_MASK
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_ERROR_MASK"
#endif
#ifdef IRQ_PENDING_UART_ERROR_RESET
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_ERROR_RESET"
#endif
#ifdef IRQ_PENDING_UART_ERROR_ACCESS
#error "Bitloom CSR macro collision: IRQ_PENDING_UART_ERROR_ACCESS"
#endif
#ifdef IRQ_PENDING_GPIO_MASK
#error "Bitloom CSR macro collision: IRQ_PENDING_GPIO_MASK"
#endif
#ifdef IRQ_PENDING_GPIO_RESET
#error "Bitloom CSR macro collision: IRQ_PENDING_GPIO_RESET"
#endif
#ifdef IRQ_PENDING_GPIO_ACCESS
#error "Bitloom CSR macro collision: IRQ_PENDING_GPIO_ACCESS"
#endif
#define IRQ_PENDING_OFFSET UINT32_C(0x00000000)
#define IRQ_PENDING_MASK UINT32_C(0x0000001f)
#define IRQ_PENDING_RESET UINT32_C(0x00000000)
#define IRQ_PENDING_ACCESS "W1C"
#define IRQ_PENDING_TIMER_MASK UINT32_C(0x00000001)
#define IRQ_PENDING_TIMER_RESET UINT32_C(0x00000000)
#define IRQ_PENDING_TIMER_ACCESS "W1C"
#define IRQ_PENDING_UART_RX_MASK UINT32_C(0x00000002)
#define IRQ_PENDING_UART_RX_RESET UINT32_C(0x00000000)
#define IRQ_PENDING_UART_RX_ACCESS "W1C"
#define IRQ_PENDING_UART_TX_MASK UINT32_C(0x00000004)
#define IRQ_PENDING_UART_TX_RESET UINT32_C(0x00000000)
#define IRQ_PENDING_UART_TX_ACCESS "W1C"
#define IRQ_PENDING_UART_ERROR_MASK UINT32_C(0x00000008)
#define IRQ_PENDING_UART_ERROR_RESET UINT32_C(0x00000000)
#define IRQ_PENDING_UART_ERROR_ACCESS "W1C"
#define IRQ_PENDING_GPIO_MASK UINT32_C(0x00000010)
#define IRQ_PENDING_GPIO_RESET UINT32_C(0x00000000)
#define IRQ_PENDING_GPIO_ACCESS "W1C"

#ifdef IRQ_ENABLE_OFFSET
#error "Bitloom CSR macro collision: IRQ_ENABLE_OFFSET"
#endif
#ifdef IRQ_ENABLE_MASK
#error "Bitloom CSR macro collision: IRQ_ENABLE_MASK"
#endif
#ifdef IRQ_ENABLE_RESET
#error "Bitloom CSR macro collision: IRQ_ENABLE_RESET"
#endif
#ifdef IRQ_ENABLE_ACCESS
#error "Bitloom CSR macro collision: IRQ_ENABLE_ACCESS"
#endif
#ifdef IRQ_ENABLE_TIMER_MASK
#error "Bitloom CSR macro collision: IRQ_ENABLE_TIMER_MASK"
#endif
#ifdef IRQ_ENABLE_TIMER_RESET
#error "Bitloom CSR macro collision: IRQ_ENABLE_TIMER_RESET"
#endif
#ifdef IRQ_ENABLE_TIMER_ACCESS
#error "Bitloom CSR macro collision: IRQ_ENABLE_TIMER_ACCESS"
#endif
#ifdef IRQ_ENABLE_UART_RX_MASK
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_RX_MASK"
#endif
#ifdef IRQ_ENABLE_UART_RX_RESET
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_RX_RESET"
#endif
#ifdef IRQ_ENABLE_UART_RX_ACCESS
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_RX_ACCESS"
#endif
#ifdef IRQ_ENABLE_UART_TX_MASK
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_TX_MASK"
#endif
#ifdef IRQ_ENABLE_UART_TX_RESET
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_TX_RESET"
#endif
#ifdef IRQ_ENABLE_UART_TX_ACCESS
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_TX_ACCESS"
#endif
#ifdef IRQ_ENABLE_UART_ERROR_MASK
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_ERROR_MASK"
#endif
#ifdef IRQ_ENABLE_UART_ERROR_RESET
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_ERROR_RESET"
#endif
#ifdef IRQ_ENABLE_UART_ERROR_ACCESS
#error "Bitloom CSR macro collision: IRQ_ENABLE_UART_ERROR_ACCESS"
#endif
#ifdef IRQ_ENABLE_GPIO_MASK
#error "Bitloom CSR macro collision: IRQ_ENABLE_GPIO_MASK"
#endif
#ifdef IRQ_ENABLE_GPIO_RESET
#error "Bitloom CSR macro collision: IRQ_ENABLE_GPIO_RESET"
#endif
#ifdef IRQ_ENABLE_GPIO_ACCESS
#error "Bitloom CSR macro collision: IRQ_ENABLE_GPIO_ACCESS"
#endif
#define IRQ_ENABLE_OFFSET UINT32_C(0x00000004)
#define IRQ_ENABLE_MASK UINT32_C(0x0000001f)
#define IRQ_ENABLE_RESET UINT32_C(0x00000000)
#define IRQ_ENABLE_ACCESS "RW"
#define IRQ_ENABLE_TIMER_MASK UINT32_C(0x00000001)
#define IRQ_ENABLE_TIMER_RESET UINT32_C(0x00000000)
#define IRQ_ENABLE_TIMER_ACCESS "RW"
#define IRQ_ENABLE_UART_RX_MASK UINT32_C(0x00000002)
#define IRQ_ENABLE_UART_RX_RESET UINT32_C(0x00000000)
#define IRQ_ENABLE_UART_RX_ACCESS "RW"
#define IRQ_ENABLE_UART_TX_MASK UINT32_C(0x00000004)
#define IRQ_ENABLE_UART_TX_RESET UINT32_C(0x00000000)
#define IRQ_ENABLE_UART_TX_ACCESS "RW"
#define IRQ_ENABLE_UART_ERROR_MASK UINT32_C(0x00000008)
#define IRQ_ENABLE_UART_ERROR_RESET UINT32_C(0x00000000)
#define IRQ_ENABLE_UART_ERROR_ACCESS "RW"
#define IRQ_ENABLE_GPIO_MASK UINT32_C(0x00000010)
#define IRQ_ENABLE_GPIO_RESET UINT32_C(0x00000000)
#define IRQ_ENABLE_GPIO_ACCESS "RW"

#ifdef IRQ_TEST_OFFSET
#error "Bitloom CSR macro collision: IRQ_TEST_OFFSET"
#endif
#ifdef IRQ_TEST_MASK
#error "Bitloom CSR macro collision: IRQ_TEST_MASK"
#endif
#ifdef IRQ_TEST_RESET
#error "Bitloom CSR macro collision: IRQ_TEST_RESET"
#endif
#ifdef IRQ_TEST_ACCESS
#error "Bitloom CSR macro collision: IRQ_TEST_ACCESS"
#endif
#ifdef IRQ_TEST_TIMER_MASK
#error "Bitloom CSR macro collision: IRQ_TEST_TIMER_MASK"
#endif
#ifdef IRQ_TEST_TIMER_RESET
#error "Bitloom CSR macro collision: IRQ_TEST_TIMER_RESET"
#endif
#ifdef IRQ_TEST_TIMER_ACCESS
#error "Bitloom CSR macro collision: IRQ_TEST_TIMER_ACCESS"
#endif
#ifdef IRQ_TEST_UART_RX_MASK
#error "Bitloom CSR macro collision: IRQ_TEST_UART_RX_MASK"
#endif
#ifdef IRQ_TEST_UART_RX_RESET
#error "Bitloom CSR macro collision: IRQ_TEST_UART_RX_RESET"
#endif
#ifdef IRQ_TEST_UART_RX_ACCESS
#error "Bitloom CSR macro collision: IRQ_TEST_UART_RX_ACCESS"
#endif
#ifdef IRQ_TEST_UART_TX_MASK
#error "Bitloom CSR macro collision: IRQ_TEST_UART_TX_MASK"
#endif
#ifdef IRQ_TEST_UART_TX_RESET
#error "Bitloom CSR macro collision: IRQ_TEST_UART_TX_RESET"
#endif
#ifdef IRQ_TEST_UART_TX_ACCESS
#error "Bitloom CSR macro collision: IRQ_TEST_UART_TX_ACCESS"
#endif
#ifdef IRQ_TEST_UART_ERROR_MASK
#error "Bitloom CSR macro collision: IRQ_TEST_UART_ERROR_MASK"
#endif
#ifdef IRQ_TEST_UART_ERROR_RESET
#error "Bitloom CSR macro collision: IRQ_TEST_UART_ERROR_RESET"
#endif
#ifdef IRQ_TEST_UART_ERROR_ACCESS
#error "Bitloom CSR macro collision: IRQ_TEST_UART_ERROR_ACCESS"
#endif
#ifdef IRQ_TEST_GPIO_MASK
#error "Bitloom CSR macro collision: IRQ_TEST_GPIO_MASK"
#endif
#ifdef IRQ_TEST_GPIO_RESET
#error "Bitloom CSR macro collision: IRQ_TEST_GPIO_RESET"
#endif
#ifdef IRQ_TEST_GPIO_ACCESS
#error "Bitloom CSR macro collision: IRQ_TEST_GPIO_ACCESS"
#endif
#define IRQ_TEST_OFFSET UINT32_C(0x00000008)
#define IRQ_TEST_MASK UINT32_C(0x0000001f)
#define IRQ_TEST_RESET UINT32_C(0x00000000)
#define IRQ_TEST_ACCESS "WO"
#define IRQ_TEST_TIMER_MASK UINT32_C(0x00000001)
#define IRQ_TEST_TIMER_RESET UINT32_C(0x00000000)
#define IRQ_TEST_TIMER_ACCESS "WO"
#define IRQ_TEST_UART_RX_MASK UINT32_C(0x00000002)
#define IRQ_TEST_UART_RX_RESET UINT32_C(0x00000000)
#define IRQ_TEST_UART_RX_ACCESS "WO"
#define IRQ_TEST_UART_TX_MASK UINT32_C(0x00000004)
#define IRQ_TEST_UART_TX_RESET UINT32_C(0x00000000)
#define IRQ_TEST_UART_TX_ACCESS "WO"
#define IRQ_TEST_UART_ERROR_MASK UINT32_C(0x00000008)
#define IRQ_TEST_UART_ERROR_RESET UINT32_C(0x00000000)
#define IRQ_TEST_UART_ERROR_ACCESS "WO"
#define IRQ_TEST_GPIO_MASK UINT32_C(0x00000010)
#define IRQ_TEST_GPIO_RESET UINT32_C(0x00000000)
#define IRQ_TEST_GPIO_ACCESS "WO"

#ifdef IRQ_RAW_OFFSET
#error "Bitloom CSR macro collision: IRQ_RAW_OFFSET"
#endif
#ifdef IRQ_RAW_MASK
#error "Bitloom CSR macro collision: IRQ_RAW_MASK"
#endif
#ifdef IRQ_RAW_RESET
#error "Bitloom CSR macro collision: IRQ_RAW_RESET"
#endif
#ifdef IRQ_RAW_ACCESS
#error "Bitloom CSR macro collision: IRQ_RAW_ACCESS"
#endif
#ifdef IRQ_RAW_TIMER_MASK
#error "Bitloom CSR macro collision: IRQ_RAW_TIMER_MASK"
#endif
#ifdef IRQ_RAW_TIMER_RESET
#error "Bitloom CSR macro collision: IRQ_RAW_TIMER_RESET"
#endif
#ifdef IRQ_RAW_TIMER_ACCESS
#error "Bitloom CSR macro collision: IRQ_RAW_TIMER_ACCESS"
#endif
#ifdef IRQ_RAW_UART_RX_MASK
#error "Bitloom CSR macro collision: IRQ_RAW_UART_RX_MASK"
#endif
#ifdef IRQ_RAW_UART_RX_RESET
#error "Bitloom CSR macro collision: IRQ_RAW_UART_RX_RESET"
#endif
#ifdef IRQ_RAW_UART_RX_ACCESS
#error "Bitloom CSR macro collision: IRQ_RAW_UART_RX_ACCESS"
#endif
#ifdef IRQ_RAW_UART_TX_MASK
#error "Bitloom CSR macro collision: IRQ_RAW_UART_TX_MASK"
#endif
#ifdef IRQ_RAW_UART_TX_RESET
#error "Bitloom CSR macro collision: IRQ_RAW_UART_TX_RESET"
#endif
#ifdef IRQ_RAW_UART_TX_ACCESS
#error "Bitloom CSR macro collision: IRQ_RAW_UART_TX_ACCESS"
#endif
#ifdef IRQ_RAW_UART_ERROR_MASK
#error "Bitloom CSR macro collision: IRQ_RAW_UART_ERROR_MASK"
#endif
#ifdef IRQ_RAW_UART_ERROR_RESET
#error "Bitloom CSR macro collision: IRQ_RAW_UART_ERROR_RESET"
#endif
#ifdef IRQ_RAW_UART_ERROR_ACCESS
#error "Bitloom CSR macro collision: IRQ_RAW_UART_ERROR_ACCESS"
#endif
#ifdef IRQ_RAW_GPIO_MASK
#error "Bitloom CSR macro collision: IRQ_RAW_GPIO_MASK"
#endif
#ifdef IRQ_RAW_GPIO_RESET
#error "Bitloom CSR macro collision: IRQ_RAW_GPIO_RESET"
#endif
#ifdef IRQ_RAW_GPIO_ACCESS
#error "Bitloom CSR macro collision: IRQ_RAW_GPIO_ACCESS"
#endif
#define IRQ_RAW_OFFSET UINT32_C(0x0000000c)
#define IRQ_RAW_MASK UINT32_C(0x0000001f)
#define IRQ_RAW_RESET UINT32_C(0x00000000)
#define IRQ_RAW_ACCESS "RO"
#define IRQ_RAW_TIMER_MASK UINT32_C(0x00000001)
#define IRQ_RAW_TIMER_RESET UINT32_C(0x00000000)
#define IRQ_RAW_TIMER_ACCESS "RO"
#define IRQ_RAW_UART_RX_MASK UINT32_C(0x00000002)
#define IRQ_RAW_UART_RX_RESET UINT32_C(0x00000000)
#define IRQ_RAW_UART_RX_ACCESS "RO"
#define IRQ_RAW_UART_TX_MASK UINT32_C(0x00000004)
#define IRQ_RAW_UART_TX_RESET UINT32_C(0x00000000)
#define IRQ_RAW_UART_TX_ACCESS "RO"
#define IRQ_RAW_UART_ERROR_MASK UINT32_C(0x00000008)
#define IRQ_RAW_UART_ERROR_RESET UINT32_C(0x00000000)
#define IRQ_RAW_UART_ERROR_ACCESS "RO"
#define IRQ_RAW_GPIO_MASK UINT32_C(0x00000010)
#define IRQ_RAW_GPIO_RESET UINT32_C(0x00000000)
#define IRQ_RAW_GPIO_ACCESS "RO"

#endif /* BITLOOM_IRQ_CSR_H */
