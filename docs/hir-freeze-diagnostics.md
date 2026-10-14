# HIR freeze diagnostics

`rhdl::E0271` is returned when a `UInt<0>` or `SInt<0>` signal declaration
reaches HIR freeze. It applies to module ports, wires, and registers from both
the Builder and FIRRTL import paths. Signals must have a positive bit width.
