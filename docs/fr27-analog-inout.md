# FR27 — top-level Analog / InOut

`GroundType::Analog` and `PortDirection::InOut` are legal only when `module.name == circuit.name`.
Non-top uses fail freeze with `rhdl::E0270`. Builder: `add_inout`.
