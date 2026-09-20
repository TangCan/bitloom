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
  reg [7:0] PI_input_data;
  reg [0:0] PI_input_valid;
  reg [0:0] PI_rst;
  reg [0:0] PI_flush;
  reg [0:0] PI_output_ready;
  wire [0:0] PI_clk = clock;
  RvRegSlice UUT (
    .input_data(PI_input_data),
    .input_valid(PI_input_valid),
    .rst(PI_rst),
    .flush(PI_flush),
    .output_ready(PI_output_ready),
    .clk(PI_clk)
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
    // UUT.$formal$design_formal.\v:76$6_EN  = 1'b0;
    // UUT.$formal$design_formal.\v:80$7_EN  = 1'b0;
    // UUT.$formal$design_formal.\v:84$11_EN  = 1'b0;
    // UUT.$formal$design_formal.\v:87$14_EN  = 1'b0;
    // UUT.$past$design_formal.\v:87$1$0  = 1'b0;
    UUT._witness_.anyinit_procdff_303 = 8'b00000000;
    UUT._witness_.anyinit_procdff_304 = 1'b0;
    UUT._witness_.anyinit_procdff_305 = 8'b00000000;
    UUT._witness_.anyinit_procdff_308 = 1'b0;
    UUT._witness_.anyinit_procdff_310 = 1'b0;
    UUT._witness_.anyinit_procdff_312 = 1'b0;
    UUT._witness_.anyinit_procdff_314 = 1'b0;
    UUT._witness_.anyinit_procdff_316 = 1'b0;
    UUT._witness_.anyinit_procdff_318 = 1'b0;
    UUT._witness_.anyinit_procdff_320 = 1'b0;
    UUT._witness_.anyinit_procdff_322 = 1'b0;
    UUT._witness_.anyinit_procdff_324 = 1'b0;
    UUT._witness_.anyinit_procdff_326 = 1'b0;
    UUT._witness_.anyinit_procdff_332 = 1'b0;
    UUT._witness_.anyinit_procdff_334 = 1'b0;
    UUT.back = 8'b00000000;
    UUT.count = 2'b00;
    UUT.front = 8'b00000000;
    UUT.past_valid = 1'b0;
    UUT.pending = 2'b00;
    UUT.q0 = 8'b10000000;
    UUT.q1 = 8'b00000000;
    UUT.seen_full = 1'b0;

    // state 0
    PI_input_data = 8'b00000000;
    PI_input_valid = 1'b0;
    PI_rst = 1'b1;
    PI_flush = 1'b0;
    PI_output_ready = 1'b0;
  end
  always @(posedge clock) begin
    // state 1
    if (cycle == 0) begin
      PI_input_data <= 8'b10000000;
      PI_input_valid <= 1'b1;
      PI_rst <= 1'b0;
      PI_flush <= 1'b0;
      PI_output_ready <= 1'b0;
    end

    // state 2
    if (cycle == 1) begin
      PI_input_data <= 8'b00000001;
      PI_input_valid <= 1'b1;
      PI_rst <= 1'b0;
      PI_flush <= 1'b0;
      PI_output_ready <= 1'b0;
    end

    // state 3
    if (cycle == 2) begin
      PI_input_data <= 8'b00000000;
      PI_input_valid <= 1'b0;
      PI_rst <= 1'b0;
      PI_flush <= 1'b0;
      PI_output_ready <= 1'b1;
    end

    // state 4
    if (cycle == 3) begin
      PI_input_data <= 8'b00000010;
      PI_input_valid <= 1'b0;
      PI_rst <= 1'b0;
      PI_flush <= 1'b0;
      PI_output_ready <= 1'b1;
    end

    // state 5
    if (cycle == 4) begin
      PI_input_data <= 8'b00000000;
      PI_input_valid <= 1'b0;
      PI_rst <= 1'b1;
      PI_flush <= 1'b0;
      PI_output_ready <= 1'b1;
    end

    // state 6
    if (cycle == 5) begin
      PI_input_data <= 8'b00000001;
      PI_input_valid <= 1'b0;
      PI_rst <= 1'b0;
      PI_flush <= 1'b0;
      PI_output_ready <= 1'b0;
    end

    genclock <= cycle < 6;
    cycle <= cycle + 1;
  end
endmodule
