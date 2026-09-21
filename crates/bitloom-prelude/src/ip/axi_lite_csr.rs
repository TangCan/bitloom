use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Fixed 16-bit address, 32-bit AXI4-Lite to CSR bridge (FR196).
///
/// AW, W and AR each have one independent capture slot. B and R each have one
/// response slot; at most one CSR request executes at a time. Apply a synchronous
/// reset edge before use, and reset every attached CSR leaf with the bridge.
/// AWPROT/ARPROT are ignored. No bypass or same-edge slot refill is promised.
pub struct AxiLiteCsrBridge;

impl Elaboratable for AxiLiteCsrBridge {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut session = ElaborateSession::new("AxiLiteCsrBridge");
        Self::define_module(&mut session, "AxiLiteCsrBridge")?;
        session.finish()
    }
}
impl AxiLiteCsrBridge {
    /// Define the fixed bridge in the caller's session using the shared body.
    pub fn define_module(
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        session.define_module(name, vec![], Self::define_body)
    }
    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let sp = Span::default();
        s.add_input("clk", GroundType::Clock, sp);
        s.add_input("rst", GroundType::Reset, sp);
        for (name, width) in [
            ("s_axi_awaddr", 16),
            ("s_axi_awprot", 3),
            ("s_axi_awvalid", 1),
            ("s_axi_wdata", 32),
            ("s_axi_wstrb", 4),
            ("s_axi_wvalid", 1),
            ("s_axi_bready", 1),
            ("s_axi_araddr", 16),
            ("s_axi_arprot", 3),
            ("s_axi_arvalid", 1),
            ("s_axi_rready", 1),
            ("csr_req_ready", 1),
            ("csr_rsp_valid", 1),
            ("csr_rdata", 32),
            ("csr_error", 2),
        ] {
            s.add_input(name, GroundType::UInt { width }, sp);
        }
        for (name, width) in [
            ("s_axi_awready", 1),
            ("s_axi_wready", 1),
            ("s_axi_arready", 1),
            ("s_axi_bvalid", 1),
            ("s_axi_bresp", 2),
            ("s_axi_rvalid", 1),
            ("s_axi_rdata", 32),
            ("s_axi_rresp", 2),
            ("csr_req_valid", 1),
            ("csr_write", 1),
            ("csr_addr", 16),
            ("csr_wdata", 32),
            ("csr_wstrb", 4),
            ("csr_rsp_ready", 1),
        ] {
            s.add_output(name, GroundType::UInt { width }, sp);
        }
        for (name, width) in [
            ("aw_full", 1),
            ("w_full", 1),
            ("ar_full", 1),
            ("aw_addr", 16),
            ("w_data", 32),
            ("w_strb", 4),
            ("ar_addr", 16),
            ("offer", 1),
            ("offer_write", 1),
            ("exec", 1),
            ("owner", 1),
            ("prefer_write", 1),
            ("b_full", 1),
            ("r_full", 1),
            ("b_error", 2),
            ("r_error", 2),
            ("r_data", 32),
        ] {
            s.declare_reg(name, GroundType::UInt { width }, sp);
        }
        s.declare_reg("offer_data", GroundType::UInt { width: 32 }, sp);
        s.declare_reg("offer_strb", GroundType::UInt { width: 4 }, sp);
        s.declare_wire("one", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("zero", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("aw_take", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("w_take", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("ar_take", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("active", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("no_offer", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("no_exec", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("idle", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("b_space", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("r_space", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("write_complete", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("we", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("re", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("eligible", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("select", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("no_read", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("write_preferred", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("choose_write", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("commit", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("read_offer", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("write_commit", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("read_commit", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("response", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("write_response", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("read_owner", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("read_response", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("b_take", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("r_take", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("aw_captured", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("aw_next", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("w_captured", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("w_next", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("ar_captured", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("ar_next", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("offer_started", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("offer_next", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("exec_started", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("exec_next", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("b_consumed", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("b_next", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("r_consumed", GroundType::UInt { width: 1 }, sp);
        s.declare_wire("r_next", GroundType::UInt { width: 1 }, sp);
        s.begin_combinational(sp);
        // The five external handshake outputs stop at occupancy registers.
        // Synchronous reset is deliberately absent from their combinational cones.
        s.assign_lit("one", 1, sp);
        s.assign_lit("zero", 0, sp);
        s.assign_xor("s_axi_awready", "aw_full", "one", sp);
        s.assign_and("aw_take", "s_axi_awready", "s_axi_awvalid", sp);
        s.assign_xor("s_axi_wready", "w_full", "one", sp);
        s.assign_and("w_take", "s_axi_wready", "s_axi_wvalid", sp);
        s.assign_xor("s_axi_arready", "ar_full", "one", sp);
        s.assign_and("ar_take", "s_axi_arready", "s_axi_arvalid", sp);
        // An offer reserves an empty response slot implicitly: neither another
        // offer nor another execution may start until this one completes. Only
        // that execution can fill its reserved response slot.
        s.assign_xor("active", "rst", "one", sp);
        s.assign_xor("no_offer", "offer", "one", sp);
        s.assign_xor("no_exec", "exec", "one", sp);
        s.assign_and("idle", "no_offer", "no_exec", sp);
        s.assign_xor("b_space", "b_full", "one", sp);
        s.assign_xor("r_space", "r_full", "one", sp);
        s.assign_and("write_complete", "aw_full", "w_full", sp);
        s.assign_and("we", "write_complete", "b_space", sp);
        s.assign_and("re", "ar_full", "r_space", sp);
        s.assign_or("eligible", "we", "re", sp);
        s.assign_and("select", "idle", "eligible", sp);
        s.assign_xor("no_read", "re", "one", sp);
        s.assign_or("write_preferred", "prefer_write", "no_read", sp);
        s.assign_and("choose_write", "we", "write_preferred", sp);
        s.assign_and("csr_req_valid", "offer", "active", sp);
        s.assign_and("commit", "csr_req_valid", "csr_req_ready", sp);
        s.assign_xor("read_offer", "offer_write", "one", sp);
        s.assign_and("write_commit", "commit", "offer_write", sp);
        s.assign_and("read_commit", "commit", "read_offer", sp);
        s.assign_and("csr_rsp_ready", "exec", "active", sp);
        s.assign_and("response", "csr_rsp_ready", "csr_rsp_valid", sp);
        s.assign_and("write_response", "response", "owner", sp);
        s.assign_xor("read_owner", "owner", "one", sp);
        s.assign_and("read_response", "response", "read_owner", sp);
        s.assign_and("b_take", "b_full", "s_axi_bready", sp);
        s.assign_and("r_take", "r_full", "s_axi_rready", sp);
        s.assign_mux("aw_captured", "aw_take", "one", "aw_full", sp);
        s.assign_mux("aw_next", "write_commit", "zero", "aw_captured", sp);
        s.assign_mux("w_captured", "w_take", "one", "w_full", sp);
        s.assign_mux("w_next", "write_commit", "zero", "w_captured", sp);
        s.assign_mux("ar_captured", "ar_take", "one", "ar_full", sp);
        s.assign_mux("ar_next", "read_commit", "zero", "ar_captured", sp);
        s.assign_mux("offer_started", "select", "one", "offer", sp);
        s.assign_mux("offer_next", "commit", "zero", "offer_started", sp);
        s.assign_mux("exec_started", "commit", "one", "exec", sp);
        s.assign_mux("exec_next", "response", "zero", "exec_started", sp);
        s.assign_mux("b_consumed", "b_take", "zero", "b_full", sp);
        s.assign_mux("b_next", "write_response", "one", "b_consumed", sp);
        s.assign_mux("r_consumed", "r_take", "zero", "r_full", sp);
        s.assign_mux("r_next", "read_response", "one", "r_consumed", sp);
        // Selected address capture cannot change before commit. Write payload
        // is separately snapshotted even for reads, preserving every CSR field
        // if a later W capture arrives while the read offer is stalled.
        s.assign_mux("csr_addr", "offer_write", "aw_addr", "ar_addr", sp);
        s.assign_net("s_axi_bvalid", "b_full", sp);
        s.assign_net("s_axi_rvalid", "r_full", sp);
        s.assign_net("s_axi_bresp", "b_error", sp);
        s.assign_net("s_axi_rresp", "r_error", sp);
        s.assign_net("s_axi_rdata", "r_data", sp);
        s.assign_net("csr_write", "offer_write", sp);
        s.assign_net("csr_wdata", "offer_data", sp);
        s.assign_net("csr_wstrb", "offer_strb", sp);
        s.end_process();
        // Builder registers synchronously reset to zero with priority over D.
        // Captures release on commit, execution releases on CSR response, and
        // response slots release independently on their AXI handshakes.
        s.begin_sequential(sp);
        s.assign_reg_d_from("aw_full", "aw_next", sp);
        s.assign_reg_d_from("w_full", "w_next", sp);
        s.assign_reg_d_from("ar_full", "ar_next", sp);
        s.assign_reg_d_from("offer", "offer_next", sp);
        s.assign_reg_d_from("exec", "exec_next", sp);
        s.assign_reg_d_from("b_full", "b_next", sp);
        s.assign_reg_d_from("r_full", "r_next", sp);
        s.assign_reg_d_mux("aw_addr", "aw_take", "s_axi_awaddr", "aw_addr", sp);
        s.assign_reg_d_mux("w_data", "w_take", "s_axi_wdata", "w_data", sp);
        s.assign_reg_d_mux("w_strb", "w_take", "s_axi_wstrb", "w_strb", sp);
        s.assign_reg_d_mux("ar_addr", "ar_take", "s_axi_araddr", "ar_addr", sp);
        s.assign_reg_d_mux("offer_write", "select", "choose_write", "offer_write", sp);
        s.assign_reg_d_mux("offer_data", "select", "w_data", "offer_data", sp);
        s.assign_reg_d_mux("offer_strb", "select", "w_strb", "offer_strb", sp);
        s.assign_reg_d_mux("owner", "commit", "offer_write", "owner", sp);
        s.assign_reg_d_mux("prefer_write", "commit", "read_offer", "prefer_write", sp);
        s.assign_reg_d_mux("b_error", "write_response", "csr_error", "b_error", sp);
        s.assign_reg_d_mux("r_error", "read_response", "csr_error", "r_error", sp);
        s.assign_reg_d_mux("r_data", "read_response", "csr_rdata", "r_data", sp);
        s.end_process();
        Ok(())
    }
}
