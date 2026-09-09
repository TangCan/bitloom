/* FR83 C harness: Adder DUT via rhdl_sim_new_dut — proves not Counter-only. */
#include "rhdl_cabi.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    Handle *h = rhdl_sim_new_dut("Adder");
    if (!h) {
        const char *err = rhdl_last_error();
        fprintf(stderr, "rhdl_sim_new_dut(Adder) failed: %s\n",
                err ? err : "(no error)");
        return 1;
    }
    rhdl_sim_set(h, "a", 5);
    rhdl_sim_set(h, "b", 7);
    rhdl_sim_tick(h);
    rhdl_abs_cycle(h);
    uint64_t rtl = rhdl_sim_get(h, "sum");
    uint64_t abs = rhdl_abs_get(h, "sum");
    const char *get_err = rhdl_last_error();
    if (get_err) {
        fprintf(stderr, "unexpected last_error after get: %s\n", get_err);
        rhdl_sim_free(h);
        return 3;
    }
    if (rtl != 12 || abs != 12) {
        fprintf(stderr, "mismatch rtl=%llu abs=%llu expected=12\n",
                (unsigned long long)rtl, (unsigned long long)abs);
        rhdl_sim_free(h);
        return 2;
    }
    rhdl_sim_free(h);
    printf("ok adder rtl=%llu abs=%llu\n", (unsigned long long)rtl,
           (unsigned long long)abs);
    return 0;
}
