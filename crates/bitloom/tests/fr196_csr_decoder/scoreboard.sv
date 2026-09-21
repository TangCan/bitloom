// Port-level independent scoreboard. Golden offsets, permissions and masks are
// literal requirements; none are extracted from the product descriptor/HIR.
integer aw_in=0, aw_out=0, w_in=0, w_out=0, ar_in=0, ar_out=0;
reg [15:0] aw_queue[0:65535], ar_queue[0:65535];
reg [31:0] w_queue[0:65535]; reg [3:0] st_queue[0:65535];
integer aw_cycle_queue[0:65535],w_cycle_queue[0:65535];
integer observed_gap=0,observed_order=2; // 0 AW first, 1 W first, 2 simultaneous
integer b_in=0,b_out=0,r_in=0,r_out=0;
reg [1:0] b_queue[0:65535],r_error_queue[0:65535];
reg [31:0] r_queue[0:65535];
reg [31:0] model[0:1023];
reg pending=0, pending_write;reg [1:0] pending_error;reg [31:0] pending_data;
integer accepted_aw=0,accepted_w=0,accepted_ar=0,commits=0,leaf_commits=0,csr_responses=0,consumed_b=0,consumed_r=0,canceled=0,side_effects=0;
integer wo_effects=0,w1c_effects=0,reject_reads=0;
integer canceled_aw=0,canceled_w=0,canceled_ar=0,canceled_csr=0,canceled_b=0,canceled_r=0,read_effects=0;
integer windows[0:4],errors[0:3],strobes[0:15],read_count=0,write_count=0;
reg hold_aw=0,hold_w=0,hold_ar=0,hold_csr=0,hold_b=0,hold_r=0;
reg [18:0] prev_aw,prev_ar;reg [35:0] prev_w;
reg [52:0] prev_csr;reg [1:0] prev_b;reg [33:0] prev_r;
integer expected_side=-1;reg expected_write=0;
integer cycle_count=0,idx,j,kind,window_index;
integer aw_stalls=0,w_stalls=0,ar_stalls=0,b_stalls=0,r_stalls=0;
reg [31:0] mask,byte_mask,old_value,next_value,read_value;
reg [1:0] result_error;reg [15:0] transaction_addr;reg [31:0] transaction_data;reg [3:0] transaction_strb;
reg snap_aw,snap_w,snap_ar;
function integer access_kind(input [15:0] a);begin
 case(a)
  16'h0000,16'h0004,16'h0100,16'h0104,16'h0200,16'h0204,16'h0208,16'h0304:access_kind=1;
  16'h0008,16'h0010,16'h0108,16'h030c:access_kind=2;
  16'h000c,16'h010c,16'h0110,16'h0308:access_kind=3;
  16'h0014,16'h0114,16'h020c,16'h0300:access_kind=4;
  default:access_kind=0;
 endcase
end endfunction
function [31:0] valid_mask(input[15:0] a);begin
 case(a)
  16'h0000,16'h020c:valid_mask=1;
  16'h0200:valid_mask=3;
  16'h0008,16'h0014:valid_mask=15;
  16'h000c,16'h0010:valid_mask=255;
  16'h0300,16'h0304,16'h0308,16'h030c:valid_mask=31;
  default:valid_mask=32'hffffffff;
 endcase
end endfunction
function [31:0] ro_value(input[15:0] a);begin
 case(a)
  16'h0008:ro_value=uart_status_value;
  16'h0010:ro_value=uart_rx_data_value;
  16'h0108:ro_value=gpio_in_value;
  16'h030c:ro_value=irq_raw_value;
  default:ro_value=0;
 endcase
end endfunction
function rejected(input[15:0] a,input wr);begin
 rejected=0;
 if(wr)case(a)
  16'h000c:rejected=uart_tx_data_write_reject;
  16'h010c:rejected=gpio_set_write_reject;
  16'h0110:rejected=gpio_clear_write_reject;
  16'h0308:rejected=irq_test_write_reject;
 endcase
 else case(a)
  16'h0008:rejected=uart_status_read_reject;
  16'h0010:rejected=uart_rx_data_read_reject;
  16'h0108:rejected=gpio_in_read_reject;
  16'h030c:rejected=irq_raw_read_reject;
 endcase
