// Called immediately before each sampled rising edge; reset cancels history.
reg hold_aw=0,hold_w=0,hold_ar=0,hold_rsp=0;
reg [18:0] held_aw,held_ar;
reg [35:0] held_w;
reg [33:0] held_rsp;
task monitor_producers; begin
 if(rst)begin hold_aw=0;hold_w=0;hold_ar=0;hold_rsp=0;end
 else begin
  // Holds extend through the eventual accepting edge. Never assume a change
  // becomes legal merely because ready has risen since the stalled edge.
  if(hold_aw && (!s_axi_awvalid || {s_axi_awaddr,s_axi_awprot}!==held_aw))$fatal(1,"AW producer hold violation");
  if(hold_w && (!s_axi_wvalid || {s_axi_wdata,s_axi_wstrb}!==held_w))$fatal(1,"W producer hold violation");
  if(hold_ar && (!s_axi_arvalid || {s_axi_araddr,s_axi_arprot}!==held_ar))$fatal(1,"AR producer hold violation");
  if(hold_rsp && (!csr_rsp_valid || {csr_rdata,csr_error}!==held_rsp))$fatal(1,"CSR responder hold violation");
  hold_aw=s_axi_awvalid&&!s_axi_awready;held_aw={s_axi_awaddr,s_axi_awprot};
  hold_w=s_axi_wvalid&&!s_axi_wready;held_w={s_axi_wdata,s_axi_wstrb};
  hold_ar=s_axi_arvalid&&!s_axi_arready;held_ar={s_axi_araddr,s_axi_arprot};
  hold_rsp=csr_rsp_valid&&!csr_rsp_ready;held_rsp={csr_rdata,csr_error};
 end
end endtask
