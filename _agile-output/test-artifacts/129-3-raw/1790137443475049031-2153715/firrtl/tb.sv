`timescale 1ns/1ps
module tb;
reg clk=0, aresetn=0;
always #5 clk=~clk;
reg [15:0] awaddr=0; reg [2:0] awprot=0; reg awvalid=0; wire awready;
reg [31:0] wdata=0; reg [3:0] wstrb=0; reg wvalid=0; wire wready;
wire bvalid; wire [1:0] bresp; reg bready=1;
reg [15:0] araddr=0; reg [2:0] arprot=0; reg arvalid=0; wire arready;
wire rvalid; wire [31:0] rdata; wire [1:0] rresp; reg rready=1;
reg u_rx=1; wire u_tx;
reg [31:0] g_pad_in=0; wire [31:0] g_pad_out,g_pad_oe;
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

Fr198Axi dut(
 .clk(clk),.aresetn(aresetn),
 .axi_s_axi_awaddr(awaddr),.axi_s_axi_awprot(awprot),.axi_s_axi_awvalid(awvalid),.axi_s_axi_awready(awready),
 .axi_s_axi_wdata(wdata),.axi_s_axi_wstrb(wstrb),.axi_s_axi_wvalid(wvalid),.axi_s_axi_wready(wready),
 .axi_s_axi_bready(bready),.axi_s_axi_bvalid(bvalid),.axi_s_axi_bresp(bresp),
 .axi_s_axi_araddr(araddr),.axi_s_axi_arprot(arprot),.axi_s_axi_arvalid(arvalid),.axi_s_axi_arready(arready),
 .axi_s_axi_rready(rready),.axi_s_axi_rvalid(rvalid),.axi_s_axi_rdata(rdata),.axi_s_axi_rresp(rresp),
 .u_rx(u_rx),.u_tx(u_tx),.g_pad_in(g_pad_in),.g_pad_out(g_pad_out),.g_pad_oe(g_pad_oe),.i_irq(i_irq));

`define CHECK(c,m) begin if (!(c)) $fatal(1,"AXI %s",m); assertions=assertions+1; end

task automatic send_aw;
 input [15:0] a;
 integer guard;
 begin
  guard=0; while(!awready && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"AW ready timeout")
  @(negedge clk); awaddr=a; awvalid=1;
  @(negedge clk); awvalid=0;
 end
endtask

task automatic send_w;
 input [31:0] d; input [3:0] s;
 integer guard;
 begin
  guard=0; while(!wready && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"W ready timeout")
  @(negedge clk); wdata=d; wstrb=s; wvalid=1;
  @(negedge clk); wvalid=0;
 end
endtask

task automatic axi_write_op;
 input [15:0] a; input [31:0] d; input [3:0] s; input [1:0] e;
 input integer order; input integer gap;
 integer guard;
 begin
  if(order==0) begin
   guard=0; while((!awready || !wready) && guard<30) begin @(negedge clk); guard=guard+1; end
   `CHECK(guard<30,"same-cycle AW/W ready timeout")
   @(negedge clk); awaddr=a; wdata=d; wstrb=s; awvalid=1; wvalid=1;
   @(negedge clk); awvalid=0; wvalid=0;
  end else if(order==1) begin
   send_aw(a); repeat(gap) @(posedge clk); send_w(d,s);
  end else begin
   send_w(d,s); repeat(gap) @(posedge clk); send_aw(a);
  end
  guard=0; while(!bvalid && guard<60) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<60,"B response timeout")
  `CHECK(bresp===e,"BRESP code")
  transactions=transactions+1;
  @(negedge clk);
 end
endtask

task automatic axi_read_op;
 input [15:0] a; input [1:0] e; output [31:0] d;
 integer guard;
 begin
  guard=0; while(!arready && guard<30) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<30,"AR ready timeout")
  @(negedge clk); araddr=a; arvalid=1;
  @(negedge clk); arvalid=0;
  guard=0; while(!rvalid && guard<60) begin @(negedge clk); guard=guard+1; end
  `CHECK(guard<60,"R response timeout")
  `CHECK(rresp===e,"RRESP code")
  d=rdata; transactions=transactions+1;
  @(negedge clk);
 end
