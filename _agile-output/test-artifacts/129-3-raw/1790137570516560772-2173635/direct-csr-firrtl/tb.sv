`timescale 1ns/1ps
module tb;
reg clk=0, aresetn=0;
always #5 clk=~clk;
reg csr_req_valid=0, csr_write=0, csr_rsp_ready=1;
reg [15:0] csr_addr=0;
reg [31:0] csr_wdata=0;
reg [3:0] csr_wstrb=0;
wire csr_req_ready, csr_rsp_valid;
wire [31:0] csr_rdata;
wire [1:0] csr_error;
reg u_rx=1;
wire u_tx;
reg [31:0] g_pad_in=0;
wire [31:0] g_pad_out, g_pad_oe;
wire i_irq;
integer assertions=0;
integer transactions=0;
integer k;
integer seed_index;
integer random_index;
integer random_transactions=0;
reg [31:0] value;
reg [31:0] expected;
reg [31:0] prng;

Fr198Direct dut(
 .clk(clk), .aresetn(aresetn),
 .csr_req_valid(csr_req_valid), .csr_write(csr_write), .csr_addr(csr_addr),
 .csr_wdata(csr_wdata), .csr_wstrb(csr_wstrb), .csr_rsp_ready(csr_rsp_ready),
 .csr_req_ready(csr_req_ready), .csr_rsp_valid(csr_rsp_valid),
 .csr_rdata(csr_rdata), .csr_error(csr_error),
 .u_rx(u_rx), .u_tx(u_tx), .g_pad_in(g_pad_in),
 .g_pad_out(g_pad_out), .g_pad_oe(g_pad_oe), .i_irq(i_irq));

`define CHECK(c,m) begin if (!(c)) $fatal(1,"DIRECT %s",m); assertions=assertions+1; end

task automatic csr_write_op;
 input [15:0] a; input [31:0] d; input [3:0] s; input [1:0] e;
 integer guard;
 begin
  @(negedge clk); csr_addr=a; csr_wdata=d; csr_wstrb=s; csr_write=1; csr_req_valid=1;
  guard=0; while (!csr_req_ready && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"write request timeout")
  @(negedge clk); csr_req_valid=0;
  guard=0; while (!csr_rsp_valid && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"write response timeout")
  `CHECK(csr_error===e,"write response code")
  transactions=transactions+1;
  @(negedge clk);
 end
endtask

task automatic csr_read_op;
 input [15:0] a; input [1:0] e; output [31:0] d;
 integer guard;
 begin
  @(negedge clk); csr_addr=a; csr_wdata=0; csr_wstrb=0; csr_write=0; csr_req_valid=1;
  guard=0; while (!csr_req_ready && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"read request timeout")
  @(negedge clk); csr_req_valid=0;
  guard=0; while (!csr_rsp_valid && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"read response timeout")
  `CHECK(csr_error===e,"read response code")
  d=csr_rdata; transactions=transactions+1;
  @(negedge clk);
 end
endtask

