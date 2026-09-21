
`timescale 1ns/1ps
module bfm_probe (
 input clk, rst,
 input [7:0] s_axi_awaddr, input s_axi_awvalid, output s_axi_awready,
 input [31:0] s_axi_wdata, input [3:0] s_axi_wstrb,
 input s_axi_wvalid, output s_axi_wready,
 output [1:0] s_axi_bresp, output reg s_axi_bvalid, input s_axi_bready,
 input [7:0] s_axi_araddr, input s_axi_arvalid, output s_axi_arready,
 output reg [31:0] s_axi_rdata, output [1:0] s_axi_rresp,
 output reg s_axi_rvalid, input s_axi_rready
);
reg aw_pending, w_pending;
reg [31:0] data, wdata;
reg [3:0] wstrb;
integer i;
assign s_axi_awready = !aw_pending && !s_axi_bvalid;
assign s_axi_wready = !w_pending && !s_axi_bvalid;
assign s_axi_arready = !s_axi_rvalid;
assign s_axi_bresp = 0;
assign s_axi_rresp = 0;
always @(posedge clk) begin
 if (rst) begin
  aw_pending <= 0; w_pending <= 0; data <= 0;
  wdata <= 0; wstrb <= 0;
  s_axi_bvalid <= 0; s_axi_rvalid <= 0; s_axi_rdata <= 0;
 end else begin
  if (s_axi_awvalid && s_axi_awready) aw_pending <= 1;
  if (s_axi_wvalid && s_axi_wready) begin
   w_pending <= 1; wdata <= s_axi_wdata; wstrb <= s_axi_wstrb;
  end
  if (aw_pending && w_pending && !s_axi_bvalid) begin
   for (i=0; i<4; i=i+1) if (wstrb[i]) data[i*8+:8] <= wdata[i*8+:8];
   aw_pending <= 0; w_pending <= 0; s_axi_bvalid <= 1;
  end
  if (s_axi_bvalid && s_axi_bready) s_axi_bvalid <= 0;
  if (s_axi_arvalid && s_axi_arready) begin
   s_axi_rdata <= data; s_axi_rvalid <= 1;
  end
  if (s_axi_rvalid && s_axi_rready) s_axi_rvalid <= 0;
 end
end
endmodule
