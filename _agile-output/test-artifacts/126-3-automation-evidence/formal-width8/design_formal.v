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
  reg past_valid = 0;
  reg [1:0] pending = 0;
  reg [7:0] q0, q1;
  reg seen_full = 0;
  wire ghost_push = input_valid && input_ready;
  wire ghost_pop = output_valid && output_ready;
  always @(posedge clk) begin
    past_valid <= 1;
    if (!past_valid) assume(rst);
    if (past_valid) begin
      // Strengthening assertions, never assumptions: relate hidden storage to
      // the independent port-transaction queue so arbitrary full stalls induct.
      assert(pending == count);
      if (pending == 2) assert(back == q1);
      assert(pending <= 2);
      assert(input_ready == (pending < 2));
      assert(output_valid == (pending != 0));
      if (output_valid) assert(output_data == q0);
      assert(!ghost_push || pending < 2);
      assert(!ghost_pop || pending != 0);
      if ($past(output_valid && !output_ready && !rst && !flush)) begin
        assert(output_valid);
        assert(output_data == $past(output_data));
      end
      // 合法生产者受阻时保持；reset/flush显式取消该epoch。
      if ($past(input_valid && !input_ready && !rst && !flush) && !rst && !flush) begin
        assume(input_valid);
        assume(input_data == $past(input_data));
      end
      cover(seen_full && pending == 0);
      cover(!rst && !flush && pending == 1 && ghost_push && ghost_pop);
      // [P1] Full-slot reset cancellation can accept a fresh transaction next cycle.
      cover($past(pending == 2 && rst) && pending == 0 && !rst && !flush && ghost_push);
      // [P1] Flush alone cancels a full slot and permits next-cycle recovery.
      cover($past(pending == 2 && !rst && flush) && pending == 0 && !rst && !flush && ghost_push);
    end
    if (rst || flush) begin
      pending <= 0;
      seen_full <= 0;
    end else begin
      if (pending == 2) seen_full <= 1;
      // 独立端口事务队列：同时握手先交付队首，再接受新payload。
      case ({ghost_push,ghost_pop})
        2'b10: begin
          if (pending == 0) q0 <= input_data;
          else q1 <= input_data;
          pending <= pending + 1;
        end
        2'b01: begin q0 <= q1; pending <= pending - 1; end
        2'b11: begin q0 <= input_data; end
      endcase
    end
  end

endmodule