task automatic clear_irq;
 input [4:0] bits;
 begin
  csr_write_op(16'h0300,{27'b0,bits},4'h1,2'b00);
  repeat(3) @(posedge clk); #1;
  csr_read_op(16'h0300,2'b00,value);
  `CHECK((value[4:0]&bits)==0,"IRQ clear or stale retrigger")
 end
endtask

task automatic wait_irq_bit;
 input integer bitno;
 integer guard;
 begin
  guard=0; while (!i_irq && guard<200) begin @(posedge clk); #1; guard=guard+1; end
  `CHECK(guard<200,"IRQ timeout")
  csr_read_op(16'h0300,2'b00,value);
  `CHECK(value[bitno]===1'b1,"wrong IRQ pending bit")
 end
endtask

task automatic uart_tx_expect;
 input [7:0] octet;
 integer bitno;
 begin
  wait(u_tx===1'b0); `CHECK(u_tx===1'b0,"UART TX start")
  for(bitno=0;bitno<8;bitno=bitno+1) begin
   repeat(4) @(posedge clk); #1;
   `CHECK(u_tx===octet[bitno],"UART TX data bit")
  end
  repeat(4) @(posedge clk); #1;
  `CHECK(u_tx===1'b1,"UART TX stop")
 end
endtask

task automatic uart_rx_send;
 input [7:0] octet; input good_stop;
 integer bitno;
 begin
  @(negedge clk); u_rx=0;
  repeat(4) @(posedge clk);
  for(bitno=0;bitno<8;bitno=bitno+1) begin
   @(negedge clk); u_rx=octet[bitno];
   repeat(4) @(posedge clk);
  end
  @(negedge clk); u_rx=good_stop;
  repeat(4) @(posedge clk);
  @(negedge clk); u_rx=1;
  repeat(8) @(posedge clk);
 end
endtask

function automatic [31:0] frozen_seed;
 input integer index;
 begin
  case(index)
   0: frozen_seed=32'h10203040; 1: frozen_seed=32'h89abcdef;
   2: frozen_seed=32'h13579bdf; 3: frozen_seed=32'h2468ace1;
   4: frozen_seed=32'hdeadbeef; 5: frozen_seed=32'hc001d00d;
   6: frozen_seed=32'h31415926; 7: frozen_seed=32'h27182818;
   8: frozen_seed=32'h0badf00d; 9: frozen_seed=32'h55aaaa55;
   10: frozen_seed=32'h76543210; 11: frozen_seed=32'hfedcba98;
   12: frozen_seed=32'h11223344; 13: frozen_seed=32'ha5a55a5a;
   14: frozen_seed=32'h7f4a7c15; 15: frozen_seed=32'h6d2b79f5;
   default: frozen_seed=32'h1;
  endcase
 end
endfunction

initial begin
 $dumpfile("direct.vcd"); $dumpvars(0,tb);
 repeat(3) @(posedge clk); aresetn=1; repeat(2) @(posedge clk); #1;
 `CHECK(g_pad_out===0 && g_pad_oe===0 && i_irq===0 && u_tx===1,"reset outputs")

 // Every byte-enable combination must update only selected GPIO OUT bytes.
 csr_write_op(16'h0100,32'hffff_ffff,4'hf,2'b00);
 expected=0;
 for(k=0;k<16;k=k+1) begin
  csr_write_op(16'h0104,32'h4433_2211+k,k[3:0],2'b00);
  if(k[0]) expected[7:0]=(8'h11+k);
  if(k[1]) expected[15:8]=8'h22;
  if(k[2]) expected[23:16]=8'h33;
  if(k[3]) expected[31:24]=8'h44;
  csr_read_op(16'h0104,2'b00,value);
  `CHECK(value===expected,"GPIO WSTRB merge")
 end
 `CHECK(g_pad_oe===32'hffff_ffff && g_pad_out===expected,"GPIO physical outputs")
 csr_write_op(16'h010c,32'h0000_0003,4'h1,2'b00);
 `CHECK(g_pad_out[1:0]===2'b11,"GPIO SET side effect")
 csr_write_op(16'h0110,32'h0000_0002,4'h1,2'b00);
 `CHECK(g_pad_out[1:0]===2'b01,"GPIO CLEAR side effect")

 // Window holes, alignment, access type, outside range and high aliases.
 csr_read_op(16'h0018,2'b10,value);
 csr_read_op(16'h0101,2'b10,value);
 csr_read_op(16'h000c,2'b10,value);
 csr_write_op(16'h0010,32'h1,4'h1,2'b10);
 csr_read_op(16'h0400,2'b11,value); `CHECK(value===0,"DECERR read data")
 csr_read_op(16'h8104,2'b11,value); `CHECK(value===0,"high address does not alias")

 // Hold a real response and require payload stability until consumption.
 csr_rsp_ready=0;
 @(negedge clk); csr_addr=16'h0104; csr_write=0; csr_req_valid=1;
 while(!csr_req_ready) @(negedge clk);
 @(negedge clk); csr_req_valid=0;
 while(!csr_rsp_valid) @(negedge clk);
 value=csr_rdata;
 repeat(4) begin @(negedge clk); `CHECK(csr_rsp_valid && csr_rdata===value && csr_error===0,"CSR response hold"); end
 csr_rsp_ready=1; @(negedge clk); transactions=transactions+1;

 // Enable all IRQ sources, then prove each raw source through public effects.
 csr_write_op(16'h0304,32'h1f,4'h1,2'b00);
 clear_irq(5'h1f);
 csr_write_op(16'h0208,32'd3,4'hf,2'b00);
 csr_write_op(16'h0204,32'd0,4'hf,2'b00);
 csr_write_op(16'h0200,32'd1,4'h1,2'b00);
 wait_irq_bit(0); clear_irq(5'h01);
 repeat(6) @(posedge clk); #1; `CHECK(!i_irq,"Timer sticky flag must not retrigger IRQ")
 // Periodic mode must produce a later distinct event after software clears.
 csr_write_op(16'h0208,32'd2,4'hf,2'b00);
 csr_write_op(16'h0204,32'd0,4'hf,2'b00);
 csr_write_op(16'h0200,32'd3,4'h1,2'b00);
 wait_irq_bit(0); csr_write_op(16'h0200,32'd0,4'h1,2'b00); clear_irq(5'h01);
 csr_write_op(16'h0204,32'd0,4'hf,2'b00);
 csr_write_op(16'h0200,32'd3,4'h1,2'b00);
 wait_irq_bit(0); csr_write_op(16'h0200,32'd0,4'h1,2'b00); clear_irq(5'h01);
 // COMPARE=0 matches through the 32-bit wrap from ffffffff to zero.
 csr_write_op(16'h0208,32'd0,4'hf,2'b00);
 csr_write_op(16'h0204,32'hffff_fffe,4'hf,2'b00);
 csr_write_op(16'h0200,32'd1,4'h1,2'b00);
 wait_irq_bit(0); clear_irq(5'h01);

 // GPIO input event requires input direction and the real two-stage synchronizer.
 csr_write_op(16'h0100,32'hffff_fe_ff,4'hf,2'b00);
 g_pad_in[8]=1; repeat(5) @(posedge clk); #1;
 wait_irq_bit(4); clear_irq(5'h10);
 g_pad_in[8]=0; repeat(5) @(posedge clk); g_pad_in[8]=1; repeat(5) @(posedge clk); #1;
 wait_irq_bit(4); clear_irq(5'h10);

 csr_write_op(16'h0004,32'd3,4'hf,2'b00);
 csr_write_op(16'h0000,32'd1,4'h1,2'b00);
 csr_write_op(16'h000c,32'h0000_00a5,4'h1,2'b00);
 uart_tx_expect(8'ha5);
 wait_irq_bit(2); clear_irq(5'h04);

 uart_rx_send(8'h3c,1'b1);
 wait_irq_bit(1);
 csr_read_op(16'h0010,2'b00,value); `CHECK(value[7:0]===8'h3c,"UART RX byte")
 clear_irq(5'h02);

 uart_rx_send(8'h55,1'b0);
 wait_irq_bit(3); clear_irq(5'h08);
 // Fill the four-entry RX FIFO, then a fifth valid frame must raise overflow.
 for(k=0;k<5;k=k+1) uart_rx_send(8'h80+k,1'b1);
 wait_irq_bit(3); clear_irq(5'h08); clear_irq(5'h02);

 // Reset cancels an accepted, blocked response and clears every leaf state.
 csr_rsp_ready=0;
 @(negedge clk); csr_addr=16'h0104; csr_write=0; csr_req_valid=1;
 while(!csr_req_ready) @(negedge clk);
 @(negedge clk); csr_req_valid=0;
 while(!csr_rsp_valid) @(negedge clk);
 aresetn=0; @(posedge clk); #1;
 `CHECK(!csr_rsp_valid && !i_irq && g_pad_out===0 && g_pad_oe===0,"reset cancels response and state")
 @(negedge clk); aresetn=1; csr_rsp_ready=1; repeat(3) @(posedge clk); #1;
 `CHECK(!csr_rsp_valid && !i_irq,"reset request is not replayed")
 csr_read_op(16'h0010,2'b10,value); // RX FIFO was flushed.
 csr_read_op(16'h0204,2'b00,value); `CHECK(value===0,"Timer reset")

 // Run the same frozen 16 x 1000 transaction budget on the bridge-free graph.
 $dumpoff;
 expected=0;
 csr_write_op(16'h0104,expected,4'hf,2'b00);
 for(seed_index=0;seed_index<16;seed_index=seed_index+1) begin
  prng=frozen_seed(seed_index);
  for(random_index=0;random_index<1000;random_index=random_index+1) begin
   prng=prng^(prng<<13); prng=prng^(prng>>17); prng=prng^(prng<<5);
   case(prng[1:0])
    2'd0: begin
     csr_write_op(16'h0104,prng,prng[7:4],2'b00);
     if(prng[4]) expected[7:0]=prng[7:0];
     if(prng[5]) expected[15:8]=prng[15:8];
     if(prng[6]) expected[23:16]=prng[23:16];
     if(prng[7]) expected[31:24]=prng[31:24];
    end
    2'd1: begin csr_read_op(16'h0104,2'b00,value); `CHECK(value===expected,"random GPIO scoreboard") end
    2'd2: begin csr_read_op(prng[2]?16'h0101:16'h0018,2'b10,value); `CHECK(value===0,"random SLVERR data") end
    2'd3: begin csr_read_op(prng[2]?16'h8104:16'h0400,2'b11,value); `CHECK(value===0,"random DECERR data") end
   endcase
   random_transactions=random_transactions+1;
  end
 end
 `CHECK(random_transactions==16000,"frozen random transaction budget")

 `CHECK(transactions>=75,"behavioral transaction count")
 $display("FR198 PASS topology=direct assertions=%0d transactions=%0d random_transactions=%0d",assertions,transactions,random_transactions);
 $finish;
end

initial begin #2000000; $fatal(1,"DIRECT timeout"); end
endmodule