endtask

task automatic clear_irq;
 input [4:0] bits;
 begin
  axi_write_op(16'h0300,{27'b0,bits},4'h1,2'b00,0,0);
  repeat(3) @(posedge clk); #1;
  axi_read_op(16'h0300,2'b00,value);
  `CHECK((value[4:0]&bits)==0,"IRQ clear or stale retrigger")
 end
endtask

task automatic wait_irq_bit;
 input integer bitno;
 integer guard;
 begin
  guard=0; while(!i_irq && guard<200) begin @(posedge clk); #1; guard=guard+1; end
  `CHECK(guard<200,"IRQ timeout")
  axi_read_op(16'h0300,2'b00,value);
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
  @(negedge clk); u_rx=0; repeat(4) @(posedge clk);
  for(bitno=0;bitno<8;bitno=bitno+1) begin
   @(negedge clk); u_rx=octet[bitno]; repeat(4) @(posedge clk);
  end
  @(negedge clk); u_rx=good_stop; repeat(4) @(posedge clk);
  @(negedge clk); u_rx=1; repeat(8) @(posedge clk);
 end
endtask

task automatic reset_one_cycle;
 begin
  @(negedge clk); aresetn=0; @(posedge clk); #1;
  @(negedge clk); aresetn=1; repeat(2) @(posedge clk); #1;
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
 $dumpfile("axi.vcd"); $dumpvars(0,tb);
 repeat(3) @(posedge clk); aresetn=1; repeat(2) @(posedge clk); #1;
 `CHECK(g_pad_out===0 && g_pad_oe===0 && i_irq===0 && u_tx===1,"reset outputs")

 // AW-first, W-first, same-cycle, and all legal byte-enable combinations.
 axi_write_op(16'h0100,32'hffff_ffff,4'hf,2'b00,1,7);
 expected=0;
 for(k=0;k<16;k=k+1) begin
  axi_write_op(16'h0104,32'h4433_2211+k,k[3:0],2'b00,k%3,(k%2)*7);
  if(k[0]) expected[7:0]=(8'h11+k);
  if(k[1]) expected[15:8]=8'h22;
  if(k[2]) expected[23:16]=8'h33;
  if(k[3]) expected[31:24]=8'h44;
  axi_read_op(16'h0104,2'b00,value);
  `CHECK(value===expected,"GPIO WSTRB merge")
 end
 `CHECK(g_pad_oe===32'hffff_ffff && g_pad_out===expected,"GPIO physical outputs")
 axi_write_op(16'h010c,32'h3,4'h1,2'b00,2,31);
 `CHECK(g_pad_out[1:0]===2'b11,"GPIO SET side effect")
 axi_write_op(16'h0110,32'h2,4'h1,2'b00,1,1);
 `CHECK(g_pad_out[1:0]===2'b01,"GPIO CLEAR side effect")

 // SLVERR classes and DECERR classes, including the prohibited high alias.
 axi_read_op(16'h0018,2'b10,value);
 axi_read_op(16'h0101,2'b10,value);
 axi_read_op(16'h000c,2'b10,value);
 axi_write_op(16'h0010,32'h1,4'h1,2'b10,0,0);
 axi_read_op(16'h0400,2'b11,value); `CHECK(value===0,"DECERR read data")
 axi_read_op(16'h8104,2'b11,value); `CHECK(value===0,"high address does not alias")

 // Independent B and R backpressure must hold payload and avoid replay.
 bready=0;
 send_aw(16'h0400); send_w(32'h55aa,4'hf);
 while(!bvalid) @(negedge clk);
 repeat(5) begin @(negedge clk); `CHECK(bvalid && bresp===2'b11,"B payload hold"); end
 bready=1; @(negedge clk); transactions=transactions+1;
 rready=0;
 send_aw(16'h0104); send_w(32'h1234_5678,4'hf);
 while(!bvalid) @(negedge clk); `CHECK(bresp===0,"setup write before R hold"); @(negedge clk); transactions=transactions+1;
 @(negedge clk); araddr=16'h0104; arvalid=1; @(negedge clk); arvalid=0;
 while(!rvalid) @(negedge clk); value=rdata;
 repeat(5) begin @(negedge clk); `CHECK(rvalid && rdata===value && rresp===0,"R payload hold"); end
 rready=1; @(negedge clk); transactions=transactions+1;

 // Simultaneous complete write and read are both retained and eventually answered.
 while(!awready || !wready || !arready) @(negedge clk);
 @(negedge clk); awaddr=16'h0104; wdata=32'h89ab_cdef; wstrb=4'hf; araddr=16'h0100; awvalid=1; wvalid=1; arvalid=1;
 @(negedge clk); awvalid=0; wvalid=0; arvalid=0;
 while(!bvalid) @(negedge clk); `CHECK(bresp===0,"concurrent write response"); @(negedge clk);
 while(!rvalid) @(negedge clk); `CHECK(rresp===0 && rdata===32'hffff_ffff,"concurrent read response"); @(negedge clk);
 transactions=transactions+2;

 axi_write_op(16'h0304,32'h1f,4'h1,2'b00,0,0);
 clear_irq(5'h1f);
 axi_write_op(16'h0208,32'd3,4'hf,2'b00,1,1);
 axi_write_op(16'h0204,32'd0,4'hf,2'b00,2,7);
 axi_write_op(16'h0200,32'd1,4'h1,2'b00,0,0);
 wait_irq_bit(0); clear_irq(5'h01);
 repeat(6) @(posedge clk); #1; `CHECK(!i_irq,"Timer sticky flag must not retrigger IRQ")
 axi_write_op(16'h0208,32'd2,4'hf,2'b00,0,0);
 axi_write_op(16'h0204,32'd0,4'hf,2'b00,1,1);
 axi_write_op(16'h0200,32'd3,4'h1,2'b00,2,7);
 wait_irq_bit(0); axi_write_op(16'h0200,32'd0,4'h1,2'b00,0,0); clear_irq(5'h01);
 axi_write_op(16'h0204,32'd0,4'hf,2'b00,0,0);
 axi_write_op(16'h0200,32'd3,4'h1,2'b00,1,1);
 wait_irq_bit(0); axi_write_op(16'h0200,32'd0,4'h1,2'b00,0,0); clear_irq(5'h01);
 axi_write_op(16'h0208,32'd0,4'hf,2'b00,0,0);
 axi_write_op(16'h0204,32'hffff_fffe,4'hf,2'b00,2,7);
 axi_write_op(16'h0200,32'd1,4'h1,2'b00,0,0);
 wait_irq_bit(0); clear_irq(5'h01);

 axi_write_op(16'h0100,32'hffff_fe_ff,4'hf,2'b00,0,0);
 g_pad_in[8]=1; repeat(5) @(posedge clk); #1;
 wait_irq_bit(4); clear_irq(5'h10);
 g_pad_in[8]=0; repeat(5) @(posedge clk); g_pad_in[8]=1; repeat(5) @(posedge clk); #1;
 wait_irq_bit(4); clear_irq(5'h10);

 axi_write_op(16'h0004,32'd3,4'hf,2'b00,1,7);
 axi_write_op(16'h0000,32'd1,4'h1,2'b00,2,1);
 axi_write_op(16'h000c,32'h5a,4'h1,2'b00,0,0);
 uart_tx_expect(8'h5a);
 wait_irq_bit(2); clear_irq(5'h04);
 uart_rx_send(8'hc3,1'b1);
 wait_irq_bit(1);
 axi_read_op(16'h0010,2'b00,value); `CHECK(value[7:0]===8'hc3,"UART RX byte")
 clear_irq(5'h02);
 uart_rx_send(8'h69,1'b0);
 wait_irq_bit(3); clear_irq(5'h08);
 for(k=0;k<5;k=k+1) uart_rx_send(8'h40+k,1'b1);
 wait_irq_bit(3); clear_irq(5'h08); clear_irq(5'h02);

 // Reset each independent capture/response position. No old operation may replay.
 while(!awready) @(negedge clk); @(negedge clk); awaddr=16'h0104; awvalid=1; @(negedge clk); awvalid=0;
 reset_one_cycle(); `CHECK(!bvalid && !rvalid,"reset AW-only")
 while(!wready) @(negedge clk); @(negedge clk); wdata=32'hdead_beef; wstrb=4'hf; wvalid=1; @(negedge clk); wvalid=0;
 reset_one_cycle(); `CHECK(!bvalid && !rvalid,"reset W-only")
 rready=0; while(!arready) @(negedge clk); @(negedge clk); araddr=16'h0104; arvalid=1; @(negedge clk); arvalid=0;
 reset_one_cycle(); rready=1; `CHECK(!rvalid,"reset captured AR")
 bready=0; send_aw(16'h0104); send_w(32'hfeed_face,4'hf); while(!bvalid) @(negedge clk);
 reset_one_cycle(); bready=1; `CHECK(!bvalid,"reset blocked B")
 rready=0; @(negedge clk); araddr=16'h0104; arvalid=1; @(negedge clk); arvalid=0; while(!rvalid) @(negedge clk);
 reset_one_cycle(); rready=1; `CHECK(!rvalid,"reset blocked R")
 `CHECK(!i_irq && g_pad_out===0 && g_pad_oe===0,"reset all leaf state")
 repeat(4) @(posedge clk); #1; `CHECK(!bvalid && !rvalid && !i_irq,"no reset replay")
 axi_read_op(16'h0010,2'b10,value); // RX FIFO was flushed.
 axi_read_op(16'h0204,2'b00,value); `CHECK(value===0,"Timer reset")

 // Frozen NFR14 budget: 16 seeds x 1000 completed, externally checked transactions.
 // Targeted vectors above retain the waveform; suppress random-stress VCD growth.
 $dumpoff;
 expected=0;
 axi_write_op(16'h0104,expected,4'hf,2'b00,0,0);
 for(seed_index=0;seed_index<16;seed_index=seed_index+1) begin
  prng=frozen_seed(seed_index);
  for(random_index=0;random_index<1000;random_index=random_index+1) begin
   prng=prng^(prng<<13); prng=prng^(prng>>17); prng=prng^(prng<<5);
   case(prng[1:0])
    2'd0: begin
     axi_write_op(16'h0104,prng,prng[7:4],2'b00,prng[9:8]%3,prng[11:10]);
     if(prng[4]) expected[7:0]=prng[7:0];
     if(prng[5]) expected[15:8]=prng[15:8];
     if(prng[6]) expected[23:16]=prng[23:16];
     if(prng[7]) expected[31:24]=prng[31:24];
    end
    2'd1: begin axi_read_op(16'h0104,2'b00,value); `CHECK(value===expected,"random GPIO scoreboard") end
    2'd2: begin axi_read_op(prng[2]?16'h0101:16'h0018,2'b10,value); `CHECK(value===0,"random SLVERR data") end
    2'd3: begin axi_read_op(prng[2]?16'h8104:16'h0400,2'b11,value); `CHECK(value===0,"random DECERR data") end
   endcase
   random_transactions=random_transactions+1;
  end
 end
 `CHECK(random_transactions==16000,"frozen random transaction budget")

 `CHECK(transactions>=80,"behavioral transaction count")
 $display("FR198 PASS topology=axi assertions=%0d transactions=%0d random_transactions=%0d",assertions,transactions,random_transactions);
 $finish;
end

initial begin #3000000; $fatal(1,"AXI timeout"); end
endmodule
