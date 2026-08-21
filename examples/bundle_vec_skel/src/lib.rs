//! FR51 fixture: documented `Bundle` + `HwVec` (`Vec<T,N>` equiv.) → flatten → emit → tick.

use bitloom_prelude::rhdl::module;
use bitloom_prelude::{
    Bundle, Clock, Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, HwVec,
    Input, Output, Reset, Span, UInt, add_port_field,
};

/// Documented Bundle of ground leaves (FR51).
#[derive(Debug, Clone, Copy, Default)]
pub struct Stream;

impl Bundle for Stream {
    fn leaves() -> &'static [(&'static str, GroundType)] {
        &[
            ("data", GroundType::UInt { width: 8 }),
            ("valid", GroundType::Bool),
        ]
    }
}

/// Macro path: composite fields flatten to scalar leaf ports.
#[module]
pub struct BundleVecPorts {
    pub clk: Input<Clock>,
    pub rst: Input<Reset>,
    pub stream: Input<Stream>,
    pub lanes: Input<HwVec<UInt<8>, 4>>,
    pub out_stream: Output<Stream>,
    pub lane0_out: Output<UInt<8>>,
}

/// Skid-style body over flattened Bundle / HwVec leaves.
pub struct BundleVecSkel;

impl Elaboratable for BundleVecSkel {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("BundleVecSkel");
        s.begin_module("BundleVecSkel", Span::default());
        add_port_field::<Input<Clock>>(&mut s, "clk", Span::default());
        add_port_field::<Input<Reset>>(&mut s, "rst", Span::default());
        add_port_field::<Input<Stream>>(&mut s, "stream", Span::default());
        add_port_field::<Input<HwVec<UInt<8>, 4>>>(&mut s, "lanes", Span::default());
        add_port_field::<Output<Stream>>(&mut s, "out_stream", Span::default());
        add_port_field::<Output<UInt<8>>>(&mut s, "lane0_out", Span::default());