end endfunction
task check_side(input integer a,input actual_read,input actual_write);begin
 if(actual_read !== (expected_side==a && !expected_write))$fatal(1,"read side effect %h",a);
 if(actual_write !== (expected_side==a && expected_write))$fatal(1,"write side effect %h",a);
end endtask
task check_wo(input integer a,input actual_commit,input[31:0] candidate,input[31:0] write_mask);begin
 if(actual_commit)begin
  if(expected_side!=a||!expected_write)$fatal(1,"unexpected WO commit %h",a);
  if(write_mask!==byte_mask||candidate!==(transaction_data&byte_mask))$fatal(1,"WO payload %h mask=%h expected=%h candidate=%h expected=%h",a,write_mask,byte_mask,candidate,transaction_data&byte_mask);
 end
end endtask
task sample;begin
 cycle_count=cycle_count+1;
 snap_aw=!rst&&s_axi_awvalid&&s_axi_awready;snap_w=!rst&&s_axi_wvalid&&s_axi_wready;snap_ar=!rst&&s_axi_arvalid&&s_axi_arready;
 expected_side=-1;expected_write=0;
 if(rst)begin
  canceled_aw=canceled_aw+aw_in-aw_out;canceled_w=canceled_w+w_in-w_out;canceled_ar=canceled_ar+ar_in-ar_out;
  canceled_csr=canceled_csr+pending;canceled_b=canceled_b+b_in-b_out;canceled_r=canceled_r+r_in-r_out;
  canceled=canceled+(aw_in-aw_out)+(w_in-w_out)+(ar_in-ar_out)+pending+(b_in-b_out)+(r_in-r_out);
  aw_out=aw_in;w_out=w_in;ar_out=ar_in;b_out=b_in;r_out=r_in;pending=0;
  hold_aw=0;hold_w=0;hold_ar=0;hold_csr=0;hold_b=0;hold_r=0;
  for(j=0;j<1024;j=j+1)model[j]=0;
 end else begin
  // Includes the edge on which READY rises: a stalled producer cannot change
  // its payload on the accepting edge. Reset clears the obligation.
  if(hold_aw&&(!s_axi_awvalid||{s_axi_awprot,s_axi_awaddr}!==prev_aw))$fatal(1,"MONITOR AW");
  if(hold_w&&(!s_axi_wvalid||{s_axi_wstrb,s_axi_wdata}!==prev_w))$fatal(1,"MONITOR W");
  if(hold_ar&&(!s_axi_arvalid||{s_axi_arprot,s_axi_araddr}!==prev_ar))$fatal(1,"MONITOR AR");
  if(hold_csr&&(!csr_req_valid||{csr_write,csr_addr,csr_wdata,csr_wstrb}!==prev_csr))$fatal(1,"MONITOR CSR");
  if(hold_b&&(!s_axi_bvalid||s_axi_bresp!==prev_b))$fatal(1,"MONITOR B");
  if(hold_r&&(!s_axi_rvalid||{s_axi_rresp,s_axi_rdata}!==prev_r))$fatal(1,"MONITOR R");
  if(s_axi_awvalid&&!s_axi_awready)aw_stalls=aw_stalls+1;
  if(s_axi_wvalid&&!s_axi_wready)w_stalls=w_stalls+1;
  if(s_axi_arvalid&&!s_axi_arready)ar_stalls=ar_stalls+1;
  if(s_axi_bvalid&&!s_axi_bready)b_stalls=b_stalls+1;
  if(s_axi_rvalid&&!s_axi_rready)r_stalls=r_stalls+1;
  hold_aw=s_axi_awvalid&&!s_axi_awready;prev_aw={s_axi_awprot,s_axi_awaddr};
  hold_w=s_axi_wvalid&&!s_axi_wready;prev_w={s_axi_wstrb,s_axi_wdata};
  hold_ar=s_axi_arvalid&&!s_axi_arready;prev_ar={s_axi_arprot,s_axi_araddr};
  hold_csr=csr_req_valid&&!csr_req_ready;prev_csr={csr_write,csr_addr,csr_wdata,csr_wstrb};
  hold_b=s_axi_bvalid&&!s_axi_bready;prev_b=s_axi_bresp;
  hold_r=s_axi_rvalid&&!s_axi_rready;prev_r={s_axi_rresp,s_axi_rdata};
  if(snap_aw)begin aw_cycle_queue[aw_in]=cycle_count;aw_queue[aw_in]=s_axi_awaddr;aw_in=aw_in+1;accepted_aw=accepted_aw+1;end
  if(snap_w)begin w_cycle_queue[w_in]=cycle_count;w_queue[w_in]=s_axi_wdata;st_queue[w_in]=s_axi_wstrb;w_in=w_in+1;accepted_w=accepted_w+1;end
  if(snap_ar)begin ar_queue[ar_in]=s_axi_araddr;ar_in=ar_in+1;accepted_ar=accepted_ar+1;end
  if(s_axi_bvalid&&s_axi_bready)begin
   if(b_out>=b_in||s_axi_bresp!==b_queue[b_out])$fatal(1,"B scoreboard");
   b_out=b_out+1;consumed_b=consumed_b+1;
  end
  if(s_axi_rvalid&&s_axi_rready)begin
   if(r_out>=r_in||s_axi_rresp!==r_error_queue[r_out]||s_axi_rdata!==r_queue[r_out])$fatal(1,"R scoreboard got=%h want=%h",s_axi_rdata,r_queue[r_out]);
   r_out=r_out+1;consumed_r=consumed_r+1;
  end
  if(csr_rsp_valid&&csr_rsp_ready&&csr_req_valid&&csr_req_ready)$fatal(1,"CSR consume/refill bubble");
  if(csr_rsp_valid&&csr_rsp_ready)begin
   if(!pending||csr_error!==pending_error||(!pending_write&&csr_rdata!==pending_data))$fatal(1,"CSR response scoreboard");
   if(pending_write)begin b_queue[b_in]=pending_error;b_in=b_in+1;end
   else begin r_queue[r_in]=pending_data;r_error_queue[r_in]=pending_error;r_in=r_in+1;end
   pending=0;csr_responses=csr_responses+1;
  end
  if(csr_req_valid&&csr_req_ready)begin
   if(pending)$fatal(1,"multiple inflight");
   transaction_addr=csr_addr;transaction_data=csr_wdata;transaction_strb=csr_wstrb;
   if(csr_write)begin
    if(aw_out>=aw_in||w_out>=w_in||csr_addr!==aw_queue[aw_out]||csr_wdata!==w_queue[w_out]||csr_wstrb!==st_queue[w_out])$fatal(1,"write pairing");
    aw_out=aw_out+1;w_out=w_out+1;write_count=write_count+1;strobes[csr_wstrb]=strobes[csr_wstrb]+1;
   end else begin
    if(ar_out>=ar_in||csr_addr!==ar_queue[ar_out])$fatal(1,"read pairing");
    ar_out=ar_out+1;read_count=read_count+1;
   end
   commits=commits+1;window_index=csr_addr < 16'h0400 ? csr_addr/256:4;windows[window_index]=windows[window_index]+1;
   if({irq_req_valid&&irq_req_ready,timer_req_valid&&timer_req_ready,gpio_req_valid&&gpio_req_ready,uart_req_valid&&uart_req_ready} !== (window_index<4?(4'b1<<window_index):4'b0))$fatal(1,"leaf same-edge commit");
   if(window_index<4)leaf_commits=leaf_commits+1;
   kind=access_kind(csr_addr);idx=csr_addr/4;mask=valid_mask(csr_addr);
   byte_mask=0;for(j=0;j<4;j=j+1)if(csr_wstrb[j])byte_mask=byte_mask|(32'hff<<(8*j));
   byte_mask=byte_mask&mask;
   result_error=0;read_value=0;
   if(window_index==4)result_error=3;
   else if(kind==0||(csr_write&&kind==2)||(!csr_write&&kind==3))result_error=2;
   else if(rejected(csr_addr,csr_write)&&(!csr_write||byte_mask!=0))begin result_error=2;if(!csr_write)reject_reads=reject_reads+1;end
   else begin
    old_value=kind==2?(ro_value(csr_addr)&mask):model[idx];
    if(!csr_write)begin read_value=old_value;expected_side=csr_addr;read_effects=read_effects+1;end
    else if(byte_mask!=0)begin
     expected_side=csr_addr;expected_write=1;side_effects=side_effects+1;
     if(kind==3)wo_effects=wo_effects+1;if(kind==4)w1c_effects=w1c_effects+1;
     if(kind==1)model[idx]=(old_value&~byte_mask)|(csr_wdata&byte_mask);
     if(kind==4)model[idx]=old_value&~(csr_wdata&byte_mask);
    end
   end
   pending=1;pending_write=csr_write;pending_error=result_error;pending_data=read_value;errors[result_error]=errors[result_error]+1;
  end else if((uart_req_valid&&uart_req_ready)||(gpio_req_valid&&gpio_req_ready)||(timer_req_valid&&timer_req_ready)||(irq_req_valid&&irq_req_ready))$fatal(1,"unmatched leaf commit");
  // Natural W1C events happen every edge, including while responses stall.
  model[5]=(model[5]|uart_EVENT_events)&15;
  model[69]=(model[69]|gpio_rise_event_events);
  model[131]=(model[131]|timer_EVENT_events)&1;
  model[192]=(model[192]|irq_pending_events)&31;
 end
 check_wo('h000c,uart_tx_data_write_commit,uart_tx_data_candidate,uart_tx_data_write_mask);
 check_wo('h010c,gpio_set_write_commit,gpio_set_candidate,gpio_set_write_mask);
 check_wo('h0110,gpio_clear_write_commit,gpio_clear_candidate,gpio_clear_write_mask);
 check_wo('h0308,irq_test_write_commit,irq_test_candidate,irq_test_write_mask);
 check_side('h0000,uart_ctrl_read_commit,uart_ctrl_write_commit);check_side('h0004,uart_baud_div_read_commit,uart_baud_div_write_commit);
 check_side('h0008,uart_status_read_commit,uart_status_write_commit);check_side('h000c,uart_tx_data_read_commit,uart_tx_data_write_commit);
 check_side('h0010,uart_rx_data_read_commit,uart_rx_data_write_commit);check_side('h0014,uart_EVENT_read_commit,uart_EVENT_write_commit);
 check_side('h0100,gpio_dir_read_commit,gpio_dir_write_commit);check_side('h0104,gpio_out_read_commit,gpio_out_write_commit);
 check_side('h0108,gpio_in_read_commit,gpio_in_write_commit);check_side('h010c,gpio_set_read_commit,gpio_set_write_commit);
 check_side('h0110,gpio_clear_read_commit,gpio_clear_write_commit);check_side('h0114,gpio_rise_event_read_commit,gpio_rise_event_write_commit);
 check_side('h0200,timer_ctrl_read_commit,timer_ctrl_write_commit);check_side('h0204,timer_count_read_commit,timer_count_write_commit);
 check_side('h0208,timer_compare_read_commit,timer_compare_write_commit);check_side('h020c,timer_EVENT_read_commit,timer_EVENT_write_commit);
 check_side('h0300,irq_pending_read_commit,irq_pending_write_commit);check_side('h0304,irq_enable_read_commit,irq_enable_write_commit);
 check_side('h0308,irq_test_read_commit,irq_test_write_commit);check_side('h030c,irq_raw_read_commit,irq_raw_write_commit);
end endtask
task tick;begin #2;sample();clk=1;#2;clk=0;#2;
 if(!rst)begin
  if(uart_ctrl_value!==model[0]||uart_baud_div_value!==model[1]||gpio_dir_value!==model[64]||gpio_out_value!==model[65]||timer_ctrl_value!==model[128]||timer_count_value!==model[129]||timer_compare_value!==model[130]||irq_enable_value!==model[193])$fatal(1,"external RW state");
  if(uart_EVENT_value!==model[5]||gpio_rise_event_value!==model[69]||timer_EVENT_value!==model[131]||irq_pending_value!==model[192])$fatal(1,"W1C state");
 end
end endtask
task send_aw(input[15:0] a);integer age;begin
 s_axi_awvalid=1;s_axi_awaddr=a;age=0;
 do begin tick();age=age+1;if(age>200)$fatal(1,"AW timeout");end while(!snap_aw);
 s_axi_awvalid=0;
end endtask
task send_w(input[31:0] d,input[3:0] st);integer age;begin
 s_axi_wvalid=1;s_axi_wdata=d;s_axi_wstrb=st;age=0;
 do begin tick();age=age+1;if(age>200)$fatal(1,"W timeout");end while(!snap_w);
 s_axi_wvalid=0;
end endtask
task send_ar(input[15:0] a);integer age;begin
 s_axi_arvalid=1;s_axi_araddr=a;age=0;
 do begin tick();age=age+1;if(age>200)$fatal(1,"AR timeout");end while(!snap_ar);
 s_axi_arvalid=0;
end endtask
task drain;integer age,old_count;begin
 s_axi_bready=1;s_axi_rready=1;age=0;
 while(aw_in!=aw_out||w_in!=w_out||ar_in!=ar_out||pending||b_in!=b_out||r_in!=r_out)begin tick();age=age+1;if(age>300)$fatal(1,"drain timeout");end
 old_count=commits+consumed_b+consumed_r;repeat(9)tick();
 if(old_count!=commits+consumed_b+consumed_r)$fatal(1,"late duplicate");
 s_axi_bready=0;s_axi_rready=0;
end endtask
task reset_all;begin rst=1;tick();rst=0;s_axi_awvalid=0;s_axi_wvalid=0;s_axi_arvalid=0;s_axi_bready=0;s_axi_rready=0;end endtask
task transact(input wr,input[15:0] a,input[31:0] d,input[3:0] st,input integer gap,input first_w,input integer stall);
 integer aw_slot,w_slot,delta;begin
 if(wr)begin
  aw_slot=aw_in;w_slot=w_in;
  if(gap==0)begin
   // Zero skew means simultaneous acceptance regardless of requested first_w.
   s_axi_awvalid=1;s_axi_awaddr=a;s_axi_wvalid=1;s_axi_wdata=d;s_axi_wstrb=st;
   while(s_axi_awvalid||s_axi_wvalid)begin tick();if(snap_aw)s_axi_awvalid=0;if(snap_w)s_axi_wvalid=0;end
  end else if(first_w)begin send_w(d,st);repeat(gap-1)tick();send_aw(a);end
  else begin send_aw(a);repeat(gap-1)tick();send_w(d,st);end
  delta=aw_cycle_queue[aw_slot]-w_cycle_queue[w_slot];
  observed_gap=delta<0?-delta:delta;
  observed_order=delta==0?2:(delta>0?1:0);
  if(observed_gap!=gap||(gap!=0&&observed_order!=first_w))$fatal(1,"accepted-edge skew got=%0d order=%0d want=%0d first_w=%0d",observed_gap,observed_order,gap,first_w);
  $display("ACCEPT AW_cycle=%0d W_cycle=%0d actual_gap=%0d order=%0d",aw_cycle_queue[aw_slot],w_cycle_queue[w_slot],observed_gap,observed_order);
 end else send_ar(a);
 repeat(stall)tick();drain();
end endtask
