// SYNTAX ONLY. All observed DUT signals are unconstrained inputs; no DUT or proof.
module observer_syntax_probe(clk,csr_addr,csr_error,csr_rdata,csr_req_ready,csr_req_valid,csr_rsp_ready,csr_rsp_valid,csr_wdata,csr_write,csr_wstrb,rst,s_axi_araddr,s_axi_arprot,s_axi_arready,s_axi_arvalid,s_axi_awaddr,s_axi_awprot,s_axi_awready,s_axi_awvalid,s_axi_bready,s_axi_bresp,s_axi_bvalid,s_axi_rdata,s_axi_rready,s_axi_rresp,s_axi_rvalid,s_axi_wdata,s_axi_wready,s_axi_wstrb,s_axi_wvalid);
input [0:0] clk;
input [15:0] csr_addr;
input [1:0] csr_error;
input [31:0] csr_rdata;
input [0:0] csr_req_ready;
input [0:0] csr_req_valid;
input [0:0] csr_rsp_ready;
input [0:0] csr_rsp_valid;
input [31:0] csr_wdata;
input [0:0] csr_write;
input [3:0] csr_wstrb;
input [0:0] rst;
input [15:0] s_axi_araddr;
input [2:0] s_axi_arprot;
input [0:0] s_axi_arready;
input [0:0] s_axi_arvalid;
input [15:0] s_axi_awaddr;
input [2:0] s_axi_awprot;
input [0:0] s_axi_awready;
input [0:0] s_axi_awvalid;
input [0:0] s_axi_bready;
input [1:0] s_axi_bresp;
input [0:0] s_axi_bvalid;
input [31:0] s_axi_rdata;
input [0:0] s_axi_rready;
input [1:0] s_axi_rresp;
input [0:0] s_axi_rvalid;
input [31:0] s_axi_wdata;
input [0:0] s_axi_wready;
input [3:0] s_axi_wstrb;
input [0:0] s_axi_wvalid;

