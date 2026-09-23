FIRRTL version 6.0.0
circuit Fr198DirectCore :
  module Fr198Bridge :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input s_axi_awaddr: UInt<16>
    input s_axi_awprot: UInt<3>
    input s_axi_awvalid: UInt<1>
    input s_axi_wdata: UInt<32>
    input s_axi_wstrb: UInt<4>
    input s_axi_wvalid: UInt<1>
    input s_axi_bready: UInt<1>
    input s_axi_araddr: UInt<16>
    input s_axi_arprot: UInt<3>
    input s_axi_arvalid: UInt<1>
    input s_axi_rready: UInt<1>
    input csr_req_ready: UInt<1>
    input csr_rsp_valid: UInt<1>
    input csr_rdata: UInt<32>
    input csr_error: UInt<2>
    output s_axi_awready: UInt<1>
    output s_axi_wready: UInt<1>
    output s_axi_arready: UInt<1>
    output s_axi_bvalid: UInt<1>
    output s_axi_bresp: UInt<2>
    output s_axi_rvalid: UInt<1>
    output s_axi_rdata: UInt<32>
    output s_axi_rresp: UInt<2>
    output csr_req_valid: UInt<1>
    output csr_write: UInt<1>
    output csr_addr: UInt<16>
    output csr_wdata: UInt<32>
    output csr_wstrb: UInt<4>
    output csr_rsp_ready: UInt<1>
    regreset aw_full: UInt<1>, clk, rst, UInt(0)
    regreset w_full: UInt<1>, clk, rst, UInt(0)
    regreset ar_full: UInt<1>, clk, rst, UInt(0)
    regreset aw_addr: UInt<16>, clk, rst, UInt(0)
    regreset w_data: UInt<32>, clk, rst, UInt(0)
    regreset w_strb: UInt<4>, clk, rst, UInt(0)
    regreset ar_addr: UInt<16>, clk, rst, UInt(0)
    regreset offer: UInt<1>, clk, rst, UInt(0)
    regreset offer_write: UInt<1>, clk, rst, UInt(0)
    regreset exec: UInt<1>, clk, rst, UInt(0)
    regreset owner: UInt<1>, clk, rst, UInt(0)
    regreset prefer_write: UInt<1>, clk, rst, UInt(0)
    regreset b_full: UInt<1>, clk, rst, UInt(0)
    regreset r_full: UInt<1>, clk, rst, UInt(0)
    regreset b_error: UInt<2>, clk, rst, UInt(0)
    regreset r_error: UInt<2>, clk, rst, UInt(0)
    regreset r_data: UInt<32>, clk, rst, UInt(0)
    regreset offer_data: UInt<32>, clk, rst, UInt(0)
    regreset offer_strb: UInt<4>, clk, rst, UInt(0)
    wire one: UInt<1>
    wire zero: UInt<1>
    wire aw_take: UInt<1>
    wire w_take: UInt<1>
    wire ar_take: UInt<1>
    wire active: UInt<1>
    wire no_offer: UInt<1>
    wire no_exec: UInt<1>
    wire idle: UInt<1>
    wire b_space: UInt<1>
    wire r_space: UInt<1>
    wire write_complete: UInt<1>
    wire we: UInt<1>
    wire re: UInt<1>
    wire eligible: UInt<1>
    wire select: UInt<1>
    wire no_read: UInt<1>
    wire write_preferred: UInt<1>
    wire choose_write: UInt<1>
    wire commit: UInt<1>
    wire read_offer: UInt<1>
    wire write_commit: UInt<1>
    wire read_commit: UInt<1>
    wire response: UInt<1>
    wire write_response: UInt<1>
    wire read_owner: UInt<1>
    wire read_response: UInt<1>
    wire b_take: UInt<1>
    wire r_take: UInt<1>
    wire aw_captured: UInt<1>
    wire aw_next: UInt<1>
    wire w_captured: UInt<1>
    wire w_next: UInt<1>
    wire ar_captured: UInt<1>
    wire ar_next: UInt<1>
    wire offer_started: UInt<1>
    wire offer_next: UInt<1>
    wire exec_started: UInt<1>
    wire exec_next: UInt<1>
    wire b_consumed: UInt<1>
    wire b_next: UInt<1>
    wire r_consumed: UInt<1>
    wire r_next: UInt<1>
    connect one, UInt(1)
    connect zero, UInt(0)
    connect s_axi_awready, xor(aw_full, one)
    connect aw_take, and(s_axi_awready, s_axi_awvalid)
    connect s_axi_wready, xor(w_full, one)
    connect w_take, and(s_axi_wready, s_axi_wvalid)
    connect s_axi_arready, xor(ar_full, one)
    connect ar_take, and(s_axi_arready, s_axi_arvalid)
    connect active, xor(rst, one)
    connect no_offer, xor(offer, one)
    connect no_exec, xor(exec, one)
    connect idle, and(no_offer, no_exec)
    connect b_space, xor(b_full, one)
    connect r_space, xor(r_full, one)
    connect write_complete, and(aw_full, w_full)
    connect we, and(write_complete, b_space)
    connect re, and(ar_full, r_space)
    connect eligible, or(we, re)
    connect select, and(idle, eligible)
    connect no_read, xor(re, one)
    connect write_preferred, or(prefer_write, no_read)
    connect choose_write, and(we, write_preferred)
    connect csr_req_valid, and(offer, active)
    connect commit, and(csr_req_valid, csr_req_ready)
    connect read_offer, xor(offer_write, one)
    connect write_commit, and(commit, offer_write)
    connect read_commit, and(commit, read_offer)
    connect csr_rsp_ready, and(exec, active)
    connect response, and(csr_rsp_ready, csr_rsp_valid)
    connect write_response, and(response, owner)
    connect read_owner, xor(owner, one)
    connect read_response, and(response, read_owner)
    connect b_take, and(b_full, s_axi_bready)
    connect r_take, and(r_full, s_axi_rready)
    connect aw_captured, mux(aw_take, one, aw_full)
    connect aw_next, mux(write_commit, zero, aw_captured)
    connect w_captured, mux(w_take, one, w_full)
    connect w_next, mux(write_commit, zero, w_captured)
    connect ar_captured, mux(ar_take, one, ar_full)
    connect ar_next, mux(read_commit, zero, ar_captured)
    connect offer_started, mux(select, one, offer)
    connect offer_next, mux(commit, zero, offer_started)
    connect exec_started, mux(commit, one, exec)
    connect exec_next, mux(response, zero, exec_started)
    connect b_consumed, mux(b_take, zero, b_full)
    connect b_next, mux(write_response, one, b_consumed)
    connect r_consumed, mux(r_take, zero, r_full)
    connect r_next, mux(read_response, one, r_consumed)
    connect csr_addr, mux(offer_write, aw_addr, ar_addr)
    connect s_axi_bvalid, b_full
    connect s_axi_rvalid, r_full
    connect s_axi_bresp, b_error
    connect s_axi_rresp, r_error
    connect s_axi_rdata, r_data
    connect csr_write, offer_write
    connect csr_wdata, offer_data
    connect csr_wstrb, offer_strb
    connect aw_full, aw_next
    connect w_full, w_next
    connect ar_full, ar_next
    connect offer, offer_next
    connect exec, exec_next
    connect b_full, b_next
    connect r_full, r_next
    connect aw_addr, mux(aw_take, s_axi_awaddr, aw_addr)
    connect w_data, mux(w_take, s_axi_wdata, w_data)
    connect w_strb, mux(w_take, s_axi_wstrb, w_strb)
    connect ar_addr, mux(ar_take, s_axi_araddr, ar_addr)
    connect offer_write, mux(select, choose_write, offer_write)
    connect offer_data, mux(select, w_data, offer_data)
    connect offer_strb, mux(select, w_strb, offer_strb)
    connect owner, mux(commit, offer_write, owner)
    connect prefer_write, mux(commit, read_offer, prefer_write)
    connect b_error, mux(write_response, csr_error, b_error)
    connect r_error, mux(read_response, csr_error, r_error)
    connect r_data, mux(read_response, csr_rdata, r_data)
  module Fr198Decoder :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    input uart_req_ready: UInt<1>
    input uart_rsp_valid: UInt<1>
    input uart_rdata: UInt<32>
    input uart_error: UInt<2>
    output uart_req_valid: UInt<1>
    output uart_write: UInt<1>
    output uart_addr: UInt<16>
    output uart_wdata: UInt<32>
    output uart_wstrb: UInt<4>
    output uart_rsp_ready: UInt<1>
    input gpio_req_ready: UInt<1>
    input gpio_rsp_valid: UInt<1>
    input gpio_rdata: UInt<32>
    input gpio_error: UInt<2>
    output gpio_req_valid: UInt<1>
    output gpio_write: UInt<1>
    output gpio_addr: UInt<16>
    output gpio_wdata: UInt<32>
    output gpio_wstrb: UInt<4>
    output gpio_rsp_ready: UInt<1>
    input timer_req_ready: UInt<1>
    input timer_rsp_valid: UInt<1>
    input timer_rdata: UInt<32>
    input timer_error: UInt<2>
    output timer_req_valid: UInt<1>
    output timer_write: UInt<1>
    output timer_addr: UInt<16>
    output timer_wdata: UInt<32>
    output timer_wstrb: UInt<4>
    output timer_rsp_ready: UInt<1>
    input irq_req_ready: UInt<1>
    input irq_rsp_valid: UInt<1>
    input irq_rdata: UInt<32>
    input irq_error: UInt<2>
    output irq_req_valid: UInt<1>
    output irq_write: UInt<1>
    output irq_addr: UInt<16>
    output irq_wdata: UInt<32>
    output irq_wstrb: UInt<4>
    output irq_rsp_ready: UInt<1>
    regreset busy: UInt<1>, clk, rst, UInt(0)
    regreset miss: UInt<1>, clk, rst, UInt(0)
    regreset owner: UInt<2>, clk, rst, UInt(0)
    wire one: UInt<1>
    wire zero: UInt<1>
    wire zero32: UInt<32>
    wire zero2: UInt<2>
    wire decerr: UInt<2>
    wire page: UInt<8>
    wire select_owner: UInt<2>
    wire active: UInt<1>
    wire idle: UInt<1>
    wire available: UInt<1>
    wire offering: UInt<1>
    wire commit: UInt<1>
    wire consume: UInt<1>
    wire busy_started: UInt<1>
    wire busy_next: UInt<1>
    wire not_miss: UInt<1>
    wire leaf_owned: UInt<1>
    wire ready_active: UInt<1>
    wire miss_valid: UInt<1>
    wire hit_0: UInt<1>
    wire ready_0: UInt<1>
    wire valid_0: UInt<1>
    wire data_0: UInt<32>
    wire error_0: UInt<2>
    wire hit_1: UInt<1>
    wire ready_1: UInt<1>
    wire valid_1: UInt<1>
    wire data_1: UInt<32>
    wire error_1: UInt<2>
    wire hit_2: UInt<1>
    wire ready_2: UInt<1>
    wire valid_2: UInt<1>
    wire data_2: UInt<32>
    wire error_2: UInt<2>
    wire hit_3: UInt<1>
    wire ready_3: UInt<1>
    wire valid_3: UInt<1>
    wire data_3: UInt<32>
    wire error_3: UInt<2>
    wire hit_4: UInt<1>
    wire ready_4: UInt<1>
    wire valid_4: UInt<1>
    wire data_4: UInt<32>
    wire error_4: UInt<2>
    wire no_hit: UInt<1>
    wire uart_page: UInt<8>
    wire uart_base: UInt<16>
    wire uart_id: UInt<2>
    wire uart_hit: UInt<1>
    wire uart_owner_eq: UInt<1>
    wire uart_owned: UInt<1>
    wire gpio_page: UInt<8>
    wire gpio_base: UInt<16>
    wire gpio_id: UInt<2>
    wire gpio_hit: UInt<1>
    wire gpio_owner_eq: UInt<1>
    wire gpio_owned: UInt<1>
    wire timer_page: UInt<8>
    wire timer_base: UInt<16>
    wire timer_id: UInt<2>
    wire timer_hit: UInt<1>
    wire timer_owner_eq: UInt<1>
    wire timer_owned: UInt<1>
    wire irq_page: UInt<8>
    wire irq_base: UInt<16>
    wire irq_id: UInt<2>
    wire irq_hit: UInt<1>
    wire irq_owner_eq: UInt<1>
    wire irq_owned: UInt<1>
    connect one, UInt(1)
    connect zero, UInt(0)
    connect zero32, UInt(0)
    connect zero2, UInt(0)
    connect decerr, UInt(3)
    connect page, bits(addr, 15, 8)
    connect select_owner, bits(addr, 9, 8)
    connect active, xor(rst, one)
    connect idle, xor(busy, one)
    connect available, and(idle, active)
    connect offering, and(available, req_valid)
    connect not_miss, xor(miss, one)
    connect leaf_owned, and(busy, not_miss)
    connect ready_active, and(rsp_ready, active)
    connect miss_valid, and(busy, miss)
    connect hit_0, UInt(0)
    connect ready_0, UInt(1)
    connect valid_0, miss_valid
    connect data_0, zero32
    connect error_0, mux(miss_valid, decerr, zero2)
    connect uart_page, UInt(0)
    connect uart_base, UInt(0)
    connect uart_id, UInt(0)
    connect uart_hit, eq(page, uart_page)
    connect hit_1, or(hit_0, uart_hit)
    connect ready_1, mux(uart_hit, uart_req_ready, ready_0)
    connect uart_req_valid, and(offering, uart_hit)
    connect uart_addr, sub(addr, uart_base)
    connect uart_write, write
    connect uart_wdata, wdata
    connect uart_wstrb, wstrb
    connect uart_owner_eq, eq(owner, uart_id)
    connect uart_owned, and(leaf_owned, uart_owner_eq)
    connect uart_rsp_ready, and(uart_owned, ready_active)
    connect valid_1, mux(uart_owned, uart_rsp_valid, valid_0)
    connect data_1, mux(uart_owned, uart_rdata, data_0)
    connect error_1, mux(uart_owned, uart_error, error_0)
    connect gpio_page, UInt(1)
    connect gpio_base, UInt(256)
    connect gpio_id, UInt(1)
    connect gpio_hit, eq(page, gpio_page)
    connect hit_2, or(hit_1, gpio_hit)
    connect ready_2, mux(gpio_hit, gpio_req_ready, ready_1)
    connect gpio_req_valid, and(offering, gpio_hit)
    connect gpio_addr, sub(addr, gpio_base)
    connect gpio_write, write
    connect gpio_wdata, wdata
    connect gpio_wstrb, wstrb
    connect gpio_owner_eq, eq(owner, gpio_id)
    connect gpio_owned, and(leaf_owned, gpio_owner_eq)
    connect gpio_rsp_ready, and(gpio_owned, ready_active)
    connect valid_2, mux(gpio_owned, gpio_rsp_valid, valid_1)
    connect data_2, mux(gpio_owned, gpio_rdata, data_1)
    connect error_2, mux(gpio_owned, gpio_error, error_1)
    connect timer_page, UInt(2)
    connect timer_base, UInt(512)
    connect timer_id, UInt(2)
    connect timer_hit, eq(page, timer_page)
    connect hit_3, or(hit_2, timer_hit)
    connect ready_3, mux(timer_hit, timer_req_ready, ready_2)
    connect timer_req_valid, and(offering, timer_hit)
    connect timer_addr, sub(addr, timer_base)
    connect timer_write, write
    connect timer_wdata, wdata
    connect timer_wstrb, wstrb
    connect timer_owner_eq, eq(owner, timer_id)
    connect timer_owned, and(leaf_owned, timer_owner_eq)
    connect timer_rsp_ready, and(timer_owned, ready_active)
    connect valid_3, mux(timer_owned, timer_rsp_valid, valid_2)
    connect data_3, mux(timer_owned, timer_rdata, data_2)
    connect error_3, mux(timer_owned, timer_error, error_2)
    connect irq_page, UInt(3)
    connect irq_base, UInt(768)
    connect irq_id, UInt(3)
    connect irq_hit, eq(page, irq_page)
    connect hit_4, or(hit_3, irq_hit)
    connect ready_4, mux(irq_hit, irq_req_ready, ready_3)
    connect irq_req_valid, and(offering, irq_hit)
    connect irq_addr, sub(addr, irq_base)
    connect irq_write, write
    connect irq_wdata, wdata
    connect irq_wstrb, wstrb
    connect irq_owner_eq, eq(owner, irq_id)
    connect irq_owned, and(leaf_owned, irq_owner_eq)
    connect irq_rsp_ready, and(irq_owned, ready_active)
    connect valid_4, mux(irq_owned, irq_rsp_valid, valid_3)
    connect data_4, mux(irq_owned, irq_rdata, data_3)
    connect error_4, mux(irq_owned, irq_error, error_3)
    connect no_hit, xor(hit_4, one)
    connect req_ready, and(available, ready_4)
    connect rsp_valid, valid_4
    connect rdata, data_4
    connect error, error_4
    connect commit, and(req_valid, req_ready)
    connect consume, and(rsp_valid, ready_active)
    connect busy_started, or(busy, commit)
    connect busy_next, mux(consume, zero, busy_started)
    connect busy, busy_next
    connect miss, mux(commit, no_hit, miss)
    connect owner, mux(commit, select_owner, owner)
  module BitloomUartCsrRegisters :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    output ctrl_read_commit: UInt<1>
    output ctrl_write_commit: UInt<1>
    output ctrl_value: UInt<32>
    output ctrl_candidate: UInt<32>
    output ctrl_write_mask: UInt<32>
    input ctrl_write_reject: UInt<1>
    output baud_div_read_commit: UInt<1>
    output baud_div_write_commit: UInt<1>
    output baud_div_value: UInt<32>
    output baud_div_candidate: UInt<32>
    output baud_div_write_mask: UInt<32>
    input baud_div_write_reject: UInt<1>
    output status_read_commit: UInt<1>
    output status_write_commit: UInt<1>
    input status_value: UInt<32>
    output tx_data_read_commit: UInt<1>
    output tx_data_write_commit: UInt<1>
    output tx_data_candidate: UInt<32>
    output tx_data_write_mask: UInt<32>
    input tx_data_write_reject: UInt<1>
    output rx_data_read_commit: UInt<1>
    output rx_data_write_commit: UInt<1>
    input rx_data_value: UInt<32>
    input rx_data_read_reject: UInt<1>
    output EVENT_read_commit: UInt<1>
    output EVENT_write_commit: UInt<1>
    output EVENT_value: UInt<32>
    output EVENT_candidate: UInt<32>
    output EVENT_write_mask: UInt<32>
    input event_bits: UInt<32>
    regreset _csr_pending: UInt<1>, clk, rst, UInt(0)
    regreset _csr_data: UInt<32>, clk, rst, UInt(0)
    regreset _csr_error: UInt<2>, clk, rst, UInt(0)
    regreset _csr_value0: UInt<32>, clk, rst, UInt(0)
    regreset _csr_value1: UInt<32>, clk, rst, UInt(0)
    regreset _csr_value5: UInt<32>, clk, rst, UInt(0)
    wire _csr_n0: UInt<1>
    wire _csr_n1: UInt<1>
    wire _csr_n2: UInt<32>
    wire _csr_n3: UInt<2>
    wire _csr_n4: UInt<2>
    wire _csr_n5: UInt<1>
    wire _csr_n6: UInt<1>
    wire _csr_n7: UInt<1>
    wire _csr_n8: UInt<1>
    wire _csr_n9: UInt<1>
    wire _csr_n10: UInt<1>
    wire _csr_n11: UInt<1>
    wire _csr_n12: UInt<1>
    wire _csr_n13: UInt<1>
    wire _csr_n14: UInt<1>
    wire _csr_n15: UInt<1>
    wire _csr_n16: UInt<32>
    wire _csr_n17: UInt<32>
    wire _csr_n18: UInt<32>
    wire _csr_n19: UInt<1>
    wire _csr_n20: UInt<32>
    wire _csr_n21: UInt<32>
    wire _csr_n22: UInt<32>
    wire _csr_n23: UInt<1>
    wire _csr_n24: UInt<32>
    wire _csr_n25: UInt<32>
    wire _csr_n26: UInt<32>
    wire _csr_n27: UInt<1>
    wire _csr_n28: UInt<32>
    wire _csr_n29: UInt<32>
    wire _csr_n30: UInt<32>
    wire _csr_n31: UInt<32>
    wire _csr_n32: UInt<32>
    wire _csr_n33: UInt<16>
    wire _csr_n34: UInt<1>
    wire _csr_n35: UInt<1>
    wire _csr_n36: UInt<1>
    wire _csr_n37: UInt<1>
    wire _csr_n38: UInt<32>
    wire _csr_n39: UInt<1>
    wire _csr_n40: UInt<1>
    wire _csr_n41: UInt<1>
    wire _csr_n42: UInt<32>
    wire _csr_n43: UInt<32>
    wire _csr_n44: UInt<32>
    wire _csr_n45: UInt<32>
    wire _csr_n46: UInt<32>
    wire _csr_n47: UInt<1>
    wire _csr_n48: UInt<1>
    wire _csr_n49: UInt<1>
    wire _csr_n50: UInt<1>
    wire _csr_n51: UInt<1>
    wire _csr_n52: UInt<1>
    wire _csr_n53: UInt<1>
    wire _csr_n54: UInt<2>
    wire _csr_n55: UInt<2>
    wire _csr_n56: UInt<32>
    wire _csr_n57: UInt<32>
    wire _csr_n58: UInt<32>
    wire _csr_n59: UInt<32>
    wire _csr_n60: UInt<32>
    wire _csr_n61: UInt<16>
    wire _csr_n62: UInt<1>
    wire _csr_n63: UInt<1>
    wire _csr_n64: UInt<1>
    wire _csr_n65: UInt<1>
    wire _csr_n66: UInt<32>
    wire _csr_n67: UInt<1>
    wire _csr_n68: UInt<1>
    wire _csr_n69: UInt<1>
    wire _csr_n70: UInt<32>
    wire _csr_n71: UInt<32>
    wire _csr_n72: UInt<32>
    wire _csr_n73: UInt<32>
    wire _csr_n74: UInt<32>
    wire _csr_n75: UInt<1>
    wire _csr_n76: UInt<1>
    wire _csr_n77: UInt<1>
    wire _csr_n78: UInt<1>
    wire _csr_n79: UInt<1>
    wire _csr_n80: UInt<1>
    wire _csr_n81: UInt<1>
    wire _csr_n82: UInt<2>
    wire _csr_n83: UInt<2>
    wire _csr_n84: UInt<32>
    wire _csr_n85: UInt<32>
    wire _csr_n86: UInt<32>
    wire _csr_n87: UInt<32>
    wire _csr_n88: UInt<32>
    wire _csr_n89: UInt<16>
    wire _csr_n90: UInt<1>
    wire _csr_n91: UInt<1>
    wire _csr_n92: UInt<1>
    wire _csr_n93: UInt<1>
    wire _csr_n94: UInt<1>
    wire _csr_n95: UInt<2>
    wire _csr_n96: UInt<2>
    wire _csr_n97: UInt<32>
    wire _csr_n98: UInt<32>
    wire _csr_n99: UInt<32>
    wire _csr_n100: UInt<16>
    wire _csr_n101: UInt<1>
    wire _csr_n102: UInt<1>
    wire _csr_n103: UInt<1>
    wire _csr_n104: UInt<1>
    wire _csr_n105: UInt<32>
    wire _csr_n106: UInt<1>
    wire _csr_n107: UInt<1>
    wire _csr_n108: UInt<1>
    wire _csr_n109: UInt<32>
    wire _csr_n110: UInt<1>
    wire _csr_n111: UInt<1>
    wire _csr_n112: UInt<1>
    wire _csr_n113: UInt<1>
    wire _csr_n114: UInt<1>
    wire _csr_n115: UInt<1>
    wire _csr_n116: UInt<1>
    wire _csr_n117: UInt<2>
    wire _csr_n118: UInt<2>
    wire _csr_n119: UInt<32>
    wire _csr_n120: UInt<32>
    wire _csr_n121: UInt<32>
    wire _csr_n122: UInt<32>
    wire _csr_n123: UInt<16>
    wire _csr_n124: UInt<1>
    wire _csr_n125: UInt<1>
    wire _csr_n126: UInt<1>
    wire _csr_n127: UInt<1>
    wire _csr_n128: UInt<1>
    wire _csr_n129: UInt<1>
    wire _csr_n130: UInt<1>
    wire _csr_n131: UInt<2>
    wire _csr_n132: UInt<2>
    wire _csr_n133: UInt<32>
    wire _csr_n134: UInt<32>
    wire _csr_n135: UInt<32>
    wire _csr_n136: UInt<32>
    wire _csr_n137: UInt<16>
    wire _csr_n138: UInt<1>
    wire _csr_n139: UInt<1>
    wire _csr_n140: UInt<1>
    wire _csr_n141: UInt<1>
    wire _csr_n142: UInt<32>
    wire _csr_n143: UInt<1>
    wire _csr_n144: UInt<1>
    wire _csr_n145: UInt<1>
    wire _csr_n146: UInt<32>
    wire _csr_n147: UInt<1>
    wire _csr_n148: UInt<1>
    wire _csr_n149: UInt<1>
    wire _csr_n150: UInt<1>
    wire _csr_n151: UInt<2>
    wire _csr_n152: UInt<2>
    wire _csr_n153: UInt<32>
    wire _csr_n154: UInt<32>
    wire _csr_n155: UInt<32>
    wire _csr_n156: UInt<32>
    wire _csr_n157: UInt<32>
    wire _csr_n158: UInt<32>
    wire _csr_n159: UInt<32>
    wire _csr_n160: UInt<32>
    wire _csr_n161: UInt<32>
    wire _csr_n162: UInt<2>
    connect _csr_n0, UInt(0)
    connect _csr_n1, UInt(1)
    connect _csr_n2, UInt(0)
    connect _csr_n3, UInt(0)
    connect _csr_n4, UInt(2)
    connect _csr_n5, UInt(1)
    connect _csr_n6, xor(_csr_pending, _csr_n5)
    connect _csr_n7, UInt(1)
    connect _csr_n8, xor(rst, _csr_n7)
    connect _csr_n9, and(_csr_n6, _csr_n8)
    connect req_ready, _csr_n9
    connect _csr_n10, and(req_valid, _csr_n9)
    connect _csr_n11, UInt(1)
    connect _csr_n12, xor(write, _csr_n11)
    connect _csr_n13, mux(rsp_ready, _csr_n0, _csr_pending)
    connect _csr_n14, mux(_csr_n10, _csr_n1, _csr_n13)
    connect rsp_valid, _csr_pending
    connect rdata, _csr_data
    connect error, _csr_error
    connect _csr_n15, bits(wstrb, 0, 0)
    connect _csr_n16, UInt(255)
    connect _csr_n17, mux(_csr_n15, _csr_n16, _csr_n2)
    connect _csr_n18, or(_csr_n2, _csr_n17)
    connect _csr_n19, bits(wstrb, 1, 1)
    connect _csr_n20, UInt(65280)
    connect _csr_n21, mux(_csr_n19, _csr_n20, _csr_n2)
    connect _csr_n22, or(_csr_n18, _csr_n21)
    connect _csr_n23, bits(wstrb, 2, 2)
    connect _csr_n24, UInt(16711680)
    connect _csr_n25, mux(_csr_n23, _csr_n24, _csr_n2)
    connect _csr_n26, or(_csr_n22, _csr_n25)
    connect _csr_n27, bits(wstrb, 3, 3)
    connect _csr_n28, UInt(4278190080)
    connect _csr_n29, mux(_csr_n27, _csr_n28, _csr_n2)
    connect _csr_n30, or(_csr_n26, _csr_n29)
    connect _csr_n31, UInt(1)
    connect _csr_n32, and(_csr_value0, _csr_n31)
    connect ctrl_value, _csr_n32
    connect _csr_n33, UInt(0)
    connect _csr_n34, eq(addr, _csr_n33)
    connect _csr_n35, and(_csr_n10, _csr_n34)
    connect _csr_n36, and(_csr_n12, _csr_n1)
    connect _csr_n37, and(_csr_n35, _csr_n36)
    connect ctrl_read_commit, _csr_n37
    connect _csr_n38, and(_csr_n30, _csr_n31)
    connect _csr_n39, eq(_csr_n38, _csr_n2)
    connect _csr_n40, UInt(1)
    connect _csr_n41, xor(_csr_n39, _csr_n40)
    connect _csr_n42, and(wdata, _csr_n38)
    connect _csr_n43, UInt(4294967295)
    connect _csr_n44, xor(_csr_n38, _csr_n43)
    connect _csr_n45, and(_csr_n32, _csr_n44)
    connect _csr_n46, or(_csr_n45, _csr_n42)
    connect ctrl_write_mask, _csr_n38
    connect ctrl_candidate, _csr_n46
    connect _csr_n47, UInt(1)
    connect _csr_n48, xor(ctrl_write_reject, _csr_n47)
    connect _csr_n49, or(_csr_n39, _csr_n48)
    connect _csr_n50, and(write, _csr_n49)
    connect _csr_n51, and(_csr_n50, _csr_n41)
    connect _csr_n52, and(_csr_n35, _csr_n51)
    connect ctrl_write_commit, _csr_n52
    connect _csr_n53, or(_csr_n36, _csr_n50)
    connect _csr_n54, mux(_csr_n53, _csr_n3, _csr_n4)
    connect _csr_n55, mux(_csr_n34, _csr_n54, _csr_n4)
    connect _csr_n56, mux(_csr_n36, _csr_n32, _csr_n2)
    connect _csr_n57, mux(_csr_n34, _csr_n56, _csr_n2)
    connect _csr_n58, mux(_csr_n52, _csr_n46, _csr_n32)
    connect _csr_n59, UInt(4294967295)
    connect _csr_n60, and(_csr_value1, _csr_n59)
    connect baud_div_value, _csr_n60
    connect _csr_n61, UInt(4)
    connect _csr_n62, eq(addr, _csr_n61)
    connect _csr_n63, and(_csr_n10, _csr_n62)
    connect _csr_n64, and(_csr_n12, _csr_n1)
    connect _csr_n65, and(_csr_n63, _csr_n64)
    connect baud_div_read_commit, _csr_n65
    connect _csr_n66, and(_csr_n30, _csr_n59)
    connect _csr_n67, eq(_csr_n66, _csr_n2)
    connect _csr_n68, UInt(1)
    connect _csr_n69, xor(_csr_n67, _csr_n68)
    connect _csr_n70, and(wdata, _csr_n66)
    connect _csr_n71, UInt(4294967295)
    connect _csr_n72, xor(_csr_n66, _csr_n71)
    connect _csr_n73, and(_csr_n60, _csr_n72)
    connect _csr_n74, or(_csr_n73, _csr_n70)
    connect baud_div_write_mask, _csr_n66
    connect baud_div_candidate, _csr_n74
    connect _csr_n75, UInt(1)
    connect _csr_n76, xor(baud_div_write_reject, _csr_n75)
    connect _csr_n77, or(_csr_n67, _csr_n76)
    connect _csr_n78, and(write, _csr_n77)
    connect _csr_n79, and(_csr_n78, _csr_n69)
    connect _csr_n80, and(_csr_n63, _csr_n79)
    connect baud_div_write_commit, _csr_n80
    connect _csr_n81, or(_csr_n64, _csr_n78)
    connect _csr_n82, mux(_csr_n81, _csr_n3, _csr_n4)
    connect _csr_n83, mux(_csr_n62, _csr_n82, _csr_n55)
    connect _csr_n84, mux(_csr_n64, _csr_n60, _csr_n2)
    connect _csr_n85, mux(_csr_n62, _csr_n84, _csr_n57)
    connect _csr_n86, mux(_csr_n80, _csr_n74, _csr_n60)
    connect _csr_n87, UInt(15)
    connect _csr_n88, and(status_value, _csr_n87)
    connect _csr_n89, UInt(8)
    connect _csr_n90, eq(addr, _csr_n89)
    connect _csr_n91, and(_csr_n10, _csr_n90)
    connect _csr_n92, and(_csr_n12, _csr_n1)
    connect _csr_n93, and(_csr_n91, _csr_n92)
    connect status_read_commit, _csr_n93
    connect status_write_commit, _csr_n0
    connect _csr_n94, or(_csr_n92, _csr_n0)
    connect _csr_n95, mux(_csr_n94, _csr_n3, _csr_n4)
    connect _csr_n96, mux(_csr_n90, _csr_n95, _csr_n83)
    connect _csr_n97, mux(_csr_n92, _csr_n88, _csr_n2)
    connect _csr_n98, mux(_csr_n90, _csr_n97, _csr_n85)
    connect _csr_n99, UInt(255)
    connect _csr_n100, UInt(12)
    connect _csr_n101, eq(addr, _csr_n100)
    connect _csr_n102, and(_csr_n10, _csr_n101)
    connect _csr_n103, and(_csr_n12, _csr_n0)
    connect _csr_n104, and(_csr_n102, _csr_n103)
    connect tx_data_read_commit, _csr_n104
    connect _csr_n105, and(_csr_n30, _csr_n99)
    connect _csr_n106, eq(_csr_n105, _csr_n2)
    connect _csr_n107, UInt(1)
    connect _csr_n108, xor(_csr_n106, _csr_n107)
    connect _csr_n109, and(wdata, _csr_n105)
    connect tx_data_write_mask, _csr_n105
    connect tx_data_candidate, _csr_n109
    connect _csr_n110, UInt(1)
    connect _csr_n111, xor(tx_data_write_reject, _csr_n110)
    connect _csr_n112, or(_csr_n106, _csr_n111)
    connect _csr_n113, and(write, _csr_n112)
    connect _csr_n114, and(_csr_n113, _csr_n108)
    connect _csr_n115, and(_csr_n102, _csr_n114)
    connect tx_data_write_commit, _csr_n115
    connect _csr_n116, or(_csr_n103, _csr_n113)
    connect _csr_n117, mux(_csr_n116, _csr_n3, _csr_n4)
    connect _csr_n118, mux(_csr_n101, _csr_n117, _csr_n96)
    connect _csr_n119, mux(_csr_n103, _csr_n2, _csr_n2)
    connect _csr_n120, mux(_csr_n101, _csr_n119, _csr_n98)
    connect _csr_n121, UInt(255)
    connect _csr_n122, and(rx_data_value, _csr_n121)
    connect _csr_n123, UInt(16)
    connect _csr_n124, eq(addr, _csr_n123)
    connect _csr_n125, and(_csr_n10, _csr_n124)
    connect _csr_n126, UInt(1)
    connect _csr_n127, xor(rx_data_read_reject, _csr_n126)
    connect _csr_n128, and(_csr_n12, _csr_n127)
    connect _csr_n129, and(_csr_n125, _csr_n128)
    connect rx_data_read_commit, _csr_n129
    connect rx_data_write_commit, _csr_n0
    connect _csr_n130, or(_csr_n128, _csr_n0)
    connect _csr_n131, mux(_csr_n130, _csr_n3, _csr_n4)
    connect _csr_n132, mux(_csr_n124, _csr_n131, _csr_n118)
    connect _csr_n133, mux(_csr_n128, _csr_n122, _csr_n2)
    connect _csr_n134, mux(_csr_n124, _csr_n133, _csr_n120)
    connect _csr_n135, UInt(15)
    connect _csr_n136, and(_csr_value5, _csr_n135)
    connect EVENT_value, _csr_n136
    connect _csr_n137, UInt(20)
    connect _csr_n138, eq(addr, _csr_n137)
    connect _csr_n139, and(_csr_n10, _csr_n138)
    connect _csr_n140, and(_csr_n12, _csr_n1)
    connect _csr_n141, and(_csr_n139, _csr_n140)
    connect EVENT_read_commit, _csr_n141
    connect _csr_n142, and(_csr_n30, _csr_n135)
    connect _csr_n143, eq(_csr_n142, _csr_n2)
    connect _csr_n144, UInt(1)
    connect _csr_n145, xor(_csr_n143, _csr_n144)
    connect _csr_n146, and(wdata, _csr_n142)
    connect EVENT_write_mask, _csr_n142
    connect EVENT_candidate, _csr_n146
    connect _csr_n147, and(write, _csr_n1)
    connect _csr_n148, and(_csr_n147, _csr_n145)
    connect _csr_n149, and(_csr_n139, _csr_n148)
    connect EVENT_write_commit, _csr_n149
    connect _csr_n150, or(_csr_n140, _csr_n147)
    connect _csr_n151, mux(_csr_n150, _csr_n3, _csr_n4)
    connect _csr_n152, mux(_csr_n138, _csr_n151, _csr_n132)
    connect _csr_n153, mux(_csr_n140, _csr_n136, _csr_n2)
    connect _csr_n154, mux(_csr_n138, _csr_n153, _csr_n134)
    connect _csr_n155, mux(_csr_n149, _csr_n146, _csr_n2)
    connect _csr_n156, UInt(4294967295)
    connect _csr_n157, xor(_csr_n155, _csr_n156)
    connect _csr_n158, and(_csr_n136, _csr_n157)
    connect _csr_n159, and(event_bits, _csr_n135)
    connect _csr_n160, or(_csr_n158, _csr_n159)
    connect _csr_n161, mux(_csr_n10, _csr_n154, _csr_data)
    connect _csr_n162, mux(_csr_n10, _csr_n152, _csr_error)
    connect _csr_pending, _csr_n14
    connect _csr_data, _csr_n161
    connect _csr_error, _csr_n162
    connect _csr_value0, _csr_n58
    connect _csr_value1, _csr_n86
    connect _csr_value5, _csr_n160
  module BitloomUartCsrFifo8x4 :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input flush: UInt<1>
    input input_valid: UInt<1>
    input output_ready: UInt<1>
    input input_data: UInt<8>
    output input_ready: UInt<1>
    output output_valid: UInt<1>
    output output_data: UInt<8>
    regreset count: UInt<3>, clk, rst, UInt(0)
    wire zero: UInt<3>
    wire one: UInt<3>
    wire capacity: UInt<3>
    wire decremented: UInt<3>
    wire after_pop: UInt<3>
    wire incremented: UInt<3>
    wire normal_count: UInt<3>
    wire next_count: UInt<3>
    wire true_bit: UInt<1>
    wire empty: UInt<1>
    wire full: UInt<1>
    wire push: UInt<1>
    wire pop: UInt<1>
    regreset slot_0: UInt<8>, clk, rst, UInt(0)
    wire index_0: UInt<3>
    wire at_0: UInt<1>
    wire write_0: UInt<1>
    wire shift_0: UInt<8>
    wire next_0: UInt<8>
    regreset slot_1: UInt<8>, clk, rst, UInt(0)
    wire index_1: UInt<3>
    wire at_1: UInt<1>
    wire write_1: UInt<1>
    wire shift_1: UInt<8>
    wire next_1: UInt<8>
    regreset slot_2: UInt<8>, clk, rst, UInt(0)
    wire index_2: UInt<3>
    wire at_2: UInt<1>
    wire write_2: UInt<1>
    wire shift_2: UInt<8>
    wire next_2: UInt<8>
    regreset slot_3: UInt<8>, clk, rst, UInt(0)
    wire index_3: UInt<3>
    wire at_3: UInt<1>
    wire write_3: UInt<1>
    wire shift_3: UInt<8>
    wire next_3: UInt<8>
    connect zero, UInt(0)
    connect one, UInt(1)
    connect capacity, UInt(4)
    connect true_bit, UInt(1)
    connect empty, eq(count, zero)
    connect full, eq(count, capacity)
    connect input_ready, xor(full, true_bit)
    connect output_valid, xor(empty, true_bit)
    connect output_data, slot_0
    connect push, and(input_valid, input_ready)
    connect pop, and(output_valid, output_ready)
    connect decremented, sub(count, one)
    connect after_pop, mux(pop, decremented, count)
    connect incremented, add(after_pop, one)
    connect normal_count, mux(push, incremented, after_pop)
    connect next_count, mux(flush, zero, normal_count)
    connect index_0, UInt(0)
    connect at_0, eq(after_pop, index_0)
    connect write_0, and(push, at_0)
    connect shift_0, mux(pop, slot_1, slot_0)
    connect next_0, mux(write_0, input_data, shift_0)
    connect index_1, UInt(1)
    connect at_1, eq(after_pop, index_1)
    connect write_1, and(push, at_1)
    connect shift_1, mux(pop, slot_2, slot_1)
    connect next_1, mux(write_1, input_data, shift_1)
    connect index_2, UInt(2)
    connect at_2, eq(after_pop, index_2)
    connect write_2, and(push, at_2)
    connect shift_2, mux(pop, slot_3, slot_2)
    connect next_2, mux(write_2, input_data, shift_2)
    connect index_3, UInt(3)
    connect at_3, eq(after_pop, index_3)
    connect write_3, and(push, at_3)
    connect shift_3, mux(pop, slot_3, slot_3)
    connect next_3, mux(write_3, input_data, shift_3)
    connect count, next_count
    connect slot_0, next_0
    connect slot_1, next_1
    connect slot_2, next_2
    connect slot_3, next_3
  module Fr198Uart :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    input rx: UInt<1>
    output tx: UInt<1>
    output raw_events: UInt<4>
    wire ctrl_read_commit: UInt<1>
    wire ctrl_write_commit: UInt<1>
    wire ctrl_value: UInt<32>
    wire ctrl_candidate: UInt<32>
    wire ctrl_write_mask: UInt<32>
    wire ctrl_write_reject: UInt<1>
    wire baud_div_read_commit: UInt<1>
    wire baud_div_write_commit: UInt<1>
    wire baud_div_value: UInt<32>
    wire baud_div_candidate: UInt<32>
    wire baud_div_write_mask: UInt<32>
    wire baud_div_write_reject: UInt<1>
    wire status_read_commit: UInt<1>
    wire status_write_commit: UInt<1>
    wire status_value: UInt<32>
    wire tx_data_read_commit: UInt<1>
    wire tx_data_write_commit: UInt<1>
    wire tx_data_candidate: UInt<32>
    wire tx_data_write_mask: UInt<32>
    wire tx_data_write_reject: UInt<1>
    wire rx_data_read_commit: UInt<1>
    wire rx_data_write_commit: UInt<1>
    wire rx_data_value: UInt<32>
    wire rx_data_read_reject: UInt<1>
    wire EVENT_read_commit: UInt<1>
    wire EVENT_write_commit: UInt<1>
    wire EVENT_value: UInt<32>
    wire EVENT_candidate: UInt<32>
    wire EVENT_write_mask: UInt<32>
    wire event_bits: UInt<32>
    inst csr of BitloomUartCsrRegisters
    connect csr.clk, clk
    connect csr.rst, rst
    connect csr.req_valid, req_valid
    connect csr.write, write
    connect csr.addr, addr
    connect csr.wdata, wdata
    connect csr.wstrb, wstrb
    connect csr.rsp_ready, rsp_ready
    connect req_ready, csr.req_ready
    connect rsp_valid, csr.rsp_valid
    connect rdata, csr.rdata
    connect error, csr.error
    connect ctrl_read_commit, csr.ctrl_read_commit
    connect ctrl_write_commit, csr.ctrl_write_commit
    connect ctrl_value, csr.ctrl_value
    connect ctrl_candidate, csr.ctrl_candidate
    connect ctrl_write_mask, csr.ctrl_write_mask
    connect csr.ctrl_write_reject, ctrl_write_reject
    connect baud_div_read_commit, csr.baud_div_read_commit
    connect baud_div_write_commit, csr.baud_div_write_commit
    connect baud_div_value, csr.baud_div_value
    connect baud_div_candidate, csr.baud_div_candidate
    connect baud_div_write_mask, csr.baud_div_write_mask
    connect csr.baud_div_write_reject, baud_div_write_reject
    connect status_read_commit, csr.status_read_commit
    connect status_write_commit, csr.status_write_commit
    connect csr.status_value, status_value
    connect tx_data_read_commit, csr.tx_data_read_commit
    connect tx_data_write_commit, csr.tx_data_write_commit
    connect tx_data_candidate, csr.tx_data_candidate
    connect tx_data_write_mask, csr.tx_data_write_mask
    connect csr.tx_data_write_reject, tx_data_write_reject
    connect rx_data_read_commit, csr.rx_data_read_commit
    connect rx_data_write_commit, csr.rx_data_write_commit
    connect csr.rx_data_value, rx_data_value
    connect csr.rx_data_read_reject, rx_data_read_reject
    connect EVENT_read_commit, csr.EVENT_read_commit
    connect EVENT_write_commit, csr.EVENT_write_commit
    connect EVENT_value, csr.EVENT_value
    connect EVENT_candidate, csr.EVENT_candidate
    connect EVENT_write_mask, csr.EVENT_write_mask
    connect csr.event_bits, event_bits
    wire zero_bit: UInt<1>
    wire txq_input_valid: UInt<1>
    wire txq_input_ready: UInt<1>
    wire txq_input_data: UInt<8>
    wire txq_output_valid: UInt<1>
    wire txq_output_ready: UInt<1>
    wire txq_output_data: UInt<8>
    inst txq of BitloomUartCsrFifo8x4
    connect txq.clk, clk
    connect txq.rst, rst
    connect txq.flush, zero_bit
    connect txq.input_valid, txq_input_valid
    connect txq_input_ready, txq.input_ready
    connect txq.input_data, txq_input_data
    connect txq_output_valid, txq.output_valid
    connect txq.output_ready, txq_output_ready
    connect txq_output_data, txq.output_data
    wire rxq_input_valid: UInt<1>
    wire rxq_input_ready: UInt<1>
    wire rxq_input_data: UInt<8>
    wire rxq_output_valid: UInt<1>
    wire rxq_output_ready: UInt<1>
    wire rxq_output_data: UInt<8>
    inst rxq of BitloomUartCsrFifo8x4
    connect rxq.clk, clk
    connect rxq.rst, rst
    connect rxq.flush, zero_bit
    connect rxq.input_valid, rxq_input_valid
    connect rxq_input_ready, rxq.input_ready
    connect rxq.input_data, rxq_input_data
    connect rxq_output_valid, rxq.output_valid
    connect rxq.output_ready, rxq_output_ready
    connect rxq_output_data, rxq.output_data
    regreset sync1: UInt<1>, clk, rst, UInt(0)
    regreset sync2: UInt<1>, clk, rst, UInt(0)
    regreset history: UInt<1>, clk, rst, UInt(0)
    regreset tx_busy: UInt<1>, clk, rst, UInt(0)
    regreset tx_line: UInt<1>, clk, rst, UInt(0)
    regreset tx_index: UInt<4>, clk, rst, UInt(0)
    regreset tx_shift: UInt<8>, clk, rst, UInt(0)
    regreset tx_timer: UInt<32>, clk, rst, UInt(0)
    regreset tx_div: UInt<32>, clk, rst, UInt(0)
    regreset rx_state: UInt<4>, clk, rst, UInt(0)
    regreset rx_shift: UInt<8>, clk, rst, UInt(0)
    regreset rx_timer: UInt<32>, clk, rst, UInt(0)
    regreset rx_div: UInt<32>, clk, rst, UInt(0)
    wire _uart_n0: UInt<1>
    wire _uart_n1: UInt<1>
    wire _uart_n2: UInt<4>
    wire _uart_n3: UInt<4>
    wire _uart_n4: UInt<32>
    wire _uart_n5: UInt<32>
    wire _uart_n6: UInt<1>
    wire _uart_n7: UInt<1>
    wire _uart_n8: UInt<1>
    wire _uart_n9: UInt<1>
    wire _uart_n10: UInt<1>
    wire _uart_n11: UInt<1>
    wire _uart_n12: UInt<1>
    wire _uart_n13: UInt<1>
    wire _uart_n14: UInt<1>
    wire _uart_n15: UInt<1>
    wire _uart_n16: UInt<1>
    wire _uart_n17: UInt<1>
    wire _uart_n18: UInt<32>
    wire _uart_n19: UInt<1>
    wire _uart_n20: UInt<1>
    wire _uart_n21: UInt<1>
    wire _uart_n22: UInt<1>
    wire _uart_n23: UInt<1>
    wire _uart_n24: UInt<1>
    wire _uart_n25: UInt<1>
    wire _uart_n26: UInt<1>
    wire _uart_n27: UInt<1>
    wire _uart_n28: UInt<1>
    wire _uart_n29: UInt<1>
    wire _uart_n30: UInt<32>
    wire _uart_n31: UInt<2>
    wire _uart_n32: UInt<2>
    wire _uart_n33: UInt<4>
    wire _uart_n34: UInt<28>
    wire _uart_n35: UInt<32>
    wire _uart_n36: UInt<24>
    wire _uart_n37: UInt<32>
    wire _uart_n38: UInt<8>
    wire _uart_n39: UInt<1>
    wire _uart_n40: UInt<1>
    wire _uart_n41: UInt<1>
    wire _uart_n42: UInt<1>
    wire _uart_n43: UInt<1>
    wire _uart_n44: UInt<1>
    wire _uart_n45: UInt<1>
    wire _uart_n46: UInt<4>
    wire _uart_n47: UInt<1>
    wire _uart_n48: UInt<1>
    wire _uart_n49: UInt<1>
    wire _uart_n50: UInt<1>
    wire _uart_n51: UInt<4>
    wire _uart_n52: UInt<4>
    wire _uart_n53: UInt<4>
    wire _uart_n54: UInt<32>
    wire _uart_n55: UInt<32>
    wire _uart_n56: UInt<32>
    wire _uart_n57: UInt<32>
    wire _uart_n58: UInt<32>
    wire _uart_n59: UInt<7>
    wire _uart_n60: UInt<8>
    wire _uart_n61: UInt<8>
    wire _uart_n62: UInt<8>
    wire _uart_n63: UInt<4>
    wire _uart_n64: UInt<1>
    wire _uart_n65: UInt<1>
    wire _uart_n66: UInt<1>
    wire _uart_n67: UInt<1>
    wire _uart_n68: UInt<1>
    wire _uart_n69: UInt<1>
    wire _uart_n70: UInt<1>
    wire _uart_n71: UInt<1>
    wire _uart_n72: UInt<1>
    wire _uart_n73: UInt<1>
    wire _uart_n74: UInt<1>
    wire _uart_n75: UInt<1>
    wire _uart_n76: UInt<31>
    wire _uart_n77: UInt<32>
    wire _uart_n78: UInt<1>
    wire _uart_n79: UInt<31>
    wire _uart_n80: UInt<32>
    wire _uart_n81: UInt<32>
    wire _uart_n82: UInt<32>
    wire _uart_n83: UInt<1>
    wire _uart_n84: UInt<1>
    wire _uart_n85: UInt<1>
    wire _uart_n86: UInt<4>
    wire _uart_n87: UInt<1>
    wire _uart_n88: UInt<1>
    wire _uart_n89: UInt<1>
    wire _uart_n90: UInt<1>
    wire _uart_n91: UInt<4>
    wire _uart_n92: UInt<4>
    wire _uart_n93: UInt<4>
    wire _uart_n94: UInt<4>
    wire _uart_n95: UInt<32>
    wire _uart_n96: UInt<32>
    wire _uart_n97: UInt<32>
    wire _uart_n98: UInt<32>
    wire _uart_n99: UInt<32>
    wire _uart_n100: UInt<1>
    wire _uart_n101: UInt<1>
    wire _uart_n102: UInt<1>
    wire _uart_n103: UInt<1>
    wire _uart_n104: UInt<1>
    wire _uart_n105: UInt<1>
    wire _uart_n106: UInt<7>
    wire _uart_n107: UInt<8>
    wire _uart_n108: UInt<8>
    wire _uart_n109: UInt<1>
    wire _uart_n110: UInt<1>
    wire _uart_n111: UInt<1>
    wire _uart_n112: UInt<1>
    wire _uart_n113: UInt<1>
    wire _uart_n114: UInt<1>
    wire _uart_n115: UInt<1>
    wire _uart_n116: UInt<1>
    wire _uart_n117: UInt<2>
    wire _uart_n118: UInt<2>
    wire _uart_n119: UInt<4>
    wire _uart_n120: UInt<32>
    connect _uart_n0, UInt(0)
    connect _uart_n1, UInt(1)
    connect _uart_n2, UInt(0)
    connect _uart_n3, UInt(1)
    connect _uart_n4, UInt(0)
    connect _uart_n5, UInt(1)
    connect zero_bit, _uart_n0
    connect _uart_n6, UInt(1)
    connect _uart_n7, xor(rst, _uart_n6)
    connect _uart_n8, eq(rx_state, _uart_n2)
    connect _uart_n9, UInt(1)
    connect _uart_n10, xor(_uart_n8, _uart_n9)
    connect _uart_n11, or(tx_busy, _uart_n10)
    connect _uart_n12, bits(ctrl_value, 0, 0)
    connect _uart_n13, bits(ctrl_candidate, 0, 0)
    connect _uart_n14, eq(_uart_n12, _uart_n13)
    connect _uart_n15, UInt(1)
    connect _uart_n16, xor(_uart_n14, _uart_n15)
    connect _uart_n17, and(_uart_n11, _uart_n16)
    connect _uart_n18, UInt(3)
    connect _uart_n19, lt(baud_div_value, _uart_n18)
    connect _uart_n20, and(_uart_n13, _uart_n19)
    connect _uart_n21, or(_uart_n17, _uart_n20)
    connect ctrl_write_reject, _uart_n21
    connect _uart_n22, lt(baud_div_candidate, _uart_n18)
    connect _uart_n23, and(_uart_n12, _uart_n22)
    connect _uart_n24, or(_uart_n11, _uart_n23)
    connect baud_div_write_reject, _uart_n24
    connect _uart_n25, UInt(1)
    connect _uart_n26, xor(txq_input_ready, _uart_n25)
    connect tx_data_write_reject, _uart_n26
    connect _uart_n27, UInt(1)
    connect _uart_n28, xor(rxq_output_valid, _uart_n27)
    connect rx_data_read_reject, _uart_n28
    connect _uart_n29, mux(ctrl_write_commit, _uart_n13, _uart_n12)
    connect _uart_n30, mux(baud_div_write_commit, baud_div_candidate, baud_div_value)
    connect _uart_n31, cat(_uart_n26, rxq_output_valid)
    connect _uart_n32, cat(_uart_n10, tx_busy)
    connect _uart_n33, cat(_uart_n32, _uart_n31)
    connect _uart_n34, UInt(0)
    connect _uart_n35, cat(_uart_n34, _uart_n33)
    connect status_value, _uart_n35
    connect _uart_n36, UInt(0)
    connect _uart_n37, cat(_uart_n36, rxq_output_data)
    connect rx_data_value, _uart_n37
    connect txq_input_valid, tx_data_write_commit
    connect _uart_n38, bits(tx_data_candidate, 7, 0)
    connect txq_input_data, _uart_n38
    connect rxq_output_ready, rx_data_read_commit
    connect rxq_input_data, rx_shift
    connect _uart_n39, UInt(1)
    connect _uart_n40, xor(tx_busy, _uart_n39)
    connect _uart_n41, and(_uart_n40, _uart_n29)
    connect _uart_n42, and(_uart_n41, txq_output_valid)
    connect _uart_n43, and(_uart_n42, _uart_n7)
    connect txq_output_ready, _uart_n43
    connect _uart_n44, eq(tx_timer, _uart_n4)
    connect _uart_n45, and(tx_busy, _uart_n44)
    connect _uart_n46, UInt(9)
    connect _uart_n47, eq(tx_index, _uart_n46)
    connect _uart_n48, and(_uart_n45, _uart_n47)
    connect _uart_n49, mux(_uart_n48, _uart_n0, tx_busy)
    connect _uart_n50, mux(_uart_n43, _uart_n1, _uart_n49)
    connect _uart_n51, add(tx_index, _uart_n3)
    connect _uart_n52, mux(_uart_n45, _uart_n51, tx_index)
    connect _uart_n53, mux(_uart_n43, _uart_n2, _uart_n52)
    connect _uart_n54, sub(tx_timer, _uart_n5)
    connect _uart_n55, mux(_uart_n44, tx_div, _uart_n54)
    connect _uart_n56, mux(tx_busy, _uart_n55, tx_timer)
    connect _uart_n57, mux(_uart_n43, _uart_n30, _uart_n56)
    connect _uart_n58, mux(_uart_n43, _uart_n30, tx_div)
    connect _uart_n59, bits(tx_shift, 7, 1)
    connect _uart_n60, cat(_uart_n0, _uart_n59)
    connect _uart_n61, mux(_uart_n45, _uart_n60, tx_shift)
    connect _uart_n62, mux(_uart_n43, txq_output_data, _uart_n61)
    connect _uart_n63, UInt(8)
    connect _uart_n64, eq(tx_index, _uart_n63)
    connect _uart_n65, bits(tx_shift, 0, 0)
    connect _uart_n66, mux(_uart_n64, _uart_n1, _uart_n65)
    connect _uart_n67, mux(_uart_n45, _uart_n66, tx_line)
    connect _uart_n68, mux(_uart_n43, _uart_n0, _uart_n67)
    connect _uart_n69, mux(tx_busy, tx_line, _uart_n1)
    connect tx, _uart_n69
    connect _uart_n70, UInt(1)
    connect _uart_n71, xor(sync2, _uart_n70)
    connect _uart_n72, and(_uart_n71, history)
    connect _uart_n73, and(_uart_n8, _uart_n72)
    connect _uart_n74, and(_uart_n73, _uart_n29)
    connect _uart_n75, and(_uart_n74, _uart_n7)
    connect _uart_n76, bits(_uart_n30, 31, 1)
    connect _uart_n77, cat(_uart_n0, _uart_n76)
    connect _uart_n78, bits(_uart_n30, 0, 0)
    connect _uart_n79, UInt(0)
    connect _uart_n80, cat(_uart_n79, _uart_n78)
    connect _uart_n81, add(_uart_n77, _uart_n80)
    connect _uart_n82, sub(_uart_n81, _uart_n5)
    connect _uart_n83, eq(rx_timer, _uart_n4)
    connect _uart_n84, and(_uart_n10, _uart_n83)
    connect _uart_n85, eq(rx_state, _uart_n3)
    connect _uart_n86, UInt(10)
    connect _uart_n87, eq(rx_state, _uart_n86)
    connect _uart_n88, and(_uart_n85, sync2)
    connect _uart_n89, or(_uart_n88, _uart_n87)
    connect _uart_n90, and(_uart_n84, _uart_n89)
    connect _uart_n91, add(rx_state, _uart_n3)
    connect _uart_n92, mux(_uart_n84, _uart_n91, rx_state)
    connect _uart_n93, mux(_uart_n90, _uart_n2, _uart_n92)
    connect _uart_n94, mux(_uart_n75, _uart_n3, _uart_n93)
    connect _uart_n95, sub(rx_timer, _uart_n5)
    connect _uart_n96, mux(_uart_n83, rx_div, _uart_n95)
    connect _uart_n97, mux(_uart_n10, _uart_n96, rx_timer)
    connect _uart_n98, mux(_uart_n75, _uart_n82, _uart_n97)
    connect _uart_n99, mux(_uart_n75, _uart_n30, rx_div)
    connect _uart_n100, UInt(1)
    connect _uart_n101, xor(_uart_n85, _uart_n100)
    connect _uart_n102, UInt(1)
    connect _uart_n103, xor(_uart_n87, _uart_n102)
    connect _uart_n104, and(_uart_n101, _uart_n103)
    connect _uart_n105, and(_uart_n84, _uart_n104)
    connect _uart_n106, bits(rx_shift, 7, 1)
    connect _uart_n107, cat(sync2, _uart_n106)
    connect _uart_n108, mux(_uart_n105, _uart_n107, rx_shift)
    connect _uart_n109, and(_uart_n84, _uart_n87)
    connect _uart_n110, and(_uart_n109, _uart_n7)
    connect _uart_n111, and(_uart_n110, sync2)
    connect _uart_n112, and(_uart_n111, rxq_input_ready)
    connect _uart_n113, UInt(1)
    connect _uart_n114, xor(rxq_input_ready, _uart_n113)
    connect _uart_n115, and(_uart_n111, _uart_n114)
    connect _uart_n116, and(_uart_n110, _uart_n71)
    connect rxq_input_valid, _uart_n112
    connect _uart_n117, cat(_uart_n43, _uart_n112)
    connect _uart_n118, cat(_uart_n116, _uart_n115)
    connect _uart_n119, cat(_uart_n118, _uart_n117)
    connect raw_events, _uart_n119
    connect _uart_n120, cat(_uart_n34, _uart_n119)
    connect event_bits, _uart_n120
    connect sync1, rx
    connect sync2, sync1
    connect history, sync2
    connect tx_busy, _uart_n50
    connect tx_index, _uart_n53
    connect tx_timer, _uart_n57
    connect tx_div, _uart_n58
    connect tx_shift, _uart_n62
    connect tx_line, _uart_n68
    connect rx_state, _uart_n94
    connect rx_timer, _uart_n98
    connect rx_div, _uart_n99
    connect rx_shift, _uart_n108
  module BitloomGpioCsrRegisters :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    output dir_read_commit: UInt<1>
    output dir_write_commit: UInt<1>
    output dir_value: UInt<32>
    output dir_candidate: UInt<32>
    output dir_write_mask: UInt<32>
    output out_read_commit: UInt<1>
    output out_write_commit: UInt<1>
    input out_value: UInt<32>
    output out_candidate: UInt<32>
    output out_write_mask: UInt<32>
    output in_read_commit: UInt<1>
    output in_write_commit: UInt<1>
    input in_value: UInt<32>
    output set_read_commit: UInt<1>
    output set_write_commit: UInt<1>
    output set_candidate: UInt<32>
    output set_write_mask: UInt<32>
    output clear_read_commit: UInt<1>
    output clear_write_commit: UInt<1>
    output clear_candidate: UInt<32>
    output clear_write_mask: UInt<32>
    output rise_event_read_commit: UInt<1>
    output rise_event_write_commit: UInt<1>
    output rise_event_value: UInt<32>
    output rise_event_candidate: UInt<32>
    output rise_event_write_mask: UInt<32>
    input rise_bits: UInt<32>
    regreset _csr_pending: UInt<1>, clk, rst, UInt(0)
    regreset _csr_data: UInt<32>, clk, rst, UInt(0)
    regreset _csr_error: UInt<2>, clk, rst, UInt(0)
    regreset _csr_value0: UInt<32>, clk, rst, UInt(0)
    regreset _csr_value5: UInt<32>, clk, rst, UInt(0)
    wire _csr_n0: UInt<1>
    wire _csr_n1: UInt<1>
    wire _csr_n2: UInt<32>
    wire _csr_n3: UInt<2>
    wire _csr_n4: UInt<2>
    wire _csr_n5: UInt<1>
    wire _csr_n6: UInt<1>
    wire _csr_n7: UInt<1>
    wire _csr_n8: UInt<1>
    wire _csr_n9: UInt<1>
    wire _csr_n10: UInt<1>
    wire _csr_n11: UInt<1>
    wire _csr_n12: UInt<1>
    wire _csr_n13: UInt<1>
    wire _csr_n14: UInt<1>
    wire _csr_n15: UInt<1>
    wire _csr_n16: UInt<32>
    wire _csr_n17: UInt<32>
    wire _csr_n18: UInt<32>
    wire _csr_n19: UInt<1>
    wire _csr_n20: UInt<32>
    wire _csr_n21: UInt<32>
    wire _csr_n22: UInt<32>
    wire _csr_n23: UInt<1>
    wire _csr_n24: UInt<32>
    wire _csr_n25: UInt<32>
    wire _csr_n26: UInt<32>
    wire _csr_n27: UInt<1>
    wire _csr_n28: UInt<32>
    wire _csr_n29: UInt<32>
    wire _csr_n30: UInt<32>
    wire _csr_n31: UInt<32>
    wire _csr_n32: UInt<32>
    wire _csr_n33: UInt<16>
    wire _csr_n34: UInt<1>
    wire _csr_n35: UInt<1>
    wire _csr_n36: UInt<1>
    wire _csr_n37: UInt<1>
    wire _csr_n38: UInt<32>
    wire _csr_n39: UInt<1>
    wire _csr_n40: UInt<1>
    wire _csr_n41: UInt<1>
    wire _csr_n42: UInt<32>
    wire _csr_n43: UInt<32>
    wire _csr_n44: UInt<32>
    wire _csr_n45: UInt<32>
    wire _csr_n46: UInt<32>
    wire _csr_n47: UInt<1>
    wire _csr_n48: UInt<1>
    wire _csr_n49: UInt<1>
    wire _csr_n50: UInt<1>
    wire _csr_n51: UInt<2>
    wire _csr_n52: UInt<2>
    wire _csr_n53: UInt<32>
    wire _csr_n54: UInt<32>
    wire _csr_n55: UInt<32>
    wire _csr_n56: UInt<32>
    wire _csr_n57: UInt<32>
    wire _csr_n58: UInt<16>
    wire _csr_n59: UInt<1>
    wire _csr_n60: UInt<1>
    wire _csr_n61: UInt<1>
    wire _csr_n62: UInt<1>
    wire _csr_n63: UInt<32>
    wire _csr_n64: UInt<1>
    wire _csr_n65: UInt<1>
    wire _csr_n66: UInt<1>
    wire _csr_n67: UInt<32>
    wire _csr_n68: UInt<32>
    wire _csr_n69: UInt<32>
    wire _csr_n70: UInt<32>
    wire _csr_n71: UInt<32>
    wire _csr_n72: UInt<1>
    wire _csr_n73: UInt<1>
    wire _csr_n74: UInt<1>
    wire _csr_n75: UInt<1>
    wire _csr_n76: UInt<2>
    wire _csr_n77: UInt<2>
    wire _csr_n78: UInt<32>
    wire _csr_n79: UInt<32>
    wire _csr_n80: UInt<32>
    wire _csr_n81: UInt<32>
    wire _csr_n82: UInt<16>
    wire _csr_n83: UInt<1>
    wire _csr_n84: UInt<1>
    wire _csr_n85: UInt<1>
    wire _csr_n86: UInt<1>
    wire _csr_n87: UInt<1>
    wire _csr_n88: UInt<2>
    wire _csr_n89: UInt<2>
    wire _csr_n90: UInt<32>
    wire _csr_n91: UInt<32>
    wire _csr_n92: UInt<32>
    wire _csr_n93: UInt<16>
    wire _csr_n94: UInt<1>
    wire _csr_n95: UInt<1>
    wire _csr_n96: UInt<1>
    wire _csr_n97: UInt<1>
    wire _csr_n98: UInt<32>
    wire _csr_n99: UInt<1>
    wire _csr_n100: UInt<1>
    wire _csr_n101: UInt<1>
    wire _csr_n102: UInt<32>
    wire _csr_n103: UInt<1>
    wire _csr_n104: UInt<1>
    wire _csr_n105: UInt<1>
    wire _csr_n106: UInt<1>
    wire _csr_n107: UInt<2>
    wire _csr_n108: UInt<2>
    wire _csr_n109: UInt<32>
    wire _csr_n110: UInt<32>
    wire _csr_n111: UInt<32>
    wire _csr_n112: UInt<16>
    wire _csr_n113: UInt<1>
    wire _csr_n114: UInt<1>
    wire _csr_n115: UInt<1>
    wire _csr_n116: UInt<1>
    wire _csr_n117: UInt<32>
    wire _csr_n118: UInt<1>
    wire _csr_n119: UInt<1>
    wire _csr_n120: UInt<1>
    wire _csr_n121: UInt<32>
    wire _csr_n122: UInt<1>
    wire _csr_n123: UInt<1>
    wire _csr_n124: UInt<1>
    wire _csr_n125: UInt<1>
    wire _csr_n126: UInt<2>
    wire _csr_n127: UInt<2>
    wire _csr_n128: UInt<32>
    wire _csr_n129: UInt<32>
    wire _csr_n130: UInt<32>
    wire _csr_n131: UInt<32>
    wire _csr_n132: UInt<16>
    wire _csr_n133: UInt<1>
    wire _csr_n134: UInt<1>
    wire _csr_n135: UInt<1>
    wire _csr_n136: UInt<1>
    wire _csr_n137: UInt<32>
    wire _csr_n138: UInt<1>
    wire _csr_n139: UInt<1>
    wire _csr_n140: UInt<1>
    wire _csr_n141: UInt<32>
    wire _csr_n142: UInt<1>
    wire _csr_n143: UInt<1>
    wire _csr_n144: UInt<1>
    wire _csr_n145: UInt<1>
    wire _csr_n146: UInt<2>
    wire _csr_n147: UInt<2>
    wire _csr_n148: UInt<32>
    wire _csr_n149: UInt<32>
    wire _csr_n150: UInt<32>
    wire _csr_n151: UInt<32>
    wire _csr_n152: UInt<32>
    wire _csr_n153: UInt<32>
    wire _csr_n154: UInt<32>
    wire _csr_n155: UInt<32>
    wire _csr_n156: UInt<32>
    wire _csr_n157: UInt<2>
    connect _csr_n0, UInt(0)
    connect _csr_n1, UInt(1)
    connect _csr_n2, UInt(0)
    connect _csr_n3, UInt(0)
    connect _csr_n4, UInt(2)
    connect _csr_n5, UInt(1)
    connect _csr_n6, xor(_csr_pending, _csr_n5)
    connect _csr_n7, UInt(1)
    connect _csr_n8, xor(rst, _csr_n7)
    connect _csr_n9, and(_csr_n6, _csr_n8)
    connect req_ready, _csr_n9
    connect _csr_n10, and(req_valid, _csr_n9)
    connect _csr_n11, UInt(1)
    connect _csr_n12, xor(write, _csr_n11)
    connect _csr_n13, mux(rsp_ready, _csr_n0, _csr_pending)
    connect _csr_n14, mux(_csr_n10, _csr_n1, _csr_n13)
    connect rsp_valid, _csr_pending
    connect rdata, _csr_data
    connect error, _csr_error
    connect _csr_n15, bits(wstrb, 0, 0)
    connect _csr_n16, UInt(255)
    connect _csr_n17, mux(_csr_n15, _csr_n16, _csr_n2)
    connect _csr_n18, or(_csr_n2, _csr_n17)
    connect _csr_n19, bits(wstrb, 1, 1)
    connect _csr_n20, UInt(65280)
    connect _csr_n21, mux(_csr_n19, _csr_n20, _csr_n2)
    connect _csr_n22, or(_csr_n18, _csr_n21)
    connect _csr_n23, bits(wstrb, 2, 2)
    connect _csr_n24, UInt(16711680)
    connect _csr_n25, mux(_csr_n23, _csr_n24, _csr_n2)
    connect _csr_n26, or(_csr_n22, _csr_n25)
    connect _csr_n27, bits(wstrb, 3, 3)
    connect _csr_n28, UInt(4278190080)
    connect _csr_n29, mux(_csr_n27, _csr_n28, _csr_n2)
    connect _csr_n30, or(_csr_n26, _csr_n29)
    connect _csr_n31, UInt(4294967295)
    connect _csr_n32, and(_csr_value0, _csr_n31)
    connect dir_value, _csr_n32
    connect _csr_n33, UInt(0)
    connect _csr_n34, eq(addr, _csr_n33)
    connect _csr_n35, and(_csr_n10, _csr_n34)
    connect _csr_n36, and(_csr_n12, _csr_n1)
    connect _csr_n37, and(_csr_n35, _csr_n36)
    connect dir_read_commit, _csr_n37
    connect _csr_n38, and(_csr_n30, _csr_n31)
    connect _csr_n39, eq(_csr_n38, _csr_n2)
    connect _csr_n40, UInt(1)
    connect _csr_n41, xor(_csr_n39, _csr_n40)
    connect _csr_n42, and(wdata, _csr_n38)
    connect _csr_n43, UInt(4294967295)
    connect _csr_n44, xor(_csr_n38, _csr_n43)
    connect _csr_n45, and(_csr_n32, _csr_n44)
    connect _csr_n46, or(_csr_n45, _csr_n42)
    connect dir_write_mask, _csr_n38
    connect dir_candidate, _csr_n46
    connect _csr_n47, and(write, _csr_n1)
    connect _csr_n48, and(_csr_n47, _csr_n41)
    connect _csr_n49, and(_csr_n35, _csr_n48)
    connect dir_write_commit, _csr_n49
    connect _csr_n50, or(_csr_n36, _csr_n47)
    connect _csr_n51, mux(_csr_n50, _csr_n3, _csr_n4)
    connect _csr_n52, mux(_csr_n34, _csr_n51, _csr_n4)
    connect _csr_n53, mux(_csr_n36, _csr_n32, _csr_n2)
    connect _csr_n54, mux(_csr_n34, _csr_n53, _csr_n2)
    connect _csr_n55, mux(_csr_n49, _csr_n46, _csr_n32)
    connect _csr_n56, UInt(4294967295)
    connect _csr_n57, and(out_value, _csr_n56)
    connect _csr_n58, UInt(4)
    connect _csr_n59, eq(addr, _csr_n58)
    connect _csr_n60, and(_csr_n10, _csr_n59)
    connect _csr_n61, and(_csr_n12, _csr_n1)
    connect _csr_n62, and(_csr_n60, _csr_n61)
    connect out_read_commit, _csr_n62
    connect _csr_n63, and(_csr_n30, _csr_n56)
    connect _csr_n64, eq(_csr_n63, _csr_n2)
    connect _csr_n65, UInt(1)
    connect _csr_n66, xor(_csr_n64, _csr_n65)
    connect _csr_n67, and(wdata, _csr_n63)
    connect _csr_n68, UInt(4294967295)
    connect _csr_n69, xor(_csr_n63, _csr_n68)
    connect _csr_n70, and(_csr_n57, _csr_n69)
    connect _csr_n71, or(_csr_n70, _csr_n67)
    connect out_write_mask, _csr_n63
    connect out_candidate, _csr_n71
    connect _csr_n72, and(write, _csr_n1)
    connect _csr_n73, and(_csr_n72, _csr_n66)
    connect _csr_n74, and(_csr_n60, _csr_n73)
    connect out_write_commit, _csr_n74
    connect _csr_n75, or(_csr_n61, _csr_n72)
    connect _csr_n76, mux(_csr_n75, _csr_n3, _csr_n4)
    connect _csr_n77, mux(_csr_n59, _csr_n76, _csr_n52)
    connect _csr_n78, mux(_csr_n61, _csr_n57, _csr_n2)
    connect _csr_n79, mux(_csr_n59, _csr_n78, _csr_n54)
    connect _csr_n80, UInt(4294967295)
    connect _csr_n81, and(in_value, _csr_n80)
    connect _csr_n82, UInt(8)
    connect _csr_n83, eq(addr, _csr_n82)
    connect _csr_n84, and(_csr_n10, _csr_n83)
    connect _csr_n85, and(_csr_n12, _csr_n1)
    connect _csr_n86, and(_csr_n84, _csr_n85)
    connect in_read_commit, _csr_n86
    connect in_write_commit, _csr_n0
    connect _csr_n87, or(_csr_n85, _csr_n0)
    connect _csr_n88, mux(_csr_n87, _csr_n3, _csr_n4)
    connect _csr_n89, mux(_csr_n83, _csr_n88, _csr_n77)
    connect _csr_n90, mux(_csr_n85, _csr_n81, _csr_n2)
    connect _csr_n91, mux(_csr_n83, _csr_n90, _csr_n79)
    connect _csr_n92, UInt(4294967295)
    connect _csr_n93, UInt(12)
    connect _csr_n94, eq(addr, _csr_n93)
    connect _csr_n95, and(_csr_n10, _csr_n94)
    connect _csr_n96, and(_csr_n12, _csr_n0)
    connect _csr_n97, and(_csr_n95, _csr_n96)
    connect set_read_commit, _csr_n97
    connect _csr_n98, and(_csr_n30, _csr_n92)
    connect _csr_n99, eq(_csr_n98, _csr_n2)
    connect _csr_n100, UInt(1)
    connect _csr_n101, xor(_csr_n99, _csr_n100)
    connect _csr_n102, and(wdata, _csr_n98)
    connect set_write_mask, _csr_n98
    connect set_candidate, _csr_n102
    connect _csr_n103, and(write, _csr_n1)
    connect _csr_n104, and(_csr_n103, _csr_n101)
    connect _csr_n105, and(_csr_n95, _csr_n104)
    connect set_write_commit, _csr_n105
    connect _csr_n106, or(_csr_n96, _csr_n103)
    connect _csr_n107, mux(_csr_n106, _csr_n3, _csr_n4)
    connect _csr_n108, mux(_csr_n94, _csr_n107, _csr_n89)
    connect _csr_n109, mux(_csr_n96, _csr_n2, _csr_n2)
    connect _csr_n110, mux(_csr_n94, _csr_n109, _csr_n91)
    connect _csr_n111, UInt(4294967295)
    connect _csr_n112, UInt(16)
    connect _csr_n113, eq(addr, _csr_n112)
    connect _csr_n114, and(_csr_n10, _csr_n113)
    connect _csr_n115, and(_csr_n12, _csr_n0)
    connect _csr_n116, and(_csr_n114, _csr_n115)
    connect clear_read_commit, _csr_n116
    connect _csr_n117, and(_csr_n30, _csr_n111)
    connect _csr_n118, eq(_csr_n117, _csr_n2)
    connect _csr_n119, UInt(1)
    connect _csr_n120, xor(_csr_n118, _csr_n119)
    connect _csr_n121, and(wdata, _csr_n117)
    connect clear_write_mask, _csr_n117
    connect clear_candidate, _csr_n121
    connect _csr_n122, and(write, _csr_n1)
    connect _csr_n123, and(_csr_n122, _csr_n120)
    connect _csr_n124, and(_csr_n114, _csr_n123)
    connect clear_write_commit, _csr_n124
    connect _csr_n125, or(_csr_n115, _csr_n122)
    connect _csr_n126, mux(_csr_n125, _csr_n3, _csr_n4)
    connect _csr_n127, mux(_csr_n113, _csr_n126, _csr_n108)
    connect _csr_n128, mux(_csr_n115, _csr_n2, _csr_n2)
    connect _csr_n129, mux(_csr_n113, _csr_n128, _csr_n110)
    connect _csr_n130, UInt(4294967295)
    connect _csr_n131, and(_csr_value5, _csr_n130)
    connect rise_event_value, _csr_n131
    connect _csr_n132, UInt(20)
    connect _csr_n133, eq(addr, _csr_n132)
    connect _csr_n134, and(_csr_n10, _csr_n133)
    connect _csr_n135, and(_csr_n12, _csr_n1)
    connect _csr_n136, and(_csr_n134, _csr_n135)
    connect rise_event_read_commit, _csr_n136
    connect _csr_n137, and(_csr_n30, _csr_n130)
    connect _csr_n138, eq(_csr_n137, _csr_n2)
    connect _csr_n139, UInt(1)
    connect _csr_n140, xor(_csr_n138, _csr_n139)
    connect _csr_n141, and(wdata, _csr_n137)
    connect rise_event_write_mask, _csr_n137
    connect rise_event_candidate, _csr_n141
    connect _csr_n142, and(write, _csr_n1)
    connect _csr_n143, and(_csr_n142, _csr_n140)
    connect _csr_n144, and(_csr_n134, _csr_n143)
    connect rise_event_write_commit, _csr_n144
    connect _csr_n145, or(_csr_n135, _csr_n142)
    connect _csr_n146, mux(_csr_n145, _csr_n3, _csr_n4)
    connect _csr_n147, mux(_csr_n133, _csr_n146, _csr_n127)
    connect _csr_n148, mux(_csr_n135, _csr_n131, _csr_n2)
    connect _csr_n149, mux(_csr_n133, _csr_n148, _csr_n129)
    connect _csr_n150, mux(_csr_n144, _csr_n141, _csr_n2)
    connect _csr_n151, UInt(4294967295)
    connect _csr_n152, xor(_csr_n150, _csr_n151)
    connect _csr_n153, and(_csr_n131, _csr_n152)
    connect _csr_n154, and(rise_bits, _csr_n130)
    connect _csr_n155, or(_csr_n153, _csr_n154)
    connect _csr_n156, mux(_csr_n10, _csr_n149, _csr_data)
    connect _csr_n157, mux(_csr_n10, _csr_n147, _csr_error)
    connect _csr_pending, _csr_n14
    connect _csr_data, _csr_n156
    connect _csr_error, _csr_n157
    connect _csr_value0, _csr_n55
    connect _csr_value5, _csr_n155
  module Fr198Gpio :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    input pad_in: UInt<32>
    output pad_out: UInt<32>
    output pad_oe: UInt<32>
    output raw_event: UInt<1>
    wire dir_read_commit: UInt<1>
    wire dir_write_commit: UInt<1>
    wire dir_value: UInt<32>
    wire dir_candidate: UInt<32>
    wire dir_write_mask: UInt<32>
    wire out_read_commit: UInt<1>
    wire out_write_commit: UInt<1>
    wire out_value: UInt<32>
    wire out_candidate: UInt<32>
    wire out_write_mask: UInt<32>
    wire in_read_commit: UInt<1>
    wire in_write_commit: UInt<1>
    wire in_value: UInt<32>
    wire set_read_commit: UInt<1>
    wire set_write_commit: UInt<1>
    wire set_candidate: UInt<32>
    wire set_write_mask: UInt<32>
    wire clear_read_commit: UInt<1>
    wire clear_write_commit: UInt<1>
    wire clear_candidate: UInt<32>
    wire clear_write_mask: UInt<32>
    wire rise_event_read_commit: UInt<1>
    wire rise_event_write_commit: UInt<1>
    wire rise_event_value: UInt<32>
    wire rise_event_candidate: UInt<32>
    wire rise_event_write_mask: UInt<32>
    regreset out_r: UInt<32>, clk, rst, UInt(0)
    regreset sync1: UInt<32>, clk, rst, UInt(0)
    regreset sync2: UInt<32>, clk, rst, UInt(0)
    regreset history: UInt<32>, clk, rst, UInt(0)
    wire rise_bits: UInt<32>
    wire all32: UInt<32>
    wire zero32: UInt<32>
    wire not_history: UInt<32>
    wire not_dir: UInt<32>
    wire rising: UInt<32>
    wire set_value: UInt<32>
    wire clear_value: UInt<32>
    wire not_clear: UInt<32>
    wire after_clear: UInt<32>
    wire after_set: UInt<32>
    wire out_next: UInt<32>
    wire empty: UInt<1>
    wire has_rise: UInt<1>
    wire one: UInt<1>
    wire not_reset: UInt<1>
    inst csr of BitloomGpioCsrRegisters
    connect csr.clk, clk
    connect csr.rst, rst
    connect csr.req_valid, req_valid
    connect csr.write, write
    connect csr.addr, addr
    connect csr.wdata, wdata
    connect csr.wstrb, wstrb
    connect csr.rsp_ready, rsp_ready
    connect req_ready, csr.req_ready
    connect rsp_valid, csr.rsp_valid
    connect rdata, csr.rdata
    connect error, csr.error
    connect dir_read_commit, csr.dir_read_commit
    connect dir_write_commit, csr.dir_write_commit
    connect dir_value, csr.dir_value
    connect dir_candidate, csr.dir_candidate
    connect dir_write_mask, csr.dir_write_mask
    connect out_read_commit, csr.out_read_commit
    connect out_write_commit, csr.out_write_commit
    connect csr.out_value, out_value
    connect out_candidate, csr.out_candidate
    connect out_write_mask, csr.out_write_mask
    connect in_read_commit, csr.in_read_commit
    connect in_write_commit, csr.in_write_commit
    connect csr.in_value, in_value
    connect set_read_commit, csr.set_read_commit
    connect set_write_commit, csr.set_write_commit
    connect set_candidate, csr.set_candidate
    connect set_write_mask, csr.set_write_mask
    connect clear_read_commit, csr.clear_read_commit
    connect clear_write_commit, csr.clear_write_commit
    connect clear_candidate, csr.clear_candidate
    connect clear_write_mask, csr.clear_write_mask
    connect rise_event_read_commit, csr.rise_event_read_commit
    connect rise_event_write_commit, csr.rise_event_write_commit
    connect rise_event_value, csr.rise_event_value
    connect rise_event_candidate, csr.rise_event_candidate
    connect rise_event_write_mask, csr.rise_event_write_mask
    connect csr.rise_bits, rise_bits
    connect all32, UInt(4294967295)
    connect zero32, UInt(0)
    connect one, UInt(1)
    connect out_value, out_r
    connect in_value, sync2
    connect pad_oe, dir_value
    connect pad_out, and(out_r, dir_value)
    connect not_history, xor(history, all32)
    connect not_dir, xor(dir_value, all32)
    connect rising, and(sync2, not_history)
    connect rise_bits, and(rising, not_dir)
    connect empty, eq(rise_bits, zero32)
    connect has_rise, xor(empty, one)
    connect not_reset, xor(rst, one)
    connect raw_event, and(has_rise, not_reset)
    connect set_value, or(out_r, set_candidate)
    connect not_clear, xor(clear_candidate, all32)
    connect clear_value, and(out_r, not_clear)
    connect after_clear, mux(clear_write_commit, clear_value, out_r)
    connect after_set, mux(set_write_commit, set_value, after_clear)
    connect out_next, mux(out_write_commit, out_candidate, after_set)
    connect out_r, out_next
    connect sync1, pad_in
    connect sync2, sync1
    connect history, sync2
  module BitloomTimerCsr :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    output ctrl_read_commit: UInt<1>
    output ctrl_write_commit: UInt<1>
    input ctrl_value: UInt<32>
    output ctrl_candidate: UInt<32>
    output ctrl_write_mask: UInt<32>
    output count_read_commit: UInt<1>
    output count_write_commit: UInt<1>
    input count_value: UInt<32>
    output count_candidate: UInt<32>
    output count_write_mask: UInt<32>
    output compare_read_commit: UInt<1>
    output compare_write_commit: UInt<1>
    output compare_value: UInt<32>
    output compare_candidate: UInt<32>
    output compare_write_mask: UInt<32>
    output EVENT_read_commit: UInt<1>
    output EVENT_write_commit: UInt<1>
    output EVENT_value: UInt<32>
    output EVENT_candidate: UInt<32>
    output EVENT_write_mask: UInt<32>
    input match_bits: UInt<32>
    regreset _csr_pending: UInt<1>, clk, rst, UInt(0)
    regreset _csr_data: UInt<32>, clk, rst, UInt(0)
    regreset _csr_error: UInt<2>, clk, rst, UInt(0)
    regreset _csr_value2: UInt<32>, clk, rst, UInt(0)
    regreset _csr_value3: UInt<32>, clk, rst, UInt(0)
    wire _csr_n0: UInt<1>
    wire _csr_n1: UInt<1>
    wire _csr_n2: UInt<32>
    wire _csr_n3: UInt<2>
    wire _csr_n4: UInt<2>
    wire _csr_n5: UInt<1>
    wire _csr_n6: UInt<1>
    wire _csr_n7: UInt<1>
    wire _csr_n8: UInt<1>
    wire _csr_n9: UInt<1>
    wire _csr_n10: UInt<1>
    wire _csr_n11: UInt<1>
    wire _csr_n12: UInt<1>
    wire _csr_n13: UInt<1>
    wire _csr_n14: UInt<1>
    wire _csr_n15: UInt<1>
    wire _csr_n16: UInt<32>
    wire _csr_n17: UInt<32>
    wire _csr_n18: UInt<32>
    wire _csr_n19: UInt<1>
    wire _csr_n20: UInt<32>
    wire _csr_n21: UInt<32>
    wire _csr_n22: UInt<32>
    wire _csr_n23: UInt<1>
    wire _csr_n24: UInt<32>
    wire _csr_n25: UInt<32>
    wire _csr_n26: UInt<32>
    wire _csr_n27: UInt<1>
    wire _csr_n28: UInt<32>
    wire _csr_n29: UInt<32>
    wire _csr_n30: UInt<32>
    wire _csr_n31: UInt<32>
    wire _csr_n32: UInt<32>
    wire _csr_n33: UInt<16>
    wire _csr_n34: UInt<1>
    wire _csr_n35: UInt<1>
    wire _csr_n36: UInt<1>
    wire _csr_n37: UInt<1>
    wire _csr_n38: UInt<32>
    wire _csr_n39: UInt<1>
    wire _csr_n40: UInt<1>
    wire _csr_n41: UInt<1>
    wire _csr_n42: UInt<32>
    wire _csr_n43: UInt<32>
    wire _csr_n44: UInt<32>
    wire _csr_n45: UInt<32>
    wire _csr_n46: UInt<32>
    wire _csr_n47: UInt<1>
    wire _csr_n48: UInt<1>
    wire _csr_n49: UInt<1>
    wire _csr_n50: UInt<1>
    wire _csr_n51: UInt<2>
    wire _csr_n52: UInt<2>
    wire _csr_n53: UInt<32>
    wire _csr_n54: UInt<32>
    wire _csr_n55: UInt<32>
    wire _csr_n56: UInt<32>
    wire _csr_n57: UInt<16>
    wire _csr_n58: UInt<1>
    wire _csr_n59: UInt<1>
    wire _csr_n60: UInt<1>
    wire _csr_n61: UInt<1>
    wire _csr_n62: UInt<32>
    wire _csr_n63: UInt<1>
    wire _csr_n64: UInt<1>
    wire _csr_n65: UInt<1>
    wire _csr_n66: UInt<32>
    wire _csr_n67: UInt<32>
    wire _csr_n68: UInt<32>
    wire _csr_n69: UInt<32>
    wire _csr_n70: UInt<32>
    wire _csr_n71: UInt<1>
    wire _csr_n72: UInt<1>
    wire _csr_n73: UInt<1>
    wire _csr_n74: UInt<1>
    wire _csr_n75: UInt<2>
    wire _csr_n76: UInt<2>
    wire _csr_n77: UInt<32>
    wire _csr_n78: UInt<32>
    wire _csr_n79: UInt<32>
    wire _csr_n80: UInt<32>
    wire _csr_n81: UInt<16>
    wire _csr_n82: UInt<1>
    wire _csr_n83: UInt<1>
    wire _csr_n84: UInt<1>
    wire _csr_n85: UInt<1>
    wire _csr_n86: UInt<32>
    wire _csr_n87: UInt<1>
    wire _csr_n88: UInt<1>
    wire _csr_n89: UInt<1>
    wire _csr_n90: UInt<32>
    wire _csr_n91: UInt<32>
    wire _csr_n92: UInt<32>
    wire _csr_n93: UInt<32>
    wire _csr_n94: UInt<32>
    wire _csr_n95: UInt<1>
    wire _csr_n96: UInt<1>
    wire _csr_n97: UInt<1>
    wire _csr_n98: UInt<1>
    wire _csr_n99: UInt<2>
    wire _csr_n100: UInt<2>
    wire _csr_n101: UInt<32>
    wire _csr_n102: UInt<32>
    wire _csr_n103: UInt<32>
    wire _csr_n104: UInt<32>
    wire _csr_n105: UInt<32>
    wire _csr_n106: UInt<16>
    wire _csr_n107: UInt<1>
    wire _csr_n108: UInt<1>
    wire _csr_n109: UInt<1>
    wire _csr_n110: UInt<1>
    wire _csr_n111: UInt<32>
    wire _csr_n112: UInt<1>
    wire _csr_n113: UInt<1>
    wire _csr_n114: UInt<1>
    wire _csr_n115: UInt<32>
    wire _csr_n116: UInt<1>
    wire _csr_n117: UInt<1>
    wire _csr_n118: UInt<1>
    wire _csr_n119: UInt<1>
    wire _csr_n120: UInt<2>
    wire _csr_n121: UInt<2>
    wire _csr_n122: UInt<32>
    wire _csr_n123: UInt<32>
    wire _csr_n124: UInt<32>
    wire _csr_n125: UInt<32>
    wire _csr_n126: UInt<32>
    wire _csr_n127: UInt<32>
    wire _csr_n128: UInt<32>
    wire _csr_n129: UInt<32>
    wire _csr_n130: UInt<32>
    wire _csr_n131: UInt<2>
    connect _csr_n0, UInt(0)
    connect _csr_n1, UInt(1)
    connect _csr_n2, UInt(0)
    connect _csr_n3, UInt(0)
    connect _csr_n4, UInt(2)
    connect _csr_n5, UInt(1)
    connect _csr_n6, xor(_csr_pending, _csr_n5)
    connect _csr_n7, UInt(1)
    connect _csr_n8, xor(rst, _csr_n7)
    connect _csr_n9, and(_csr_n6, _csr_n8)
    connect req_ready, _csr_n9
    connect _csr_n10, and(req_valid, _csr_n9)
    connect _csr_n11, UInt(1)
    connect _csr_n12, xor(write, _csr_n11)
    connect _csr_n13, mux(rsp_ready, _csr_n0, _csr_pending)
    connect _csr_n14, mux(_csr_n10, _csr_n1, _csr_n13)
    connect rsp_valid, _csr_pending
    connect rdata, _csr_data
    connect error, _csr_error
    connect _csr_n15, bits(wstrb, 0, 0)
    connect _csr_n16, UInt(255)
    connect _csr_n17, mux(_csr_n15, _csr_n16, _csr_n2)
    connect _csr_n18, or(_csr_n2, _csr_n17)
    connect _csr_n19, bits(wstrb, 1, 1)
    connect _csr_n20, UInt(65280)
    connect _csr_n21, mux(_csr_n19, _csr_n20, _csr_n2)
    connect _csr_n22, or(_csr_n18, _csr_n21)
    connect _csr_n23, bits(wstrb, 2, 2)
    connect _csr_n24, UInt(16711680)
    connect _csr_n25, mux(_csr_n23, _csr_n24, _csr_n2)
    connect _csr_n26, or(_csr_n22, _csr_n25)
    connect _csr_n27, bits(wstrb, 3, 3)
    connect _csr_n28, UInt(4278190080)
    connect _csr_n29, mux(_csr_n27, _csr_n28, _csr_n2)
    connect _csr_n30, or(_csr_n26, _csr_n29)
    connect _csr_n31, UInt(3)
    connect _csr_n32, and(ctrl_value, _csr_n31)
    connect _csr_n33, UInt(0)
    connect _csr_n34, eq(addr, _csr_n33)
    connect _csr_n35, and(_csr_n10, _csr_n34)
    connect _csr_n36, and(_csr_n12, _csr_n1)
    connect _csr_n37, and(_csr_n35, _csr_n36)
    connect ctrl_read_commit, _csr_n37
    connect _csr_n38, and(_csr_n30, _csr_n31)
    connect _csr_n39, eq(_csr_n38, _csr_n2)
    connect _csr_n40, UInt(1)
    connect _csr_n41, xor(_csr_n39, _csr_n40)
    connect _csr_n42, and(wdata, _csr_n38)
    connect _csr_n43, UInt(4294967295)
    connect _csr_n44, xor(_csr_n38, _csr_n43)
    connect _csr_n45, and(_csr_n32, _csr_n44)
    connect _csr_n46, or(_csr_n45, _csr_n42)
    connect ctrl_write_mask, _csr_n38
    connect ctrl_candidate, _csr_n46
    connect _csr_n47, and(write, _csr_n1)
    connect _csr_n48, and(_csr_n47, _csr_n41)
    connect _csr_n49, and(_csr_n35, _csr_n48)
    connect ctrl_write_commit, _csr_n49
    connect _csr_n50, or(_csr_n36, _csr_n47)
    connect _csr_n51, mux(_csr_n50, _csr_n3, _csr_n4)
    connect _csr_n52, mux(_csr_n34, _csr_n51, _csr_n4)
    connect _csr_n53, mux(_csr_n36, _csr_n32, _csr_n2)
    connect _csr_n54, mux(_csr_n34, _csr_n53, _csr_n2)
    connect _csr_n55, UInt(4294967295)
    connect _csr_n56, and(count_value, _csr_n55)
    connect _csr_n57, UInt(4)
    connect _csr_n58, eq(addr, _csr_n57)
    connect _csr_n59, and(_csr_n10, _csr_n58)
    connect _csr_n60, and(_csr_n12, _csr_n1)
    connect _csr_n61, and(_csr_n59, _csr_n60)
    connect count_read_commit, _csr_n61
    connect _csr_n62, and(_csr_n30, _csr_n55)
    connect _csr_n63, eq(_csr_n62, _csr_n2)
    connect _csr_n64, UInt(1)
    connect _csr_n65, xor(_csr_n63, _csr_n64)
    connect _csr_n66, and(wdata, _csr_n62)
    connect _csr_n67, UInt(4294967295)
    connect _csr_n68, xor(_csr_n62, _csr_n67)
    connect _csr_n69, and(_csr_n56, _csr_n68)
    connect _csr_n70, or(_csr_n69, _csr_n66)
    connect count_write_mask, _csr_n62
    connect count_candidate, _csr_n70
    connect _csr_n71, and(write, _csr_n1)
    connect _csr_n72, and(_csr_n71, _csr_n65)
    connect _csr_n73, and(_csr_n59, _csr_n72)
    connect count_write_commit, _csr_n73
    connect _csr_n74, or(_csr_n60, _csr_n71)
    connect _csr_n75, mux(_csr_n74, _csr_n3, _csr_n4)
    connect _csr_n76, mux(_csr_n58, _csr_n75, _csr_n52)
    connect _csr_n77, mux(_csr_n60, _csr_n56, _csr_n2)
    connect _csr_n78, mux(_csr_n58, _csr_n77, _csr_n54)
    connect _csr_n79, UInt(4294967295)
    connect _csr_n80, and(_csr_value2, _csr_n79)
    connect compare_value, _csr_n80
    connect _csr_n81, UInt(8)
    connect _csr_n82, eq(addr, _csr_n81)
    connect _csr_n83, and(_csr_n10, _csr_n82)
    connect _csr_n84, and(_csr_n12, _csr_n1)
    connect _csr_n85, and(_csr_n83, _csr_n84)
    connect compare_read_commit, _csr_n85
    connect _csr_n86, and(_csr_n30, _csr_n79)
    connect _csr_n87, eq(_csr_n86, _csr_n2)
    connect _csr_n88, UInt(1)
    connect _csr_n89, xor(_csr_n87, _csr_n88)
    connect _csr_n90, and(wdata, _csr_n86)
    connect _csr_n91, UInt(4294967295)
    connect _csr_n92, xor(_csr_n86, _csr_n91)
    connect _csr_n93, and(_csr_n80, _csr_n92)
    connect _csr_n94, or(_csr_n93, _csr_n90)
    connect compare_write_mask, _csr_n86
    connect compare_candidate, _csr_n94
    connect _csr_n95, and(write, _csr_n1)
    connect _csr_n96, and(_csr_n95, _csr_n89)
    connect _csr_n97, and(_csr_n83, _csr_n96)
    connect compare_write_commit, _csr_n97
    connect _csr_n98, or(_csr_n84, _csr_n95)
    connect _csr_n99, mux(_csr_n98, _csr_n3, _csr_n4)
    connect _csr_n100, mux(_csr_n82, _csr_n99, _csr_n76)
    connect _csr_n101, mux(_csr_n84, _csr_n80, _csr_n2)
    connect _csr_n102, mux(_csr_n82, _csr_n101, _csr_n78)
    connect _csr_n103, mux(_csr_n97, _csr_n94, _csr_n80)
    connect _csr_n104, UInt(1)
    connect _csr_n105, and(_csr_value3, _csr_n104)
    connect EVENT_value, _csr_n105
    connect _csr_n106, UInt(12)
    connect _csr_n107, eq(addr, _csr_n106)
    connect _csr_n108, and(_csr_n10, _csr_n107)
    connect _csr_n109, and(_csr_n12, _csr_n1)
    connect _csr_n110, and(_csr_n108, _csr_n109)
    connect EVENT_read_commit, _csr_n110
    connect _csr_n111, and(_csr_n30, _csr_n104)
    connect _csr_n112, eq(_csr_n111, _csr_n2)
    connect _csr_n113, UInt(1)
    connect _csr_n114, xor(_csr_n112, _csr_n113)
    connect _csr_n115, and(wdata, _csr_n111)
    connect EVENT_write_mask, _csr_n111
    connect EVENT_candidate, _csr_n115
    connect _csr_n116, and(write, _csr_n1)
    connect _csr_n117, and(_csr_n116, _csr_n114)
    connect _csr_n118, and(_csr_n108, _csr_n117)
    connect EVENT_write_commit, _csr_n118
    connect _csr_n119, or(_csr_n109, _csr_n116)
    connect _csr_n120, mux(_csr_n119, _csr_n3, _csr_n4)
    connect _csr_n121, mux(_csr_n107, _csr_n120, _csr_n100)
    connect _csr_n122, mux(_csr_n109, _csr_n105, _csr_n2)
    connect _csr_n123, mux(_csr_n107, _csr_n122, _csr_n102)
    connect _csr_n124, mux(_csr_n118, _csr_n115, _csr_n2)
    connect _csr_n125, UInt(4294967295)
    connect _csr_n126, xor(_csr_n124, _csr_n125)
    connect _csr_n127, and(_csr_n105, _csr_n126)
    connect _csr_n128, and(match_bits, _csr_n104)
    connect _csr_n129, or(_csr_n127, _csr_n128)
    connect _csr_n130, mux(_csr_n10, _csr_n123, _csr_data)
    connect _csr_n131, mux(_csr_n10, _csr_n121, _csr_error)
    connect _csr_pending, _csr_n14
    connect _csr_data, _csr_n130
    connect _csr_error, _csr_n131
    connect _csr_value2, _csr_n103
    connect _csr_value3, _csr_n129
  module Fr198Timer :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    output match_event: UInt<1>
    regreset ctrl_value: UInt<32>, clk, rst, UInt(0)
    regreset count_value: UInt<32>, clk, rst, UInt(0)
    wire ctrl_candidate: UInt<32>
    wire ctrl_write_mask: UInt<32>
    wire ctrl_read_commit: UInt<1>
    wire ctrl_write_commit: UInt<1>
    wire count_candidate: UInt<32>
    wire count_write_mask: UInt<32>
    wire count_read_commit: UInt<1>
    wire count_write_commit: UInt<1>
    wire compare_value: UInt<32>
    wire compare_candidate: UInt<32>
    wire compare_write_mask: UInt<32>
    wire compare_read_commit: UInt<1>
    wire compare_write_commit: UInt<1>
    wire EVENT_value: UInt<32>
    wire EVENT_candidate: UInt<32>
    wire EVENT_write_mask: UInt<32>
    wire EVENT_read_commit: UInt<1>
    wire EVENT_write_commit: UInt<1>
    wire match_bits: UInt<32>
    wire zero32: UInt<32>
    wire one32: UInt<32>
    wire ctrl_keep: UInt<32>
    wire one: UInt<1>
    wire active: UInt<1>
    wire enabled: UInt<1>
    wire periodic: UInt<1>
    wire not_periodic: UInt<1>
    wire write_ab: UInt<1>
    wire config_write: UInt<1>
    wire no_write: UInt<1>
    wire running: UInt<1>
    wire advance: UInt<1>
    wire increment: UInt<32>
    wire equal: UInt<1>
    wire restart: UInt<1>
    wire stop: UInt<1>
    wire ctrl_stopped: UInt<32>
    wire ctrl_auto: UInt<32>
    wire ctrl_next: UInt<32>
    wire count_auto: UInt<32>
    wire count_running: UInt<32>
    wire count_next: UInt<32>
    inst csr of BitloomTimerCsr
    connect csr.clk, clk
    connect csr.rst, rst
    connect csr.req_valid, req_valid
    connect csr.write, write
    connect csr.addr, addr
    connect csr.wdata, wdata
    connect csr.wstrb, wstrb
    connect csr.rsp_ready, rsp_ready
    connect req_ready, csr.req_ready
    connect rsp_valid, csr.rsp_valid
    connect rdata, csr.rdata
    connect error, csr.error
    connect csr.ctrl_value, ctrl_value
    connect ctrl_candidate, csr.ctrl_candidate
    connect ctrl_write_mask, csr.ctrl_write_mask
    connect ctrl_read_commit, csr.ctrl_read_commit
    connect ctrl_write_commit, csr.ctrl_write_commit
    connect csr.count_value, count_value
    connect count_candidate, csr.count_candidate
    connect count_write_mask, csr.count_write_mask
    connect count_read_commit, csr.count_read_commit
    connect count_write_commit, csr.count_write_commit
    connect compare_value, csr.compare_value
    connect compare_candidate, csr.compare_candidate
    connect compare_write_mask, csr.compare_write_mask
    connect compare_read_commit, csr.compare_read_commit
    connect compare_write_commit, csr.compare_write_commit
    connect EVENT_value, csr.EVENT_value
    connect EVENT_candidate, csr.EVENT_candidate
    connect EVENT_write_mask, csr.EVENT_write_mask
    connect EVENT_read_commit, csr.EVENT_read_commit
    connect EVENT_write_commit, csr.EVENT_write_commit
    connect csr.match_bits, match_bits
    connect zero32, UInt(0)
    connect one32, UInt(1)
    connect ctrl_keep, UInt(2)
    connect one, UInt(1)
    connect active, xor(rst, one)
    connect enabled, bits(ctrl_value, 0, 0)
    connect periodic, bits(ctrl_value, 1, 1)
    connect not_periodic, xor(periodic, one)
    connect write_ab, or(ctrl_write_commit, count_write_commit)
    connect config_write, or(write_ab, compare_write_commit)
    connect no_write, xor(config_write, one)
    connect running, and(enabled, active)
    connect advance, and(running, no_write)
    connect increment, add(count_value, one32)
    connect equal, eq(increment, compare_value)
    connect match_event, and(advance, equal)
    connect match_bits, mux(match_event, one32, zero32)
    connect restart, and(match_event, periodic)
    connect stop, and(match_event, not_periodic)
    connect ctrl_stopped, and(ctrl_value, ctrl_keep)
    connect ctrl_auto, mux(stop, ctrl_stopped, ctrl_value)
    connect ctrl_next, mux(ctrl_write_commit, ctrl_candidate, ctrl_auto)
    connect count_auto, mux(restart, zero32, increment)
    connect count_running, mux(advance, count_auto, count_value)
    connect count_next, mux(count_write_commit, count_candidate, count_running)
    connect ctrl_value, ctrl_next
    connect count_value, count_next
  module BitloomIrqCsr :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    output pending_read_commit: UInt<1>
    output pending_write_commit: UInt<1>
    output pending_value: UInt<32>
    output pending_candidate: UInt<32>
    output pending_write_mask: UInt<32>
    input event_bits: UInt<32>
    output enable_read_commit: UInt<1>
    output enable_write_commit: UInt<1>
    output enable_value: UInt<32>
    output enable_candidate: UInt<32>
    output enable_write_mask: UInt<32>
    output test_read_commit: UInt<1>
    output test_write_commit: UInt<1>
    output test_candidate: UInt<32>
    output test_write_mask: UInt<32>
    output raw_read_commit: UInt<1>
    output raw_write_commit: UInt<1>
    input raw_value: UInt<32>
    regreset _csr_pending: UInt<1>, clk, rst, UInt(0)
    regreset _csr_data: UInt<32>, clk, rst, UInt(0)
    regreset _csr_error: UInt<2>, clk, rst, UInt(0)
    regreset _csr_value0: UInt<32>, clk, rst, UInt(0)
    regreset _csr_value1: UInt<32>, clk, rst, UInt(0)
    wire _csr_n0: UInt<1>
    wire _csr_n1: UInt<1>
    wire _csr_n2: UInt<32>
    wire _csr_n3: UInt<2>
    wire _csr_n4: UInt<2>
    wire _csr_n5: UInt<1>
    wire _csr_n6: UInt<1>
    wire _csr_n7: UInt<1>
    wire _csr_n8: UInt<1>
    wire _csr_n9: UInt<1>
    wire _csr_n10: UInt<1>
    wire _csr_n11: UInt<1>
    wire _csr_n12: UInt<1>
    wire _csr_n13: UInt<1>
    wire _csr_n14: UInt<1>
    wire _csr_n15: UInt<1>
    wire _csr_n16: UInt<32>
    wire _csr_n17: UInt<32>
    wire _csr_n18: UInt<32>
    wire _csr_n19: UInt<1>
    wire _csr_n20: UInt<32>
    wire _csr_n21: UInt<32>
    wire _csr_n22: UInt<32>
    wire _csr_n23: UInt<1>
    wire _csr_n24: UInt<32>
    wire _csr_n25: UInt<32>
    wire _csr_n26: UInt<32>
    wire _csr_n27: UInt<1>
    wire _csr_n28: UInt<32>
    wire _csr_n29: UInt<32>
    wire _csr_n30: UInt<32>
    wire _csr_n31: UInt<32>
    wire _csr_n32: UInt<32>
    wire _csr_n33: UInt<16>
    wire _csr_n34: UInt<1>
    wire _csr_n35: UInt<1>
    wire _csr_n36: UInt<1>
    wire _csr_n37: UInt<1>
    wire _csr_n38: UInt<32>
    wire _csr_n39: UInt<1>
    wire _csr_n40: UInt<1>
    wire _csr_n41: UInt<1>
    wire _csr_n42: UInt<32>
    wire _csr_n43: UInt<1>
    wire _csr_n44: UInt<1>
    wire _csr_n45: UInt<1>
    wire _csr_n46: UInt<1>
    wire _csr_n47: UInt<2>
    wire _csr_n48: UInt<2>
    wire _csr_n49: UInt<32>
    wire _csr_n50: UInt<32>
    wire _csr_n51: UInt<32>
    wire _csr_n52: UInt<32>
    wire _csr_n53: UInt<32>
    wire _csr_n54: UInt<32>
    wire _csr_n55: UInt<32>
    wire _csr_n56: UInt<32>
    wire _csr_n57: UInt<32>
    wire _csr_n58: UInt<32>
    wire _csr_n59: UInt<16>
    wire _csr_n60: UInt<1>
    wire _csr_n61: UInt<1>
    wire _csr_n62: UInt<1>
    wire _csr_n63: UInt<1>
    wire _csr_n64: UInt<32>
    wire _csr_n65: UInt<1>
    wire _csr_n66: UInt<1>
    wire _csr_n67: UInt<1>
    wire _csr_n68: UInt<32>
    wire _csr_n69: UInt<32>
    wire _csr_n70: UInt<32>
    wire _csr_n71: UInt<32>
    wire _csr_n72: UInt<32>
    wire _csr_n73: UInt<1>
    wire _csr_n74: UInt<1>
    wire _csr_n75: UInt<1>
    wire _csr_n76: UInt<1>
    wire _csr_n77: UInt<2>
    wire _csr_n78: UInt<2>
    wire _csr_n79: UInt<32>
    wire _csr_n80: UInt<32>
    wire _csr_n81: UInt<32>
    wire _csr_n82: UInt<32>
    wire _csr_n83: UInt<16>
    wire _csr_n84: UInt<1>
    wire _csr_n85: UInt<1>
    wire _csr_n86: UInt<1>
    wire _csr_n87: UInt<1>
    wire _csr_n88: UInt<32>
    wire _csr_n89: UInt<1>
    wire _csr_n90: UInt<1>
    wire _csr_n91: UInt<1>
    wire _csr_n92: UInt<32>
    wire _csr_n93: UInt<1>
    wire _csr_n94: UInt<1>
    wire _csr_n95: UInt<1>
    wire _csr_n96: UInt<1>
    wire _csr_n97: UInt<2>
    wire _csr_n98: UInt<2>
    wire _csr_n99: UInt<32>
    wire _csr_n100: UInt<32>
    wire _csr_n101: UInt<32>
    wire _csr_n102: UInt<32>
    wire _csr_n103: UInt<16>
    wire _csr_n104: UInt<1>
    wire _csr_n105: UInt<1>
    wire _csr_n106: UInt<1>
    wire _csr_n107: UInt<1>
    wire _csr_n108: UInt<1>
    wire _csr_n109: UInt<2>
    wire _csr_n110: UInt<2>
    wire _csr_n111: UInt<32>
    wire _csr_n112: UInt<32>
    wire _csr_n113: UInt<32>
    wire _csr_n114: UInt<2>
    connect _csr_n0, UInt(0)
    connect _csr_n1, UInt(1)
    connect _csr_n2, UInt(0)
    connect _csr_n3, UInt(0)
    connect _csr_n4, UInt(2)
    connect _csr_n5, UInt(1)
    connect _csr_n6, xor(_csr_pending, _csr_n5)
    connect _csr_n7, UInt(1)
    connect _csr_n8, xor(rst, _csr_n7)
    connect _csr_n9, and(_csr_n6, _csr_n8)
    connect req_ready, _csr_n9
    connect _csr_n10, and(req_valid, _csr_n9)
    connect _csr_n11, UInt(1)
    connect _csr_n12, xor(write, _csr_n11)
    connect _csr_n13, mux(rsp_ready, _csr_n0, _csr_pending)
    connect _csr_n14, mux(_csr_n10, _csr_n1, _csr_n13)
    connect rsp_valid, _csr_pending
    connect rdata, _csr_data
    connect error, _csr_error
    connect _csr_n15, bits(wstrb, 0, 0)
    connect _csr_n16, UInt(255)
    connect _csr_n17, mux(_csr_n15, _csr_n16, _csr_n2)
    connect _csr_n18, or(_csr_n2, _csr_n17)
    connect _csr_n19, bits(wstrb, 1, 1)
    connect _csr_n20, UInt(65280)
    connect _csr_n21, mux(_csr_n19, _csr_n20, _csr_n2)
    connect _csr_n22, or(_csr_n18, _csr_n21)
    connect _csr_n23, bits(wstrb, 2, 2)
    connect _csr_n24, UInt(16711680)
    connect _csr_n25, mux(_csr_n23, _csr_n24, _csr_n2)
    connect _csr_n26, or(_csr_n22, _csr_n25)
    connect _csr_n27, bits(wstrb, 3, 3)
    connect _csr_n28, UInt(4278190080)
    connect _csr_n29, mux(_csr_n27, _csr_n28, _csr_n2)
    connect _csr_n30, or(_csr_n26, _csr_n29)
    connect _csr_n31, UInt(31)
    connect _csr_n32, and(_csr_value0, _csr_n31)
    connect pending_value, _csr_n32
    connect _csr_n33, UInt(0)
    connect _csr_n34, eq(addr, _csr_n33)
    connect _csr_n35, and(_csr_n10, _csr_n34)
    connect _csr_n36, and(_csr_n12, _csr_n1)
    connect _csr_n37, and(_csr_n35, _csr_n36)
    connect pending_read_commit, _csr_n37
    connect _csr_n38, and(_csr_n30, _csr_n31)
    connect _csr_n39, eq(_csr_n38, _csr_n2)
    connect _csr_n40, UInt(1)
    connect _csr_n41, xor(_csr_n39, _csr_n40)
    connect _csr_n42, and(wdata, _csr_n38)
    connect pending_write_mask, _csr_n38
    connect pending_candidate, _csr_n42
    connect _csr_n43, and(write, _csr_n1)
    connect _csr_n44, and(_csr_n43, _csr_n41)
    connect _csr_n45, and(_csr_n35, _csr_n44)
    connect pending_write_commit, _csr_n45
    connect _csr_n46, or(_csr_n36, _csr_n43)
    connect _csr_n47, mux(_csr_n46, _csr_n3, _csr_n4)
    connect _csr_n48, mux(_csr_n34, _csr_n47, _csr_n4)
    connect _csr_n49, mux(_csr_n36, _csr_n32, _csr_n2)
    connect _csr_n50, mux(_csr_n34, _csr_n49, _csr_n2)
    connect _csr_n51, mux(_csr_n45, _csr_n42, _csr_n2)
    connect _csr_n52, UInt(4294967295)
    connect _csr_n53, xor(_csr_n51, _csr_n52)
    connect _csr_n54, and(_csr_n32, _csr_n53)
    connect _csr_n55, and(event_bits, _csr_n31)
    connect _csr_n56, or(_csr_n54, _csr_n55)
    connect _csr_n57, UInt(31)
    connect _csr_n58, and(_csr_value1, _csr_n57)
    connect enable_value, _csr_n58
    connect _csr_n59, UInt(4)
    connect _csr_n60, eq(addr, _csr_n59)
    connect _csr_n61, and(_csr_n10, _csr_n60)
    connect _csr_n62, and(_csr_n12, _csr_n1)
    connect _csr_n63, and(_csr_n61, _csr_n62)
    connect enable_read_commit, _csr_n63
    connect _csr_n64, and(_csr_n30, _csr_n57)
    connect _csr_n65, eq(_csr_n64, _csr_n2)
    connect _csr_n66, UInt(1)
    connect _csr_n67, xor(_csr_n65, _csr_n66)
    connect _csr_n68, and(wdata, _csr_n64)
    connect _csr_n69, UInt(4294967295)
    connect _csr_n70, xor(_csr_n64, _csr_n69)
    connect _csr_n71, and(_csr_n58, _csr_n70)
    connect _csr_n72, or(_csr_n71, _csr_n68)
    connect enable_write_mask, _csr_n64
    connect enable_candidate, _csr_n72
    connect _csr_n73, and(write, _csr_n1)
    connect _csr_n74, and(_csr_n73, _csr_n67)
    connect _csr_n75, and(_csr_n61, _csr_n74)
    connect enable_write_commit, _csr_n75
    connect _csr_n76, or(_csr_n62, _csr_n73)
    connect _csr_n77, mux(_csr_n76, _csr_n3, _csr_n4)
    connect _csr_n78, mux(_csr_n60, _csr_n77, _csr_n48)
    connect _csr_n79, mux(_csr_n62, _csr_n58, _csr_n2)
    connect _csr_n80, mux(_csr_n60, _csr_n79, _csr_n50)
    connect _csr_n81, mux(_csr_n75, _csr_n72, _csr_n58)
    connect _csr_n82, UInt(31)
    connect _csr_n83, UInt(8)
    connect _csr_n84, eq(addr, _csr_n83)
    connect _csr_n85, and(_csr_n10, _csr_n84)
    connect _csr_n86, and(_csr_n12, _csr_n0)
    connect _csr_n87, and(_csr_n85, _csr_n86)
    connect test_read_commit, _csr_n87
    connect _csr_n88, and(_csr_n30, _csr_n82)
    connect _csr_n89, eq(_csr_n88, _csr_n2)
    connect _csr_n90, UInt(1)
    connect _csr_n91, xor(_csr_n89, _csr_n90)
    connect _csr_n92, and(wdata, _csr_n88)
    connect test_write_mask, _csr_n88
    connect test_candidate, _csr_n92
    connect _csr_n93, and(write, _csr_n1)
    connect _csr_n94, and(_csr_n93, _csr_n91)
    connect _csr_n95, and(_csr_n85, _csr_n94)
    connect test_write_commit, _csr_n95
    connect _csr_n96, or(_csr_n86, _csr_n93)
    connect _csr_n97, mux(_csr_n96, _csr_n3, _csr_n4)
    connect _csr_n98, mux(_csr_n84, _csr_n97, _csr_n78)
    connect _csr_n99, mux(_csr_n86, _csr_n2, _csr_n2)
    connect _csr_n100, mux(_csr_n84, _csr_n99, _csr_n80)
    connect _csr_n101, UInt(31)
    connect _csr_n102, and(raw_value, _csr_n101)
    connect _csr_n103, UInt(12)
    connect _csr_n104, eq(addr, _csr_n103)
    connect _csr_n105, and(_csr_n10, _csr_n104)
    connect _csr_n106, and(_csr_n12, _csr_n1)
    connect _csr_n107, and(_csr_n105, _csr_n106)
    connect raw_read_commit, _csr_n107
    connect raw_write_commit, _csr_n0
    connect _csr_n108, or(_csr_n106, _csr_n0)
    connect _csr_n109, mux(_csr_n108, _csr_n3, _csr_n4)
    connect _csr_n110, mux(_csr_n104, _csr_n109, _csr_n98)
    connect _csr_n111, mux(_csr_n106, _csr_n102, _csr_n2)
    connect _csr_n112, mux(_csr_n104, _csr_n111, _csr_n100)
    connect _csr_n113, mux(_csr_n10, _csr_n112, _csr_data)
    connect _csr_n114, mux(_csr_n10, _csr_n110, _csr_error)
    connect _csr_pending, _csr_n14
    connect _csr_data, _csr_n113
    connect _csr_error, _csr_n114
    connect _csr_value0, _csr_n56
    connect _csr_value1, _csr_n81
  module Fr198Irq :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input req_valid: UInt<1>
    input write: UInt<1>
    input addr: UInt<16>
    input wdata: UInt<32>
    input wstrb: UInt<4>
    input rsp_ready: UInt<1>
    output req_ready: UInt<1>
    output rsp_valid: UInt<1>
    output rdata: UInt<32>
    output error: UInt<2>
    input raw_events: UInt<5>
    output irq: UInt<1>
    wire pending_read_commit: UInt<1>
    wire pending_write_commit: UInt<1>
    wire pending_value: UInt<32>
    wire pending_candidate: UInt<32>
    wire pending_write_mask: UInt<32>
    wire enable_read_commit: UInt<1>
    wire enable_write_commit: UInt<1>
    wire enable_value: UInt<32>
    wire enable_candidate: UInt<32>
    wire enable_write_mask: UInt<32>
    wire test_read_commit: UInt<1>
    wire test_write_commit: UInt<1>
    wire test_candidate: UInt<32>
    wire test_write_mask: UInt<32>
    wire raw_read_commit: UInt<1>
    wire raw_write_commit: UInt<1>
    wire raw_value: UInt<32>
    wire event_bits: UInt<32>
    wire test_bits: UInt<32>
    wire masked: UInt<32>
    wire zero32: UInt<32>
    wire zero27: UInt<27>
    wire empty: UInt<1>
    wire one: UInt<1>
    inst csr of BitloomIrqCsr
    connect csr.clk, clk
    connect csr.rst, rst
    connect csr.req_valid, req_valid
    connect csr.write, write
    connect csr.addr, addr
    connect csr.wdata, wdata
    connect csr.wstrb, wstrb
    connect csr.rsp_ready, rsp_ready
    connect req_ready, csr.req_ready
    connect rsp_valid, csr.rsp_valid
    connect rdata, csr.rdata
    connect error, csr.error
    connect pending_read_commit, csr.pending_read_commit
    connect pending_write_commit, csr.pending_write_commit
    connect pending_value, csr.pending_value
    connect pending_candidate, csr.pending_candidate
    connect pending_write_mask, csr.pending_write_mask
    connect enable_read_commit, csr.enable_read_commit
    connect enable_write_commit, csr.enable_write_commit
    connect enable_value, csr.enable_value
    connect enable_candidate, csr.enable_candidate
    connect enable_write_mask, csr.enable_write_mask
    connect test_read_commit, csr.test_read_commit
    connect test_write_commit, csr.test_write_commit
    connect test_candidate, csr.test_candidate
    connect test_write_mask, csr.test_write_mask
    connect raw_read_commit, csr.raw_read_commit
    connect raw_write_commit, csr.raw_write_commit
    connect csr.raw_value, raw_value
    connect csr.event_bits, event_bits
    connect zero32, UInt(0)
    connect zero27, UInt(0)
    connect one, UInt(1)
    connect raw_value, cat(zero27, raw_events)
    connect test_bits, mux(test_write_commit, test_candidate, zero32)
    connect event_bits, or(raw_value, test_bits)
    connect masked, and(pending_value, enable_value)
    connect empty, eq(masked, zero32)
    connect irq, xor(empty, one)
  public module Fr198DirectCore :
    input clk: Clock
    input rst: UInt<1> ; bitloom-type: Reset
    input csr_req_valid: UInt<1>
    input csr_write: UInt<1>
    input csr_addr: UInt<16>
    input csr_wdata: UInt<32>
    input csr_wstrb: UInt<4>
    input csr_rsp_ready: UInt<1>
    output csr_req_ready: UInt<1>
    output csr_rsp_valid: UInt<1>
    output csr_rdata: UInt<32>
    output csr_error: UInt<2>
    input u_rx: UInt<1>
    output u_tx: UInt<1>
    input g_pad_in: UInt<32>
    output g_pad_out: UInt<32>
    output g_pad_oe: UInt<32>
    output i_irq: UInt<1>
    wire csr_uart_req_ready: UInt<1>
    wire csr_uart_rsp_valid: UInt<1>
    wire csr_uart_rdata: UInt<32>
    wire csr_uart_error: UInt<2>
    wire csr_uart_req_valid: UInt<1>
    wire csr_uart_write: UInt<1>
    wire csr_uart_addr: UInt<16>
    wire csr_uart_wdata: UInt<32>
    wire csr_uart_wstrb: UInt<4>
    wire csr_uart_rsp_ready: UInt<1>
    wire csr_gpio_req_ready: UInt<1>
    wire csr_gpio_rsp_valid: UInt<1>
    wire csr_gpio_rdata: UInt<32>
    wire csr_gpio_error: UInt<2>
    wire csr_gpio_req_valid: UInt<1>
    wire csr_gpio_write: UInt<1>
    wire csr_gpio_addr: UInt<16>
    wire csr_gpio_wdata: UInt<32>
    wire csr_gpio_wstrb: UInt<4>
    wire csr_gpio_rsp_ready: UInt<1>
    wire csr_timer_req_ready: UInt<1>
    wire csr_timer_rsp_valid: UInt<1>
    wire csr_timer_rdata: UInt<32>
    wire csr_timer_error: UInt<2>
    wire csr_timer_req_valid: UInt<1>
    wire csr_timer_write: UInt<1>
    wire csr_timer_addr: UInt<16>
    wire csr_timer_wdata: UInt<32>
    wire csr_timer_wstrb: UInt<4>
    wire csr_timer_rsp_ready: UInt<1>
    wire csr_irq_req_ready: UInt<1>
    wire csr_irq_rsp_valid: UInt<1>
    wire csr_irq_rdata: UInt<32>
    wire csr_irq_error: UInt<2>
    wire csr_irq_req_valid: UInt<1>
    wire csr_irq_write: UInt<1>
    wire csr_irq_addr: UInt<16>
    wire csr_irq_wdata: UInt<32>
    wire csr_irq_wstrb: UInt<4>
    wire csr_irq_rsp_ready: UInt<1>
    wire u_req_valid: UInt<1>
    wire u_write: UInt<1>
    wire u_addr: UInt<16>
    wire u_wdata: UInt<32>
    wire u_wstrb: UInt<4>
    wire u_rsp_ready: UInt<1>
    wire u_req_ready: UInt<1>
    wire u_rsp_valid: UInt<1>
    wire u_rdata: UInt<32>
    wire u_error: UInt<2>
    wire u_raw_events: UInt<4>
    wire g_req_valid: UInt<1>
    wire g_write: UInt<1>
    wire g_addr: UInt<16>
    wire g_wdata: UInt<32>
    wire g_wstrb: UInt<4>
    wire g_rsp_ready: UInt<1>
    wire g_req_ready: UInt<1>
    wire g_rsp_valid: UInt<1>
    wire g_rdata: UInt<32>
    wire g_error: UInt<2>
    wire g_raw_event: UInt<1>
    wire t_req_valid: UInt<1>
    wire t_write: UInt<1>
    wire t_addr: UInt<16>
    wire t_wdata: UInt<32>
    wire t_wstrb: UInt<4>
    wire t_rsp_ready: UInt<1>
    wire t_req_ready: UInt<1>
    wire t_rsp_valid: UInt<1>
    wire t_rdata: UInt<32>
    wire t_error: UInt<2>
    wire t_match_event: UInt<1>
    wire i_req_valid: UInt<1>
    wire i_write: UInt<1>
    wire i_addr: UInt<16>
    wire i_wdata: UInt<32>
    wire i_wstrb: UInt<4>
    wire i_rsp_ready: UInt<1>
    wire i_req_ready: UInt<1>
    wire i_rsp_valid: UInt<1>
    wire i_rdata: UInt<32>
    wire i_error: UInt<2>
    wire i_raw_events: UInt<5>
    wire uart_error_event: UInt<1>
    wire irq_mid_a: UInt<2>
    wire irq_mid_b: UInt<3>
    wire irq_mid_c: UInt<4>
    wire irq_events: UInt<5>
    wire u_rx_event: UInt<1>
    wire u_tx_event: UInt<1>
    wire u_err_event: UInt<1>
    wire u_framing_event: UInt<1>
    inst uart of Fr198Uart
    connect uart.clk, clk
    connect uart.rst, rst
    connect uart.req_valid, u_req_valid
    connect uart.write, u_write
    connect uart.addr, u_addr
    connect uart.wdata, u_wdata
    connect uart.wstrb, u_wstrb
    connect uart.rsp_ready, u_rsp_ready
    connect u_req_ready, uart.req_ready
    connect u_rsp_valid, uart.rsp_valid
    connect u_rdata, uart.rdata
    connect u_error, uart.error
    connect uart.rx, u_rx
    connect u_tx, uart.tx
    connect u_raw_events, uart.raw_events
    inst gpio of Fr198Gpio
    connect gpio.clk, clk
    connect gpio.rst, rst
    connect gpio.req_valid, g_req_valid
    connect gpio.write, g_write
    connect gpio.addr, g_addr
    connect gpio.wdata, g_wdata
    connect gpio.wstrb, g_wstrb
    connect gpio.rsp_ready, g_rsp_ready
    connect g_req_ready, gpio.req_ready
    connect g_rsp_valid, gpio.rsp_valid
    connect g_rdata, gpio.rdata
    connect g_error, gpio.error
    connect gpio.pad_in, g_pad_in
    connect g_pad_out, gpio.pad_out
    connect g_pad_oe, gpio.pad_oe
    connect g_raw_event, gpio.raw_event
    inst timer of Fr198Timer
    connect timer.clk, clk
    connect timer.rst, rst
    connect timer.req_valid, t_req_valid
    connect timer.write, t_write
    connect timer.addr, t_addr
    connect timer.wdata, t_wdata
    connect timer.wstrb, t_wstrb
    connect timer.rsp_ready, t_rsp_ready
    connect t_req_ready, timer.req_ready
    connect t_rsp_valid, timer.rsp_valid
    connect t_rdata, timer.rdata
    connect t_error, timer.error
    connect t_match_event, timer.match_event
    inst irq of Fr198Irq
    connect irq.clk, clk
    connect irq.rst, rst
    connect irq.req_valid, i_req_valid
    connect irq.write, i_write
    connect irq.addr, i_addr
    connect irq.wdata, i_wdata
    connect irq.wstrb, i_wstrb
    connect irq.rsp_ready, i_rsp_ready
    connect i_req_ready, irq.req_ready
    connect i_rsp_valid, irq.rsp_valid
    connect i_rdata, irq.rdata
    connect i_error, irq.error
    connect irq.raw_events, i_raw_events
    connect i_irq, irq.irq
    inst decoder of Fr198Decoder
    connect decoder.clk, clk
    connect decoder.rst, rst
    connect decoder.req_valid, csr_req_valid
    connect decoder.write, csr_write
    connect decoder.addr, csr_addr
    connect decoder.wdata, csr_wdata
    connect decoder.wstrb, csr_wstrb
    connect decoder.rsp_ready, csr_rsp_ready
    connect csr_req_ready, decoder.req_ready
    connect csr_rsp_valid, decoder.rsp_valid
    connect csr_rdata, decoder.rdata
    connect csr_error, decoder.error
    connect decoder.uart_req_ready, csr_uart_req_ready
    connect decoder.uart_rsp_valid, csr_uart_rsp_valid
    connect decoder.uart_rdata, csr_uart_rdata
    connect decoder.uart_error, csr_uart_error
    connect csr_uart_req_valid, decoder.uart_req_valid
    connect csr_uart_write, decoder.uart_write
    connect csr_uart_addr, decoder.uart_addr
    connect csr_uart_wdata, decoder.uart_wdata
    connect csr_uart_wstrb, decoder.uart_wstrb
    connect csr_uart_rsp_ready, decoder.uart_rsp_ready
    connect decoder.gpio_req_ready, csr_gpio_req_ready
    connect decoder.gpio_rsp_valid, csr_gpio_rsp_valid
    connect decoder.gpio_rdata, csr_gpio_rdata
    connect decoder.gpio_error, csr_gpio_error
    connect csr_gpio_req_valid, decoder.gpio_req_valid
    connect csr_gpio_write, decoder.gpio_write
    connect csr_gpio_addr, decoder.gpio_addr
    connect csr_gpio_wdata, decoder.gpio_wdata
    connect csr_gpio_wstrb, decoder.gpio_wstrb
    connect csr_gpio_rsp_ready, decoder.gpio_rsp_ready
    connect decoder.timer_req_ready, csr_timer_req_ready
    connect decoder.timer_rsp_valid, csr_timer_rsp_valid
    connect decoder.timer_rdata, csr_timer_rdata
    connect decoder.timer_error, csr_timer_error
    connect csr_timer_req_valid, decoder.timer_req_valid
    connect csr_timer_write, decoder.timer_write
    connect csr_timer_addr, decoder.timer_addr
    connect csr_timer_wdata, decoder.timer_wdata
    connect csr_timer_wstrb, decoder.timer_wstrb
    connect csr_timer_rsp_ready, decoder.timer_rsp_ready
    connect decoder.irq_req_ready, csr_irq_req_ready
    connect decoder.irq_rsp_valid, csr_irq_rsp_valid
    connect decoder.irq_rdata, csr_irq_rdata
    connect decoder.irq_error, csr_irq_error
    connect csr_irq_req_valid, decoder.irq_req_valid
    connect csr_irq_write, decoder.irq_write
    connect csr_irq_addr, decoder.irq_addr
    connect csr_irq_wdata, decoder.irq_wdata
    connect csr_irq_wstrb, decoder.irq_wstrb
    connect csr_irq_rsp_ready, decoder.irq_rsp_ready
    connect u_req_valid, csr_uart_req_valid
    connect u_write, csr_uart_write
    connect u_addr, csr_uart_addr
    connect u_wdata, csr_uart_wdata
    connect u_wstrb, csr_uart_wstrb
    connect u_rsp_ready, csr_uart_rsp_ready
    connect csr_uart_req_ready, u_req_ready
    connect csr_uart_rsp_valid, u_rsp_valid
    connect csr_uart_rdata, u_rdata
    connect csr_uart_error, u_error
    connect g_req_valid, csr_gpio_req_valid
    connect g_write, csr_gpio_write
    connect g_addr, csr_gpio_addr
    connect g_wdata, csr_gpio_wdata
    connect g_wstrb, csr_gpio_wstrb
    connect g_rsp_ready, csr_gpio_rsp_ready
    connect csr_gpio_req_ready, g_req_ready
    connect csr_gpio_rsp_valid, g_rsp_valid
    connect csr_gpio_rdata, g_rdata
    connect csr_gpio_error, g_error
    connect t_req_valid, csr_timer_req_valid
    connect t_write, csr_timer_write
    connect t_addr, csr_timer_addr
    connect t_wdata, csr_timer_wdata
    connect t_wstrb, csr_timer_wstrb
    connect t_rsp_ready, csr_timer_rsp_ready
    connect csr_timer_req_ready, t_req_ready
    connect csr_timer_rsp_valid, t_rsp_valid
    connect csr_timer_rdata, t_rdata
    connect csr_timer_error, t_error
    connect i_req_valid, csr_irq_req_valid
    connect i_write, csr_irq_write
    connect i_addr, csr_irq_addr
    connect i_wdata, csr_irq_wdata
    connect i_wstrb, csr_irq_wstrb
    connect i_rsp_ready, csr_irq_rsp_ready
    connect csr_irq_req_ready, i_req_ready
    connect csr_irq_rsp_valid, i_rsp_valid
    connect csr_irq_rdata, i_rdata
    connect csr_irq_error, i_error
    connect u_rx_event, bits(u_raw_events, 0, 0)
    connect u_tx_event, bits(u_raw_events, 1, 1)
    connect u_err_event, bits(u_raw_events, 2, 2)
    connect u_framing_event, bits(u_raw_events, 3, 3)
    connect uart_error_event, or(u_err_event, u_framing_event)
    connect irq_mid_a, cat(u_rx_event, t_match_event)
    connect irq_mid_b, cat(u_tx_event, irq_mid_a)
    connect irq_mid_c, cat(uart_error_event, irq_mid_b)
    connect irq_events, cat(g_raw_event, irq_mid_c)
    connect i_raw_events, irq_events
