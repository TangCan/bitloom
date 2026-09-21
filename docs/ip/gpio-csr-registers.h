/* Local byte offsets: caller supplies the base address. */
#ifndef BITLOOM_GPIOCSR_CSR_H
#define BITLOOM_GPIOCSR_CSR_H
#include <stdint.h>

#ifdef GPIOCSR_DIR_OFFSET
#error "Bitloom CSR macro collision: GPIOCSR_DIR_OFFSET"
#endif
#ifdef GPIOCSR_DIR_MASK
#error "Bitloom CSR macro collision: GPIOCSR_DIR_MASK"
#endif
#ifdef GPIOCSR_DIR_RESET
#error "Bitloom CSR macro collision: GPIOCSR_DIR_RESET"
#endif
#ifdef GPIOCSR_DIR_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_DIR_ACCESS"
#endif
#ifdef GPIOCSR_DIR_BITS_MASK
#error "Bitloom CSR macro collision: GPIOCSR_DIR_BITS_MASK"
#endif
#ifdef GPIOCSR_DIR_BITS_RESET
#error "Bitloom CSR macro collision: GPIOCSR_DIR_BITS_RESET"
#endif
#ifdef GPIOCSR_DIR_BITS_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_DIR_BITS_ACCESS"
#endif
#define GPIOCSR_DIR_OFFSET UINT32_C(0x00000000)
#define GPIOCSR_DIR_MASK UINT32_C(0xffffffff)
#define GPIOCSR_DIR_RESET UINT32_C(0x00000000)
#define GPIOCSR_DIR_ACCESS "RW"
#define GPIOCSR_DIR_BITS_MASK UINT32_C(0xffffffff)
#define GPIOCSR_DIR_BITS_RESET UINT32_C(0x00000000)
#define GPIOCSR_DIR_BITS_ACCESS "RW"

#ifdef GPIOCSR_OUT_OFFSET
#error "Bitloom CSR macro collision: GPIOCSR_OUT_OFFSET"
#endif
#ifdef GPIOCSR_OUT_MASK
#error "Bitloom CSR macro collision: GPIOCSR_OUT_MASK"
#endif
#ifdef GPIOCSR_OUT_RESET
#error "Bitloom CSR macro collision: GPIOCSR_OUT_RESET"
#endif
#ifdef GPIOCSR_OUT_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_OUT_ACCESS"
#endif
#ifdef GPIOCSR_OUT_BITS_MASK
#error "Bitloom CSR macro collision: GPIOCSR_OUT_BITS_MASK"
#endif
#ifdef GPIOCSR_OUT_BITS_RESET
#error "Bitloom CSR macro collision: GPIOCSR_OUT_BITS_RESET"
#endif
#ifdef GPIOCSR_OUT_BITS_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_OUT_BITS_ACCESS"
#endif
#define GPIOCSR_OUT_OFFSET UINT32_C(0x00000004)
#define GPIOCSR_OUT_MASK UINT32_C(0xffffffff)
#define GPIOCSR_OUT_RESET UINT32_C(0x00000000)
#define GPIOCSR_OUT_ACCESS "RW"
#define GPIOCSR_OUT_BITS_MASK UINT32_C(0xffffffff)
#define GPIOCSR_OUT_BITS_RESET UINT32_C(0x00000000)
#define GPIOCSR_OUT_BITS_ACCESS "RW"

#ifdef GPIOCSR_IN_OFFSET
#error "Bitloom CSR macro collision: GPIOCSR_IN_OFFSET"
#endif
#ifdef GPIOCSR_IN_MASK
#error "Bitloom CSR macro collision: GPIOCSR_IN_MASK"
#endif
#ifdef GPIOCSR_IN_RESET
#error "Bitloom CSR macro collision: GPIOCSR_IN_RESET"
#endif
#ifdef GPIOCSR_IN_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_IN_ACCESS"
#endif
#ifdef GPIOCSR_IN_BITS_MASK
#error "Bitloom CSR macro collision: GPIOCSR_IN_BITS_MASK"
#endif
#ifdef GPIOCSR_IN_BITS_RESET
#error "Bitloom CSR macro collision: GPIOCSR_IN_BITS_RESET"
#endif
#ifdef GPIOCSR_IN_BITS_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_IN_BITS_ACCESS"
#endif
#define GPIOCSR_IN_OFFSET UINT32_C(0x00000008)
#define GPIOCSR_IN_MASK UINT32_C(0xffffffff)
#define GPIOCSR_IN_RESET UINT32_C(0x00000000)
#define GPIOCSR_IN_ACCESS "RO"
#define GPIOCSR_IN_BITS_MASK UINT32_C(0xffffffff)
#define GPIOCSR_IN_BITS_RESET UINT32_C(0x00000000)
#define GPIOCSR_IN_BITS_ACCESS "RO"

