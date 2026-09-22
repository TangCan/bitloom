use bitloom_prelude::{Diagnostics, ElaborateSession, FrozenHir, GroundType, Span};
pub fn graph(typed: bool) -> FrozenHir {
    let mut s = ElaborateSession::new("ResetCore");
    let p = Span::default();
    s.define_module("Cell", vec![], |s, _| -> Result<(), Diagnostics> {
        let p = Span::default();
        s.add_input("clk", GroundType::Clock, p);
        s.add_input("rst", GroundType::Reset, p);
        s.add_input("enable", GroundType::UInt { width: 1 }, p);
        s.add_input("data", GroundType::UInt { width: 8 }, p);
        s.add_output("out", GroundType::UInt { width: 8 }, p);
        s.declare_reg("state", GroundType::UInt { width: 8 }, p);
        s.begin_combinational(p);
        s.assign_net("out", "state", p);
        s.end_process();
        s.begin_sequential(p);
        s.assign_reg_d_mux("state", "enable", "data", "state", p);
        s.end_process();
        Ok(())
    })
    .unwrap();
    s.begin_module("ResetCore", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input(if typed { "aresetn" } else { "rst" }, GroundType::Reset, p);
    if typed {
        for n in ["zero", "a_u"] {
            s.declare_wire(n, GroundType::UInt { width: 1 }, p);
        }
        s.declare_wire("core_reset", GroundType::Reset, p);
        s.begin_combinational(p);
        s.assign_lit("zero", 0, p);
        s.assign_xor("a_u", "aresetn", "zero", p);
        s.assign_eq("core_reset", "a_u", "zero", p);
        s.end_process();
    }
    s.add_input("enable", GroundType::UInt { width: 1 }, p);
    for i in 0..2 {
        s.add_input(format!("data{i}"), GroundType::UInt { width: 8 }, p);
        s.add_output(format!("out{i}"), GroundType::UInt { width: 8 }, p);
        s.add_instance(
            format!("cell{i}"),
            "Cell",
            vec![
                ("clk".into(), "clk".into()),
                (
                    "rst".into(),
                    if typed { "core_reset" } else { "rst" }.into(),
                ),
                ("enable".into(), "enable".into()),
                ("data".into(), format!("data{i}")),
                ("out".into(), format!("out{i}")),
            ],
            vec![],
            p,
        );
    }
    s.end_module();
    s.finish().unwrap()
}
