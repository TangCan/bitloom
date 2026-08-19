/* FR33 C harness: load rhdl-cabi, tick RTL + abstraction, match Rust golden (3). */
#include "rhdl_cabi.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    Handle *h = rhdl_sim_new();
    if (!h) {
        fprintf(stderr, "rhdl_sim_new failed\n");
        return 1;
    }
    rhdl_sim_set(h, "rst", 1);
    rhdl_sim_tick(h);
    rhdl_abs_cycle(h);
    rhdl_sim_set(h, "rst", 0);
    for (int i = 0; i < 3; i++) {
        rhdl_sim_tick(h);
        rhdl_abs_cycle(h);
    }
    uint64_t rtl = rhdl_sim_get(h, "data_out");
    uint64_t abs = rhdl_abs_get(h, "data_out");
    rhdl_sim_free(h);
    if (rtl != 3 || abs != 3) {
        fprintf(stderr, "mismatch rtl=%llu abs=%llu expected=3\n",
                (unsigned long long)rtl, (unsigned long long)abs);
        return 2;
    }
    printf("ok rtl=%llu abs=%llu\n", (unsigned long long)rtl,
           (unsigned long long)abs);
    return 0;
}