#ifdef GPIOCSR_SET_OFFSET
#error "Bitloom CSR macro collision: GPIOCSR_SET_OFFSET"
#endif
#ifdef GPIOCSR_SET_MASK
#error "Bitloom CSR macro collision: GPIOCSR_SET_MASK"
#endif
#ifdef GPIOCSR_SET_RESET
#error "Bitloom CSR macro collision: GPIOCSR_SET_RESET"
#endif
#ifdef GPIOCSR_SET_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_SET_ACCESS"
#endif
#ifdef GPIOCSR_SET_BITS_MASK
#error "Bitloom CSR macro collision: GPIOCSR_SET_BITS_MASK"
#endif
#ifdef GPIOCSR_SET_BITS_RESET
#error "Bitloom CSR macro collision: GPIOCSR_SET_BITS_RESET"
#endif
#ifdef GPIOCSR_SET_BITS_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_SET_BITS_ACCESS"
#endif
#define GPIOCSR_SET_OFFSET UINT32_C(0x0000000c)
#define GPIOCSR_SET_MASK UINT32_C(0xffffffff)
#define GPIOCSR_SET_RESET UINT32_C(0x00000000)
#define GPIOCSR_SET_ACCESS "WO"
#define GPIOCSR_SET_BITS_MASK UINT32_C(0xffffffff)
#define GPIOCSR_SET_BITS_RESET UINT32_C(0x00000000)
#define GPIOCSR_SET_BITS_ACCESS "WO"

#ifdef GPIOCSR_CLEAR_OFFSET
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_OFFSET"
#endif
#ifdef GPIOCSR_CLEAR_MASK
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_MASK"
#endif
#ifdef GPIOCSR_CLEAR_RESET
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_RESET"
#endif
#ifdef GPIOCSR_CLEAR_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_ACCESS"
#endif
#ifdef GPIOCSR_CLEAR_BITS_MASK
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_BITS_MASK"
#endif
#ifdef GPIOCSR_CLEAR_BITS_RESET
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_BITS_RESET"
#endif
#ifdef GPIOCSR_CLEAR_BITS_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_CLEAR_BITS_ACCESS"
#endif
#define GPIOCSR_CLEAR_OFFSET UINT32_C(0x00000010)
#define GPIOCSR_CLEAR_MASK UINT32_C(0xffffffff)
#define GPIOCSR_CLEAR_RESET UINT32_C(0x00000000)
#define GPIOCSR_CLEAR_ACCESS "WO"
#define GPIOCSR_CLEAR_BITS_MASK UINT32_C(0xffffffff)
#define GPIOCSR_CLEAR_BITS_RESET UINT32_C(0x00000000)
#define GPIOCSR_CLEAR_BITS_ACCESS "WO"

#ifdef GPIOCSR_RISE_EVENT_OFFSET
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_OFFSET"
#endif
#ifdef GPIOCSR_RISE_EVENT_MASK
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_MASK"
#endif
#ifdef GPIOCSR_RISE_EVENT_RESET
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_RESET"
#endif
#ifdef GPIOCSR_RISE_EVENT_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_ACCESS"
#endif
#ifdef GPIOCSR_RISE_EVENT_BITS_MASK
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_BITS_MASK"
#endif
#ifdef GPIOCSR_RISE_EVENT_BITS_RESET
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_BITS_RESET"
#endif
#ifdef GPIOCSR_RISE_EVENT_BITS_ACCESS
#error "Bitloom CSR macro collision: GPIOCSR_RISE_EVENT_BITS_ACCESS"
#endif
#define GPIOCSR_RISE_EVENT_OFFSET UINT32_C(0x00000014)
#define GPIOCSR_RISE_EVENT_MASK UINT32_C(0xffffffff)
#define GPIOCSR_RISE_EVENT_RESET UINT32_C(0x00000000)
#define GPIOCSR_RISE_EVENT_ACCESS "W1C"
#define GPIOCSR_RISE_EVENT_BITS_MASK UINT32_C(0xffffffff)
#define GPIOCSR_RISE_EVENT_BITS_RESET UINT32_C(0x00000000)
#define GPIOCSR_RISE_EVENT_BITS_ACCESS "W1C"

#endif /* BITLOOM_GPIOCSR_CSR_H */
