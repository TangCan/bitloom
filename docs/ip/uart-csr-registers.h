/* Local byte offsets: caller supplies the base address. */
#ifndef BITLOOM_UARTCSR_CSR_H
#define BITLOOM_UARTCSR_CSR_H
#include <stdint.h>

#ifdef UARTCSR_CTRL_OFFSET
#error "Bitloom CSR macro collision: UARTCSR_CTRL_OFFSET"
#endif
#ifdef UARTCSR_CTRL_MASK
#error "Bitloom CSR macro collision: UARTCSR_CTRL_MASK"
#endif
#ifdef UARTCSR_CTRL_RESET
#error "Bitloom CSR macro collision: UARTCSR_CTRL_RESET"
#endif
#ifdef UARTCSR_CTRL_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_CTRL_ACCESS"
#endif
#ifdef UARTCSR_CTRL_BITS_MASK
#error "Bitloom CSR macro collision: UARTCSR_CTRL_BITS_MASK"
#endif
#ifdef UARTCSR_CTRL_BITS_RESET
#error "Bitloom CSR macro collision: UARTCSR_CTRL_BITS_RESET"
#endif
#ifdef UARTCSR_CTRL_BITS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_CTRL_BITS_ACCESS"
#endif
#define UARTCSR_CTRL_OFFSET UINT32_C(0x00000000)
#define UARTCSR_CTRL_MASK UINT32_C(0x00000001)
#define UARTCSR_CTRL_RESET UINT32_C(0x00000000)
#define UARTCSR_CTRL_ACCESS "RW"
#define UARTCSR_CTRL_BITS_MASK UINT32_C(0x00000001)
#define UARTCSR_CTRL_BITS_RESET UINT32_C(0x00000000)
#define UARTCSR_CTRL_BITS_ACCESS "RW"

#ifdef UARTCSR_BAUD_DIV_OFFSET
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_OFFSET"
#endif
#ifdef UARTCSR_BAUD_DIV_MASK
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_MASK"
#endif
#ifdef UARTCSR_BAUD_DIV_RESET
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_RESET"
#endif
#ifdef UARTCSR_BAUD_DIV_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_ACCESS"
#endif
#ifdef UARTCSR_BAUD_DIV_BITS_MASK
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_BITS_MASK"
#endif
#ifdef UARTCSR_BAUD_DIV_BITS_RESET
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_BITS_RESET"
#endif
#ifdef UARTCSR_BAUD_DIV_BITS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_BAUD_DIV_BITS_ACCESS"
#endif
#define UARTCSR_BAUD_DIV_OFFSET UINT32_C(0x00000004)
#define UARTCSR_BAUD_DIV_MASK UINT32_C(0xffffffff)
#define UARTCSR_BAUD_DIV_RESET UINT32_C(0x00000000)
#define UARTCSR_BAUD_DIV_ACCESS "RW"
#define UARTCSR_BAUD_DIV_BITS_MASK UINT32_C(0xffffffff)
#define UARTCSR_BAUD_DIV_BITS_RESET UINT32_C(0x00000000)
#define UARTCSR_BAUD_DIV_BITS_ACCESS "RW"

#ifdef UARTCSR_STATUS_OFFSET
#error "Bitloom CSR macro collision: UARTCSR_STATUS_OFFSET"
#endif
#ifdef UARTCSR_STATUS_MASK
#error "Bitloom CSR macro collision: UARTCSR_STATUS_MASK"
#endif
#ifdef UARTCSR_STATUS_RESET
#error "Bitloom CSR macro collision: UARTCSR_STATUS_RESET"
#endif
#ifdef UARTCSR_STATUS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_STATUS_ACCESS"
#endif
#ifdef UARTCSR_STATUS_BITS_MASK
#error "Bitloom CSR macro collision: UARTCSR_STATUS_BITS_MASK"
#endif
#ifdef UARTCSR_STATUS_BITS_RESET
#error "Bitloom CSR macro collision: UARTCSR_STATUS_BITS_RESET"
#endif
#ifdef UARTCSR_STATUS_BITS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_STATUS_BITS_ACCESS"
#endif
#define UARTCSR_STATUS_OFFSET UINT32_C(0x00000008)
#define UARTCSR_STATUS_MASK UINT32_C(0x0000000f)
#define UARTCSR_STATUS_RESET UINT32_C(0x00000000)
#define UARTCSR_STATUS_ACCESS "RO"
#define UARTCSR_STATUS_BITS_MASK UINT32_C(0x0000000f)
#define UARTCSR_STATUS_BITS_RESET UINT32_C(0x00000000)
#define UARTCSR_STATUS_BITS_ACCESS "RO"

