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
  reg [2:0] PI_axi_s_axi_arprot;
  wire [0:0] PI_clk = clock;
  reg [31:0] PI_axi_s_axi_wdata;
  reg [31:0] PI_g_pad_in;
  reg [0:0] PI_u_rx;
  reg [0:0] PI_axi_s_axi_rready;
  reg [3:0] PI_axi_s_axi_wstrb;
  reg [15:0] PI_axi_s_axi_araddr;
  reg [0:0] PI_axi_s_axi_awvalid;
  reg [15:0] PI_axi_s_axi_awaddr;
  reg [0:0] PI_axi_s_axi_arvalid;
  reg [0:0] PI_rst;
  reg [0:0] PI_axi_s_axi_bready;
  reg [0:0] PI_axi_s_axi_wvalid;
  reg [2:0] PI_axi_s_axi_awprot;
  Fr198AxiCore UUT (
    .axi_s_axi_arprot(PI_axi_s_axi_arprot),
    .clk(PI_clk),
    .axi_s_axi_wdata(PI_axi_s_axi_wdata),
    .g_pad_in(PI_g_pad_in),
    .u_rx(PI_u_rx),
    .axi_s_axi_rready(PI_axi_s_axi_rready),
    .axi_s_axi_wstrb(PI_axi_s_axi_wstrb),
    .axi_s_axi_araddr(PI_axi_s_axi_araddr),
    .axi_s_axi_awvalid(PI_axi_s_axi_awvalid),
    .axi_s_axi_awaddr(PI_axi_s_axi_awaddr),
    .axi_s_axi_arvalid(PI_axi_s_axi_arvalid),
    .rst(PI_rst),
    .axi_s_axi_bready(PI_axi_s_axi_bready),
    .axi_s_axi_wvalid(PI_axi_s_axi_wvalid),
    .axi_s_axi_awprot(PI_axi_s_axi_awprot)
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
    // UUT.$formal$mutant_request.\v:3182$49_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3187$52_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3191$54_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3196$57_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3232$62_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3233$63_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3235$64_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3239$66_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3243$68_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3251$72_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3257$74_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3263$76_EN  = 1'b0;
    // UUT.$formal$mutant_request.\v:3269$78_EN  = 1'b0;
    UUT._witness_.anyinit_procdff_1506 = 1'b0;
    UUT._witness_.anyinit_procdff_1507 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1508 = 1'b0;
    UUT._witness_.anyinit_procdff_1510 = 1'b0;
    UUT._witness_.anyinit_procdff_1512 = 1'b0;
    UUT._witness_.anyinit_procdff_1513 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1514 = 1'b0;
    UUT._witness_.anyinit_procdff_1516 = 1'b0;
    UUT._witness_.anyinit_procdff_1518 = 1'b0;
    UUT._witness_.anyinit_procdff_1519 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1520 = 1'b0;
    UUT._witness_.anyinit_procdff_1522 = 1'b0;
    UUT._witness_.anyinit_procdff_1524 = 1'b0;
    UUT._witness_.anyinit_procdff_1525 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1526 = 1'b0;
    UUT._witness_.anyinit_procdff_1528 = 1'b0;
    UUT._witness_.anyinit_procdff_1536 = 1'b0;
    UUT._witness_.anyinit_procdff_1537 = 53'b00000000000000000000000000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1538 = 1'b0;
    UUT._witness_.anyinit_procdff_1541 = 1'b0;
    UUT._witness_.anyinit_procdff_1543 = 1'b0;
    UUT._witness_.anyinit_procdff_1545 = 1'b0;
    UUT._witness_.anyinit_procdff_1547 = 1'b0;
    UUT._witness_.anyinit_procdff_1549 = 1'b0;
    UUT._witness_.anyinit_procdff_1551 = 1'b0;
    UUT._witness_.anyinit_procdff_1553 = 1'b0;
    UUT._witness_.anyinit_procdff_1562 = 1'b0;
    UUT._witness_.anyinit_procdff_1563 = 1'b0;
    UUT._witness_.anyinit_procdff_1564 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1565 = 2'b00;
    UUT._witness_.anyinit_procdff_1566 = 1'b0;
    UUT._witness_.anyinit_procdff_1567 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1568 = 2'b00;
    UUT._witness_.anyinit_procdff_1569 = 1'b0;
    UUT._witness_.anyinit_procdff_1570 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1571 = 2'b00;
    UUT._witness_.anyinit_procdff_1572 = 1'b0;
    UUT._witness_.anyinit_procdff_1573 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1574 = 2'b00;
    UUT._witness_.anyinit_procdff_1575 = 1'b0;
    UUT._witness_.anyinit_procdff_1576 = 1'b0;
    UUT._witness_.anyinit_procdff_1577 = 2'b00;
    UUT._witness_.anyinit_procdff_1578 = 1'b0;
    UUT._witness_.anyinit_procdff_1579 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1580 = 2'b00;
    UUT._witness_.anyinit_procdff_1581 = 1'b0;
    UUT._witness_.anyinit_procdff_1582 = 32'b00000000000000000000000000000000;
    UUT._witness_.anyinit_procdff_1583 = 2'b00;
    UUT._witness_.anyinit_procdff_1596 = 1'b0;
    UUT._witness_.anyinit_procdff_1598 = 1'b0;
    UUT._witness_.anyinit_procdff_1600 = 1'b0;
    UUT._witness_.anyinit_procdff_1602 = 1'b0;
    UUT._witness_.anyinit_procdff_1604 = 1'b0;
    UUT._witness_.anyinit_procdff_1606 = 1'b0;
    UUT._witness_.anyinit_procdff_1608 = 1'b0;
    UUT._witness_.anyinit_procdff_1610 = 1'b0;
    UUT._witness_.anyinit_procdff_1612 = 1'b0;
    UUT._witness_.anyinit_procdff_1614 = 1'b0;
    UUT._witness_.anyinit_procdff_1616 = 1'b0;
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
    UUT.bridge.r_full = 1'b1;
    UUT.bridge.w_data = 32'b00000000000000000000000000000000;
    UUT.bridge.w_full = 1'b0;
    UUT.bridge.w_strb = 4'b0000;
    UUT.decoder.busy = 1'b1;
    UUT.decoder.miss = 1'b0;
    UUT.decoder.owner = 2'b00;
    UUT.fr201_ar_credit = 1'b0;
    UUT.fr201_aw_credit = 1'b0;
    UUT.fr201_epoch_accept = 1'b0;
    UUT.fr201_past_valid = 1'b0;
    UUT.fr201_w_credit = 1'b0;

    // state 0
    PI_axi_s_axi_arprot = 3'b000;
    PI_axi_s_axi_wdata = 32'b00000000000000000000000000000000;
    PI_g_pad_in = 32'b00000000000000000000000000000000;
    PI_u_rx = 1'b0;
    PI_axi_s_axi_rready = 1'b0;
    PI_axi_s_axi_wstrb = 4'b0000;
    PI_axi_s_axi_araddr = 16'b0000000000000000;
    PI_axi_s_axi_awvalid = 1'b0;
    PI_axi_s_axi_awaddr = 16'b0000000000000000;
    PI_axi_s_axi_arvalid = 1'b0;
    PI_rst = 1'b1;
    PI_axi_s_axi_bready = 1'b0;
    PI_axi_s_axi_wvalid = 1'b0;
    PI_axi_s_axi_awprot = 3'b000;
  end
  always @(posedge clock) begin
    // state 1
    if (cycle == 0) begin
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_araddr <= 16'b0000000000000001;
      PI_axi_s_axi_awvalid <= 1'b1;
      PI_axi_s_axi_awaddr <= 16'b0000001100000000;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_awprot <= 3'b000;
    end

    // state 2
    if (cycle == 1) begin
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_u_rx <= 1'b1;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0100;
      PI_axi_s_axi_araddr <= 16'b1000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000000000000010;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_wvalid <= 1'b0;
      PI_axi_s_axi_awprot <= 3'b000;
    end

    // state 3
    if (cycle == 2) begin
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_araddr <= 16'b0000000010000000;
      PI_axi_s_axi_awvalid <= 1'b1;
      PI_axi_s_axi_awaddr <= 16'b0000001100000000;
      PI_axi_s_axi_arvalid <= 1'b1;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_awprot <= 3'b000;
    end

    // state 4
    if (cycle == 3) begin
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_u_rx <= 1'b1;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_rst <= 1'b0;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_awprot <= 3'b000;
    end

    // state 5
    if (cycle == 4) begin
      PI_axi_s_axi_arprot <= 3'b000;
      PI_axi_s_axi_wdata <= 32'b00000000000000000000000000000000;
      PI_g_pad_in <= 32'b00000000000000000000000000000000;
      PI_u_rx <= 1'b0;
      PI_axi_s_axi_rready <= 1'b0;
      PI_axi_s_axi_wstrb <= 4'b0000;
      PI_axi_s_axi_araddr <= 16'b0000000000000000;
      PI_axi_s_axi_awvalid <= 1'b0;
      PI_axi_s_axi_awaddr <= 16'b0000000000000000;
      PI_axi_s_axi_arvalid <= 1'b0;
      PI_rst <= 1'b1;
      PI_axi_s_axi_bready <= 1'b0;
      PI_axi_s_axi_wvalid <= 1'b1;
      PI_axi_s_axi_awprot <= 3'b000;
    end

    genclock <= cycle < 5;
    cycle <= cycle + 1;
  end
endmodule
