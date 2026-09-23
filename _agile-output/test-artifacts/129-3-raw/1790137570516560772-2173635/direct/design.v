module Fr198Bridge (
  input clk,
  input rst,
  input [15:0] s_axi_awaddr,
  input [2:0] s_axi_awprot,
  input s_axi_awvalid,
  input [31:0] s_axi_wdata,
  input [3:0] s_axi_wstrb,
  input s_axi_wvalid,
  input s_axi_bready,
  input [15:0] s_axi_araddr,
  input [2:0] s_axi_arprot,
  input s_axi_arvalid,
  input s_axi_rready,
  input csr_req_ready,
  input csr_rsp_valid,
  input [31:0] csr_rdata,
  input [1:0] csr_error,
  output s_axi_awready,
  output s_axi_wready,
  output s_axi_arready,
  output s_axi_bvalid,
  output [1:0] s_axi_bresp,
  output s_axi_rvalid,
  output [31:0] s_axi_rdata,
  output [1:0] s_axi_rresp,
  output csr_req_valid,
  output csr_write,
  output [15:0] csr_addr,
  output [31:0] csr_wdata,
  output [3:0] csr_wstrb,
  output csr_rsp_ready
);
  reg aw_full;
  reg w_full;
  reg ar_full;
  reg [15:0] aw_addr;
  reg [31:0] w_data;
  reg [3:0] w_strb;
  reg [15:0] ar_addr;
  reg offer;
  reg offer_write;
  reg exec;
  reg owner;
  reg prefer_write;
  reg b_full;
  reg r_full;
  reg [1:0] b_error;
  reg [1:0] r_error;
  reg [31:0] r_data;
  reg [31:0] offer_data;
  reg [3:0] offer_strb;
  wire one;
  wire zero;
  wire aw_take;
  wire w_take;
  wire ar_take;
  wire active;
  wire no_offer;
  wire no_exec;
  wire idle;
  wire b_space;
  wire r_space;
  wire write_complete;
  wire we;
  wire re;
  wire eligible;
  wire select;
  wire no_read;
  wire write_preferred;
  wire choose_write;
  wire commit;
  wire read_offer;
  wire write_commit;
  wire read_commit;
  wire response;
  wire write_response;
  wire read_owner;
  wire read_response;
  wire b_take;
  wire r_take;
  wire aw_captured;
  wire aw_next;
  wire w_captured;
  wire w_next;
  wire ar_captured;
  wire ar_next;
  wire offer_started;
  wire offer_next;
  wire exec_started;
  wire exec_next;
  wire b_consumed;
  wire b_next;
  wire r_consumed;
  wire r_next;
  assign one = 1;
  assign zero = 0;
  assign s_axi_awready = (aw_full ^ one);
  assign aw_take = (s_axi_awready & s_axi_awvalid);
  assign s_axi_wready = (w_full ^ one);
  assign w_take = (s_axi_wready & s_axi_wvalid);
  assign s_axi_arready = (ar_full ^ one);
  assign ar_take = (s_axi_arready & s_axi_arvalid);
  assign active = (rst ^ one);
  assign no_offer = (offer ^ one);
  assign no_exec = (exec ^ one);
  assign idle = (no_offer & no_exec);
  assign b_space = (b_full ^ one);
  assign r_space = (r_full ^ one);
  assign write_complete = (aw_full & w_full);
  assign we = (write_complete & b_space);
  assign re = (ar_full & r_space);
  assign eligible = (we | re);
  assign select = (idle & eligible);
  assign no_read = (re ^ one);
  assign write_preferred = (prefer_write | no_read);
  assign choose_write = (we & write_preferred);
  assign csr_req_valid = (offer & active);
  assign commit = (csr_req_valid & csr_req_ready);
  assign read_offer = (offer_write ^ one);
  assign write_commit = (commit & offer_write);
  assign read_commit = (commit & read_offer);
  assign csr_rsp_ready = (exec & active);
  assign response = (csr_rsp_ready & csr_rsp_valid);
  assign write_response = (response & owner);
  assign read_owner = (owner ^ one);
  assign read_response = (response & read_owner);
  assign b_take = (b_full & s_axi_bready);
  assign r_take = (r_full & s_axi_rready);
  assign aw_captured = (aw_take ? one : aw_full);
  assign aw_next = (write_commit ? zero : aw_captured);
  assign w_captured = (w_take ? one : w_full);
  assign w_next = (write_commit ? zero : w_captured);
  assign ar_captured = (ar_take ? one : ar_full);
  assign ar_next = (read_commit ? zero : ar_captured);
  assign offer_started = (select ? one : offer);
  assign offer_next = (commit ? zero : offer_started);
  assign exec_started = (commit ? one : exec);
  assign exec_next = (response ? zero : exec_started);
  assign b_consumed = (b_take ? zero : b_full);
  assign b_next = (write_response ? one : b_consumed);
  assign r_consumed = (r_take ? zero : r_full);
  assign r_next = (read_response ? one : r_consumed);
  assign csr_addr = (offer_write ? aw_addr : ar_addr);
  assign s_axi_bvalid = b_full;
  assign s_axi_rvalid = r_full;
  assign s_axi_bresp = b_error;
  assign s_axi_rresp = r_error;
  assign s_axi_rdata = r_data;
  assign csr_write = offer_write;
  assign csr_wdata = offer_data;
  assign csr_wstrb = offer_strb;
  always @(posedge clk) begin
    if (rst) begin
      aw_full <= 0;
      w_full <= 0;
      ar_full <= 0;
      offer <= 0;
      exec <= 0;
      b_full <= 0;
      r_full <= 0;
      aw_addr <= 0;
      w_data <= 0;
      w_strb <= 0;
      ar_addr <= 0;
      offer_write <= 0;
      offer_data <= 0;
      offer_strb <= 0;
      owner <= 0;
      prefer_write <= 0;
      b_error <= 0;
      r_error <= 0;
      r_data <= 0;
    end else begin
      aw_full <= aw_next;
      w_full <= w_next;
      ar_full <= ar_next;
      offer <= offer_next;
      exec <= exec_next;
      b_full <= b_next;
      r_full <= r_next;
      aw_addr <= (aw_take ? s_axi_awaddr : aw_addr);
      w_data <= (w_take ? s_axi_wdata : w_data);
      w_strb <= (w_take ? s_axi_wstrb : w_strb);
      ar_addr <= (ar_take ? s_axi_araddr : ar_addr);
      offer_write <= (select ? choose_write : offer_write);
      offer_data <= (select ? w_data : offer_data);
      offer_strb <= (select ? w_strb : offer_strb);
      owner <= (commit ? offer_write : owner);
      prefer_write <= (commit ? read_offer : prefer_write);
      b_error <= (write_response ? csr_error : b_error);
      r_error <= (read_response ? csr_error : r_error);
      r_data <= (read_response ? csr_rdata : r_data);
    end
  end
endmodule

module Fr198Decoder (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  input uart_req_ready,
  input uart_rsp_valid,
  input [31:0] uart_rdata,
  input [1:0] uart_error,
  output uart_req_valid,
  output uart_write,
  output [15:0] uart_addr,
  output [31:0] uart_wdata,
  output [3:0] uart_wstrb,
  output uart_rsp_ready,
  input gpio_req_ready,
  input gpio_rsp_valid,
  input [31:0] gpio_rdata,
  input [1:0] gpio_error,
  output gpio_req_valid,
  output gpio_write,
  output [15:0] gpio_addr,
  output [31:0] gpio_wdata,
  output [3:0] gpio_wstrb,
  output gpio_rsp_ready,
  input timer_req_ready,
  input timer_rsp_valid,
  input [31:0] timer_rdata,
  input [1:0] timer_error,
  output timer_req_valid,
  output timer_write,
  output [15:0] timer_addr,
  output [31:0] timer_wdata,
  output [3:0] timer_wstrb,
  output timer_rsp_ready,
  input irq_req_ready,
  input irq_rsp_valid,
  input [31:0] irq_rdata,
  input [1:0] irq_error,
  output irq_req_valid,
  output irq_write,
  output [15:0] irq_addr,
  output [31:0] irq_wdata,
  output [3:0] irq_wstrb,
  output irq_rsp_ready
);
  reg busy;
  reg miss;
  reg [1:0] owner;
  wire one;
  wire zero;
  wire [31:0] zero32;
  wire [1:0] zero2;
  wire [1:0] decerr;
  wire [7:0] page;
  wire [1:0] select_owner;
  wire active;
  wire idle;
  wire available;
  wire offering;
  wire commit;
  wire consume;
  wire busy_started;
  wire busy_next;
  wire not_miss;
  wire leaf_owned;
  wire ready_active;
  wire miss_valid;
  wire hit_0;
  wire ready_0;
  wire valid_0;
  wire [31:0] data_0;
  wire [1:0] error_0;
  wire hit_1;
  wire ready_1;
  wire valid_1;
  wire [31:0] data_1;
  wire [1:0] error_1;
  wire hit_2;
  wire ready_2;
  wire valid_2;
  wire [31:0] data_2;
  wire [1:0] error_2;
  wire hit_3;
  wire ready_3;
  wire valid_3;
  wire [31:0] data_3;
  wire [1:0] error_3;
  wire hit_4;
  wire ready_4;
  wire valid_4;
  wire [31:0] data_4;
  wire [1:0] error_4;
  wire no_hit;
  wire [7:0] uart_page;
  wire [15:0] uart_base;
  wire [1:0] uart_id;
  wire uart_hit;
  wire uart_owner_eq;
  wire uart_owned;
  wire [7:0] gpio_page;
  wire [15:0] gpio_base;
  wire [1:0] gpio_id;
  wire gpio_hit;
  wire gpio_owner_eq;
  wire gpio_owned;
  wire [7:0] timer_page;
  wire [15:0] timer_base;
  wire [1:0] timer_id;
  wire timer_hit;
  wire timer_owner_eq;
  wire timer_owned;
  wire [7:0] irq_page;
  wire [15:0] irq_base;
  wire [1:0] irq_id;
  wire irq_hit;
  wire irq_owner_eq;
  wire irq_owned;
  assign one = 1;
  assign zero = 0;
  assign zero32 = 0;
  assign zero2 = 0;
  assign decerr = 3;
  assign page = addr[15:8];
  assign select_owner = addr[9:8];
  assign active = (rst ^ one);
  assign idle = (busy ^ one);
  assign available = (idle & active);
  assign offering = (available & req_valid);
  assign not_miss = (miss ^ one);
  assign leaf_owned = (busy & not_miss);
  assign ready_active = (rsp_ready & active);
  assign miss_valid = (busy & miss);
  assign hit_0 = 0;
  assign ready_0 = 1;
  assign valid_0 = miss_valid;
  assign data_0 = zero32;
  assign error_0 = (miss_valid ? decerr : zero2);
  assign uart_page = 0;
  assign uart_base = 0;
  assign uart_id = 0;
  assign uart_hit = (page == uart_page);
  assign hit_1 = (hit_0 | uart_hit);
  assign ready_1 = (uart_hit ? uart_req_ready : ready_0);
  assign uart_req_valid = (offering & uart_hit);
  assign uart_addr = addr - uart_base;
  assign uart_write = write;
  assign uart_wdata = wdata;
  assign uart_wstrb = wstrb;
  assign uart_owner_eq = (owner == uart_id);
  assign uart_owned = (leaf_owned & uart_owner_eq);
  assign uart_rsp_ready = (uart_owned & ready_active);
  assign valid_1 = (uart_owned ? uart_rsp_valid : valid_0);
  assign data_1 = (uart_owned ? uart_rdata : data_0);
  assign error_1 = (uart_owned ? uart_error : error_0);
  assign gpio_page = 1;
  assign gpio_base = 256;
  assign gpio_id = 1;
  assign gpio_hit = (page == gpio_page);
  assign hit_2 = (hit_1 | gpio_hit);
  assign ready_2 = (gpio_hit ? gpio_req_ready : ready_1);
  assign gpio_req_valid = (offering & gpio_hit);
  assign gpio_addr = addr - gpio_base;
  assign gpio_write = write;
  assign gpio_wdata = wdata;
  assign gpio_wstrb = wstrb;
  assign gpio_owner_eq = (owner == gpio_id);
  assign gpio_owned = (leaf_owned & gpio_owner_eq);
  assign gpio_rsp_ready = (gpio_owned & ready_active);
  assign valid_2 = (gpio_owned ? gpio_rsp_valid : valid_1);
  assign data_2 = (gpio_owned ? gpio_rdata : data_1);
  assign error_2 = (gpio_owned ? gpio_error : error_1);
  assign timer_page = 2;
  assign timer_base = 512;
  assign timer_id = 2;
  assign timer_hit = (page == timer_page);
  assign hit_3 = (hit_2 | timer_hit);
  assign ready_3 = (timer_hit ? timer_req_ready : ready_2);
  assign timer_req_valid = (offering & timer_hit);
  assign timer_addr = addr - timer_base;
  assign timer_write = write;
  assign timer_wdata = wdata;
  assign timer_wstrb = wstrb;
  assign timer_owner_eq = (owner == timer_id);
  assign timer_owned = (leaf_owned & timer_owner_eq);
  assign timer_rsp_ready = (timer_owned & ready_active);
  assign valid_3 = (timer_owned ? timer_rsp_valid : valid_2);
  assign data_3 = (timer_owned ? timer_rdata : data_2);
  assign error_3 = (timer_owned ? timer_error : error_2);
  assign irq_page = 3;
  assign irq_base = 768;
  assign irq_id = 3;
  assign irq_hit = (page == irq_page);
  assign hit_4 = (hit_3 | irq_hit);
  assign ready_4 = (irq_hit ? irq_req_ready : ready_3);
  assign irq_req_valid = (offering & irq_hit);
  assign irq_addr = addr - irq_base;
  assign irq_write = write;
  assign irq_wdata = wdata;
  assign irq_wstrb = wstrb;
  assign irq_owner_eq = (owner == irq_id);
  assign irq_owned = (leaf_owned & irq_owner_eq);
  assign irq_rsp_ready = (irq_owned & ready_active);
  assign valid_4 = (irq_owned ? irq_rsp_valid : valid_3);
  assign data_4 = (irq_owned ? irq_rdata : data_3);
  assign error_4 = (irq_owned ? irq_error : error_3);
  assign no_hit = (hit_4 ^ one);
  assign req_ready = (available & ready_4);
  assign rsp_valid = valid_4;
  assign rdata = data_4;
  assign error = error_4;
  assign commit = (req_valid & req_ready);
  assign consume = (rsp_valid & ready_active);
  assign busy_started = (busy | commit);
  assign busy_next = (consume ? zero : busy_started);
  always @(posedge clk) begin
    if (rst) begin
      busy <= 0;
      miss <= 0;
      owner <= 0;
    end else begin
      busy <= busy_next;
      miss <= (commit ? no_hit : miss);
      owner <= (commit ? select_owner : owner);
    end
  end
endmodule