#ifdef UARTCSR_TX_DATA_OFFSET
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_OFFSET"
#endif
#ifdef UARTCSR_TX_DATA_MASK
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_MASK"
#endif
#ifdef UARTCSR_TX_DATA_RESET
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_RESET"
#endif
#ifdef UARTCSR_TX_DATA_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_ACCESS"
#endif
#ifdef UARTCSR_TX_DATA_BITS_MASK
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_BITS_MASK"
#endif
#ifdef UARTCSR_TX_DATA_BITS_RESET
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_BITS_RESET"
#endif
#ifdef UARTCSR_TX_DATA_BITS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_TX_DATA_BITS_ACCESS"
#endif
#define UARTCSR_TX_DATA_OFFSET UINT32_C(0x0000000c)
#define UARTCSR_TX_DATA_MASK UINT32_C(0x000000ff)
#define UARTCSR_TX_DATA_RESET UINT32_C(0x00000000)
#define UARTCSR_TX_DATA_ACCESS "WO"
#define UARTCSR_TX_DATA_BITS_MASK UINT32_C(0x000000ff)
#define UARTCSR_TX_DATA_BITS_RESET UINT32_C(0x00000000)
#define UARTCSR_TX_DATA_BITS_ACCESS "WO"

#ifdef UARTCSR_RX_DATA_OFFSET
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_OFFSET"
#endif
#ifdef UARTCSR_RX_DATA_MASK
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_MASK"
#endif
#ifdef UARTCSR_RX_DATA_RESET
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_RESET"
#endif
#ifdef UARTCSR_RX_DATA_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_ACCESS"
#endif
#ifdef UARTCSR_RX_DATA_BITS_MASK
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_BITS_MASK"
#endif
#ifdef UARTCSR_RX_DATA_BITS_RESET
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_BITS_RESET"
#endif
#ifdef UARTCSR_RX_DATA_BITS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_RX_DATA_BITS_ACCESS"
#endif
#define UARTCSR_RX_DATA_OFFSET UINT32_C(0x00000010)
#define UARTCSR_RX_DATA_MASK UINT32_C(0x000000ff)
#define UARTCSR_RX_DATA_RESET UINT32_C(0x00000000)
#define UARTCSR_RX_DATA_ACCESS "RO"
#define UARTCSR_RX_DATA_BITS_MASK UINT32_C(0x000000ff)
#define UARTCSR_RX_DATA_BITS_RESET UINT32_C(0x00000000)
#define UARTCSR_RX_DATA_BITS_ACCESS "RO"

#ifdef UARTCSR_EVENT_OFFSET
#error "Bitloom CSR macro collision: UARTCSR_EVENT_OFFSET"
#endif
#ifdef UARTCSR_EVENT_MASK
#error "Bitloom CSR macro collision: UARTCSR_EVENT_MASK"
#endif
#ifdef UARTCSR_EVENT_RESET
#error "Bitloom CSR macro collision: UARTCSR_EVENT_RESET"
#endif
#ifdef UARTCSR_EVENT_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_EVENT_ACCESS"
#endif
#ifdef UARTCSR_EVENT_BITS_MASK
#error "Bitloom CSR macro collision: UARTCSR_EVENT_BITS_MASK"
#endif
#ifdef UARTCSR_EVENT_BITS_RESET
#error "Bitloom CSR macro collision: UARTCSR_EVENT_BITS_RESET"
#endif
#ifdef UARTCSR_EVENT_BITS_ACCESS
#error "Bitloom CSR macro collision: UARTCSR_EVENT_BITS_ACCESS"
#endif
#define UARTCSR_EVENT_OFFSET UINT32_C(0x00000014)
#define UARTCSR_EVENT_MASK UINT32_C(0x0000000f)
#define UARTCSR_EVENT_RESET UINT32_C(0x00000000)
#define UARTCSR_EVENT_ACCESS "W1C"
#define UARTCSR_EVENT_BITS_MASK UINT32_C(0x0000000f)
#define UARTCSR_EVENT_BITS_RESET UINT32_C(0x00000000)
#define UARTCSR_EVENT_BITS_ACCESS "W1C"

#endif /* BITLOOM_UARTCSR_CSR_H */
