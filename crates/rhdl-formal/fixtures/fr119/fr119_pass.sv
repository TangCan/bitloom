// FR119 SymbiYosys fixture — PASS (assume + assert; BMC should succeed).
// Product: Bitloom. Unrelated to samitbasu/rhdl.
module fr119_pass (
  input wire clk,
  input wire rst,
  input wire [3:0] din
);
  reg [3:0] q;

  always @(posedge clk) begin
    // Establish a defined state before checking the sampled register value.
    if ($initstate) assume property (rst);
    if (rst) q <= 4'd0;
    else q <= din;
    if (!rst) begin
      assume property (din < 4'd10);
      assert property (q < 4'd10);
    end
  end
endmodule