module BitloomUartCsrRegisters (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  output ctrl_read_commit,
  output ctrl_write_commit,
  output [31:0] ctrl_value,
  output [31:0] ctrl_candidate,
  output [31:0] ctrl_write_mask,
  input ctrl_write_reject,
  output baud_div_read_commit,
  output baud_div_write_commit,
  output [31:0] baud_div_value,
  output [31:0] baud_div_candidate,
  output [31:0] baud_div_write_mask,
  input baud_div_write_reject,
  output status_read_commit,
  output status_write_commit,
  input [31:0] status_value,
  output tx_data_read_commit,
  output tx_data_write_commit,
  output [31:0] tx_data_candidate,
  output [31:0] tx_data_write_mask,
  input tx_data_write_reject,
  output rx_data_read_commit,
  output rx_data_write_commit,
  input [31:0] rx_data_value,
  input rx_data_read_reject,
  output EVENT_read_commit,
  output EVENT_write_commit,
  output [31:0] EVENT_value,
  output [31:0] EVENT_candidate,
  output [31:0] EVENT_write_mask,
  input [31:0] event_bits
);
  reg _csr_pending;
  reg [31:0] _csr_data;
  reg [1:0] _csr_error;
  reg [31:0] _csr_value0;
  reg [31:0] _csr_value1;
  reg [31:0] _csr_value5;
  wire _csr_n0;
  wire _csr_n1;
  wire [31:0] _csr_n2;
  wire [1:0] _csr_n3;
  wire [1:0] _csr_n4;
  wire _csr_n5;
  wire _csr_n6;
  wire _csr_n7;
  wire _csr_n8;
  wire _csr_n9;
  wire _csr_n10;
  wire _csr_n11;
  wire _csr_n12;
  wire _csr_n13;
  wire _csr_n14;
  wire _csr_n15;
  wire [31:0] _csr_n16;
  wire [31:0] _csr_n17;
  wire [31:0] _csr_n18;
  wire _csr_n19;
  wire [31:0] _csr_n20;
  wire [31:0] _csr_n21;
  wire [31:0] _csr_n22;
  wire _csr_n23;
  wire [31:0] _csr_n24;
  wire [31:0] _csr_n25;
  wire [31:0] _csr_n26;
  wire _csr_n27;
  wire [31:0] _csr_n28;
  wire [31:0] _csr_n29;
  wire [31:0] _csr_n30;
  wire [31:0] _csr_n31;
  wire [31:0] _csr_n32;
  wire [15:0] _csr_n33;
  wire _csr_n34;
  wire _csr_n35;
  wire _csr_n36;
  wire _csr_n37;
  wire [31:0] _csr_n38;
  wire _csr_n39;
  wire _csr_n40;
  wire _csr_n41;
  wire [31:0] _csr_n42;
  wire [31:0] _csr_n43;
  wire [31:0] _csr_n44;
  wire [31:0] _csr_n45;
  wire [31:0] _csr_n46;
  wire _csr_n47;
  wire _csr_n48;
  wire _csr_n49;
  wire _csr_n50;
  wire _csr_n51;
  wire _csr_n52;
  wire _csr_n53;
  wire [1:0] _csr_n54;
  wire [1:0] _csr_n55;
  wire [31:0] _csr_n56;
  wire [31:0] _csr_n57;
  wire [31:0] _csr_n58;
  wire [31:0] _csr_n59;
  wire [31:0] _csr_n60;
  wire [15:0] _csr_n61;
  wire _csr_n62;
  wire _csr_n63;
  wire _csr_n64;
  wire _csr_n65;
  wire [31:0] _csr_n66;
  wire _csr_n67;
  wire _csr_n68;
  wire _csr_n69;
  wire [31:0] _csr_n70;
  wire [31:0] _csr_n71;
  wire [31:0] _csr_n72;
  wire [31:0] _csr_n73;
  wire [31:0] _csr_n74;
  wire _csr_n75;
  wire _csr_n76;
  wire _csr_n77;
  wire _csr_n78;
  wire _csr_n79;
  wire _csr_n80;
  wire _csr_n81;
  wire [1:0] _csr_n82;
  wire [1:0] _csr_n83;
  wire [31:0] _csr_n84;
  wire [31:0] _csr_n85;
  wire [31:0] _csr_n86;
  wire [31:0] _csr_n87;
  wire [31:0] _csr_n88;
  wire [15:0] _csr_n89;
  wire _csr_n90;
  wire _csr_n91;
  wire _csr_n92;
  wire _csr_n93;
  wire _csr_n94;
  wire [1:0] _csr_n95;
  wire [1:0] _csr_n96;
  wire [31:0] _csr_n97;
  wire [31:0] _csr_n98;
  wire [31:0] _csr_n99;
  wire [15:0] _csr_n100;
  wire _csr_n101;
  wire _csr_n102;
  wire _csr_n103;
  wire _csr_n104;
  wire [31:0] _csr_n105;
  wire _csr_n106;
  wire _csr_n107;
  wire _csr_n108;
  wire [31:0] _csr_n109;
  wire _csr_n110;
  wire _csr_n111;
  wire _csr_n112;
  wire _csr_n113;
  wire _csr_n114;
  wire _csr_n115;
  wire _csr_n116;
  wire [1:0] _csr_n117;
  wire [1:0] _csr_n118;
  wire [31:0] _csr_n119;
  wire [31:0] _csr_n120;
  wire [31:0] _csr_n121;
  wire [31:0] _csr_n122;
  wire [15:0] _csr_n123;
  wire _csr_n124;
  wire _csr_n125;
  wire _csr_n126;
  wire _csr_n127;
  wire _csr_n128;
  wire _csr_n129;
  wire _csr_n130;
  wire [1:0] _csr_n131;
  wire [1:0] _csr_n132;
  wire [31:0] _csr_n133;
  wire [31:0] _csr_n134;
  wire [31:0] _csr_n135;
  wire [31:0] _csr_n136;
  wire [15:0] _csr_n137;
  wire _csr_n138;
  wire _csr_n139;
  wire _csr_n140;
  wire _csr_n141;
  wire [31:0] _csr_n142;
  wire _csr_n143;
  wire _csr_n144;
  wire _csr_n145;
  wire [31:0] _csr_n146;
  wire _csr_n147;
  wire _csr_n148;
  wire _csr_n149;
  wire _csr_n150;
  wire [1:0] _csr_n151;
  wire [1:0] _csr_n152;
  wire [31:0] _csr_n153;
  wire [31:0] _csr_n154;
  wire [31:0] _csr_n155;
  wire [31:0] _csr_n156;
  wire [31:0] _csr_n157;
  wire [31:0] _csr_n158;
  wire [31:0] _csr_n159;
  wire [31:0] _csr_n160;
  wire [31:0] _csr_n161;
  wire [1:0] _csr_n162;
  assign _csr_n0 = 0;
  assign _csr_n1 = 1;
  assign _csr_n2 = 0;
  assign _csr_n3 = 0;
  assign _csr_n4 = 2;
  assign _csr_n5 = 1;
  assign _csr_n6 = (_csr_pending ^ _csr_n5);
  assign _csr_n7 = 1;
  assign _csr_n8 = (rst ^ _csr_n7);
  assign _csr_n9 = (_csr_n6 & _csr_n8);
  assign req_ready = _csr_n9;
  assign _csr_n10 = (req_valid & _csr_n9);
  assign _csr_n11 = 1;
  assign _csr_n12 = (write ^ _csr_n11);
  assign _csr_n13 = (rsp_ready ? _csr_n0 : _csr_pending);
  assign _csr_n14 = (_csr_n10 ? _csr_n1 : _csr_n13);
  assign rsp_valid = _csr_pending;
  assign rdata = _csr_data;
  assign error = _csr_error;
  assign _csr_n15 = wstrb[0:0];
  assign _csr_n16 = 255;
  assign _csr_n17 = (_csr_n15 ? _csr_n16 : _csr_n2);
  assign _csr_n18 = (_csr_n2 | _csr_n17);
  assign _csr_n19 = wstrb[1:1];
  assign _csr_n20 = 65280;
  assign _csr_n21 = (_csr_n19 ? _csr_n20 : _csr_n2);
  assign _csr_n22 = (_csr_n18 | _csr_n21);
  assign _csr_n23 = wstrb[2:2];
  assign _csr_n24 = 16711680;
  assign _csr_n25 = (_csr_n23 ? _csr_n24 : _csr_n2);
  assign _csr_n26 = (_csr_n22 | _csr_n25);
  assign _csr_n27 = wstrb[3:3];
  assign _csr_n28 = 4278190080;
  assign _csr_n29 = (_csr_n27 ? _csr_n28 : _csr_n2);
  assign _csr_n30 = (_csr_n26 | _csr_n29);
  assign _csr_n31 = 1;
  assign _csr_n32 = (_csr_value0 & _csr_n31);
  assign ctrl_value = _csr_n32;
  assign _csr_n33 = 0;
  assign _csr_n34 = (addr == _csr_n33);
  assign _csr_n35 = (_csr_n10 & _csr_n34);
  assign _csr_n36 = (_csr_n12 & _csr_n1);
  assign _csr_n37 = (_csr_n35 & _csr_n36);
  assign ctrl_read_commit = _csr_n37;
  assign _csr_n38 = (_csr_n30 & _csr_n31);
  assign _csr_n39 = (_csr_n38 == _csr_n2);
  assign _csr_n40 = 1;
  assign _csr_n41 = (_csr_n39 ^ _csr_n40);
  assign _csr_n42 = (wdata & _csr_n38);
  assign _csr_n43 = 4294967295;
  assign _csr_n44 = (_csr_n38 ^ _csr_n43);
  assign _csr_n45 = (_csr_n32 & _csr_n44);
  assign _csr_n46 = (_csr_n45 | _csr_n42);
  assign ctrl_write_mask = _csr_n38;
  assign ctrl_candidate = _csr_n46;
  assign _csr_n47 = 1;
  assign _csr_n48 = (ctrl_write_reject ^ _csr_n47);
  assign _csr_n49 = (_csr_n39 | _csr_n48);
  assign _csr_n50 = (write & _csr_n49);
  assign _csr_n51 = (_csr_n50 & _csr_n41);
  assign _csr_n52 = (_csr_n35 & _csr_n51);
  assign ctrl_write_commit = _csr_n52;
  assign _csr_n53 = (_csr_n36 | _csr_n50);
  assign _csr_n54 = (_csr_n53 ? _csr_n3 : _csr_n4);
  assign _csr_n55 = (_csr_n34 ? _csr_n54 : _csr_n4);
  assign _csr_n56 = (_csr_n36 ? _csr_n32 : _csr_n2);
  assign _csr_n57 = (_csr_n34 ? _csr_n56 : _csr_n2);
  assign _csr_n58 = (_csr_n52 ? _csr_n46 : _csr_n32);
  assign _csr_n59 = 4294967295;
  assign _csr_n60 = (_csr_value1 & _csr_n59);
  assign baud_div_value = _csr_n60;
  assign _csr_n61 = 4;
  assign _csr_n62 = (addr == _csr_n61);
  assign _csr_n63 = (_csr_n10 & _csr_n62);
  assign _csr_n64 = (_csr_n12 & _csr_n1);
  assign _csr_n65 = (_csr_n63 & _csr_n64);
  assign baud_div_read_commit = _csr_n65;
  assign _csr_n66 = (_csr_n30 & _csr_n59);
  assign _csr_n67 = (_csr_n66 == _csr_n2);
  assign _csr_n68 = 1;
  assign _csr_n69 = (_csr_n67 ^ _csr_n68);
  assign _csr_n70 = (wdata & _csr_n66);
  assign _csr_n71 = 4294967295;
  assign _csr_n72 = (_csr_n66 ^ _csr_n71);
  assign _csr_n73 = (_csr_n60 & _csr_n72);
  assign _csr_n74 = (_csr_n73 | _csr_n70);
  assign baud_div_write_mask = _csr_n66;
  assign baud_div_candidate = _csr_n74;
  assign _csr_n75 = 1;
  assign _csr_n76 = (baud_div_write_reject ^ _csr_n75);
  assign _csr_n77 = (_csr_n67 | _csr_n76);
  assign _csr_n78 = (write & _csr_n77);
  assign _csr_n79 = (_csr_n78 & _csr_n69);
  assign _csr_n80 = (_csr_n63 & _csr_n79);
  assign baud_div_write_commit = _csr_n80;
  assign _csr_n81 = (_csr_n64 | _csr_n78);
  assign _csr_n82 = (_csr_n81 ? _csr_n3 : _csr_n4);
  assign _csr_n83 = (_csr_n62 ? _csr_n82 : _csr_n55);
  assign _csr_n84 = (_csr_n64 ? _csr_n60 : _csr_n2);
  assign _csr_n85 = (_csr_n62 ? _csr_n84 : _csr_n57);
  assign _csr_n86 = (_csr_n80 ? _csr_n74 : _csr_n60);
  assign _csr_n87 = 15;
  assign _csr_n88 = (status_value & _csr_n87);
  assign _csr_n89 = 8;
  assign _csr_n90 = (addr == _csr_n89);
  assign _csr_n91 = (_csr_n10 & _csr_n90);
  assign _csr_n92 = (_csr_n12 & _csr_n1);
  assign _csr_n93 = (_csr_n91 & _csr_n92);
  assign status_read_commit = _csr_n93;
  assign status_write_commit = _csr_n0;
  assign _csr_n94 = (_csr_n92 | _csr_n0);
  assign _csr_n95 = (_csr_n94 ? _csr_n3 : _csr_n4);
  assign _csr_n96 = (_csr_n90 ? _csr_n95 : _csr_n83);
  assign _csr_n97 = (_csr_n92 ? _csr_n88 : _csr_n2);
  assign _csr_n98 = (_csr_n90 ? _csr_n97 : _csr_n85);
  assign _csr_n99 = 255;
  assign _csr_n100 = 12;
  assign _csr_n101 = (addr == _csr_n100);
  assign _csr_n102 = (_csr_n10 & _csr_n101);
  assign _csr_n103 = (_csr_n12 & _csr_n0);
  assign _csr_n104 = (_csr_n102 & _csr_n103);
  assign tx_data_read_commit = _csr_n104;
  assign _csr_n105 = (_csr_n30 & _csr_n99);
  assign _csr_n106 = (_csr_n105 == _csr_n2);
  assign _csr_n107 = 1;
  assign _csr_n108 = (_csr_n106 ^ _csr_n107);
  assign _csr_n109 = (wdata & _csr_n105);
  assign tx_data_write_mask = _csr_n105;
  assign tx_data_candidate = _csr_n109;
  assign _csr_n110 = 1;
  assign _csr_n111 = (tx_data_write_reject ^ _csr_n110);
  assign _csr_n112 = (_csr_n106 | _csr_n111);
  assign _csr_n113 = (write & _csr_n112);
  assign _csr_n114 = (_csr_n113 & _csr_n108);
  assign _csr_n115 = (_csr_n102 & _csr_n114);
  assign tx_data_write_commit = _csr_n115;
  assign _csr_n116 = (_csr_n103 | _csr_n113);
  assign _csr_n117 = (_csr_n116 ? _csr_n3 : _csr_n4);
  assign _csr_n118 = (_csr_n101 ? _csr_n117 : _csr_n96);
  assign _csr_n119 = (_csr_n103 ? _csr_n2 : _csr_n2);
  assign _csr_n120 = (_csr_n101 ? _csr_n119 : _csr_n98);
  assign _csr_n121 = 255;
  assign _csr_n122 = (rx_data_value & _csr_n121);
  assign _csr_n123 = 16;
  assign _csr_n124 = (addr == _csr_n123);
  assign _csr_n125 = (_csr_n10 & _csr_n124);
  assign _csr_n126 = 1;
  assign _csr_n127 = (rx_data_read_reject ^ _csr_n126);
  assign _csr_n128 = (_csr_n12 & _csr_n127);
  assign _csr_n129 = (_csr_n125 & _csr_n128);
  assign rx_data_read_commit = _csr_n129;
  assign rx_data_write_commit = _csr_n0;
  assign _csr_n130 = (_csr_n128 | _csr_n0);
  assign _csr_n131 = (_csr_n130 ? _csr_n3 : _csr_n4);
  assign _csr_n132 = (_csr_n124 ? _csr_n131 : _csr_n118);
  assign _csr_n133 = (_csr_n128 ? _csr_n122 : _csr_n2);
  assign _csr_n134 = (_csr_n124 ? _csr_n133 : _csr_n120);
  assign _csr_n135 = 15;
  assign _csr_n136 = (_csr_value5 & _csr_n135);
  assign EVENT_value = _csr_n136;
  assign _csr_n137 = 20;
  assign _csr_n138 = (addr == _csr_n137);
  assign _csr_n139 = (_csr_n10 & _csr_n138);
  assign _csr_n140 = (_csr_n12 & _csr_n1);
  assign _csr_n141 = (_csr_n139 & _csr_n140);
  assign EVENT_read_commit = _csr_n141;
  assign _csr_n142 = (_csr_n30 & _csr_n135);
  assign _csr_n143 = (_csr_n142 == _csr_n2);
  assign _csr_n144 = 1;
  assign _csr_n145 = (_csr_n143 ^ _csr_n144);
  assign _csr_n146 = (wdata & _csr_n142);
  assign EVENT_write_mask = _csr_n142;
  assign EVENT_candidate = _csr_n146;
  assign _csr_n147 = (write & _csr_n1);
  assign _csr_n148 = (_csr_n147 & _csr_n145);
  assign _csr_n149 = (_csr_n139 & _csr_n148);
  assign EVENT_write_commit = _csr_n149;
  assign _csr_n150 = (_csr_n140 | _csr_n147);
  assign _csr_n151 = (_csr_n150 ? _csr_n3 : _csr_n4);
  assign _csr_n152 = (_csr_n138 ? _csr_n151 : _csr_n132);
  assign _csr_n153 = (_csr_n140 ? _csr_n136 : _csr_n2);
  assign _csr_n154 = (_csr_n138 ? _csr_n153 : _csr_n134);
  assign _csr_n155 = (_csr_n149 ? _csr_n146 : _csr_n2);
  assign _csr_n156 = 4294967295;
  assign _csr_n157 = (_csr_n155 ^ _csr_n156);
  assign _csr_n158 = (_csr_n136 & _csr_n157);
  assign _csr_n159 = (event_bits & _csr_n135);
  assign _csr_n160 = (_csr_n158 | _csr_n159);
  assign _csr_n161 = (_csr_n10 ? _csr_n154 : _csr_data);
  assign _csr_n162 = (_csr_n10 ? _csr_n152 : _csr_error);
  always @(posedge clk) begin
    if (rst) begin
      _csr_pending <= 0;
      _csr_data <= 0;
      _csr_error <= 0;
      _csr_value0 <= 0;
      _csr_value1 <= 0;
      _csr_value5 <= 0;
    end else begin
      _csr_pending <= _csr_n14;
      _csr_data <= _csr_n161;
      _csr_error <= _csr_n162;
      _csr_value0 <= _csr_n58;
      _csr_value1 <= _csr_n86;
      _csr_value5 <= _csr_n160;
    end
  end
