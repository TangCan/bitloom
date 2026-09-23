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
  reg [31:0] PI_g_pad_in;
  reg [0:0] PI_axi_s_axi_wvalid;
  reg [0:0] PI_axi_s_axi_bready;
  reg [0:0] PI_axi_s_axi_rready;
  reg [0:0] PI_axi_s_axi_arvalid;
  reg [31:0] PI_axi_s_axi_wdata;
  reg [2:0] PI_axi_s_axi_arprot;
  reg [0:0] PI_u_rx;
  reg [15:0] PI_axi_s_axi_araddr;
  reg [2:0] PI_axi_s_axi_awprot;
  wire [0:0] PI_clk = clock;
  reg [0:0] PI_rst;
  reg [15:0] PI_axi_s_axi_awaddr;
  reg [0:0] PI_axi_s_axi_awvalid;
  reg [3:0] PI_axi_s_axi_wstrb;
  Fr198AxiCore UUT (
    .g_pad_in(PI_g_pad_in),
    .axi_s_axi_wvalid(PI_axi_s_axi_wvalid),
    .axi_s_axi_bready(PI_axi_s_axi_bready),
    .axi_s_axi_rready(PI_axi_s_axi_rready),
    .axi_s_axi_arvalid(PI_axi_s_axi_arvalid),
    .axi_s_axi_wdata(PI_axi_s_axi_wdata),
    .axi_s_axi_arprot(PI_axi_s_axi_arprot),
    .u_rx(PI_u_rx),
    .axi_s_axi_araddr(PI_axi_s_axi_araddr),
    .axi_s_axi_awprot(PI_axi_s_axi_awprot),
    .clk(PI_clk),
    .rst(PI_rst),
    .axi_s_axi_awaddr(PI_axi_s_axi_awaddr),
    .axi_s_axi_awvalid(PI_axi_s_axi_awvalid),
    .axi_s_axi_wstrb(PI_axi_s_axi_wstrb)
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
    // UUT.$formal$design.\v:3182$49_EN  = 1'b0;
    // UUT.$formal$design.\v:3187$52_EN  = 1'b0;
    // UUT.$formal$design.\v:3191$54_EN  = 1'b0;
    // UUT.$formal$design.\v:3196$57_EN  = 1'b0;
    // UUT.$formal$design.\v:3232$62_EN  = 1'b0;
    // UUT.$formal$design.\v:3233$63_EN  = 1'b0;
    // UUT.$formal$design.\v:3235$64_EN  = 1'b0;
    // UUT.$formal$design.\v:3239$66_EN  = 1'b0;
    // UUT.$formal$design.\v:3243$68_EN  = 1'b0;
    // UUT.$formal$design.\v:3251$72_EN  = 1'b0;
    // UUT.$formal$design.\v:3257$74_EN  = 1'b0;
    // UUT.$formal$design.\v:3263$76_EN  = 1'b0;
    // UUT.$formal$design.\v:3269$78_EN  = 1'b0;
    UUT._witness_.anyinit_procdff_1505 = 1'b0;
    UUT._witness_.anyinit_procdff_1506 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1507 = 1'b0;
    UUT._witness_.anyinit_procdff_1509 = 1'b0;
    UUT._witness_.anyinit_procdff_1511 = 1'b0;
    UUT._witness_.anyinit_procdff_1512 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1513 = 1'b0;
    UUT._witness_.anyinit_procdff_1515 = 1'b0;
    UUT._witness_.anyinit_procdff_1517 = 1'b0;
    UUT._witness_.anyinit_procdff_1518 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1519 = 1'b0;
    UUT._witness_.anyinit_procdff_1521 = 1'b0;
    UUT._witness_.anyinit_procdff_1523 = 1'b0;
    UUT._witness_.anyinit_procdff_1524 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1525 = 1'b0;
    UUT._witness_.anyinit_procdff_1527 = 1'b0;
    UUT._witness_.anyinit_procdff_1534 = 1'b0;
    UUT._witness_.anyinit_procdff_1535 = 1'b0;
    UUT._witness_.anyinit_procdff_1536 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1537 = 1'b0;
    UUT._witness_.anyinit_procdff_1538 = 1'b0;
    UUT._witness_.anyinit_procdff_1539 = 1'b0;
    UUT._witness_.anyinit_procdff_1540 = 1'b0;
    UUT._witness_.anyinit_procdff_1542 = 1'b0;
    UUT._witness_.anyinit_procdff_1544 = 1'b0;
    UUT._witness_.anyinit_procdff_1546 = 1'b0;
    UUT._witness_.anyinit_procdff_1548 = 1'b0;
    UUT._witness_.anyinit_procdff_1550 = 1'b0;
    UUT._witness_.anyinit_procdff_1552 = 1'b0;
    UUT._witness_.anyinit_procdff_1554 = 1'b0;
    UUT._witness_.anyinit_procdff_1556 = 1'b0;
    UUT._witness_.anyinit_procdff_1558 = 1'b0;
    UUT._witness_.anyinit_procdff_1561 = 1'b0;
    UUT._witness_.anyinit_procdff_1562 = 1'b0;
    UUT._witness_.anyinit_procdff_1563 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1564 = 2'b00;
    UUT._witness_.anyinit_procdff_1565 = 1'b0;
    UUT._witness_.anyinit_procdff_1566 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1567 = 2'b00;
    UUT._witness_.anyinit_procdff_1568 = 1'b0;
    UUT._witness_.anyinit_procdff_1569 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1570 = 2'b00;
    UUT._witness_.anyinit_procdff_1571 = 1'b0;
    UUT._witness_.anyinit_procdff_1572 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1573 = 2'b00;
    UUT._witness_.anyinit_procdff_1574 = 1'b0;
    UUT._witness_.anyinit_procdff_1575 = 1'b0;
    UUT._witness_.anyinit_procdff_1576 = 2'b00;
    UUT._witness_.anyinit_procdff_1577 = 1'b0;
    UUT._witness_.anyinit_procdff_1578 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1579 = 2'b00;
    UUT._witness_.anyinit_procdff_1580 = 1'b0;
    UUT._witness_.anyinit_procdff_1581 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1582 = 2'b00;
    UUT._witness_.anyinit_procdff_1595 = 1'b0;
    UUT._witness_.anyinit_procdff_1597 = 1'b0;
    UUT._witness_.anyinit_procdff_1599 = 1'b0;
    UUT._witness_.anyinit_procdff_1601 = 1'b0;
    UUT._witness_.anyinit_procdff_1603 = 1'b0;
    UUT._witness_.anyinit_procdff_1605 = 1'b0;
    UUT._witness_.anyinit_procdff_1607 = 1'b0;
    UUT._witness_.anyinit_procdff_1609 = 1'b0;
    UUT._witness_.anyinit_procdff_1611 = 1'b0;
    UUT._witness_.anyinit_procdff_1613 = 1'b0;
    UUT._witness_.anyinit_procdff_1615 = 1'b0;
    UUT._witness_.anyinit_procdff_1617 = 1'b0;
    UUT._witness_.anyinit_procdff_1619 = 1'b0;
    UUT.bridge.ar_addr = 16'b0000000000000000;
    UUT.bridge.ar_full = 1'b0;
    UUT.bridge.aw_addr = 16'b0000000000000001;
    UUT.bridge.aw_full = 1'b0;
    UUT.bridge.b_error = 2'b00;
    UUT.bridge.b_full = 1'b1;
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
    UUT.bridge.w_full = 1'b1;
    UUT.bridge.w_strb = 4'b0000;
    UUT.decoder.busy = 1'b1;
    UUT.decoder.miss = 1'b0;
    UUT.decoder.owner = 2'b10;
    UUT.fr201_ar_credit = 1'b0;
    UUT.fr201_aw_credit = 1'b0;
    UUT.fr201_cancelled_pending = 1'b0;
    UUT.fr201_epoch_accept = 1'b0;
    UUT.fr201_past_valid = 1'b0;
    UUT.fr201_w_credit = 1'b0;

    // state 0
    PI_g_pad_in = 32'b00000000000000000000000000000000;
    PI_axi_s_axi_wvalid = 1'b0;
    PI_axi_s_axi_bready = 1'b0;
    PI_axi_s_axi_rready = 1'b0;
    PI_axi_s_axi_arvalid = 1'b0;
    PI_axi_s_axi_wdata = 32'b00000000000000000000000000000000;
    PI_axi_s_axi_arprot = 3'b000;
    PI_u_rx = 1'b0;
    PI_axi_s_axi_araddr = 16'b0000000000000000;
    PI_axi_s_axi_awprot = 3'b000;
    PI_rst = 1'b1;
    PI_axi_s_axi_awaddr = 16'b0000000000000000;
    PI_axi_s_axi_awvalid = 1'b0;
    PI_axi_s_axi_wstrb = 4'b0000;
  end
  always @(posedge clock) begin
    // state 1
    if (cycle == 0) begin
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000001;
      PI_axi_s_axi_awprot <= 3'b000;
      PI_rst <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000001100000100;
      PI_axi_s_axi_awvalid <= 1'b1;
      PI_axi_s_axi_wstrb <= 4'b0101;
    end

    // state 2
    if (cycle == 1) begin
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000001;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000001;
      PI_axi_s_axi_awprot <= 3'b000;
      PI_rst <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0001;
    end

    // state 3
    if (cycle == 2) begin
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_arvalid <= 1'b1;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000010;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000001;
      PI_axi_s_axi_awprot <= 3'b000;
      PI_rst <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0001;
    end

    // state 4
    if (cycle == 3) begin
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_wvalid <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000001;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_awprot <= 3'b000;
      PI_rst <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0001;
    end

    // state 5
    if (cycle == 4) begin
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_axi_s_axi_arprot <= 3'b000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_awprot <= 3'b000;
      PI_rst <= 1'b1;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0000;
    end

    genclock <= cycle < 5;
    cycle <= cycle + 1;
  end
endmodule
