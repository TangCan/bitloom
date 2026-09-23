`ifndef VERILATOR
module testbench;
  reg [4095:0] vcdfile;
  reg clock;
`else
module testbench(input clock, output reg genclock);
  initial genclock = 1;
`endif
  reg genclock = 1;
  reg [31:0] cycle = 0;
  reg [0:0] PI_axi_s_axi_wvalid;
  reg [3:0] PI_axi_s_axi_wstrb;
  reg [0:0] PI_axi_s_axi_rready;
  reg [2:0] PI_axi_s_axi_arprot;
  reg [15:0] PI_axi_s_axi_araddr;
  reg [31:0] PI_g_pad_in;
  wire [0:0] PI_clk = clock;
  reg [15:0] PI_axi_s_axi_awaddr;
  reg [2:0] PI_axi_s_axi_awprot;
  reg [0:0] PI_axi_s_axi_awvalid;
  reg [0:0] PI_u_rx;
  reg [0:0] PI_axi_s_axi_arvalid;
  reg [0:0] PI_axi_s_axi_bready;
  reg [31:0] PI_axi_s_axi_wdata;
  reg [0:0] PI_rst;
  Fr198AxiCore UUT (
    .axi_s_axi_wvalid(PI_axi_s_axi_wvalid),
    .axi_s_axi_wstrb(PI_axi_s_axi_wstrb),
    .axi_s_axi_rready(PI_axi_s_axi_rready),
    .axi_s_axi_arprot(PI_axi_s_axi_arprot),
    .axi_s_axi_araddr(PI_axi_s_axi_araddr),
    .g_pad_in(PI_g_pad_in),
    .clk(PI_clk),
    .axi_s_axi_awaddr(PI_axi_s_axi_awaddr),
    .axi_s_axi_awprot(PI_axi_s_axi_awprot),
    .axi_s_axi_awvalid(PI_axi_s_axi_awvalid),
    .u_rx(PI_u_rx),
    .axi_s_axi_arvalid(PI_axi_s_axi_arvalid),
    .axi_s_axi_bready(PI_axi_s_axi_bready),
    .axi_s_axi_wdata(PI_axi_s_axi_wdata),
    .rst(PI_rst)
  );
`ifndef VERILATOR
  initial begin
    if ($value$plusargs("vcd=%s", vcdfile)) begin
      $dumpfile(vcdfile);
      $dumpvars(0, testbench);
    end
    #5 clock = 0;
    while (genclock) begin
      #5 clock = 0;
      #5 clock = 1;
    end
  end
`endif
  initial begin
`ifndef VERILATOR
    #1;
`endif
    // UUT.$formal$design.\v:3205$7_EN  = 1'b0;
    UUT._witness_.anyinit_procdff_837 = 1'b1;
    UUT.bridge.ar_addr = 16'b0000001100000000;
    UUT.bridge.ar_full = 1'b0;
    UUT.bridge.aw_addr = 16'b0000000000000000;
    UUT.bridge.aw_full = 1'b0;
    UUT.bridge.b_error = 2'b00;
    UUT.bridge.b_full = 1'b1;
    UUT.bridge.exec = 1'b0;
    UUT.bridge.offer = 1'b0;
    UUT.bridge.offer_data = 32'b00000000000000000000000000000000;
    UUT.bridge.offer_strb = 4'b0000;
    UUT.bridge.offer_write = 1'b0;
    UUT.bridge.owner = 1'b1;
    UUT.bridge.prefer_write = 1'b0;
    UUT.bridge.r_data = 32'b00000000000000000000000000000000;
    UUT.bridge.r_error = 2'b01;
    UUT.bridge.r_full = 1'b1;
    UUT.bridge.w_data = 32'b00000000000000000000000000000001;
    UUT.bridge.w_full = 1'b0;
    UUT.bridge.w_strb = 4'b0001;
    UUT.decoder.busy = 1'b0;
    UUT.decoder.miss = 1'b1;
    UUT.decoder.owner = 2'b10;

    // state 0
    PI_axi_s_axi_wvalid = 1'b1;
    PI_axi_s_axi_wstrb = 4'b0000;
    PI_axi_s_axi_rready = 1'b0;
    PI_axi_s_axi_arprot = 3'b000;
    PI_axi_s_axi_araddr = 16'b0000000000000001;
    PI_g_pad_in = 32'b00000000000000000000000000000000;
    PI_axi_s_axi_awaddr = 16'b0000000000000010;
    PI_axi_s_axi_awprot = 3'b000;
    PI_axi_s_axi_awvalid = 1'b1;
    PI_u_rx = 1'b0;
    PI_axi_s_axi_arvalid = 1'b1;
    PI_axi_s_axi_bready = 1'b0;
    PI_axi_s_axi_wdata = 32'b00000000000000000000000000000000;
    PI_rst = 1'b0;
  end
  always @(posedge clock) begin
    // state 1
    if (cycle == 0) begin
      PI_axi_s_axi_wvalid <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_rst <= 1'b0;
    end

    genclock <= cycle < 1;
    cycle <= cycle + 1;
  end
endmodule