endmodule

module BitloomUartCsrFifo8x4 (
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
  reg [2:0] count;
  wire [2:0] zero;
  wire [2:0] one;
  wire [2:0] capacity;
  wire [2:0] decremented;
  wire [2:0] after_pop;
  wire [2:0] incremented;
  wire [2:0] normal_count;
  wire [2:0] next_count;
  wire true_bit;
  wire empty;
  wire full;
  wire push;
  wire pop;
  reg [7:0] slot_0;
  wire [2:0] index_0;
  wire at_0;
  wire write_0;
  wire [7:0] shift_0;
  wire [7:0] next_0;
  reg [7:0] slot_1;
  wire [2:0] index_1;
  wire at_1;
  wire write_1;
  wire [7:0] shift_1;
  wire [7:0] next_1;
  reg [7:0] slot_2;
  wire [2:0] index_2;
  wire at_2;
  wire write_2;
  wire [7:0] shift_2;
  wire [7:0] next_2;
  reg [7:0] slot_3;
  wire [2:0] index_3;
  wire at_3;
  wire write_3;
  wire [7:0] shift_3;
  wire [7:0] next_3;
  assign zero = 0;
  assign one = 1;
  assign capacity = 4;
  assign true_bit = 1;
  assign empty = (count == zero);
  assign full = (count == capacity);
  assign input_ready = (full ^ true_bit);
  assign output_valid = (empty ^ true_bit);
  assign output_data = slot_0;
  assign push = (input_valid & input_ready);
  assign pop = (output_valid & output_ready);
  assign decremented = count - one;
  assign after_pop = (pop ? decremented : count);
  assign incremented = after_pop + one;
  assign normal_count = (push ? incremented : after_pop);
  assign next_count = (flush ? zero : normal_count);
  assign index_0 = 0;
  assign at_0 = (after_pop == index_0);
  assign write_0 = (push & at_0);
  assign shift_0 = (pop ? slot_1 : slot_0);
  assign next_0 = (write_0 ? input_data : shift_0);
  assign index_1 = 1;
  assign at_1 = (after_pop == index_1);
  assign write_1 = (push & at_1);
  assign shift_1 = (pop ? slot_2 : slot_1);
  assign next_1 = (write_1 ? input_data : shift_1);
  assign index_2 = 2;
  assign at_2 = (after_pop == index_2);
  assign write_2 = (push & at_2);
  assign shift_2 = (pop ? slot_3 : slot_2);
  assign next_2 = (write_2 ? input_data : shift_2);
  assign index_3 = 3;
  assign at_3 = (after_pop == index_3);
  assign write_3 = (push & at_3);
  assign shift_3 = (pop ? slot_3 : slot_3);
  assign next_3 = (write_3 ? input_data : shift_3);
  always @(posedge clk) begin
    if (rst) begin
      count <= 0;
      slot_0 <= 0;
      slot_1 <= 0;
      slot_2 <= 0;
      slot_3 <= 0;
    end else begin
      count <= next_count;
      slot_0 <= next_0;
      slot_1 <= next_1;
      slot_2 <= next_2;
      slot_3 <= next_3;
    end
  end
endmodule

module Fr198Uart (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  input rx,
  output tx,
  output [3:0] raw_events
);
  wire ctrl_read_commit;
  wire ctrl_write_commit;
  wire [31:0] ctrl_value;
  wire [31:0] ctrl_candidate;
  wire [31:0] ctrl_write_mask;
  wire ctrl_write_reject;
  wire baud_div_read_commit;
  wire baud_div_write_commit;
  wire [31:0] baud_div_value;
  wire [31:0] baud_div_candidate;
  wire [31:0] baud_div_write_mask;
  wire baud_div_write_reject;
  wire status_read_commit;
  wire status_write_commit;
  wire [31:0] status_value;
  wire tx_data_read_commit;
  wire tx_data_write_commit;
  wire [31:0] tx_data_candidate;
  wire [31:0] tx_data_write_mask;
  wire tx_data_write_reject;
  wire rx_data_read_commit;
  wire rx_data_write_commit;
  wire [31:0] rx_data_value;
  wire rx_data_read_reject;
  wire EVENT_read_commit;
  wire EVENT_write_commit;
  wire [31:0] EVENT_value;
  wire [31:0] EVENT_candidate;
  wire [31:0] EVENT_write_mask;
  wire [31:0] event_bits;
  BitloomUartCsrRegisters csr (
    .clk(clk),
    .rst(rst),
    .req_valid(req_valid),
    .write(write),
    .addr(addr),
    .wdata(wdata),
    .wstrb(wstrb),
    .rsp_ready(rsp_ready),
    .req_ready(req_ready),
    .rsp_valid(rsp_valid),
    .rdata(rdata),
    .error(error),
    .ctrl_read_commit(ctrl_read_commit),
    .ctrl_write_commit(ctrl_write_commit),
    .ctrl_value(ctrl_value),
    .ctrl_candidate(ctrl_candidate),
    .ctrl_write_mask(ctrl_write_mask),
    .ctrl_write_reject(ctrl_write_reject),
    .baud_div_read_commit(baud_div_read_commit),
    .baud_div_write_commit(baud_div_write_commit),
    .baud_div_value(baud_div_value),
    .baud_div_candidate(baud_div_candidate),
    .baud_div_write_mask(baud_div_write_mask),
    .baud_div_write_reject(baud_div_write_reject),
    .status_read_commit(status_read_commit),
    .status_write_commit(status_write_commit),
    .status_value(status_value),
    .tx_data_read_commit(tx_data_read_commit),
    .tx_data_write_commit(tx_data_write_commit),
    .tx_data_candidate(tx_data_candidate),
    .tx_data_write_mask(tx_data_write_mask),
    .tx_data_write_reject(tx_data_write_reject),
    .rx_data_read_commit(rx_data_read_commit),
    .rx_data_write_commit(rx_data_write_commit),
    .rx_data_value(rx_data_value),
    .rx_data_read_reject(rx_data_read_reject),
    .EVENT_read_commit(EVENT_read_commit),
    .EVENT_write_commit(EVENT_write_commit),
    .EVENT_value(EVENT_value),
    .EVENT_candidate(EVENT_candidate),
    .EVENT_write_mask(EVENT_write_mask),
    .event_bits(event_bits)
  );
  wire zero_bit;
  wire txq_input_valid;
  wire txq_input_ready;
  wire [7:0] txq_input_data;
  wire txq_output_valid;
  wire txq_output_ready;
  wire [7:0] txq_output_data;
  BitloomUartCsrFifo8x4 txq (
    .clk(clk),
    .rst(rst),
    .flush(zero_bit),
    .input_valid(txq_input_valid),
    .input_ready(txq_input_ready),
    .input_data(txq_input_data),
    .output_valid(txq_output_valid),
    .output_ready(txq_output_ready),
    .output_data(txq_output_data)
  );
  wire rxq_input_valid;
  wire rxq_input_ready;
  wire [7:0] rxq_input_data;
  wire rxq_output_valid;
  wire rxq_output_ready;
  wire [7:0] rxq_output_data;
  BitloomUartCsrFifo8x4 rxq (
    .clk(clk),
    .rst(rst),
    .flush(zero_bit),
    .input_valid(rxq_input_valid),
    .input_ready(rxq_input_ready),
    .input_data(rxq_input_data),
    .output_valid(rxq_output_valid),
    .output_ready(rxq_output_ready),
    .output_data(rxq_output_data)
  );
  reg sync1;
  reg sync2;
  reg history;
  reg tx_busy;
  reg tx_line;
  reg [3:0] tx_index;
  reg [7:0] tx_shift;
  reg [31:0] tx_timer;
  reg [31:0] tx_div;
  reg [3:0] rx_state;
  reg [7:0] rx_shift;
  reg [31:0] rx_timer;
  reg [31:0] rx_div;
  wire _uart_n0;
  wire _uart_n1;
  wire [3:0] _uart_n2;
  wire [3:0] _uart_n3;
  wire [31:0] _uart_n4;
  wire [31:0] _uart_n5;
  wire _uart_n6;
  wire _uart_n7;
  wire _uart_n8;
  wire _uart_n9;
  wire _uart_n10;
  wire _uart_n11;
  wire _uart_n12;
  wire _uart_n13;
  wire _uart_n14;
  wire _uart_n15;
  wire _uart_n16;
  wire _uart_n17;
  wire [31:0] _uart_n18;
  wire _uart_n19;
  wire _uart_n20;
  wire _uart_n21;
  wire _uart_n22;
  wire _uart_n23;
  wire _uart_n24;
  wire _uart_n25;
  wire _uart_n26;
  wire _uart_n27;
  wire _uart_n28;
  wire _uart_n29;
  wire [31:0] _uart_n30;
  wire [1:0] _uart_n31;
  wire [1:0] _uart_n32;
  wire [3:0] _uart_n33;
  wire [27:0] _uart_n34;
  wire [31:0] _uart_n35;
  wire [23:0] _uart_n36;
  wire [31:0] _uart_n37;
  wire [7:0] _uart_n38;
  wire _uart_n39;
  wire _uart_n40;
  wire _uart_n41;
  wire _uart_n42;
  wire _uart_n43;
  wire _uart_n44;
  wire _uart_n45;
  wire [3:0] _uart_n46;
  wire _uart_n47;
  wire _uart_n48;
  wire _uart_n49;
  wire _uart_n50;
  wire [3:0] _uart_n51;
  wire [3:0] _uart_n52;
  wire [3:0] _uart_n53;
  wire [31:0] _uart_n54;
  wire [31:0] _uart_n55;
  wire [31:0] _uart_n56;
  wire [31:0] _uart_n57;
  wire [31:0] _uart_n58;
  wire [6:0] _uart_n59;
  wire [7:0] _uart_n60;
  wire [7:0] _uart_n61;
  wire [7:0] _uart_n62;
  wire [3:0] _uart_n63;
  wire _uart_n64;
  wire _uart_n65;
  wire _uart_n66;
  wire _uart_n67;
  wire _uart_n68;
  wire _uart_n69;
  wire _uart_n70;
  wire _uart_n71;
  wire _uart_n72;
  wire _uart_n73;
  wire _uart_n74;
  wire _uart_n75;
  wire [30:0] _uart_n76;
  wire [31:0] _uart_n77;
  wire _uart_n78;
  wire [30:0] _uart_n79;
  wire [31:0] _uart_n80;
  wire [31:0] _uart_n81;
  wire [31:0] _uart_n82;
  wire _uart_n83;
  wire _uart_n84;
  wire _uart_n85;
  wire [3:0] _uart_n86;
  wire _uart_n87;
  wire _uart_n88;
  wire _uart_n89;
  wire _uart_n90;
  wire [3:0] _uart_n91;
  wire [3:0] _uart_n92;
  wire [3:0] _uart_n93;
  wire [3:0] _uart_n94;
  wire [31:0] _uart_n95;
  wire [31:0] _uart_n96;
  wire [31:0] _uart_n97;
  wire [31:0] _uart_n98;
  wire [31:0] _uart_n99;
  wire _uart_n100;
  wire _uart_n101;
  wire _uart_n102;
  wire _uart_n103;
  wire _uart_n104;
  wire _uart_n105;
  wire [6:0] _uart_n106;
  wire [7:0] _uart_n107;
  wire [7:0] _uart_n108;
  wire _uart_n109;
  wire _uart_n110;
  wire _uart_n111;
  wire _uart_n112;
  wire _uart_n113;
  wire _uart_n114;
  wire _uart_n115;
  wire _uart_n116;
  wire [1:0] _uart_n117;
  wire [1:0] _uart_n118;
  wire [3:0] _uart_n119;
  wire [31:0] _uart_n120;
  assign _uart_n0 = 0;
  assign _uart_n1 = 1;
  assign _uart_n2 = 0;
  assign _uart_n3 = 1;
  assign _uart_n4 = 0;
  assign _uart_n5 = 1;
  assign zero_bit = _uart_n0;
  assign _uart_n6 = 1;
  assign _uart_n7 = (rst ^ _uart_n6);
  assign _uart_n8 = (rx_state == _uart_n2);
  assign _uart_n9 = 1;
  assign _uart_n10 = (_uart_n8 ^ _uart_n9);
  assign _uart_n11 = (tx_busy | _uart_n10);
  assign _uart_n12 = ctrl_value[0:0];
  assign _uart_n13 = ctrl_candidate[0:0];
  assign _uart_n14 = (_uart_n12 == _uart_n13);
  assign _uart_n15 = 1;
  assign _uart_n16 = (_uart_n14 ^ _uart_n15);
  assign _uart_n17 = (_uart_n11 & _uart_n16);
  assign _uart_n18 = 3;
  assign _uart_n19 = (baud_div_value < _uart_n18);
  assign _uart_n20 = (_uart_n13 & _uart_n19);
  assign _uart_n21 = (_uart_n17 | _uart_n20);
  assign ctrl_write_reject = _uart_n21;
  assign _uart_n22 = (baud_div_candidate < _uart_n18);
  assign _uart_n23 = (_uart_n12 & _uart_n22);
  assign _uart_n24 = (_uart_n11 | _uart_n23);
  assign baud_div_write_reject = _uart_n24;
  assign _uart_n25 = 1;
  assign _uart_n26 = (txq_input_ready ^ _uart_n25);
  assign tx_data_write_reject = _uart_n26;
  assign _uart_n27 = 1;
  assign _uart_n28 = (rxq_output_valid ^ _uart_n27);
  assign rx_data_read_reject = _uart_n28;
  assign _uart_n29 = (ctrl_write_commit ? _uart_n13 : _uart_n12);
  assign _uart_n30 = (baud_div_write_commit ? baud_div_candidate : baud_div_value);
  assign _uart_n31 = {_uart_n26, rxq_output_valid};
  assign _uart_n32 = {_uart_n10, tx_busy};
  assign _uart_n33 = {_uart_n32, _uart_n31};
  assign _uart_n34 = 0;
  assign _uart_n35 = {_uart_n34, _uart_n33};
  assign status_value = _uart_n35;
  assign _uart_n36 = 0;
  assign _uart_n37 = {_uart_n36, rxq_output_data};
  assign rx_data_value = _uart_n37;
  assign txq_input_valid = tx_data_write_commit;
  assign _uart_n38 = tx_data_candidate[7:0];
  assign txq_input_data = _uart_n38;
  assign rxq_output_ready = rx_data_read_commit;
  assign rxq_input_data = rx_shift;
  assign _uart_n39 = 1;
  assign _uart_n40 = (tx_busy ^ _uart_n39);
  assign _uart_n41 = (_uart_n40 & _uart_n29);
  assign _uart_n42 = (_uart_n41 & txq_output_valid);
  assign _uart_n43 = (_uart_n42 & _uart_n7);
  assign txq_output_ready = _uart_n43;
  assign _uart_n44 = (tx_timer == _uart_n4);
  assign _uart_n45 = (tx_busy & _uart_n44);
  assign _uart_n46 = 9;
  assign _uart_n47 = (tx_index == _uart_n46);
  assign _uart_n48 = (_uart_n45 & _uart_n47);
  assign _uart_n49 = (_uart_n48 ? _uart_n0 : tx_busy);
  assign _uart_n50 = (_uart_n43 ? _uart_n1 : _uart_n49);
  assign _uart_n51 = tx_index + _uart_n3;
  assign _uart_n52 = (_uart_n45 ? _uart_n51 : tx_index);
  assign _uart_n53 = (_uart_n43 ? _uart_n2 : _uart_n52);
  assign _uart_n54 = tx_timer - _uart_n5;
  assign _uart_n55 = (_uart_n44 ? tx_div : _uart_n54);
  assign _uart_n56 = (tx_busy ? _uart_n55 : tx_timer);
  assign _uart_n57 = (_uart_n43 ? _uart_n30 : _uart_n56);
  assign _uart_n58 = (_uart_n43 ? _uart_n30 : tx_div);
  assign _uart_n59 = tx_shift[7:1];
  assign _uart_n60 = {_uart_n0, _uart_n59};
  assign _uart_n61 = (_uart_n45 ? _uart_n60 : tx_shift);
  assign _uart_n62 = (_uart_n43 ? txq_output_data : _uart_n61);
  assign _uart_n63 = 8;
  assign _uart_n64 = (tx_index == _uart_n63);
  assign _uart_n65 = tx_shift[0:0];
  assign _uart_n66 = (_uart_n64 ? _uart_n1 : _uart_n65);
  assign _uart_n67 = (_uart_n45 ? _uart_n66 : tx_line);
  assign _uart_n68 = (_uart_n43 ? _uart_n0 : _uart_n67);
  assign _uart_n69 = (tx_busy ? tx_line : _uart_n1);
  assign tx = _uart_n69;
  assign _uart_n70 = 1;
  assign _uart_n71 = (sync2 ^ _uart_n70);
  assign _uart_n72 = (_uart_n71 & history);
  assign _uart_n73 = (_uart_n8 & _uart_n72);
  assign _uart_n74 = (_uart_n73 & _uart_n29);
  assign _uart_n75 = (_uart_n74 & _uart_n7);
  assign _uart_n76 = _uart_n30[31:1];
  assign _uart_n77 = {_uart_n0, _uart_n76};
  assign _uart_n78 = _uart_n30[0:0];
  assign _uart_n79 = 0;
  assign _uart_n80 = {_uart_n79, _uart_n78};
  assign _uart_n81 = _uart_n77 + _uart_n80;
  assign _uart_n82 = _uart_n81 - _uart_n5;
  assign _uart_n83 = (rx_timer == _uart_n4);
  assign _uart_n84 = (_uart_n10 & _uart_n83);
  assign _uart_n85 = (rx_state == _uart_n3);
  assign _uart_n86 = 10;
  assign _uart_n87 = (rx_state == _uart_n86);
  assign _uart_n88 = (_uart_n85 & sync2);
  assign _uart_n89 = (_uart_n88 | _uart_n87);
  assign _uart_n90 = (_uart_n84 & _uart_n89);
  assign _uart_n91 = rx_state + _uart_n3;
  assign _uart_n92 = (_uart_n84 ? _uart_n91 : rx_state);
  assign _uart_n93 = (_uart_n90 ? _uart_n2 : _uart_n92);
  assign _uart_n94 = (_uart_n75 ? _uart_n3 : _uart_n93);
  assign _uart_n95 = rx_timer - _uart_n5;
  assign _uart_n96 = (_uart_n83 ? rx_div : _uart_n95);
  assign _uart_n97 = (_uart_n10 ? _uart_n96 : rx_timer);
  assign _uart_n98 = (_uart_n75 ? _uart_n82 : _uart_n97);
  assign _uart_n99 = (_uart_n75 ? _uart_n30 : rx_div);
  assign _uart_n100 = 1;
  assign _uart_n101 = (_uart_n85 ^ _uart_n100);
  assign _uart_n102 = 1;
  assign _uart_n103 = (_uart_n87 ^ _uart_n102);
  assign _uart_n104 = (_uart_n101 & _uart_n103);
  assign _uart_n105 = (_uart_n84 & _uart_n104);
  assign _uart_n106 = rx_shift[7:1];
  assign _uart_n107 = {sync2, _uart_n106};
  assign _uart_n108 = (_uart_n105 ? _uart_n107 : rx_shift);
  assign _uart_n109 = (_uart_n84 & _uart_n87);
  assign _uart_n110 = (_uart_n109 & _uart_n7);
  assign _uart_n111 = (_uart_n110 & sync2);
  assign _uart_n112 = (_uart_n111 & rxq_input_ready);
  assign _uart_n113 = 1;
  assign _uart_n114 = (rxq_input_ready ^ _uart_n113);
  assign _uart_n115 = (_uart_n111 & _uart_n114);
  assign _uart_n116 = (_uart_n110 & _uart_n71);
  assign rxq_input_valid = _uart_n112;
  assign _uart_n117 = {_uart_n43, _uart_n112};
  assign _uart_n118 = {_uart_n116, _uart_n115};
  assign _uart_n119 = {_uart_n118, _uart_n117};
  assign raw_events = _uart_n119;
  assign _uart_n120 = {_uart_n34, _uart_n119};
  assign event_bits = _uart_n120;
  always @(posedge clk) begin
    if (rst) begin
      sync1 <= 0;
      sync2 <= 0;
      history <= 0;
      tx_busy <= 0;
      tx_index <= 0;
      tx_timer <= 0;
      tx_div <= 0;
      tx_shift <= 0;
      tx_line <= 0;
      rx_state <= 0;
      rx_timer <= 0;
      rx_div <= 0;
      rx_shift <= 0;
    end else begin
      sync1 <= rx;
      sync2 <= sync1;
      history <= sync2;
      tx_busy <= _uart_n50;
      tx_index <= _uart_n53;
      tx_timer <= _uart_n57;
      tx_div <= _uart_n58;
      tx_shift <= _uart_n62;
      tx_line <= _uart_n68;
      rx_state <= _uart_n94;
      rx_timer <= _uart_n98;
      rx_div <= _uart_n99;
      rx_shift <= _uart_n108;
    end
  end
endmodule

module BitloomGpioCsrRegisters (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  output dir_read_commit,
  output dir_write_commit,
  output [31:0] dir_value,
  output [31:0] dir_candidate,
  output [31:0] dir_write_mask,
  output out_read_commit,
  output out_write_commit,
  input [31:0] out_value,
  output [31:0] out_candidate,
  output [31:0] out_write_mask,
  output in_read_commit,
  output in_write_commit,
  input [31:0] in_value,
  output set_read_commit,
  output set_write_commit,
  output [31:0] set_candidate,
  output [31:0] set_write_mask,
  output clear_read_commit,
  output clear_write_commit,
  output [31:0] clear_candidate,
  output [31:0] clear_write_mask,
  output rise_event_read_commit,
  output rise_event_write_commit,
  output [31:0] rise_event_value,
  output [31:0] rise_event_candidate,
  output [31:0] rise_event_write_mask,
  input [31:0] rise_bits
);
  reg _csr_pending;
  reg [31:0] _csr_data;
  reg [1:0] _csr_error;
  reg [31:0] _csr_value0;
  reg [31:0] _csr_value5;
  wire _csr_n0;
  wire _csr_n1;
  wire [31:0] _csr_n2;
  wire [1:0] _csr_n3;
  wire [1:0] _csr_n4;
  wire _csr_n5;
  wire _csr_n6;
  wire _csr_n7;
  wire _csr_n8;
  wire _csr_n9;
  wire _csr_n10;
  wire _csr_n11;
  wire _csr_n12;
  wire _csr_n13;
  wire _csr_n14;
  wire _csr_n15;
  wire [31:0] _csr_n16;
  wire [31:0] _csr_n17;
  wire [31:0] _csr_n18;
  wire _csr_n19;
  wire [31:0] _csr_n20;
  wire [31:0] _csr_n21;
  wire [31:0] _csr_n22;
  wire _csr_n23;
  wire [31:0] _csr_n24;
  wire [31:0] _csr_n25;
  wire [31:0] _csr_n26;
  wire _csr_n27;
  wire [31:0] _csr_n28;
  wire [31:0] _csr_n29;
  wire [31:0] _csr_n30;
  wire [31:0] _csr_n31;
  wire [31:0] _csr_n32;
  wire [15:0] _csr_n33;
  wire _csr_n34;
  wire _csr_n35;
  wire _csr_n36;
  wire _csr_n37;
  wire [31:0] _csr_n38;
  wire _csr_n39;
  wire _csr_n40;
  wire _csr_n41;
  wire [31:0] _csr_n42;
  wire [31:0] _csr_n43;
  wire [31:0] _csr_n44;
  wire [31:0] _csr_n45;
  wire [31:0] _csr_n46;
  wire _csr_n47;
  wire _csr_n48;
  wire _csr_n49;
  wire _csr_n50;
  wire [1:0] _csr_n51;
  wire [1:0] _csr_n52;
  wire [31:0] _csr_n53;
  wire [31:0] _csr_n54;
  wire [31:0] _csr_n55;
  wire [31:0] _csr_n56;
  wire [31:0] _csr_n57;
  wire [15:0] _csr_n58;
  wire _csr_n59;
  wire _csr_n60;
  wire _csr_n61;
  wire _csr_n62;
  wire [31:0] _csr_n63;
  wire _csr_n64;
  wire _csr_n65;
  wire _csr_n66;
  wire [31:0] _csr_n67;
  wire [31:0] _csr_n68;
  wire [31:0] _csr_n69;
  wire [31:0] _csr_n70;
  wire [31:0] _csr_n71;
  wire _csr_n72;
  wire _csr_n73;
  wire _csr_n74;
  wire _csr_n75;
  wire [1:0] _csr_n76;
  wire [1:0] _csr_n77;
  wire [31:0] _csr_n78;
  wire [31:0] _csr_n79;
  wire [31:0] _csr_n80;
  wire [31:0] _csr_n81;
  wire [15:0] _csr_n82;
  wire _csr_n83;
  wire _csr_n84;
  wire _csr_n85;
  wire _csr_n86;
  wire _csr_n87;
  wire [1:0] _csr_n88;
  wire [1:0] _csr_n89;
  wire [31:0] _csr_n90;
  wire [31:0] _csr_n91;
  wire [31:0] _csr_n92;
  wire [15:0] _csr_n93;
  wire _csr_n94;
  wire _csr_n95;
  wire _csr_n96;
  wire _csr_n97;
  wire [31:0] _csr_n98;
  wire _csr_n99;
  wire _csr_n100;
  wire _csr_n101;
  wire [31:0] _csr_n102;
  wire _csr_n103;
  wire _csr_n104;
  wire _csr_n105;
  wire _csr_n106;
  wire [1:0] _csr_n107;
  wire [1:0] _csr_n108;
  wire [31:0] _csr_n109;
  wire [31:0] _csr_n110;
  wire [31:0] _csr_n111;
  wire [15:0] _csr_n112;
  wire _csr_n113;
  wire _csr_n114;
  wire _csr_n115;
  wire _csr_n116;
  wire [31:0] _csr_n117;
  wire _csr_n118;
  wire _csr_n119;
  wire _csr_n120;
  wire [31:0] _csr_n121;
  wire _csr_n122;
  wire _csr_n123;
  wire _csr_n124;
  wire _csr_n125;
  wire [1:0] _csr_n126;
  wire [1:0] _csr_n127;
  wire [31:0] _csr_n128;
  wire [31:0] _csr_n129;
  wire [31:0] _csr_n130;
  wire [31:0] _csr_n131;
  wire [15:0] _csr_n132;
  wire _csr_n133;
  wire _csr_n134;
  wire _csr_n135;
  wire _csr_n136;
  wire [31:0] _csr_n137;
  wire _csr_n138;
  wire _csr_n139;
  wire _csr_n140;
  wire [31:0] _csr_n141;
  wire _csr_n142;
  wire _csr_n143;
  wire _csr_n144;
  wire _csr_n145;
  wire [1:0] _csr_n146;
  wire [1:0] _csr_n147;
  wire [31:0] _csr_n148;
  wire [31:0] _csr_n149;
  wire [31:0] _csr_n150;
  wire [31:0] _csr_n151;
  wire [31:0] _csr_n152;
  wire [31:0] _csr_n153;
  wire [31:0] _csr_n154;
  wire [31:0] _csr_n155;
  wire [31:0] _csr_n156;
  wire [1:0] _csr_n157;
  assign _csr_n0 = 0;
  assign _csr_n1 = 1;
  assign _csr_n2 = 0;
  assign _csr_n3 = 0;
  assign _csr_n4 = 2;
  assign _csr_n5 = 1;
  assign _csr_n6 = (_csr_pending ^ _csr_n5);
  assign _csr_n7 = 1;
  assign _csr_n8 = (rst ^ _csr_n7);
  assign _csr_n9 = (_csr_n6 & _csr_n8);
  assign req_ready = _csr_n9;
  assign _csr_n10 = (req_valid & _csr_n9);
  assign _csr_n11 = 1;
  assign _csr_n12 = (write ^ _csr_n11);
  assign _csr_n13 = (rsp_ready ? _csr_n0 : _csr_pending);
  assign _csr_n14 = (_csr_n10 ? _csr_n1 : _csr_n13);
  assign rsp_valid = _csr_pending;
  assign rdata = _csr_data;
  assign error = _csr_error;
  assign _csr_n15 = wstrb[0:0];
  assign _csr_n16 = 255;
  assign _csr_n17 = (_csr_n15 ? _csr_n16 : _csr_n2);
  assign _csr_n18 = (_csr_n2 | _csr_n17);
  assign _csr_n19 = wstrb[1:1];
  assign _csr_n20 = 65280;
  assign _csr_n21 = (_csr_n19 ? _csr_n20 : _csr_n2);
  assign _csr_n22 = (_csr_n18 | _csr_n21);
  assign _csr_n23 = wstrb[2:2];
  assign _csr_n24 = 16711680;
  assign _csr_n25 = (_csr_n23 ? _csr_n24 : _csr_n2);
  assign _csr_n26 = (_csr_n22 | _csr_n25);
  assign _csr_n27 = wstrb[3:3];
  assign _csr_n28 = 4278190080;
  assign _csr_n29 = (_csr_n27 ? _csr_n28 : _csr_n2);
  assign _csr_n30 = (_csr_n26 | _csr_n29);
  assign _csr_n31 = 4294967295;
  assign _csr_n32 = (_csr_value0 & _csr_n31);
  assign dir_value = _csr_n32;
  assign _csr_n33 = 0;
  assign _csr_n34 = (addr == _csr_n33);
  assign _csr_n35 = (_csr_n10 & _csr_n34);
  assign _csr_n36 = (_csr_n12 & _csr_n1);
  assign _csr_n37 = (_csr_n35 & _csr_n36);
  assign dir_read_commit = _csr_n37;
  assign _csr_n38 = (_csr_n30 & _csr_n31);
  assign _csr_n39 = (_csr_n38 == _csr_n2);
  assign _csr_n40 = 1;
  assign _csr_n41 = (_csr_n39 ^ _csr_n40);
  assign _csr_n42 = (wdata & _csr_n38);
  assign _csr_n43 = 4294967295;
  assign _csr_n44 = (_csr_n38 ^ _csr_n43);
  assign _csr_n45 = (_csr_n32 & _csr_n44);
  assign _csr_n46 = (_csr_n45 | _csr_n42);
  assign dir_write_mask = _csr_n38;
  assign dir_candidate = _csr_n46;
  assign _csr_n47 = (write & _csr_n1);
  assign _csr_n48 = (_csr_n47 & _csr_n41);
  assign _csr_n49 = (_csr_n35 & _csr_n48);
  assign dir_write_commit = _csr_n49;
  assign _csr_n50 = (_csr_n36 | _csr_n47);
  assign _csr_n51 = (_csr_n50 ? _csr_n3 : _csr_n4);
  assign _csr_n52 = (_csr_n34 ? _csr_n51 : _csr_n4);
  assign _csr_n53 = (_csr_n36 ? _csr_n32 : _csr_n2);
  assign _csr_n54 = (_csr_n34 ? _csr_n53 : _csr_n2);
  assign _csr_n55 = (_csr_n49 ? _csr_n46 : _csr_n32);
  assign _csr_n56 = 4294967295;
  assign _csr_n57 = (out_value & _csr_n56);
  assign _csr_n58 = 4;
  assign _csr_n59 = (addr == _csr_n58);
  assign _csr_n60 = (_csr_n10 & _csr_n59);
  assign _csr_n61 = (_csr_n12 & _csr_n1);
  assign _csr_n62 = (_csr_n60 & _csr_n61);
  assign out_read_commit = _csr_n62;
  assign _csr_n63 = (_csr_n30 & _csr_n56);
  assign _csr_n64 = (_csr_n63 == _csr_n2);
  assign _csr_n65 = 1;
  assign _csr_n66 = (_csr_n64 ^ _csr_n65);
  assign _csr_n67 = (wdata & _csr_n63);
  assign _csr_n68 = 4294967295;
  assign _csr_n69 = (_csr_n63 ^ _csr_n68);
  assign _csr_n70 = (_csr_n57 & _csr_n69);
  assign _csr_n71 = (_csr_n70 | _csr_n67);
  assign out_write_mask = _csr_n63;
  assign out_candidate = _csr_n71;
  assign _csr_n72 = (write & _csr_n1);
  assign _csr_n73 = (_csr_n72 & _csr_n66);
  assign _csr_n74 = (_csr_n60 & _csr_n73);
  assign out_write_commit = _csr_n74;
  assign _csr_n75 = (_csr_n61 | _csr_n72);
  assign _csr_n76 = (_csr_n75 ? _csr_n3 : _csr_n4);
  assign _csr_n77 = (_csr_n59 ? _csr_n76 : _csr_n52);
  assign _csr_n78 = (_csr_n61 ? _csr_n57 : _csr_n2);
  assign _csr_n79 = (_csr_n59 ? _csr_n78 : _csr_n54);
  assign _csr_n80 = 4294967295;
  assign _csr_n81 = (in_value & _csr_n80);
  assign _csr_n82 = 8;
  assign _csr_n83 = (addr == _csr_n82);
  assign _csr_n84 = (_csr_n10 & _csr_n83);
  assign _csr_n85 = (_csr_n12 & _csr_n1);
  assign _csr_n86 = (_csr_n84 & _csr_n85);
  assign in_read_commit = _csr_n86;
  assign in_write_commit = _csr_n0;
  assign _csr_n87 = (_csr_n85 | _csr_n0);
  assign _csr_n88 = (_csr_n87 ? _csr_n3 : _csr_n4);
  assign _csr_n89 = (_csr_n83 ? _csr_n88 : _csr_n77);
  assign _csr_n90 = (_csr_n85 ? _csr_n81 : _csr_n2);
  assign _csr_n91 = (_csr_n83 ? _csr_n90 : _csr_n79);
  assign _csr_n92 = 4294967295;
  assign _csr_n93 = 12;
  assign _csr_n94 = (addr == _csr_n93);
  assign _csr_n95 = (_csr_n10 & _csr_n94);
  assign _csr_n96 = (_csr_n12 & _csr_n0);
  assign _csr_n97 = (_csr_n95 & _csr_n96);
  assign set_read_commit = _csr_n97;
  assign _csr_n98 = (_csr_n30 & _csr_n92);
  assign _csr_n99 = (_csr_n98 == _csr_n2);
  assign _csr_n100 = 1;
  assign _csr_n101 = (_csr_n99 ^ _csr_n100);
  assign _csr_n102 = (wdata & _csr_n98);
  assign set_write_mask = _csr_n98;
  assign set_candidate = _csr_n102;
  assign _csr_n103 = (write & _csr_n1);
  assign _csr_n104 = (_csr_n103 & _csr_n101);
  assign _csr_n105 = (_csr_n95 & _csr_n104);
  assign set_write_commit = _csr_n105;
  assign _csr_n106 = (_csr_n96 | _csr_n103);
  assign _csr_n107 = (_csr_n106 ? _csr_n3 : _csr_n4);
  assign _csr_n108 = (_csr_n94 ? _csr_n107 : _csr_n89);
  assign _csr_n109 = (_csr_n96 ? _csr_n2 : _csr_n2);
  assign _csr_n110 = (_csr_n94 ? _csr_n109 : _csr_n91);
  assign _csr_n111 = 4294967295;
  assign _csr_n112 = 16;
  assign _csr_n113 = (addr == _csr_n112);
  assign _csr_n114 = (_csr_n10 & _csr_n113);
  assign _csr_n115 = (_csr_n12 & _csr_n0);
  assign _csr_n116 = (_csr_n114 & _csr_n115);
  assign clear_read_commit = _csr_n116;
  assign _csr_n117 = (_csr_n30 & _csr_n111);
  assign _csr_n118 = (_csr_n117 == _csr_n2);
  assign _csr_n119 = 1;
  assign _csr_n120 = (_csr_n118 ^ _csr_n119);
  assign _csr_n121 = (wdata & _csr_n117);
  assign clear_write_mask = _csr_n117;
  assign clear_candidate = _csr_n121;
  assign _csr_n122 = (write & _csr_n1);
  assign _csr_n123 = (_csr_n122 & _csr_n120);
  assign _csr_n124 = (_csr_n114 & _csr_n123);
  assign clear_write_commit = _csr_n124;
  assign _csr_n125 = (_csr_n115 | _csr_n122);
  assign _csr_n126 = (_csr_n125 ? _csr_n3 : _csr_n4);
  assign _csr_n127 = (_csr_n113 ? _csr_n126 : _csr_n108);
  assign _csr_n128 = (_csr_n115 ? _csr_n2 : _csr_n2);
  assign _csr_n129 = (_csr_n113 ? _csr_n128 : _csr_n110);
  assign _csr_n130 = 4294967295;
  assign _csr_n131 = (_csr_value5 & _csr_n130);
  assign rise_event_value = _csr_n131;
  assign _csr_n132 = 20;
  assign _csr_n133 = (addr == _csr_n132);
  assign _csr_n134 = (_csr_n10 & _csr_n133);
  assign _csr_n135 = (_csr_n12 & _csr_n1);
  assign _csr_n136 = (_csr_n134 & _csr_n135);
  assign rise_event_read_commit = _csr_n136;
  assign _csr_n137 = (_csr_n30 & _csr_n130);
  assign _csr_n138 = (_csr_n137 == _csr_n2);
  assign _csr_n139 = 1;
  assign _csr_n140 = (_csr_n138 ^ _csr_n139);
  assign _csr_n141 = (wdata & _csr_n137);
  assign rise_event_write_mask = _csr_n137;
  assign rise_event_candidate = _csr_n141;
  assign _csr_n142 = (write & _csr_n1);
  assign _csr_n143 = (_csr_n142 & _csr_n140);
  assign _csr_n144 = (_csr_n134 & _csr_n143);
  assign rise_event_write_commit = _csr_n144;
  assign _csr_n145 = (_csr_n135 | _csr_n142);
  assign _csr_n146 = (_csr_n145 ? _csr_n3 : _csr_n4);
  assign _csr_n147 = (_csr_n133 ? _csr_n146 : _csr_n127);
  assign _csr_n148 = (_csr_n135 ? _csr_n131 : _csr_n2);
  assign _csr_n149 = (_csr_n133 ? _csr_n148 : _csr_n129);
  assign _csr_n150 = (_csr_n144 ? _csr_n141 : _csr_n2);
  assign _csr_n151 = 4294967295;
  assign _csr_n152 = (_csr_n150 ^ _csr_n151);
  assign _csr_n153 = (_csr_n131 & _csr_n152);
  assign _csr_n154 = (rise_bits & _csr_n130);
  assign _csr_n155 = (_csr_n153 | _csr_n154);
  assign _csr_n156 = (_csr_n10 ? _csr_n149 : _csr_data);
  assign _csr_n157 = (_csr_n10 ? _csr_n147 : _csr_error);
  always @(posedge clk) begin
    if (rst) begin
      _csr_pending <= 0;
      _csr_data <= 0;
      _csr_error <= 0;
      _csr_value0 <= 0;
      _csr_value5 <= 0;
    end else begin
      _csr_pending <= _csr_n14;
      _csr_data <= _csr_n156;
      _csr_error <= _csr_n157;
      _csr_value0 <= _csr_n55;
      _csr_value5 <= _csr_n155;
    end
  end
endmodule

module Fr198Gpio (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  input [31:0] pad_in,
  output [31:0] pad_out,
  output [31:0] pad_oe,
  output raw_event
);
  wire dir_read_commit;
  wire dir_write_commit;
  wire [31:0] dir_value;
  wire [31:0] dir_candidate;
  wire [31:0] dir_write_mask;
  wire out_read_commit;
  wire out_write_commit;
  wire [31:0] out_value;
  wire [31:0] out_candidate;
  wire [31:0] out_write_mask;
  wire in_read_commit;
  wire in_write_commit;
  wire [31:0] in_value;
  wire set_read_commit;
  wire set_write_commit;
  wire [31:0] set_candidate;
  wire [31:0] set_write_mask;
  wire clear_read_commit;
  wire clear_write_commit;
  wire [31:0] clear_candidate;
  wire [31:0] clear_write_mask;
  wire rise_event_read_commit;
  wire rise_event_write_commit;
  wire [31:0] rise_event_value;
  wire [31:0] rise_event_candidate;
  wire [31:0] rise_event_write_mask;
  reg [31:0] out_r;
  reg [31:0] sync1;
  reg [31:0] sync2;
  reg [31:0] history;
  wire [31:0] rise_bits;
  wire [31:0] all32;
  wire [31:0] zero32;
  wire [31:0] not_history;
  wire [31:0] not_dir;
  wire [31:0] rising;
  wire [31:0] set_value;
  wire [31:0] clear_value;
  wire [31:0] not_clear;
  wire [31:0] after_clear;
  wire [31:0] after_set;
  wire [31:0] out_next;
  wire empty;
  wire has_rise;
  wire one;
  wire not_reset;
  BitloomGpioCsrRegisters csr (
    .clk(clk),
    .rst(rst),
    .req_valid(req_valid),
    .write(write),
    .addr(addr),
    .wdata(wdata),
    .wstrb(wstrb),
    .rsp_ready(rsp_ready),
    .req_ready(req_ready),
    .rsp_valid(rsp_valid),
    .rdata(rdata),
    .error(error),
    .dir_read_commit(dir_read_commit),
    .dir_write_commit(dir_write_commit),
    .dir_value(dir_value),
    .dir_candidate(dir_candidate),
    .dir_write_mask(dir_write_mask),
    .out_read_commit(out_read_commit),
    .out_write_commit(out_write_commit),
    .out_value(out_value),
    .out_candidate(out_candidate),
    .out_write_mask(out_write_mask),
    .in_read_commit(in_read_commit),
    .in_write_commit(in_write_commit),
    .in_value(in_value),
    .set_read_commit(set_read_commit),
    .set_write_commit(set_write_commit),
    .set_candidate(set_candidate),
    .set_write_mask(set_write_mask),
    .clear_read_commit(clear_read_commit),
    .clear_write_commit(clear_write_commit),
    .clear_candidate(clear_candidate),
    .clear_write_mask(clear_write_mask),
    .rise_event_read_commit(rise_event_read_commit),
    .rise_event_write_commit(rise_event_write_commit),
    .rise_event_value(rise_event_value),
    .rise_event_candidate(rise_event_candidate),
    .rise_event_write_mask(rise_event_write_mask),
    .rise_bits(rise_bits)
  );
  assign all32 = 4294967295;
  assign zero32 = 0;
  assign one = 1;
  assign out_value = out_r;
  assign in_value = sync2;
  assign pad_oe = dir_value;
  assign pad_out = (out_r & dir_value);
  assign not_history = (history ^ all32);
  assign not_dir = (dir_value ^ all32);
  assign rising = (sync2 & not_history);
  assign rise_bits = (rising & not_dir);
  assign empty = (rise_bits == zero32);
  assign has_rise = (empty ^ one);
  assign not_reset = (rst ^ one);
  assign raw_event = (has_rise & not_reset);
  assign set_value = (out_r | set_candidate);
  assign not_clear = (clear_candidate ^ all32);
  assign clear_value = (out_r & not_clear);
  assign after_clear = (clear_write_commit ? clear_value : out_r);
  assign after_set = (set_write_commit ? set_value : after_clear);
  assign out_next = (out_write_commit ? out_candidate : after_set);
  always @(posedge clk) begin
    if (rst) begin
      out_r <= 0;
      sync1 <= 0;
      sync2 <= 0;
      history <= 0;
    end else begin
      out_r <= out_next;
      sync1 <= pad_in;
      sync2 <= sync1;
      history <= sync2;
    end
  end
endmodule

module BitloomTimerCsr (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  output ctrl_read_commit,
  output ctrl_write_commit,
  input [31:0] ctrl_value,
  output [31:0] ctrl_candidate,
  output [31:0] ctrl_write_mask,
  output count_read_commit,
  output count_write_commit,
  input [31:0] count_value,
  output [31:0] count_candidate,
  output [31:0] count_write_mask,
  output compare_read_commit,
  output compare_write_commit,
  output [31:0] compare_value,
  output [31:0] compare_candidate,
  output [31:0] compare_write_mask,
  output EVENT_read_commit,
  output EVENT_write_commit,
  output [31:0] EVENT_value,
  output [31:0] EVENT_candidate,
  output [31:0] EVENT_write_mask,
  input [31:0] match_bits
);
  reg _csr_pending;
  reg [31:0] _csr_data;
  reg [1:0] _csr_error;
  reg [31:0] _csr_value2;
  reg [31:0] _csr_value3;
  wire _csr_n0;
  wire _csr_n1;
  wire [31:0] _csr_n2;
  wire [1:0] _csr_n3;
  wire [1:0] _csr_n4;
  wire _csr_n5;
  wire _csr_n6;
  wire _csr_n7;
  wire _csr_n8;
  wire _csr_n9;
  wire _csr_n10;
  wire _csr_n11;
  wire _csr_n12;
  wire _csr_n13;
  wire _csr_n14;
  wire _csr_n15;
  wire [31:0] _csr_n16;
  wire [31:0] _csr_n17;
  wire [31:0] _csr_n18;
  wire _csr_n19;
  wire [31:0] _csr_n20;
  wire [31:0] _csr_n21;
  wire [31:0] _csr_n22;
  wire _csr_n23;
  wire [31:0] _csr_n24;
  wire [31:0] _csr_n25;
  wire [31:0] _csr_n26;
  wire _csr_n27;
  wire [31:0] _csr_n28;
  wire [31:0] _csr_n29;
  wire [31:0] _csr_n30;
  wire [31:0] _csr_n31;
  wire [31:0] _csr_n32;
  wire [15:0] _csr_n33;
  wire _csr_n34;
  wire _csr_n35;
  wire _csr_n36;
  wire _csr_n37;
  wire [31:0] _csr_n38;
  wire _csr_n39;
  wire _csr_n40;
  wire _csr_n41;
  wire [31:0] _csr_n42;
  wire [31:0] _csr_n43;
  wire [31:0] _csr_n44;
  wire [31:0] _csr_n45;
  wire [31:0] _csr_n46;
  wire _csr_n47;
  wire _csr_n48;
  wire _csr_n49;
  wire _csr_n50;
  wire [1:0] _csr_n51;
  wire [1:0] _csr_n52;
  wire [31:0] _csr_n53;
  wire [31:0] _csr_n54;
  wire [31:0] _csr_n55;
  wire [31:0] _csr_n56;
  wire [15:0] _csr_n57;
  wire _csr_n58;
  wire _csr_n59;
  wire _csr_n60;
  wire _csr_n61;
  wire [31:0] _csr_n62;
  wire _csr_n63;
  wire _csr_n64;
  wire _csr_n65;
  wire [31:0] _csr_n66;
  wire [31:0] _csr_n67;
  wire [31:0] _csr_n68;
  wire [31:0] _csr_n69;
  wire [31:0] _csr_n70;
  wire _csr_n71;
  wire _csr_n72;
  wire _csr_n73;
  wire _csr_n74;
  wire [1:0] _csr_n75;
  wire [1:0] _csr_n76;
  wire [31:0] _csr_n77;
  wire [31:0] _csr_n78;
  wire [31:0] _csr_n79;
  wire [31:0] _csr_n80;
  wire [15:0] _csr_n81;
  wire _csr_n82;
  wire _csr_n83;
  wire _csr_n84;
  wire _csr_n85;
  wire [31:0] _csr_n86;
  wire _csr_n87;
  wire _csr_n88;
  wire _csr_n89;
  wire [31:0] _csr_n90;
  wire [31:0] _csr_n91;
  wire [31:0] _csr_n92;
  wire [31:0] _csr_n93;
  wire [31:0] _csr_n94;
  wire _csr_n95;
  wire _csr_n96;
  wire _csr_n97;
  wire _csr_n98;
  wire [1:0] _csr_n99;
  wire [1:0] _csr_n100;
  wire [31:0] _csr_n101;
  wire [31:0] _csr_n102;
  wire [31:0] _csr_n103;
  wire [31:0] _csr_n104;
  wire [31:0] _csr_n105;
  wire [15:0] _csr_n106;
  wire _csr_n107;
  wire _csr_n108;
  wire _csr_n109;
  wire _csr_n110;
  wire [31:0] _csr_n111;
  wire _csr_n112;
  wire _csr_n113;
  wire _csr_n114;
  wire [31:0] _csr_n115;
  wire _csr_n116;
  wire _csr_n117;
  wire _csr_n118;
  wire _csr_n119;
  wire [1:0] _csr_n120;
  wire [1:0] _csr_n121;
  wire [31:0] _csr_n122;
  wire [31:0] _csr_n123;
  wire [31:0] _csr_n124;
  wire [31:0] _csr_n125;
  wire [31:0] _csr_n126;
  wire [31:0] _csr_n127;
  wire [31:0] _csr_n128;
  wire [31:0] _csr_n129;
  wire [31:0] _csr_n130;
  wire [1:0] _csr_n131;
  assign _csr_n0 = 0;
  assign _csr_n1 = 1;
  assign _csr_n2 = 0;
  assign _csr_n3 = 0;
  assign _csr_n4 = 2;
  assign _csr_n5 = 1;
  assign _csr_n6 = (_csr_pending ^ _csr_n5);
  assign _csr_n7 = 1;
  assign _csr_n8 = (rst ^ _csr_n7);
  assign _csr_n9 = (_csr_n6 & _csr_n8);
  assign req_ready = _csr_n9;
  assign _csr_n10 = (req_valid & _csr_n9);
  assign _csr_n11 = 1;
  assign _csr_n12 = (write ^ _csr_n11);
  assign _csr_n13 = (rsp_ready ? _csr_n0 : _csr_pending);
  assign _csr_n14 = (_csr_n10 ? _csr_n1 : _csr_n13);
  assign rsp_valid = _csr_pending;
  assign rdata = _csr_data;
  assign error = _csr_error;
  assign _csr_n15 = wstrb[0:0];
  assign _csr_n16 = 255;
  assign _csr_n17 = (_csr_n15 ? _csr_n16 : _csr_n2);
  assign _csr_n18 = (_csr_n2 | _csr_n17);
  assign _csr_n19 = wstrb[1:1];
  assign _csr_n20 = 65280;
  assign _csr_n21 = (_csr_n19 ? _csr_n20 : _csr_n2);
  assign _csr_n22 = (_csr_n18 | _csr_n21);
  assign _csr_n23 = wstrb[2:2];
  assign _csr_n24 = 16711680;
  assign _csr_n25 = (_csr_n23 ? _csr_n24 : _csr_n2);
  assign _csr_n26 = (_csr_n22 | _csr_n25);
  assign _csr_n27 = wstrb[3:3];
  assign _csr_n28 = 4278190080;
  assign _csr_n29 = (_csr_n27 ? _csr_n28 : _csr_n2);
  assign _csr_n30 = (_csr_n26 | _csr_n29);
  assign _csr_n31 = 3;
  assign _csr_n32 = (ctrl_value & _csr_n31);
  assign _csr_n33 = 0;
  assign _csr_n34 = (addr == _csr_n33);
  assign _csr_n35 = (_csr_n10 & _csr_n34);
  assign _csr_n36 = (_csr_n12 & _csr_n1);
  assign _csr_n37 = (_csr_n35 & _csr_n36);
  assign ctrl_read_commit = _csr_n37;
  assign _csr_n38 = (_csr_n30 & _csr_n31);
  assign _csr_n39 = (_csr_n38 == _csr_n2);
  assign _csr_n40 = 1;
  assign _csr_n41 = (_csr_n39 ^ _csr_n40);
  assign _csr_n42 = (wdata & _csr_n38);
  assign _csr_n43 = 4294967295;
  assign _csr_n44 = (_csr_n38 ^ _csr_n43);
  assign _csr_n45 = (_csr_n32 & _csr_n44);
  assign _csr_n46 = (_csr_n45 | _csr_n42);
  assign ctrl_write_mask = _csr_n38;
  assign ctrl_candidate = _csr_n46;
  assign _csr_n47 = (write & _csr_n1);
  assign _csr_n48 = (_csr_n47 & _csr_n41);
  assign _csr_n49 = (_csr_n35 & _csr_n48);
  assign ctrl_write_commit = _csr_n49;
  assign _csr_n50 = (_csr_n36 | _csr_n47);
  assign _csr_n51 = (_csr_n50 ? _csr_n3 : _csr_n4);
  assign _csr_n52 = (_csr_n34 ? _csr_n51 : _csr_n4);
  assign _csr_n53 = (_csr_n36 ? _csr_n32 : _csr_n2);
  assign _csr_n54 = (_csr_n34 ? _csr_n53 : _csr_n2);
  assign _csr_n55 = 4294967295;
  assign _csr_n56 = (count_value & _csr_n55);
  assign _csr_n57 = 4;
  assign _csr_n58 = (addr == _csr_n57);
  assign _csr_n59 = (_csr_n10 & _csr_n58);
  assign _csr_n60 = (_csr_n12 & _csr_n1);
  assign _csr_n61 = (_csr_n59 & _csr_n60);
  assign count_read_commit = _csr_n61;
  assign _csr_n62 = (_csr_n30 & _csr_n55);
  assign _csr_n63 = (_csr_n62 == _csr_n2);
  assign _csr_n64 = 1;
  assign _csr_n65 = (_csr_n63 ^ _csr_n64);
  assign _csr_n66 = (wdata & _csr_n62);
  assign _csr_n67 = 4294967295;
  assign _csr_n68 = (_csr_n62 ^ _csr_n67);
  assign _csr_n69 = (_csr_n56 & _csr_n68);
  assign _csr_n70 = (_csr_n69 | _csr_n66);
  assign count_write_mask = _csr_n62;
  assign count_candidate = _csr_n70;
  assign _csr_n71 = (write & _csr_n1);
  assign _csr_n72 = (_csr_n71 & _csr_n65);
  assign _csr_n73 = (_csr_n59 & _csr_n72);
  assign count_write_commit = _csr_n73;
  assign _csr_n74 = (_csr_n60 | _csr_n71);
  assign _csr_n75 = (_csr_n74 ? _csr_n3 : _csr_n4);
  assign _csr_n76 = (_csr_n58 ? _csr_n75 : _csr_n52);
  assign _csr_n77 = (_csr_n60 ? _csr_n56 : _csr_n2);
  assign _csr_n78 = (_csr_n58 ? _csr_n77 : _csr_n54);
  assign _csr_n79 = 4294967295;
  assign _csr_n80 = (_csr_value2 & _csr_n79);
  assign compare_value = _csr_n80;
  assign _csr_n81 = 8;
  assign _csr_n82 = (addr == _csr_n81);
  assign _csr_n83 = (_csr_n10 & _csr_n82);
  assign _csr_n84 = (_csr_n12 & _csr_n1);
  assign _csr_n85 = (_csr_n83 & _csr_n84);
  assign compare_read_commit = _csr_n85;
  assign _csr_n86 = (_csr_n30 & _csr_n79);
  assign _csr_n87 = (_csr_n86 == _csr_n2);
  assign _csr_n88 = 1;
  assign _csr_n89 = (_csr_n87 ^ _csr_n88);
  assign _csr_n90 = (wdata & _csr_n86);
  assign _csr_n91 = 4294967295;
  assign _csr_n92 = (_csr_n86 ^ _csr_n91);
  assign _csr_n93 = (_csr_n80 & _csr_n92);
  assign _csr_n94 = (_csr_n93 | _csr_n90);
  assign compare_write_mask = _csr_n86;
  assign compare_candidate = _csr_n94;
  assign _csr_n95 = (write & _csr_n1);
  assign _csr_n96 = (_csr_n95 & _csr_n89);
  assign _csr_n97 = (_csr_n83 & _csr_n96);
  assign compare_write_commit = _csr_n97;
  assign _csr_n98 = (_csr_n84 | _csr_n95);
  assign _csr_n99 = (_csr_n98 ? _csr_n3 : _csr_n4);
  assign _csr_n100 = (_csr_n82 ? _csr_n99 : _csr_n76);
  assign _csr_n101 = (_csr_n84 ? _csr_n80 : _csr_n2);
  assign _csr_n102 = (_csr_n82 ? _csr_n101 : _csr_n78);
  assign _csr_n103 = (_csr_n97 ? _csr_n94 : _csr_n80);
  assign _csr_n104 = 1;
  assign _csr_n105 = (_csr_value3 & _csr_n104);
  assign EVENT_value = _csr_n105;
  assign _csr_n106 = 12;
  assign _csr_n107 = (addr == _csr_n106);
  assign _csr_n108 = (_csr_n10 & _csr_n107);
  assign _csr_n109 = (_csr_n12 & _csr_n1);
  assign _csr_n110 = (_csr_n108 & _csr_n109);
  assign EVENT_read_commit = _csr_n110;
  assign _csr_n111 = (_csr_n30 & _csr_n104);
  assign _csr_n112 = (_csr_n111 == _csr_n2);
  assign _csr_n113 = 1;
  assign _csr_n114 = (_csr_n112 ^ _csr_n113);
  assign _csr_n115 = (wdata & _csr_n111);
  assign EVENT_write_mask = _csr_n111;
  assign EVENT_candidate = _csr_n115;
  assign _csr_n116 = (write & _csr_n1);
  assign _csr_n117 = (_csr_n116 & _csr_n114);
  assign _csr_n118 = (_csr_n108 & _csr_n117);
  assign EVENT_write_commit = _csr_n118;
  assign _csr_n119 = (_csr_n109 | _csr_n116);
  assign _csr_n120 = (_csr_n119 ? _csr_n3 : _csr_n4);
  assign _csr_n121 = (_csr_n107 ? _csr_n120 : _csr_n100);
  assign _csr_n122 = (_csr_n109 ? _csr_n105 : _csr_n2);
  assign _csr_n123 = (_csr_n107 ? _csr_n122 : _csr_n102);
  assign _csr_n124 = (_csr_n118 ? _csr_n115 : _csr_n2);
  assign _csr_n125 = 4294967295;
  assign _csr_n126 = (_csr_n124 ^ _csr_n125);
  assign _csr_n127 = (_csr_n105 & _csr_n126);
  assign _csr_n128 = (match_bits & _csr_n104);
  assign _csr_n129 = (_csr_n127 | _csr_n128);
  assign _csr_n130 = (_csr_n10 ? _csr_n123 : _csr_data);
  assign _csr_n131 = (_csr_n10 ? _csr_n121 : _csr_error);
  always @(posedge clk) begin
    if (rst) begin
      _csr_pending <= 0;
      _csr_data <= 0;
      _csr_error <= 0;
      _csr_value2 <= 0;
      _csr_value3 <= 0;
    end else begin
      _csr_pending <= _csr_n14;
      _csr_data <= _csr_n130;
      _csr_error <= _csr_n131;
      _csr_value2 <= _csr_n103;
      _csr_value3 <= _csr_n129;
    end
  end
endmodule

module Fr198Timer (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  output match_event
);
  reg [31:0] ctrl_value;
  reg [31:0] count_value;
  wire [31:0] ctrl_candidate;
  wire [31:0] ctrl_write_mask;
  wire ctrl_read_commit;
  wire ctrl_write_commit;
  wire [31:0] count_candidate;
  wire [31:0] count_write_mask;
  wire count_read_commit;
  wire count_write_commit;
  wire [31:0] compare_value;
  wire [31:0] compare_candidate;
  wire [31:0] compare_write_mask;
  wire compare_read_commit;
  wire compare_write_commit;
  wire [31:0] EVENT_value;
  wire [31:0] EVENT_candidate;
  wire [31:0] EVENT_write_mask;
  wire EVENT_read_commit;
  wire EVENT_write_commit;
  wire [31:0] match_bits;
  wire [31:0] zero32;
  wire [31:0] one32;
  wire [31:0] ctrl_keep;
  wire one;
  wire active;
  wire enabled;
  wire periodic;
  wire not_periodic;
  wire write_ab;
  wire config_write;
  wire no_write;
  wire running;
  wire advance;
  wire [31:0] increment;
  wire equal;
  wire restart;
  wire stop;
  wire [31:0] ctrl_stopped;
  wire [31:0] ctrl_auto;
  wire [31:0] ctrl_next;
  wire [31:0] count_auto;
  wire [31:0] count_running;
  wire [31:0] count_next;
  BitloomTimerCsr csr (
    .clk(clk),
    .rst(rst),
    .req_valid(req_valid),
    .write(write),
    .addr(addr),
    .wdata(wdata),
    .wstrb(wstrb),
    .rsp_ready(rsp_ready),
    .req_ready(req_ready),
    .rsp_valid(rsp_valid),
    .rdata(rdata),
    .error(error),
    .ctrl_value(ctrl_value),
    .ctrl_candidate(ctrl_candidate),
    .ctrl_write_mask(ctrl_write_mask),
    .ctrl_read_commit(ctrl_read_commit),
    .ctrl_write_commit(ctrl_write_commit),
    .count_value(count_value),
    .count_candidate(count_candidate),
    .count_write_mask(count_write_mask),
    .count_read_commit(count_read_commit),
    .count_write_commit(count_write_commit),
    .compare_value(compare_value),
    .compare_candidate(compare_candidate),
    .compare_write_mask(compare_write_mask),
    .compare_read_commit(compare_read_commit),
    .compare_write_commit(compare_write_commit),
    .EVENT_value(EVENT_value),
    .EVENT_candidate(EVENT_candidate),
    .EVENT_write_mask(EVENT_write_mask),
    .EVENT_read_commit(EVENT_read_commit),
    .EVENT_write_commit(EVENT_write_commit),
    .match_bits(match_bits)
  );
  assign zero32 = 0;
  assign one32 = 1;
  assign ctrl_keep = 2;
  assign one = 1;
  assign active = (rst ^ one);
  assign enabled = ctrl_value[0:0];
  assign periodic = ctrl_value[1:1];
  assign not_periodic = (periodic ^ one);
  assign write_ab = (ctrl_write_commit | count_write_commit);
  assign config_write = (write_ab | compare_write_commit);
  assign no_write = (config_write ^ one);
  assign running = (enabled & active);
  assign advance = (running & no_write);
  assign increment = count_value + one32;
  assign equal = (increment == compare_value);
  assign match_event = (advance & equal);
  assign match_bits = (match_event ? one32 : zero32);
  assign restart = (match_event & periodic);
  assign stop = (match_event & not_periodic);
  assign ctrl_stopped = (ctrl_value & ctrl_keep);
  assign ctrl_auto = (stop ? ctrl_stopped : ctrl_value);
  assign ctrl_next = (ctrl_write_commit ? ctrl_candidate : ctrl_auto);
  assign count_auto = (restart ? zero32 : increment);
  assign count_running = (advance ? count_auto : count_value);
  assign count_next = (count_write_commit ? count_candidate : count_running);
  always @(posedge clk) begin
    if (rst) begin
      ctrl_value <= 0;
      count_value <= 0;
    end else begin
      ctrl_value <= ctrl_next;
      count_value <= count_next;
    end
  end
endmodule

module BitloomIrqCsr (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  output pending_read_commit,
  output pending_write_commit,
  output [31:0] pending_value,
  output [31:0] pending_candidate,
  output [31:0] pending_write_mask,
  input [31:0] event_bits,
  output enable_read_commit,
  output enable_write_commit,
  output [31:0] enable_value,
  output [31:0] enable_candidate,
  output [31:0] enable_write_mask,
  output test_read_commit,
  output test_write_commit,
  output [31:0] test_candidate,
  output [31:0] test_write_mask,
  output raw_read_commit,
  output raw_write_commit,
  input [31:0] raw_value
);
  reg _csr_pending;
  reg [31:0] _csr_data;
  reg [1:0] _csr_error;
  reg [31:0] _csr_value0;
  reg [31:0] _csr_value1;
  wire _csr_n0;
  wire _csr_n1;
  wire [31:0] _csr_n2;
  wire [1:0] _csr_n3;
  wire [1:0] _csr_n4;
  wire _csr_n5;
  wire _csr_n6;
  wire _csr_n7;
  wire _csr_n8;
  wire _csr_n9;
  wire _csr_n10;
  wire _csr_n11;
  wire _csr_n12;
  wire _csr_n13;
  wire _csr_n14;
  wire _csr_n15;
  wire [31:0] _csr_n16;
  wire [31:0] _csr_n17;
  wire [31:0] _csr_n18;
  wire _csr_n19;
  wire [31:0] _csr_n20;
  wire [31:0] _csr_n21;
  wire [31:0] _csr_n22;
  wire _csr_n23;
  wire [31:0] _csr_n24;
  wire [31:0] _csr_n25;
  wire [31:0] _csr_n26;
  wire _csr_n27;
  wire [31:0] _csr_n28;
  wire [31:0] _csr_n29;
  wire [31:0] _csr_n30;
  wire [31:0] _csr_n31;
  wire [31:0] _csr_n32;
  wire [15:0] _csr_n33;
  wire _csr_n34;
  wire _csr_n35;
  wire _csr_n36;
  wire _csr_n37;
  wire [31:0] _csr_n38;
  wire _csr_n39;
  wire _csr_n40;
  wire _csr_n41;
  wire [31:0] _csr_n42;
  wire _csr_n43;
  wire _csr_n44;
  wire _csr_n45;
  wire _csr_n46;
  wire [1:0] _csr_n47;
  wire [1:0] _csr_n48;
  wire [31:0] _csr_n49;
  wire [31:0] _csr_n50;
  wire [31:0] _csr_n51;
  wire [31:0] _csr_n52;
  wire [31:0] _csr_n53;
  wire [31:0] _csr_n54;
  wire [31:0] _csr_n55;
  wire [31:0] _csr_n56;
  wire [31:0] _csr_n57;
  wire [31:0] _csr_n58;
  wire [15:0] _csr_n59;
  wire _csr_n60;
  wire _csr_n61;
  wire _csr_n62;
  wire _csr_n63;
  wire [31:0] _csr_n64;
  wire _csr_n65;
  wire _csr_n66;
  wire _csr_n67;
  wire [31:0] _csr_n68;
  wire [31:0] _csr_n69;
  wire [31:0] _csr_n70;
  wire [31:0] _csr_n71;
  wire [31:0] _csr_n72;
  wire _csr_n73;
  wire _csr_n74;
  wire _csr_n75;
  wire _csr_n76;
  wire [1:0] _csr_n77;
  wire [1:0] _csr_n78;
  wire [31:0] _csr_n79;
  wire [31:0] _csr_n80;
  wire [31:0] _csr_n81;
  wire [31:0] _csr_n82;
  wire [15:0] _csr_n83;
  wire _csr_n84;
  wire _csr_n85;
  wire _csr_n86;
  wire _csr_n87;
  wire [31:0] _csr_n88;
  wire _csr_n89;
  wire _csr_n90;
  wire _csr_n91;
  wire [31:0] _csr_n92;
  wire _csr_n93;
  wire _csr_n94;
  wire _csr_n95;
  wire _csr_n96;
  wire [1:0] _csr_n97;
  wire [1:0] _csr_n98;
  wire [31:0] _csr_n99;
  wire [31:0] _csr_n100;
  wire [31:0] _csr_n101;
  wire [31:0] _csr_n102;
  wire [15:0] _csr_n103;
  wire _csr_n104;
  wire _csr_n105;
  wire _csr_n106;
  wire _csr_n107;
  wire _csr_n108;
  wire [1:0] _csr_n109;
  wire [1:0] _csr_n110;
  wire [31:0] _csr_n111;
  wire [31:0] _csr_n112;
  wire [31:0] _csr_n113;
  wire [1:0] _csr_n114;
  assign _csr_n0 = 0;
  assign _csr_n1 = 1;
  assign _csr_n2 = 0;
  assign _csr_n3 = 0;
  assign _csr_n4 = 2;
  assign _csr_n5 = 1;
  assign _csr_n6 = (_csr_pending ^ _csr_n5);
  assign _csr_n7 = 1;
  assign _csr_n8 = (rst ^ _csr_n7);
  assign _csr_n9 = (_csr_n6 & _csr_n8);
  assign req_ready = _csr_n9;
  assign _csr_n10 = (req_valid & _csr_n9);
  assign _csr_n11 = 1;
  assign _csr_n12 = (write ^ _csr_n11);
  assign _csr_n13 = (rsp_ready ? _csr_n0 : _csr_pending);
  assign _csr_n14 = (_csr_n10 ? _csr_n1 : _csr_n13);
  assign rsp_valid = _csr_pending;
  assign rdata = _csr_data;
  assign error = _csr_error;
  assign _csr_n15 = wstrb[0:0];
  assign _csr_n16 = 255;
  assign _csr_n17 = (_csr_n15 ? _csr_n16 : _csr_n2);
  assign _csr_n18 = (_csr_n2 | _csr_n17);
  assign _csr_n19 = wstrb[1:1];
  assign _csr_n20 = 65280;
  assign _csr_n21 = (_csr_n19 ? _csr_n20 : _csr_n2);
  assign _csr_n22 = (_csr_n18 | _csr_n21);
  assign _csr_n23 = wstrb[2:2];
  assign _csr_n24 = 16711680;
  assign _csr_n25 = (_csr_n23 ? _csr_n24 : _csr_n2);
  assign _csr_n26 = (_csr_n22 | _csr_n25);
  assign _csr_n27 = wstrb[3:3];
  assign _csr_n28 = 4278190080;
  assign _csr_n29 = (_csr_n27 ? _csr_n28 : _csr_n2);
  assign _csr_n30 = (_csr_n26 | _csr_n29);
  assign _csr_n31 = 31;
  assign _csr_n32 = (_csr_value0 & _csr_n31);
  assign pending_value = _csr_n32;
  assign _csr_n33 = 0;
  assign _csr_n34 = (addr == _csr_n33);
  assign _csr_n35 = (_csr_n10 & _csr_n34);
  assign _csr_n36 = (_csr_n12 & _csr_n1);
  assign _csr_n37 = (_csr_n35 & _csr_n36);
  assign pending_read_commit = _csr_n37;
  assign _csr_n38 = (_csr_n30 & _csr_n31);
  assign _csr_n39 = (_csr_n38 == _csr_n2);
  assign _csr_n40 = 1;
  assign _csr_n41 = (_csr_n39 ^ _csr_n40);
  assign _csr_n42 = (wdata & _csr_n38);
  assign pending_write_mask = _csr_n38;
  assign pending_candidate = _csr_n42;
  assign _csr_n43 = (write & _csr_n1);
  assign _csr_n44 = (_csr_n43 & _csr_n41);
  assign _csr_n45 = (_csr_n35 & _csr_n44);
  assign pending_write_commit = _csr_n45;
  assign _csr_n46 = (_csr_n36 | _csr_n43);
  assign _csr_n47 = (_csr_n46 ? _csr_n3 : _csr_n4);
  assign _csr_n48 = (_csr_n34 ? _csr_n47 : _csr_n4);
  assign _csr_n49 = (_csr_n36 ? _csr_n32 : _csr_n2);
  assign _csr_n50 = (_csr_n34 ? _csr_n49 : _csr_n2);
  assign _csr_n51 = (_csr_n45 ? _csr_n42 : _csr_n2);
  assign _csr_n52 = 4294967295;
  assign _csr_n53 = (_csr_n51 ^ _csr_n52);
  assign _csr_n54 = (_csr_n32 & _csr_n53);
  assign _csr_n55 = (event_bits & _csr_n31);
  assign _csr_n56 = (_csr_n54 | _csr_n55);
  assign _csr_n57 = 31;
  assign _csr_n58 = (_csr_value1 & _csr_n57);
  assign enable_value = _csr_n58;
  assign _csr_n59 = 4;
  assign _csr_n60 = (addr == _csr_n59);
  assign _csr_n61 = (_csr_n10 & _csr_n60);
  assign _csr_n62 = (_csr_n12 & _csr_n1);
  assign _csr_n63 = (_csr_n61 & _csr_n62);
  assign enable_read_commit = _csr_n63;
  assign _csr_n64 = (_csr_n30 & _csr_n57);
  assign _csr_n65 = (_csr_n64 == _csr_n2);
  assign _csr_n66 = 1;
  assign _csr_n67 = (_csr_n65 ^ _csr_n66);
  assign _csr_n68 = (wdata & _csr_n64);
  assign _csr_n69 = 4294967295;
  assign _csr_n70 = (_csr_n64 ^ _csr_n69);
  assign _csr_n71 = (_csr_n58 & _csr_n70);
  assign _csr_n72 = (_csr_n71 | _csr_n68);
  assign enable_write_mask = _csr_n64;
  assign enable_candidate = _csr_n72;
  assign _csr_n73 = (write & _csr_n1);
  assign _csr_n74 = (_csr_n73 & _csr_n67);
  assign _csr_n75 = (_csr_n61 & _csr_n74);
  assign enable_write_commit = _csr_n75;
  assign _csr_n76 = (_csr_n62 | _csr_n73);
  assign _csr_n77 = (_csr_n76 ? _csr_n3 : _csr_n4);
  assign _csr_n78 = (_csr_n60 ? _csr_n77 : _csr_n48);
  assign _csr_n79 = (_csr_n62 ? _csr_n58 : _csr_n2);
  assign _csr_n80 = (_csr_n60 ? _csr_n79 : _csr_n50);
  assign _csr_n81 = (_csr_n75 ? _csr_n72 : _csr_n58);
  assign _csr_n82 = 31;
  assign _csr_n83 = 8;
  assign _csr_n84 = (addr == _csr_n83);
  assign _csr_n85 = (_csr_n10 & _csr_n84);
  assign _csr_n86 = (_csr_n12 & _csr_n0);
  assign _csr_n87 = (_csr_n85 & _csr_n86);
  assign test_read_commit = _csr_n87;
  assign _csr_n88 = (_csr_n30 & _csr_n82);
  assign _csr_n89 = (_csr_n88 == _csr_n2);
  assign _csr_n90 = 1;
  assign _csr_n91 = (_csr_n89 ^ _csr_n90);
  assign _csr_n92 = (wdata & _csr_n88);
  assign test_write_mask = _csr_n88;
  assign test_candidate = _csr_n92;
  assign _csr_n93 = (write & _csr_n1);
  assign _csr_n94 = (_csr_n93 & _csr_n91);
  assign _csr_n95 = (_csr_n85 & _csr_n94);
  assign test_write_commit = _csr_n95;
  assign _csr_n96 = (_csr_n86 | _csr_n93);
  assign _csr_n97 = (_csr_n96 ? _csr_n3 : _csr_n4);
  assign _csr_n98 = (_csr_n84 ? _csr_n97 : _csr_n78);
  assign _csr_n99 = (_csr_n86 ? _csr_n2 : _csr_n2);
  assign _csr_n100 = (_csr_n84 ? _csr_n99 : _csr_n80);
  assign _csr_n101 = 31;
  assign _csr_n102 = (raw_value & _csr_n101);
  assign _csr_n103 = 12;
  assign _csr_n104 = (addr == _csr_n103);
  assign _csr_n105 = (_csr_n10 & _csr_n104);
  assign _csr_n106 = (_csr_n12 & _csr_n1);
  assign _csr_n107 = (_csr_n105 & _csr_n106);
  assign raw_read_commit = _csr_n107;
  assign raw_write_commit = _csr_n0;
  assign _csr_n108 = (_csr_n106 | _csr_n0);
  assign _csr_n109 = (_csr_n108 ? _csr_n3 : _csr_n4);
  assign _csr_n110 = (_csr_n104 ? _csr_n109 : _csr_n98);
  assign _csr_n111 = (_csr_n106 ? _csr_n102 : _csr_n2);
  assign _csr_n112 = (_csr_n104 ? _csr_n111 : _csr_n100);
  assign _csr_n113 = (_csr_n10 ? _csr_n112 : _csr_data);
  assign _csr_n114 = (_csr_n10 ? _csr_n110 : _csr_error);
  always @(posedge clk) begin
    if (rst) begin
      _csr_pending <= 0;
      _csr_data <= 0;
      _csr_error <= 0;
      _csr_value0 <= 0;
      _csr_value1 <= 0;
    end else begin
      _csr_pending <= _csr_n14;
      _csr_data <= _csr_n113;
      _csr_error <= _csr_n114;
      _csr_value0 <= _csr_n56;
      _csr_value1 <= _csr_n81;
    end
  end
endmodule

module Fr198Irq (
  input clk,
  input rst,
  input req_valid,
  input write,
  input [15:0] addr,
  input [31:0] wdata,
  input [3:0] wstrb,
  input rsp_ready,
  output req_ready,
  output rsp_valid,
  output [31:0] rdata,
  output [1:0] error,
  input [4:0] raw_events,
  output irq
);
  wire pending_read_commit;
  wire pending_write_commit;
  wire [31:0] pending_value;
  wire [31:0] pending_candidate;
  wire [31:0] pending_write_mask;
  wire enable_read_commit;
  wire enable_write_commit;
  wire [31:0] enable_value;
  wire [31:0] enable_candidate;
  wire [31:0] enable_write_mask;
  wire test_read_commit;
  wire test_write_commit;
  wire [31:0] test_candidate;
  wire [31:0] test_write_mask;
  wire raw_read_commit;
  wire raw_write_commit;
  wire [31:0] raw_value;
  wire [31:0] event_bits;
  wire [31:0] test_bits;
  wire [31:0] masked;
  wire [31:0] zero32;
  wire [26:0] zero27;
  wire empty;
  wire one;
  BitloomIrqCsr csr (
    .clk(clk),
    .rst(rst),
    .req_valid(req_valid),
    .write(write),
    .addr(addr),
    .wdata(wdata),
    .wstrb(wstrb),
    .rsp_ready(rsp_ready),
    .req_ready(req_ready),
    .rsp_valid(rsp_valid),
    .rdata(rdata),
    .error(error),
    .pending_read_commit(pending_read_commit),
    .pending_write_commit(pending_write_commit),
    .pending_value(pending_value),
    .pending_candidate(pending_candidate),
    .pending_write_mask(pending_write_mask),
    .enable_read_commit(enable_read_commit),
    .enable_write_commit(enable_write_commit),
    .enable_value(enable_value),
    .enable_candidate(enable_candidate),
    .enable_write_mask(enable_write_mask),
    .test_read_commit(test_read_commit),
    .test_write_commit(test_write_commit),
    .test_candidate(test_candidate),
    .test_write_mask(test_write_mask),
    .raw_read_commit(raw_read_commit),
    .raw_write_commit(raw_write_commit),
    .raw_value(raw_value),
    .event_bits(event_bits)
  );
  assign zero32 = 0;
  assign zero27 = 0;
  assign one = 1;
  assign raw_value = {zero27, raw_events};
  assign test_bits = (test_write_commit ? test_candidate : zero32);
  assign event_bits = (raw_value | test_bits);
  assign masked = (pending_value & enable_value);
  assign empty = (masked == zero32);
  assign irq = (empty ^ one);
endmodule

module Fr198AxiCore (
  input clk,
  input rst,
  input [15:0] axi_s_axi_awaddr,
  input [2:0] axi_s_axi_awprot,
  input axi_s_axi_awvalid,
  input [31:0] axi_s_axi_wdata,
  input [3:0] axi_s_axi_wstrb,
  input axi_s_axi_wvalid,
  input axi_s_axi_bready,
  input [15:0] axi_s_axi_araddr,
  input [2:0] axi_s_axi_arprot,
  input axi_s_axi_arvalid,
  input axi_s_axi_rready,
  output axi_s_axi_awready,
  output axi_s_axi_wready,
  output axi_s_axi_arready,
  output axi_s_axi_bvalid,
  output [1:0] axi_s_axi_bresp,
  output axi_s_axi_rvalid,
  output [31:0] axi_s_axi_rdata,
  output [1:0] axi_s_axi_rresp,
  input u_rx,
  output u_tx,
  input [31:0] g_pad_in,
  output [31:0] g_pad_out,
  output [31:0] g_pad_oe,
  output i_irq
);
  wire axi_csr_req_ready;
  wire axi_csr_rsp_valid;
  wire [31:0] axi_csr_rdata;
  wire [1:0] axi_csr_error;
  wire axi_csr_req_valid;
  wire axi_csr_write;
  wire [15:0] axi_csr_addr;
  wire [31:0] axi_csr_wdata;
  wire [3:0] axi_csr_wstrb;
  wire axi_csr_rsp_ready;
  wire csr_req_valid;
  wire csr_write;
  wire [15:0] csr_addr;
  wire [31:0] csr_wdata;
  wire [3:0] csr_wstrb;
  wire csr_rsp_ready;
  wire csr_req_ready;
  wire csr_rsp_valid;
  wire [31:0] csr_rdata;
  wire [1:0] csr_error;
  wire csr_uart_req_ready;
  wire csr_uart_rsp_valid;
  wire [31:0] csr_uart_rdata;
  wire [1:0] csr_uart_error;
  wire csr_uart_req_valid;
  wire csr_uart_write;
  wire [15:0] csr_uart_addr;
  wire [31:0] csr_uart_wdata;
  wire [3:0] csr_uart_wstrb;
  wire csr_uart_rsp_ready;
  wire csr_gpio_req_ready;
  wire csr_gpio_rsp_valid;
  wire [31:0] csr_gpio_rdata;
  wire [1:0] csr_gpio_error;
  wire csr_gpio_req_valid;
  wire csr_gpio_write;
  wire [15:0] csr_gpio_addr;
  wire [31:0] csr_gpio_wdata;
  wire [3:0] csr_gpio_wstrb;
  wire csr_gpio_rsp_ready;
  wire csr_timer_req_ready;
  wire csr_timer_rsp_valid;
  wire [31:0] csr_timer_rdata;
  wire [1:0] csr_timer_error;
  wire csr_timer_req_valid;
  wire csr_timer_write;
  wire [15:0] csr_timer_addr;
  wire [31:0] csr_timer_wdata;
  wire [3:0] csr_timer_wstrb;
  wire csr_timer_rsp_ready;
  wire csr_irq_req_ready;
  wire csr_irq_rsp_valid;
  wire [31:0] csr_irq_rdata;
  wire [1:0] csr_irq_error;
  wire csr_irq_req_valid;
  wire csr_irq_write;
  wire [15:0] csr_irq_addr;
  wire [31:0] csr_irq_wdata;
  wire [3:0] csr_irq_wstrb;
  wire csr_irq_rsp_ready;
  wire u_req_valid;
  wire u_write;
  wire [15:0] u_addr;
  wire [31:0] u_wdata;
  wire [3:0] u_wstrb;
  wire u_rsp_ready;
  wire u_req_ready;
  wire u_rsp_valid;
  wire [31:0] u_rdata;
  wire [1:0] u_error;
  wire [3:0] u_raw_events;
  wire g_req_valid;
  wire g_write;
  wire [15:0] g_addr;
  wire [31:0] g_wdata;
  wire [3:0] g_wstrb;
  wire g_rsp_ready;
  wire g_req_ready;
  wire g_rsp_valid;
  wire [31:0] g_rdata;
  wire [1:0] g_error;
  wire g_raw_event;
  wire t_req_valid;
  wire t_write;
  wire [15:0] t_addr;
  wire [31:0] t_wdata;
  wire [3:0] t_wstrb;
  wire t_rsp_ready;
  wire t_req_ready;
  wire t_rsp_valid;
  wire [31:0] t_rdata;
  wire [1:0] t_error;
  wire t_match_event;
  wire i_req_valid;
  wire i_write;
  wire [15:0] i_addr;
  wire [31:0] i_wdata;
  wire [3:0] i_wstrb;
  wire i_rsp_ready;
  wire i_req_ready;
  wire i_rsp_valid;
  wire [31:0] i_rdata;
  wire [1:0] i_error;
  wire [4:0] i_raw_events;
  wire uart_error_event;
  wire [1:0] irq_mid_a;
  wire [2:0] irq_mid_b;
  wire [3:0] irq_mid_c;
  wire [4:0] irq_events;
  wire u_rx_event;
  wire u_tx_event;
  wire u_err_event;
  wire u_framing_event;
  Fr198Uart uart (
    .clk(clk),
    .rst(rst),
    .req_valid(u_req_valid),
    .write(u_write),
    .addr(u_addr),
    .wdata(u_wdata),
    .wstrb(u_wstrb),
    .rsp_ready(u_rsp_ready),
    .req_ready(u_req_ready),
    .rsp_valid(u_rsp_valid),
    .rdata(u_rdata),
    .error(u_error),
    .rx(u_rx),
    .tx(u_tx),
    .raw_events(u_raw_events)
  );
  Fr198Gpio gpio (
    .clk(clk),
    .rst(rst),
    .req_valid(g_req_valid),
    .write(g_write),
    .addr(g_addr),
    .wdata(g_wdata),
    .wstrb(g_wstrb),
    .rsp_ready(g_rsp_ready),
    .req_ready(g_req_ready),
    .rsp_valid(g_rsp_valid),
    .rdata(g_rdata),
    .error(g_error),
    .pad_in(g_pad_in),
    .pad_out(g_pad_out),
    .pad_oe(g_pad_oe),
    .raw_event(g_raw_event)
  );
  Fr198Timer timer (
    .clk(clk),
    .rst(rst),
    .req_valid(t_req_valid),
    .write(t_write),
    .addr(t_addr),
    .wdata(t_wdata),
    .wstrb(t_wstrb),
    .rsp_ready(t_rsp_ready),
    .req_ready(t_req_ready),
    .rsp_valid(t_rsp_valid),
    .rdata(t_rdata),
    .error(t_error),
    .match_event(t_match_event)
  );
  Fr198Irq irq (
    .clk(clk),
    .rst(rst),
    .req_valid(i_req_valid),
    .write(i_write),
    .addr(i_addr),
    .wdata(i_wdata),
    .wstrb(i_wstrb),
    .rsp_ready(i_rsp_ready),
    .req_ready(i_req_ready),
    .rsp_valid(i_rsp_valid),
    .rdata(i_rdata),
    .error(i_error),
    .raw_events(i_raw_events),
    .irq(i_irq)
  );
  Fr198Bridge bridge (
    .clk(clk),
    .rst(rst),
    .s_axi_awaddr(axi_s_axi_awaddr),
    .s_axi_awprot(axi_s_axi_awprot),
    .s_axi_awvalid(axi_s_axi_awvalid),
    .s_axi_wdata(axi_s_axi_wdata),
    .s_axi_wstrb(axi_s_axi_wstrb),
    .s_axi_wvalid(axi_s_axi_wvalid),
    .s_axi_bready(axi_s_axi_bready),
    .s_axi_araddr(axi_s_axi_araddr),
    .s_axi_arprot(axi_s_axi_arprot),
    .s_axi_arvalid(axi_s_axi_arvalid),
    .s_axi_rready(axi_s_axi_rready),
    .csr_req_ready(axi_csr_req_ready),
    .csr_rsp_valid(axi_csr_rsp_valid),
    .csr_rdata(axi_csr_rdata),
    .csr_error(axi_csr_error),
    .s_axi_awready(axi_s_axi_awready),
    .s_axi_wready(axi_s_axi_wready),
    .s_axi_arready(axi_s_axi_arready),
    .s_axi_bvalid(axi_s_axi_bvalid),
    .s_axi_bresp(axi_s_axi_bresp),
    .s_axi_rvalid(axi_s_axi_rvalid),
    .s_axi_rdata(axi_s_axi_rdata),
    .s_axi_rresp(axi_s_axi_rresp),
    .csr_req_valid(axi_csr_req_valid),
    .csr_write(axi_csr_write),
    .csr_addr(axi_csr_addr),
    .csr_wdata(axi_csr_wdata),
    .csr_wstrb(axi_csr_wstrb),
    .csr_rsp_ready(axi_csr_rsp_ready)
  );
  Fr198Decoder decoder (
    .clk(clk),
    .rst(rst),
    .req_valid(csr_req_valid),
    .write(csr_write),
    .addr(csr_addr),
    .wdata(csr_wdata),
    .wstrb(csr_wstrb),
    .rsp_ready(csr_rsp_ready),
    .req_ready(csr_req_ready),
    .rsp_valid(csr_rsp_valid),
    .rdata(csr_rdata),
    .error(csr_error),
    .uart_req_ready(csr_uart_req_ready),
    .uart_rsp_valid(csr_uart_rsp_valid),
    .uart_rdata(csr_uart_rdata),
    .uart_error(csr_uart_error),
    .uart_req_valid(csr_uart_req_valid),
    .uart_write(csr_uart_write),
    .uart_addr(csr_uart_addr),
    .uart_wdata(csr_uart_wdata),
    .uart_wstrb(csr_uart_wstrb),
    .uart_rsp_ready(csr_uart_rsp_ready),
    .gpio_req_ready(csr_gpio_req_ready),
    .gpio_rsp_valid(csr_gpio_rsp_valid),
    .gpio_rdata(csr_gpio_rdata),
    .gpio_error(csr_gpio_error),
    .gpio_req_valid(csr_gpio_req_valid),
    .gpio_write(csr_gpio_write),
    .gpio_addr(csr_gpio_addr),
    .gpio_wdata(csr_gpio_wdata),
    .gpio_wstrb(csr_gpio_wstrb),
    .gpio_rsp_ready(csr_gpio_rsp_ready),
    .timer_req_ready(csr_timer_req_ready),
    .timer_rsp_valid(csr_timer_rsp_valid),
    .timer_rdata(csr_timer_rdata),
    .timer_error(csr_timer_error),
    .timer_req_valid(csr_timer_req_valid),
    .timer_write(csr_timer_write),
    .timer_addr(csr_timer_addr),
    .timer_wdata(csr_timer_wdata),
    .timer_wstrb(csr_timer_wstrb),
    .timer_rsp_ready(csr_timer_rsp_ready),
    .irq_req_ready(csr_irq_req_ready),
    .irq_rsp_valid(csr_irq_rsp_valid),
    .irq_rdata(csr_irq_rdata),
    .irq_error(csr_irq_error),
    .irq_req_valid(csr_irq_req_valid),
    .irq_write(csr_irq_write),
    .irq_addr(csr_irq_addr),
    .irq_wdata(csr_irq_wdata),
    .irq_wstrb(csr_irq_wstrb),
    .irq_rsp_ready(csr_irq_rsp_ready)
  );
  assign axi_csr_req_ready = csr_req_ready;
  assign axi_csr_rsp_valid = csr_rsp_valid;
  assign axi_csr_rdata = csr_rdata;
  assign axi_csr_error = csr_error;
  assign csr_req_valid = axi_csr_req_valid;
  assign csr_write = axi_csr_write;
  assign csr_addr = axi_csr_addr;
  assign csr_wdata = axi_csr_wdata;
  assign csr_wstrb = axi_csr_wstrb;
  assign csr_rsp_ready = axi_csr_rsp_ready;
  assign u_req_valid = csr_uart_req_valid;
  assign u_write = csr_uart_write;
  assign u_addr = csr_uart_addr;
  assign u_wdata = csr_uart_wdata;
  assign u_wstrb = csr_uart_wstrb;
  assign u_rsp_ready = csr_uart_rsp_ready;
  assign csr_uart_req_ready = u_req_ready;
  assign csr_uart_rsp_valid = u_rsp_valid;
  assign csr_uart_rdata = u_rdata;
  assign csr_uart_error = u_error;
  assign g_req_valid = csr_gpio_req_valid;
  assign g_write = csr_gpio_write;
  assign g_addr = csr_gpio_addr;
  assign g_wdata = csr_gpio_wdata;
  assign g_wstrb = csr_gpio_wstrb;
  assign g_rsp_ready = csr_gpio_rsp_ready;
  assign csr_gpio_req_ready = g_req_ready;
  assign csr_gpio_rsp_valid = g_rsp_valid;
  assign csr_gpio_rdata = g_rdata;
  assign csr_gpio_error = g_error;
  assign t_req_valid = csr_timer_req_valid;
  assign t_write = csr_timer_write;
  assign t_addr = csr_timer_addr;
  assign t_wdata = csr_timer_wdata;
  assign t_wstrb = csr_timer_wstrb;
  assign t_rsp_ready = csr_timer_rsp_ready;
  assign csr_timer_req_ready = t_req_ready;
  assign csr_timer_rsp_valid = t_rsp_valid;
  assign csr_timer_rdata = t_rdata;
  assign csr_timer_error = t_error;
  assign i_req_valid = csr_irq_req_valid;
  assign i_write = csr_irq_write;
  assign i_addr = csr_irq_addr;
  assign i_wdata = csr_irq_wdata;
  assign i_wstrb = csr_irq_wstrb;
  assign i_rsp_ready = csr_irq_rsp_ready;
  assign csr_irq_req_ready = i_req_ready;
  assign csr_irq_rsp_valid = i_rsp_valid;
  assign csr_irq_rdata = i_rdata;
  assign csr_irq_error = i_error;
  assign u_rx_event = u_raw_events[0:0];
  assign u_tx_event = u_raw_events[1:1];
  assign u_err_event = u_raw_events[2:2];
  assign u_framing_event = u_raw_events[3:3];
  assign uart_error_event = (u_err_event | u_framing_event);
  assign irq_mid_a = {u_rx_event, t_match_event};
  assign irq_mid_b = {u_tx_event, irq_mid_a};
  assign irq_mid_c = {uart_error_event, irq_mid_b};
  assign irq_events = {g_raw_event, irq_mid_c};
  assign i_raw_events = irq_events;
endmodule


module Fr198Axi(
 input clk,input aresetn,
 input [15:0] axi_s_axi_awaddr,input [2:0] axi_s_axi_awprot,input axi_s_axi_awvalid,output axi_s_axi_awready,
 input [31:0] axi_s_axi_wdata,input [3:0] axi_s_axi_wstrb,input axi_s_axi_wvalid,output axi_s_axi_wready,
 input axi_s_axi_bready,output axi_s_axi_bvalid,output [1:0] axi_s_axi_bresp,
 input [15:0] axi_s_axi_araddr,input [2:0] axi_s_axi_arprot,input axi_s_axi_arvalid,output axi_s_axi_arready,
 input axi_s_axi_rready,output axi_s_axi_rvalid,output [31:0] axi_s_axi_rdata,output [1:0] axi_s_axi_rresp,
 input u_rx,output u_tx,input [31:0] g_pad_in,output [31:0] g_pad_out,output [31:0] g_pad_oe,output i_irq);
 wire core_reset = ~aresetn;
 Fr198AxiCore core(.clk(clk),.rst(core_reset),.*);
endmodule
