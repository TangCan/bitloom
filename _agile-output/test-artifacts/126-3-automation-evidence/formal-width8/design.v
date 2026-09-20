module RvRegSlice (
  input clk,
  input rst,
  input flush,
  input input_valid,
  input output_ready,
  input [7:0] input_data,
  output input_ready,
  output output_valid,
  output [7:0] output_data
);
  reg [1:0] count;
  reg [7:0] front;
  reg [7:0] back;
  wire [1:0] zero;
  wire [1:0] one;
  wire [1:0] two;
  wire [1:0] inc;
  wire [1:0] dec;
  wire [1:0] after_push;
  wire [1:0] normal_count;
  wire [1:0] next_count;
  wire true_bit;
  wire empty;
  wire single;
  wire full;
  wire push;
  wire pop;
  wire replace_single;
  wire load_front;
  wire push_front;
  wire [7:0] after_pop;
  wire [7:0] next_front;
  assign zero = 0;
  assign one = 1;
  assign two = 2;
  assign true_bit = 1;
  assign empty = (count == zero);
  assign single = (count == one);
  assign full = (count == two);
  assign input_ready = (full ^ true_bit);
  assign output_valid = (empty ^ true_bit);
  assign output_data = front;
  assign push = (input_valid & input_ready);
  assign pop = (output_valid & output_ready);
  assign inc = count + one;
  assign after_push = (push ? inc : count);
  assign dec = after_push - one;
  assign normal_count = (pop ? dec : after_push);
  assign next_count = (flush ? zero : normal_count);
  assign replace_single = (pop & single);
  assign load_front = (empty | replace_single);
  assign push_front = (push & load_front);
  assign after_pop = (pop ? back : front);
  assign next_front = (push_front ? input_data : after_pop);
  always @(posedge clk) begin
    if (rst) begin
      count <= 0;
      front <= 0;
      back <= 0;
    end else begin
      count <= next_count;
      front <= next_front;
      back <= (push ? input_data : back);
    end
  end
endmodule