        s.declare_reg("q_data", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("q_valid", GroundType::Bool, Span::default());
        s.declare_reg("q_lane0", GroundType::UInt { width: 8 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_net("out_stream_data", "q_data", Span::default());
        s.assign_net("out_stream_valid", "q_valid", Span::default());
        s.assign_net("lane0_out", "q_lane0", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q_data", "stream_data", Span::default());
        s.assign_reg_d_from("q_valid", "stream_valid", Span::default());
        s.assign_reg_d_from("q_lane0", "lanes_0", Span::default());
        s.end_process();

        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    BundleVecSkel::elaborate()
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::{Elaboratable, GroundType, PortDirection, PortField};
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    use super::*;

    #[test]
    fn flatten_names_bundle_and_hwvec() {
        let stream = <Input<Stream> as PortField>::flatten("stream");
        assert_eq!(
            stream,
            vec![
                (
                    "stream_data".into(),
                    bitloom_prelude::PortDir::Input,
                    GroundType::UInt { width: 8 }
                ),
                (
                    "stream_valid".into(),
                    bitloom_prelude::PortDir::Input,
                    GroundType::Bool
                ),
            ]
        );
        let lanes = <Input<HwVec<UInt<8>, 4>> as PortField>::flatten("lanes");
        assert_eq!(lanes.len(), 4);
        assert_eq!(lanes[0].0, "lanes_0");
        assert_eq!(lanes[3].0, "lanes_3");
        assert_eq!(lanes[1].2, GroundType::UInt { width: 8 });
    }

    #[test]
    fn module_macro_registers_leaf_ports() {
        let frozen = BundleVecPorts::elaborate().expect("elaborate");
        let ports = &frozen.circuit().modules[0].ports;
        let names: Vec<_> = ports.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"stream_data"));
        assert!(names.contains(&"stream_valid"));
        assert!(names.contains(&"lanes_0"));
        assert!(names.contains(&"lanes_3"));
        assert!(names.contains(&"out_stream_data"));
        assert!(names.contains(&"out_stream_valid"));
        assert!(names.contains(&"lane0_out"));
        let data = ports.iter().find(|p| p.name == "stream_data").unwrap();
        assert_eq!(data.direction, PortDirection::Input);
        assert_eq!(data.ty, GroundType::UInt { width: 8 });
        let art = emit(&frozen);
        assert_eq!(art.filelist, vec!["BundleVecPorts.v"]);
        assert!(art.files[0].contents.contains("stream_data"));
        assert!(art.files[0].contents.contains("lanes_3"));
    }

    #[test]
    fn design_crate_depends_only_on_prelude() {
        let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
        let deps = manifest
            .split("[dev-dependencies]")
            .next()
            .expect("deps section");
        assert!(
            deps.contains("bitloom-prelude"),
            "design must depend on bitloom-prelude"
        );
        for banned in [
            "bitloom-builder",
            "bitloom-hir",
            "bitloom-vlog",
            "bitloom-sim",
            "bitloom-macro",
            "bitloom ",
        ] {
            assert!(
                !deps.contains(banned),
                "design [dependencies] must not include {banned}"
            );
        }
    }

    #[test]
    fn elaborate_emit_tick_bundle_hwvec() {
        let hir = BundleVecSkel::elaborate().expect("elaborate");
        let art = emit(&hir);
        assert_eq!(art.filelist, vec!["BundleVecSkel.v"]);
        let v = &art.files[0].contents;
        assert!(v.contains("module BundleVecSkel"));
        assert!(v.contains("stream_data"));
        assert!(v.contains("lanes_0"));
        assert!(v.contains("out_stream_valid"));

        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("stream_data", 0);
        pv.set("stream_valid", 0);
        pv.set("lanes_0", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("out_stream_data"), Some(0));
        assert_eq!(sim.ports().get("lane0_out"), Some(0));

        pv.set("rst", 0);
        pv.set("stream_data", 0xA5);
        pv.set("stream_valid", 1);
        pv.set("lanes_0", 0x3C);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("out_stream_data"), Some(0xA5));
        assert_eq!(sim.ports().get("out_stream_valid"), Some(1));
        assert_eq!(sim.ports().get("lane0_out"), Some(0x3C));
    }

    #[test]
    fn width_mismatch_fails_before_emit() {
        let mut s = ElaborateSession::new("WidthBad");
        s.begin_module("WidthBad", Span::default());
        add_port_field::<Input<Clock>>(&mut s, "clk", Span::default());
        add_port_field::<Input<Reset>>(&mut s, "rst", Span::default());
        add_port_field::<Input<Stream>>(&mut s, "stream", Span::default());
        s.add_output("narrow", GroundType::UInt { width: 4 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("narrow", "stream_data", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("width mismatch must fail");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0131"),
            "expected E0131, got {err:?}"
        );
        // No FrozenHir → cannot emit
    }

    #[test]
    fn dir_mismatch_fails_before_emit() {
        let mut s = ElaborateSession::new("DirBad");
        s.begin_module("DirBad", Span::default());
        add_port_field::<Input<Clock>>(&mut s, "clk", Span::default());
        add_port_field::<Input<Reset>>(&mut s, "rst", Span::default());
        add_port_field::<Input<Stream>>(&mut s, "stream", Span::default());
        s.add_input("other_data", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        // Same-width connect into an input leaf → direction error (E0112), not width.
        s.assign_net("stream_data", "other_data", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("dir mismatch must fail");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0112"),
            "expected E0112, got {err:?}"
        );
        // No FrozenHir → cannot emit
    }

    #[test]
    fn width_mismatch_on_reg_d_fails_before_emit() {
        let mut s = ElaborateSession::new("RegWidthBad");
        s.begin_module("RegWidthBad", Span::default());
        add_port_field::<Input<Clock>>(&mut s, "clk", Span::default());
        add_port_field::<Input<Reset>>(&mut s, "rst", Span::default());
        add_port_field::<Input<Stream>>(&mut s, "stream", Span::default());
        s.declare_reg("q_narrow", GroundType::UInt { width: 4 }, Span::default());
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q_narrow", "stream_data", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("Reg.d width mismatch must fail");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0131"),
            "expected E0131 on Reg.d, got {err:?}"
        );
    }

    /// field `a` + member `b_c` and field `a_b` + member `c` both flatten to `a_b_c`.
    #[test]
    fn flatten_leaf_name_collision_fails_before_emit() {
        struct LeafBc;
        impl Bundle for LeafBc {
            fn leaves() -> &'static [(&'static str, GroundType)] {
                &[("b_c", GroundType::Bool)]
            }
        }
        struct LeafC;
        impl Bundle for LeafC {
            fn leaves() -> &'static [(&'static str, GroundType)] {
                &[("c", GroundType::Bool)]
            }
        }

        let mut s = ElaborateSession::new("LeafClash");
        s.begin_module("LeafClash", Span::default());
        add_port_field::<Input<Clock>>(&mut s, "clk", Span::default());
        add_port_field::<Input<Reset>>(&mut s, "rst", Span::default());
        add_port_field::<Input<LeafBc>>(&mut s, "a", Span::default());
        add_port_field::<Input<LeafC>>(&mut s, "a_b", Span::default());
        s.end_module();
        let err = s.finish().expect_err("leaf name collision must fail before emit");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0152"),
            "expected E0152, got {err:?}"
        );
        assert!(
            err.0.iter().any(|d| d.en.contains("a_b_c")),
            "diagnostic should name colliding leaf, got {err:?}"
        );
    }

    #[test]
    fn nested_hwvec_bundle_rejected_at_compile_time() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/nested_hwvec_bundle.rs");
    }

    #[test]
    fn derive_bundle_unavailable_at_compile_time() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/derive_bundle_unavailable.rs");
    }
}
