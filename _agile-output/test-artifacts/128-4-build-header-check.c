#include "gpio-csr-registers.h"
_Static_assert(GPIOCSR_DIR_OFFSET == 0 && GPIOCSR_OUT_OFFSET == 4 && GPIOCSR_IN_OFFSET == 8, "local offsets");
_Static_assert(GPIOCSR_SET_OFFSET == 12 && GPIOCSR_CLEAR_OFFSET == 16 && GPIOCSR_RISE_EVENT_OFFSET == 20, "local offsets");
_Static_assert(GPIOCSR_DIR_MASK == UINT32_MAX && GPIOCSR_OUT_MASK == UINT32_MAX && GPIOCSR_IN_MASK == UINT32_MAX, "32 pins");
_Static_assert(GPIOCSR_SET_MASK == UINT32_MAX && GPIOCSR_CLEAR_MASK == UINT32_MAX && GPIOCSR_RISE_EVENT_MASK == UINT32_MAX, "32 effects");
_Static_assert(0x100 + GPIOCSR_RISE_EVENT_OFFSET == 0x114, "caller supplies base");
int main(void) { return 0; }
