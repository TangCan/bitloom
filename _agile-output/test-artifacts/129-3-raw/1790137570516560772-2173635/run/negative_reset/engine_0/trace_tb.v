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
  reg [2:0] PI_axi_s_axi_awprot;
  reg [0:0] PI_axi_s_axi_wvalid;
  reg [0:0] PI_u_rx;
  wire [0:0] PI_clk = clock;
  reg [0:0] PI_axi_s_axi_rready;
  reg [15:0] PI_axi_s_axi_araddr;
  reg [3:0] PI_axi_s_axi_wstrb;
  reg [0:0] PI_axi_s_axi_arvalid;
  reg [31:0] PI_g_pad_in;
  reg [15:0] PI_axi_s_axi_awaddr;
  reg [31:0] PI_axi_s_axi_wdata;
  reg [2:0] PI_axi_s_axi_arprot;
  reg [0:0] PI_axi_s_axi_awvalid;
  reg [0:0] PI_rst;
  reg [0:0] PI_axi_s_axi_bready;
  Fr198AxiCore UUT (
    .axi_s_axi_awprot(PI_axi_s_axi_awprot),
    .axi_s_axi_wvalid(PI_axi_s_axi_wvalid),
    .u_rx(PI_u_rx),
    .clk(PI_clk),
    .axi_s_axi_rready(PI_axi_s_axi_rready),
    .axi_s_axi_araddr(PI_axi_s_axi_araddr),
    .axi_s_axi_wstrb(PI_axi_s_axi_wstrb),
    .axi_s_axi_arvalid(PI_axi_s_axi_arvalid),
    .g_pad_in(PI_g_pad_in),
    .axi_s_axi_awaddr(PI_axi_s_axi_awaddr),
    .axi_s_axi_wdata(PI_axi_s_axi_wdata),
    .axi_s_axi_arprot(PI_axi_s_axi_arprot),
    .axi_s_axi_awvalid(PI_axi_s_axi_awvalid),
    .rst(PI_rst),
    .axi_s_axi_bready(PI_axi_s_axi_bready)
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
    // UUT.$formal$mutant_reset.\v:3186$50_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3191$53_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3195$55_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3200$58_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3236$63_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3237$64_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3239$65_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3243$67_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3247$69_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3255$73_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3261$75_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3267$77_EN  = 1'b0;
    // UUT.$formal$mutant_reset.\v:3273$79_EN  = 1'b0;
    UUT._witness_.anyinit_procdff_1516 = 1'b0;
    UUT._witness_.anyinit_procdff_1517 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1518 = 1'b0;
    UUT._witness_.anyinit_procdff_1520 = 1'b0;
    UUT._witness_.anyinit_procdff_1522 = 1'b0;
    UUT._witness_.anyinit_procdff_1523 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1524 = 1'b0;
    UUT._witness_.anyinit_procdff_1526 = 1'b0;
    UUT._witness_.anyinit_procdff_1528 = 1'b0;
    UUT._witness_.anyinit_procdff_1529 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1530 = 1'b0;
    UUT._witness_.anyinit_procdff_1532 = 1'b0;
    UUT._witness_.anyinit_procdff_1534 = 1'b0;
    UUT._witness_.anyinit_procdff_1535 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1536 = 1'b0;
    UUT._witness_.anyinit_procdff_1538 = 1'b0;
    UUT._witness_.anyinit_procdff_1546 = 1'b0;
    UUT._witness_.anyinit_procdff_1547 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1548 = 1'b0;
    UUT._witness_.anyinit_procdff_1551 = 1'b0;
    UUT._witness_.anyinit_procdff_1553 = 1'b0;
    UUT._witness_.anyinit_procdff_1555 = 1'b0;
    UUT._witness_.anyinit_procdff_1557 = 1'b0;
    UUT._witness_.anyinit_procdff_1559 = 1'b0;
    UUT._witness_.anyinit_procdff_1561 = 1'b0;
    UUT._witness_.anyinit_procdff_1563 = 1'b0;
    UUT._witness_.anyinit_procdff_1572 = 1'b0;
    UUT._witness_.anyinit_procdff_1573 = 1'b0;
    UUT._witness_.anyinit_procdff_1574 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1575 = 2'b00;
    UUT._witness_.anyinit_procdff_1576 = 1'b0;
    UUT._witness_.anyinit_procdff_1577 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1578 = 2'b00;
    UUT._witness_.anyinit_procdff_1579 = 1'b0;
    UUT._witness_.anyinit_procdff_1580 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1581 = 2'b00;
    UUT._witness_.anyinit_procdff_1582 = 1'b0;
    UUT._witness_.anyinit_procdff_1583 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1584 = 2'b00;
    UUT._witness_.anyinit_procdff_1585 = 1'b0;
    UUT._witness_.anyinit_procdff_1586 = 1'b0;
    UUT._witness_.anyinit_procdff_1587 = 2'b00;
    UUT._witness_.anyinit_procdff_1588 = 1'b0;
    UUT._witness_.anyinit_procdff_1589 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1590 = 2'b00;
    UUT._witness_.anyinit_procdff_1591 = 1'b0;
    UUT._witness_.anyinit_procdff_1592 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1593 = 2'b00;
    UUT._witness_.anyinit_procdff_1606 = 1'b0;
    UUT._witness_.anyinit_procdff_1608 = 1'b0;
    UUT._witness_.anyinit_procdff_1610 = 1'b0;
    UUT._witness_.anyinit_procdff_1612 = 1'b0;
    UUT._witness_.anyinit_procdff_1614 = 1'b0;
    UUT._witness_.anyinit_procdff_1616 = 1'b0;
    UUT._witness_.anyinit_procdff_1618 = 1'b0;
    UUT._witness_.anyinit_procdff_1620 = 1'b0;
    UUT._witness_.anyinit_procdff_1622 = 1'b0;
    UUT._witness_.anyinit_procdff_1624 = 1'b0;
    UUT._witness_.anyinit_procdff_1626 = 1'b0;
    UUT._witness_.anyinit_procdff_1633 = 1'b0;
    UUT.bridge.ar_addr = 16'b0000001100000000;
    UUT.bridge.ar_full = 1'b0;
    UUT.bridge.aw_addr = 16'b0000000000000000;
    UUT.bridge.aw_full = 1'b1;
    UUT.bridge.b_error = 2'b00;
    UUT.bridge.b_full = 1'b0;
    UUT.bridge.exec = 1'b0;
    UUT.bridge.offer = 1'b0;
    UUT.bridge.offer_data = 32'b00000000000000000000000000000000;
    UUT.bridge.offer_strb = 4'b0000;
    UUT.bridge.offer_write = 1'b0;
    UUT.bridge.owner = 1'b0;
    UUT.bridge.prefer_write = 1'b0;
    UUT.bridge.r_data = 32'b00000000000000000000000000000000;
    UUT.bridge.r_error = 2'b00;
    UUT.bridge.r_full = 1'b0;
    UUT.bridge.w_data = 32'b00000000000000000000000000000000;
    UUT.bridge.w_full = 1'b0;
    UUT.bridge.w_strb = 4'b0000;
    UUT.decoder.busy = 1'b1;
    UUT.decoder.miss = 1'b0;
    UUT.decoder.owner = 2'b00;
    UUT.fr201_ar_credit = 1'b0;
    UUT.fr201_aw_credit = 1'b0;
    UUT.fr201_epoch_accept = 1'b0;
    UUT.fr201_mutant_replay = 1'b0;
    UUT.fr201_past_valid = 1'b0;
    UUT.fr201_w_credit = 1'b0;

    // state 0
    PI_axi_s_axi_awprot = 3'b000;
    PI_axi_s_axi_wvalid = 1'b0;
    PI_u_rx = 1'b0;
    PI_axi_s_axi_rready = 1'b0;
    PI_axi_s_axi_araddr = 16'b0000000000000000;
    PI_axi_s_axi_wstrb = 4'b0000;
    PI_axi_s_axi_arvalid = 1'b0;
    PI_g_pad_in = 32'b00000000000000000000000000000000;
    PI_axi_s_axi_awaddr = 16'b0000000000000000;
    PI_axi_s_axi_wdata = 32'b00000000000000000000000000000000;
    PI_axi_s_axi_arprot = 3'b000;
    PI_axi_s_axi_awvalid = 1'b0;
    PI_rst = 1'b1;
    PI_axi_s_axi_bready = 1'b0;
  end
  always @(posedge clock) begin
    // state 1
    if (cycle == 0) begin
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_arvalid <= 1'b1;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000010000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b1;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
    end

    // state 2
    if (cycle == 1) begin
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_wstrb <= 4'b0001;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_wdata <= 32'b01000000000000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
    end

    // state 3
    if (cycle == 2) begin
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_wvalid <= 1'b0;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000010;
      PI_axi_s_axi_wstrb <= 4'b1000;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_wdata <= 32'b10000000000000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
    end

    // state 4
    if (cycle == 3) begin
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_wstrb <= 4'b0100;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_wdata <= 32'b00000000000000001000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_rst <= 1'b1;
      PI_axi_s_axi_bready <= 1'b0;
    end

    // state 5
    if (cycle == 4) begin
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000010;
      PI_axi_s_axi_wstrb <= 4'b1000;
      PI_axi_s_axi_arvalid <= 1'b1;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_wdata <= 32'b00000000001000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
    end

    // state 6
    if (cycle == 5) begin
      PI_axi_s_axi_awprot <= 3'b000;
      PI_axi_s_axi_wvalid <= 1'b0;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
    end

    genclock <= cycle < 6;
    cycle <= cycle + 1;
  end
endmodule
