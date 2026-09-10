// FR119 SymbiYosys fixture — PASS (assume + assert; BMC should succeed).
// Product: Bitloom. Unrelated to samitbasu/rhdl.
module fr119_pass (
  input wire clk,
  input wire rst,
  input wire [3:0] din
);
  reg [3:0] q;

  always @(posedge clk) begin
    if (rst)
      q <= 4'd0;
    else
      q <= din;
  end

  // Environment: din stays below 10 when out of reset.
  assume property (@(posedge clk) disable iff (rst) din < 4'd10);
  // Property: q inherits the same bound.
  assert property (@(posedge clk) disable iff (rst) q < 4'd10);
endmodule
