#include "timer-registers.h"
_Static_assert(TIMER_CTRL_OFFSET == 0 && TIMER_COUNT_OFFSET == 4, "local offsets");
_Static_assert(TIMER_COMPARE_OFFSET == 8 && TIMER_EVENT_OFFSET == 12, "local offsets");
_Static_assert(TIMER_COUNT_MASK == UINT32_MAX && TIMER_COMPARE_MASK == UINT32_MAX, "full width");
_Static_assert(TIMER_CTRL_MASK == 3 && TIMER_EVENT_MASK == 1, "reserved bits");
int main(void) { return 0; }
