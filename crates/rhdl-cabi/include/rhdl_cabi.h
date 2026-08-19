#ifndef RHDL_CABI_H
#define RHDL_CABI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Handle Handle;

Handle *rhdl_sim_new(void);
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
