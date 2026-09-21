/* Local byte offsets: caller supplies the base address. */
#ifndef BITLOOM_TIMER_CSR_H
#define BITLOOM_TIMER_CSR_H
#include <stdint.h>

#ifdef TIMER_CTRL_OFFSET
#error "Bitloom CSR macro collision: TIMER_CTRL_OFFSET"
#endif
#ifdef TIMER_CTRL_MASK
#error "Bitloom CSR macro collision: TIMER_CTRL_MASK"
#endif
#ifdef TIMER_CTRL_RESET
#error "Bitloom CSR macro collision: TIMER_CTRL_RESET"
#endif
#ifdef TIMER_CTRL_ACCESS
#error "Bitloom CSR macro collision: TIMER_CTRL_ACCESS"
#endif
#ifdef TIMER_CTRL_BITS_MASK
#error "Bitloom CSR macro collision: TIMER_CTRL_BITS_MASK"
#endif
#ifdef TIMER_CTRL_BITS_RESET
#error "Bitloom CSR macro collision: TIMER_CTRL_BITS_RESET"
#endif
#ifdef TIMER_CTRL_BITS_ACCESS
#error "Bitloom CSR macro collision: TIMER_CTRL_BITS_ACCESS"
#endif
#define TIMER_CTRL_OFFSET UINT32_C(0x00000000)
#define TIMER_CTRL_MASK UINT32_C(0x00000003)
#define TIMER_CTRL_RESET UINT32_C(0x00000000)
#define TIMER_CTRL_ACCESS "RW"
#define TIMER_CTRL_BITS_MASK UINT32_C(0x00000003)
#define TIMER_CTRL_BITS_RESET UINT32_C(0x00000000)
#define TIMER_CTRL_BITS_ACCESS "RW"

#ifdef TIMER_COUNT_OFFSET
#error "Bitloom CSR macro collision: TIMER_COUNT_OFFSET"
#endif
#ifdef TIMER_COUNT_MASK
#error "Bitloom CSR macro collision: TIMER_COUNT_MASK"
#endif
#ifdef TIMER_COUNT_RESET
#error "Bitloom CSR macro collision: TIMER_COUNT_RESET"
#endif
#ifdef TIMER_COUNT_ACCESS
#error "Bitloom CSR macro collision: TIMER_COUNT_ACCESS"
#endif
#ifdef TIMER_COUNT_BITS_MASK
#error "Bitloom CSR macro collision: TIMER_COUNT_BITS_MASK"
#endif
#ifdef TIMER_COUNT_BITS_RESET
#error "Bitloom CSR macro collision: TIMER_COUNT_BITS_RESET"
#endif
#ifdef TIMER_COUNT_BITS_ACCESS
#error "Bitloom CSR macro collision: TIMER_COUNT_BITS_ACCESS"
#endif
#define TIMER_COUNT_OFFSET UINT32_C(0x00000004)
#define TIMER_COUNT_MASK UINT32_C(0xffffffff)
#define TIMER_COUNT_RESET UINT32_C(0x00000000)
#define TIMER_COUNT_ACCESS "RW"
#define TIMER_COUNT_BITS_MASK UINT32_C(0xffffffff)
#define TIMER_COUNT_BITS_RESET UINT32_C(0x00000000)
#define TIMER_COUNT_BITS_ACCESS "RW"

#ifdef TIMER_COMPARE_OFFSET
#error "Bitloom CSR macro collision: TIMER_COMPARE_OFFSET"
#endif
#ifdef TIMER_COMPARE_MASK
#error "Bitloom CSR macro collision: TIMER_COMPARE_MASK"
#endif
#ifdef TIMER_COMPARE_RESET
#error "Bitloom CSR macro collision: TIMER_COMPARE_RESET"
#endif
#ifdef TIMER_COMPARE_ACCESS
#error "Bitloom CSR macro collision: TIMER_COMPARE_ACCESS"
#endif
#ifdef TIMER_COMPARE_BITS_MASK
#error "Bitloom CSR macro collision: TIMER_COMPARE_BITS_MASK"
#endif
#ifdef TIMER_COMPARE_BITS_RESET
#error "Bitloom CSR macro collision: TIMER_COMPARE_BITS_RESET"
#endif
#ifdef TIMER_COMPARE_BITS_ACCESS
#error "Bitloom CSR macro collision: TIMER_COMPARE_BITS_ACCESS"
#endif
#define TIMER_COMPARE_OFFSET UINT32_C(0x00000008)
#define TIMER_COMPARE_MASK UINT32_C(0xffffffff)
#define TIMER_COMPARE_RESET UINT32_C(0x00000000)
#define TIMER_COMPARE_ACCESS "RW"
#define TIMER_COMPARE_BITS_MASK UINT32_C(0xffffffff)
#define TIMER_COMPARE_BITS_RESET UINT32_C(0x00000000)
#define TIMER_COMPARE_BITS_ACCESS "RW"

#ifdef TIMER_EVENT_OFFSET
#error "Bitloom CSR macro collision: TIMER_EVENT_OFFSET"
#endif
#ifdef TIMER_EVENT_MASK
#error "Bitloom CSR macro collision: TIMER_EVENT_MASK"
#endif
#ifdef TIMER_EVENT_RESET
#error "Bitloom CSR macro collision: TIMER_EVENT_RESET"
#endif
#ifdef TIMER_EVENT_ACCESS
#error "Bitloom CSR macro collision: TIMER_EVENT_ACCESS"
#endif
#ifdef TIMER_EVENT_BITS_MASK
#error "Bitloom CSR macro collision: TIMER_EVENT_BITS_MASK"
#endif
#ifdef TIMER_EVENT_BITS_RESET
#error "Bitloom CSR macro collision: TIMER_EVENT_BITS_RESET"
#endif
#ifdef TIMER_EVENT_BITS_ACCESS
#error "Bitloom CSR macro collision: TIMER_EVENT_BITS_ACCESS"
#endif
#define TIMER_EVENT_OFFSET UINT32_C(0x0000000c)
#define TIMER_EVENT_MASK UINT32_C(0x00000001)
#define TIMER_EVENT_RESET UINT32_C(0x00000000)
#define TIMER_EVENT_ACCESS "W1C"
#define TIMER_EVENT_BITS_MASK UINT32_C(0x00000001)
#define TIMER_EVENT_BITS_RESET UINT32_C(0x00000000)
#define TIMER_EVENT_BITS_ACCESS "W1C"

#endif /* BITLOOM_TIMER_CSR_H */