reg f_past = 0;
reg f_aw = 0, f_w = 0, f_ar = 0;
reg [15:0] f_awaddr, f_araddr;
reg [31:0] f_wdata;
reg [3:0] f_wstrb;
reg f_exec = 0, f_owner = 0, f_prefer_write = 0;
reg f_b = 0, f_r = 0;
reg [1:0] f_bresp, f_rresp;
reg [31:0] f_rdata;
reg [31:0] f_naw=0, f_nw=0, f_nar=0, f_nwr=0, f_nrd=0;
reg [31:0] f_caw=0, f_cw=0, f_car=0;
reg [31:0] f_submit=0, f_rsp=0, f_cexec=0;
reg [31:0] f_nbmake=0, f_nrmake=0, f_nb=0, f_nr=0, f_cb=0, f_cr=0;
reg f_seen_contention=0, f_read_during_bstall=0, f_write_during_rstall=0, f_seen_cancel=0;
reg [3:0] f_peer_age=0;
wire f_awtake=s_axi_awvalid && s_axi_awready;
wire f_wtake=s_axi_wvalid && s_axi_wready;
wire f_artake=s_axi_arvalid && s_axi_arready;
wire f_commit=csr_req_valid && csr_req_ready;
wire f_response=csr_rsp_valid && csr_rsp_ready;
wire f_btake=s_axi_bvalid && s_axi_bready;
wire f_rtake=s_axi_rvalid && s_axi_rready;
wire f_we=f_aw && f_w && !f_b;
wire f_re=f_ar && !f_r;
always @(posedge clk) begin
  f_past <= 1;
  if (!f_past) assume(rst);
  if (f_past) begin
    // Independent accounting includes partial AW/W cancellation, not fake writes.
    assert(f_naw == f_nwr + f_caw + f_aw);
    assert(f_nw == f_nwr + f_cw + f_w);
    assert(f_nar == f_nrd + f_car + f_ar);
    assert(f_submit == f_rsp + f_cexec + f_exec);
    assert(f_nbmake == f_nb + f_cb + f_b);
    assert(f_nrmake == f_nr + f_cr + f_r);
    if ($past(rst)) begin
      assert(!s_axi_bvalid && !s_axi_rvalid);
      assert(!f_aw && !f_w && !f_ar && !f_exec && !f_b && !f_r);
    end
    if (!rst) begin
      // Legal AXI producers: only these input holds are assumptions.
      if ($past(!rst && s_axi_awvalid && !s_axi_awready)) begin
        assume(s_axi_awvalid);
        assume({s_axi_awaddr,s_axi_awprot} == $past({s_axi_awaddr,s_axi_awprot}));
      end
      if ($past(!rst && s_axi_wvalid && !s_axi_wready)) begin
        assume(s_axi_wvalid);
        assume({s_axi_wdata,s_axi_wstrb} == $past({s_axi_wdata,s_axi_wstrb}));
      end
      if ($past(!rst && s_axi_arvalid && !s_axi_arready)) begin
        assume(s_axi_arvalid);
        assume({s_axi_araddr,s_axi_arprot} == $past({s_axi_araddr,s_axi_arprot}));
      end
      // Causal single response peer with finite latency <= 7 clocks, then holds.
      // No restrictions on BREADY/RREADY (arbitrarily long AXI backpressure).
      if (!f_exec) assume(!csr_rsp_valid);
      if (f_exec && f_peer_age == 7) assume(csr_rsp_valid);
      if (csr_rsp_valid) begin
        assume(csr_error != 2'b01);
        if (csr_error != 0) assume(csr_rdata == 0);
      end
      if ($past(!rst && csr_rsp_valid && !csr_rsp_ready)) begin
        assume(csr_rsp_valid);
        assume({csr_rdata,csr_error} == $past({csr_rdata,csr_error}));
      end
      // Bridge obligations are ASSERTS, never environmental assumptions.
      if (f_awtake) assert(!f_aw);
      if (f_wtake) assert(!f_w);
      if (f_artake) assert(!f_ar);
      if (csr_req_valid) begin
        assert(!f_exec);
        if (csr_write) begin
          assert(f_aw && f_w && !f_b);
          assert({csr_addr,csr_wdata,csr_wstrb} == {f_awaddr,f_wdata,f_wstrb});
        end else begin
          assert(f_ar && !f_r);
          assert(csr_addr == f_araddr);
        end
        // Arbitration is checked on first observed offer only; later arrivals
        // cannot change it. Eligibility is measured before the offer appeared.
        if (!$past(!rst && csr_req_valid && !csr_req_ready) &&
            $past(!rst && !f_exec && f_we && f_re))
          assert(csr_write == f_prefer_write);
      end
      if ($past(!rst && csr_req_valid && !csr_req_ready)) begin
        assert(csr_req_valid);
        assert({csr_write,csr_addr,csr_wdata,csr_wstrb} ==
               $past({csr_write,csr_addr,csr_wdata,csr_wstrb}));
      end
      if (f_commit) assert(!f_exec);
      if (f_response) begin
        assert(f_exec);
        if (f_owner) assert(!f_b); else assert(!f_r);
      end
      if (s_axi_bvalid) begin assert(f_b); assert(s_axi_bresp == f_bresp); end
      if (s_axi_rvalid) begin
        assert(f_r); assert({s_axi_rdata,s_axi_rresp} == {f_rdata,f_rresp});
      end
      if ($past(!rst && s_axi_bvalid && !s_axi_bready)) begin
        assert(s_axi_bvalid); assert(s_axi_bresp == $past(s_axi_bresp));
      end
      if ($past(!rst && s_axi_rvalid && !s_axi_rready)) begin
        assert(s_axi_rvalid);
        assert({s_axi_rdata,s_axi_rresp} == $past({s_axi_rdata,s_axi_rresp}));
      end
      cover(f_seen_contention && f_nwr != 0 && f_nrd != 0 && f_nb != 0 && f_nr != 0);
      cover(f_read_during_bstall && f_btake);
      cover(f_write_during_rstall && f_rtake);
      cover(f_seen_cancel && f_commit);
      cover(f_b && f_r && !s_axi_bready && !s_axi_rready);
    end
  end
  if (rst) begin
    if (f_aw || f_w || f_ar || f_exec || f_b || f_r) f_seen_cancel <= 1;
    f_caw <= f_caw + f_aw; f_cw <= f_cw + f_w; f_car <= f_car + f_ar;
    f_cexec <= f_cexec + f_exec; f_cb <= f_cb + f_b; f_cr <= f_cr + f_r;
    f_aw<=0; f_w<=0; f_ar<=0; f_exec<=0; f_owner<=0;
    f_b<=0; f_r<=0; f_prefer_write<=0; f_peer_age<=0;
    f_seen_contention<=0; f_read_during_bstall<=0; f_write_during_rstall<=0;
  end else begin
    if (f_we && f_re && !f_exec) f_seen_contention<=1;
    // Witness opposite-class progress DURING this blocked response, then its
    // recovery. Clear on consumption so earlier traffic cannot satisfy a cover.
    if (s_axi_bvalid && !s_axi_bready && f_commit && !csr_write)
      f_read_during_bstall<=1;
    if (s_axi_rvalid && !s_axi_rready && f_commit && csr_write)
      f_write_during_rstall<=1;
    if (f_btake) f_read_during_bstall<=0;
    if (f_rtake) f_write_during_rstall<=0;
    if (f_exec && !csr_rsp_valid && f_peer_age < 7) f_peer_age<=f_peer_age+1;
    if (f_awtake) begin f_aw<=1; f_awaddr<=s_axi_awaddr; f_naw<=f_naw+1; end
    if (f_wtake) begin f_w<=1; f_wdata<=s_axi_wdata; f_wstrb<=s_axi_wstrb; f_nw<=f_nw+1; end
    if (f_artake) begin f_ar<=1; f_araddr<=s_axi_araddr; f_nar<=f_nar+1; end
    if (f_commit) begin
      f_exec<=1; f_owner<=csr_write; f_prefer_write<=!csr_write;
      f_submit<=f_submit+1; f_peer_age<=0;
      if (csr_write) begin f_aw<=0; f_w<=0; f_nwr<=f_nwr+1; end
      else begin f_ar<=0; f_nrd<=f_nrd+1; end
    end
    if (f_response) begin
      f_exec<=0; f_rsp<=f_rsp+1;
      if (f_owner) begin f_b<=1; f_bresp<=csr_error; f_nbmake<=f_nbmake+1; end
      else begin f_r<=1; f_rresp<=csr_error; f_rdata<=csr_rdata; f_nrmake<=f_nrmake+1; end
    end
    if (f_btake) begin f_b<=0; f_nb<=f_nb+1; end
    if (f_rtake) begin f_r<=0; f_nr<=f_nr+1; end
  end
end

endmodule
