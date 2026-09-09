#ifndef RHDL_CABI_H
#define RHDL_CABI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Handle Handle;

/* Legacy Counter DUT (FR33). Equivalent to rhdl_sim_new_dut("Counter"). */
Handle *rhdl_sim_new(void);
/* Documented DUT select (FR83): "Counter" | "Adder". Null + rhdl_last_error on failure. */
Handle *rhdl_sim_new_dut(const char *dut_name);
/* Thread-local last error; null if none. Valid until next rhdl_* that sets/clears it. */
const char *rhdl_last_error(void);
void rhdl_sim_free(Handle *h);
void rhdl_sim_set(Handle *h, const char *name, uint64_t val);
void rhdl_sim_tick(Handle *h);
uint64_t rhdl_sim_get(Handle *h, const char *name);
void rhdl_abs_cycle(Handle *h);
uint64_t rhdl_abs_get(Handle *h, const char *name);

#ifdef __cplusplus
}
#endif

#endif
