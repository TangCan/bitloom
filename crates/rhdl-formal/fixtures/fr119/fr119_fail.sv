// FR119 SymbiYosys fixture — FAIL (assert deliberately broken; BMC should FAIL).
// Product: Bitloom. Unrelated to samitbasu/rhdl.
module fr119_fail (
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

  assume property (@(posedge clk) disable iff (rst) din < 4'd10);
  // Deliberately false: claims q is always zero after reset clears.
  assert property (@(posedge clk) disable iff (rst) q == 4'd0);
endmodule
