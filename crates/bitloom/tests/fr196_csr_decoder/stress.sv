integer seed,seed_arg,n,st,gap,first,stall,base,offset,wr_flag,before_count,before_other,age;
integer aw_sent,w_sent,ar_sent,concurrent_offer=0;integer random_gaps[0:3],random_first[0:2];integer random_read_start,random_write_start,concurrent_wo_start,concurrent_w1c_start,concurrent_reject_start,concurrent_slverr_start,concurrent_decerr_start;reg [31:0] rng,random_word;
function [31:0] random_next(input[31:0] v);reg[31:0] x;begin x=v^(v<<13);x=x^(x>>17);random_next=x^(x<<5);end endfunction
function [15:0] random_address(input[31:0] v);reg[15:0] a;begin
 case(v[7:4])
  0:a=16'h0400;1:a=16'hffff;2:a=16'h8104;3:a={v[1:0],8'hff};
  4:a={v[1:0],8'h01};5:a={v[1:0],8'hfc};
  default:a=v[1:0]*256+v[11:8]%6*4;
 endcase
 random_address=a;
end endfunction
function [15:0] concurrent_write_address(input integer index);begin
 case(index%8)
  0:concurrent_write_address='h0104;1:concurrent_write_address='h000c;
  2:concurrent_write_address='h010c;3:concurrent_write_address='h0110;
  4:concurrent_write_address='h0308;5:concurrent_write_address='h0014;
  6:concurrent_write_address='h0310;7:concurrent_write_address='hffff;
 endcase
end endfunction
function [15:0] concurrent_read_address(input integer index);begin
 case(index%6)
  0:concurrent_read_address='h0204;1:concurrent_read_address='h0014;
  2:concurrent_read_address='h0010;3:concurrent_read_address='h000c;
  4:concurrent_read_address='h0118;5:concurrent_read_address='h8104;
 endcase
end endfunction
// Repeated genuine two-class decisions: fill both response slots, queue both
// complete requests, then release both slots together. This prevents a full
// response slot from masquerading as an eligible competitor. Priming order
// alternates the last actual CSR commit, so fixed read/write priority fails.
task sustained_two_class_arbitration;
 integer round,timeout_age;reg expected_write;begin
 for(round=0;round<16;round=round+1)begin
  expected_write=round%2;
  if(expected_write)begin
   send_aw('h0104);send_w(round,15);while(!s_axi_bvalid)tick();
   send_ar('h0204);while(!s_axi_rvalid)tick();
  end else begin
   send_ar('h0204);while(!s_axi_rvalid)tick();
   send_aw('h0104);send_w(round,15);while(!s_axi_bvalid)tick();
  end
  s_axi_awvalid=1;s_axi_awaddr='h0004;s_axi_wvalid=1;s_axi_wdata=round+32;s_axi_wstrb=15;s_axi_arvalid=1;s_axi_araddr='h0304;
  tick();if(!snap_aw||!snap_w||!snap_ar)$fatal(1,"arbitration captures");
  s_axi_awvalid=0;s_axi_wvalid=0;s_axi_arvalid=0;
  repeat(3)begin if(csr_req_valid||!s_axi_bvalid||!s_axi_rvalid)$fatal(1,"capacity must block both classes");tick();end
  s_axi_bready=1;s_axi_rready=1;tick();
  if(csr_req_valid)$fatal(1,"full response slot reused on consume edge");
  if(s_axi_bvalid||s_axi_rvalid||pending||aw_in==aw_out||w_in==w_out||ar_in==ar_out)$fatal(1,"both classes must really be eligible");
  tick();
  if(!csr_req_valid||csr_write!==expected_write)$fatal(1,"round robin round=%0d expected_write=%0d got=%0d",round,expected_write,csr_write);
  $display("ARBITRATION round=%0d both_eligible=1 chosen_write=%0d",round,csr_write);
  // Once selected, the ordinary CSR hold monitor protects the locked offer;
  // do not recalculate its class from newly changing eligibility.
  drain();
 end
end endtask
initial begin
 seed_arg=1;if($value$plusargs("seed=%d",seed_arg))begin end
 rng=seed_arg;for(n=0;n<5;n=n+1)windows[n]=0;for(n=0;n<4;n=n+1)errors[n]=0;for(n=0;n<16;n=n+1)strobes[n]=0;
 reset_all();
 // Reset arbitration: simultaneous complete write and read choose read first.
 s_axi_awvalid=1;s_axi_awaddr='h0104;s_axi_wvalid=1;s_axi_wdata=32'h12345678;s_axi_wstrb=15;s_axi_arvalid=1;s_axi_araddr='h0204;
 tick();if(!snap_aw||!snap_w||!snap_ar)$fatal(1,"initial simultaneous capture");
 s_axi_awvalid=0;s_axi_wvalid=0;s_axi_arvalid=0;
 while(!csr_req_valid)tick();if(csr_write)$fatal(1,"reset read priority");
 tick();while(!csr_req_valid)tick();if(!csr_write)$fatal(1,"write after read");drain();
 sustained_two_class_arbitration();
 // Every strobe on complementary old/new values, across every RW register.
 for(base=0;base<4;base=base+1)begin
  for(offset=0;offset<=8;offset=offset+4)begin
   if(access_kind(base*256+offset)==1)for(st=0;st<16;st=st+1)begin
    transact(1,base*256+offset,32'ha55ac33c,15,0,0,0);
    transact(1,base*256+offset,32'h5aa53cc3,st,0,0,7);
    transact(0,base*256+offset,0,0,0,0,0);
   end
  end
 end
 // RO zero-strobe writes and WO reads remain permission failures.
 transact(1,'h0008,0,0,0,0,0);transact(1,'h0010,0,0,0,0,0);transact(1,'h0108,0,0,0,0,0);transact(1,'h030c,0,0,0,0,0);
 transact(0,'h000c,0,0,0,0,0);transact(0,'h010c,0,0,0,0,0);transact(0,'h0110,0,0,0,0,0);transact(0,'h0308,0,0,0,0,0);
 uart_EVENT_events=8;gpio_rise_event_events=32'h80000000;timer_EVENT_events=1;irq_pending_events=16;
 uart_tx_data_write_reject=1;gpio_set_write_reject=1;gpio_clear_write_reject=1;irq_test_write_reject=1;
 // Zero effective mask is OKAY even for a rejecting WO, but no push pulse.
 transact(1,'h000c,32'hffffffff,14,1,0,12);transact(1,'h000c,32'hffffffff,1,7,1,12);
 transact(1,'h010c,32'hffffffff,0,31,0,12);transact(1,'h010c,32'hffffffff,15,0,0,12);
 transact(1,'h0110,32'hffffffff,15,0,0,12);transact(1,'h0308,32'hffffffff,15,0,0,12);
 uart_tx_data_write_reject=0;gpio_set_write_reject=0;gpio_clear_write_reject=0;irq_test_write_reject=0;
 uart_status_read_reject=1;uart_rx_data_read_reject=1;gpio_in_read_reject=1;irq_raw_read_reject=1;
 transact(0,'h0008,0,0,0,0,0);transact(0,'h0010,0,0,0,0,0);transact(0,'h0108,0,0,0,0,0);transact(0,'h030c,0,0,0,0,0);
 uart_status_read_reject=0;uart_rx_data_read_reject=0;gpio_in_read_reject=0;irq_raw_read_reject=0;
 uart_EVENT_events=0;gpio_rise_event_events=0;timer_EVENT_events=0;irq_pending_events=0;
 transact(0,'h0014,0,0,0,0,0);transact(0,'h0114,0,0,0,0,0);transact(0,'h020c,0,0,0,0,0);transact(0,'h0300,0,0,0,0,0);
 uart_status_value=32'hffffffff;uart_rx_data_value=32'hffffffff;gpio_in_value=32'hffffffff;irq_raw_value=32'hffffffff;
 transact(0,'h0008,0,0,0,0,0);transact(0,'h0010,0,0,0,0,0);transact(0,'h0108,0,0,0,0,0);transact(0,'h030c,0,0,0,0,0);
 transact(1,'h0014,32'hffffffff,0,0,0,7);transact(1,'h0114,32'hffffffff,0,0,0,7);transact(1,'h020c,32'hffffffff,0,0,0,7);transact(1,'h0300,32'hffffffff,0,0,0,7);
 // Set wins simultaneous clear, and natural events keep running during stalls.
 uart_EVENT_events=15;gpio_rise_event_events=32'h80000001;timer_EVENT_events=1;irq_pending_events=31;tick();
 transact(1,'h0014,15,15,0,0,15);transact(1,'h0114,32'hffffffff,15,0,0,15);transact(1,'h020c,1,15,0,0,15);transact(1,'h0300,31,15,0,0,15);
 uart_EVENT_events=0;gpio_rise_event_events=0;timer_EVENT_events=0;irq_pending_events=0;
 transact(0,'h0014,0,0,0,0,0);transact(0,'h0114,0,0,0,0,0);transact(0,'h020c,0,0,0,0,0);transact(0,'h0300,0,0,0,0,0);
 // Snapshot captured before peer changes, both CSR and AXI response checked.
 uart_rx_data_value=8'h35;send_ar('h0010);uart_rx_data_value=8'h59;
 while(!csr_req_valid||!csr_req_ready)tick();tick();uart_rx_data_value=8'ha7;
 repeat(17)tick();drain();
 // Partial AW and partial W independently allow qualified read progress.
 send_aw('h0104);before_other=consumed_r;send_ar('h0204);s_axi_rready=1;
 age=0;while(consumed_r==before_other)begin tick();age=age+1;if(age>64)$fatal(1,"partial AW blocked read");end
 s_axi_rready=0;send_w(32'h55,15);drain();
 send_w(32'haa,15);before_other=consumed_r;send_ar('h0004);s_axi_rready=1;
 age=0;while(consumed_r==before_other)begin tick();age=age+1;if(age>64)$fatal(1,"partial W blocked read");end
 s_axi_rready=0;send_aw('h0304);drain();
 // B blocked: consume a newly issued read while that B remains blocked.
 send_aw('h0104);send_w(123,15);while(!s_axi_bvalid)tick();before_other=consumed_r;
 send_ar('h0204);s_axi_rready=1;age=0;
 while(consumed_r==before_other)begin tick();age=age+1;if(age>64)$fatal(1,"B blocks read");if(!s_axi_bvalid)$fatal(1,"B lost");end
 drain();
 // R blocked: consume a newly issued write while that R remains blocked.
 send_ar('h0004);while(!s_axi_rvalid)tick();before_other=consumed_b;
 send_aw('h0304);send_w(15,15);s_axi_bready=1;age=0;
 while(consumed_b==before_other)begin tick();age=age+1;if(age>64)$fatal(1,"R blocks write");if(!s_axi_rvalid)$fatal(1,"R lost");end
 drain();
 // Common reset: AW only, W only (then late counterpart), AR capture, offer,
 // commit/owner, pending R/B, and simultaneous ready/valid cancellation.
 send_aw('h0104);reset_all();send_w(9,15);repeat(12)tick();if(pending||s_axi_bvalid)$fatal(1,"canceled AW replay");reset_all();
 send_w(9,15);reset_all();send_aw('h0204);repeat(12)tick();if(pending||s_axi_bvalid)$fatal(1,"canceled W replay");reset_all();
 send_ar('h0004);reset_all();drain();
 send_ar('h0304);while(!csr_req_valid)tick();reset_all();drain();
 send_ar('h0204);while(!csr_req_valid||!csr_req_ready)tick();tick();reset_all();drain();
 send_ar('hffff);while(!csr_req_valid||!csr_req_ready)tick();tick();reset_all();drain();
 send_ar('h0104);while(!s_axi_rvalid)tick();s_axi_rready=1;reset_all();drain();
 send_aw('h0004);send_w(32'hdeadbeef,15);while(!s_axi_bvalid)tick();s_axi_bready=1;reset_all();drain();
 s_axi_awvalid=1;s_axi_wvalid=1;s_axi_arvalid=1;reset_all();drain();
 transact(1,'h0304,17,15,0,0,0);transact(0,'h0304,0,0,0,0,0);
 // Exactly 1000 completed randomly selected transactions per recorded seed.
 before_count=consumed_b+consumed_r;
 for(n=0;n<5;n=n+1)windows[n]=0;for(n=0;n<4;n=n+1)begin errors[n]=0;random_gaps[n]=0;end
 for(n=0;n<16;n=n+1)strobes[n]=0;random_first[0]=0;random_first[1]=0;random_first[2]=0;
 random_read_start=read_count;random_write_start=write_count;
 for(n=0;n<1000;n=n+1)begin
  rng=random_next(rng);random_word=rng;wr_flag=rng[0];st=rng[7:4];first=rng[8];stall=rng[13:9];
  case(rng[15:14])0:gap=0;1:gap=1;2:gap=7;3:gap=31;endcase
  rng=random_next(rng);
  transact(wr_flag,random_address(random_word>>1),rng,st,gap,first,stall);
  if(wr_flag)begin
   case(observed_gap)0:random_gaps[0]=random_gaps[0]+1;1:random_gaps[1]=random_gaps[1]+1;7:random_gaps[2]=random_gaps[2]+1;31:random_gaps[3]=random_gaps[3]+1;default:$fatal(1,"uncovered measured gap");endcase
   random_first[observed_order]=random_first[observed_order]+1;
  end
 end
 if(consumed_b+consumed_r-before_count!=1000)$fatal(1,"random budget incomplete");
 for(n=0;n<5;n=n+1)if(windows[n]==0)$fatal(1,"random missing window");
 for(n=0;n<16;n=n+1)if(strobes[n]==0)$fatal(1,"random missing strobe");
 for(n=0;n<4;n=n+1)if(random_gaps[n]==0)$fatal(1,"random missing gap");
 if(random_first[0]==0||random_first[1]==0||random_first[2]==0||errors[0]==0||errors[2]==0||errors[3]==0)$fatal(1,"random category");
 $display("RANDOM seed=%0d windows=%0d,%0d,%0d,%0d,%0d errors=%0d,%0d,%0d gaps=%0d,%0d,%0d,%0d order=%0d,%0d,%0d",seed_arg,windows[0],windows[1],windows[2],windows[3],windows[4],errors[0],errors[2],errors[3],random_gaps[0],random_gaps[1],random_gaps[2],random_gaps[3],random_first[0],random_first[1],random_first[2]);
 $display("RANDOM_READ_WRITE seed=%0d read=%0d write=%0d",seed_arg,read_count-random_read_start,write_count-random_write_start);
 for(n=0;n<16;n=n+1)$display("RANDOM_WSTRB seed=%0d strobe=%0d count=%0d",seed_arg,n,strobes[n]);
 // Three independent producer indices: AW, W and AR overlap. Each holds its
 // own current offer until its own handshake, despite the other channels.
 uart_rx_data_read_reject=1;uart_EVENT_events=8;
 concurrent_wo_start=wo_effects;concurrent_w1c_start=w1c_effects;concurrent_reject_start=reject_reads;
 concurrent_slverr_start=errors[2];concurrent_decerr_start=errors[3];
 aw_stalls=0;w_stalls=0;ar_stalls=0;b_stalls=0;r_stalls=0;
 aw_sent=0;w_sent=0;ar_sent=0;age=0;before_count=consumed_b+consumed_r;
 while(aw_sent<128||w_sent<128||ar_sent<128||s_axi_awvalid||s_axi_wvalid||s_axi_arvalid)begin
  rng=random_next(rng);
  if(!s_axi_awvalid&&aw_sent<128&&rng[0])begin s_axi_awvalid=1;s_axi_awaddr=concurrent_write_address(aw_sent);end
  if(!s_axi_wvalid&&w_sent<128&&rng[1])begin s_axi_wvalid=1;s_axi_wdata=32'hb1230000+w_sent;s_axi_wstrb=w_sent%16;end
  if(!s_axi_arvalid&&ar_sent<128&&rng[2])begin s_axi_arvalid=1;s_axi_araddr=concurrent_read_address(ar_sent);end
  s_axi_bready=age%31>14;s_axi_rready=age%23>10;
  if(s_axi_awvalid&&s_axi_wvalid&&s_axi_arvalid)concurrent_offer=concurrent_offer+1;
  tick();
  if(snap_aw)begin aw_sent=aw_sent+1;s_axi_awvalid=0;end
  if(snap_w)begin w_sent=w_sent+1;s_axi_wvalid=0;end
  if(snap_ar)begin ar_sent=ar_sent+1;s_axi_arvalid=0;end
  age=age+1;if(age>20000)$fatal(1,"concurrent timeout");
 end
 drain();uart_rx_data_read_reject=0;uart_EVENT_events=0;
 if(wo_effects==concurrent_wo_start||w1c_effects==concurrent_w1c_start||reject_reads==concurrent_reject_start||errors[2]==concurrent_slverr_start||errors[3]==concurrent_decerr_start)$fatal(1,"missing concurrent side effect/error interleave");
 $display("CONCURRENT_CLASSES WO=%0d W1C=%0d rejected_reads=%0d SLVERR=%0d DECERR=%0d",wo_effects-concurrent_wo_start,w1c_effects-concurrent_w1c_start,reject_reads-concurrent_reject_start,errors[2]-concurrent_slverr_start,errors[3]-concurrent_decerr_start);
 if(aw_stalls==0||w_stalls==0||ar_stalls==0||b_stalls==0||r_stalls==0)$fatal(1,"missing concurrent stalls");
 $display("STALLS AW=%0d W=%0d AR=%0d B=%0d R=%0d",aw_stalls,w_stalls,ar_stalls,b_stalls,r_stalls);
 if(concurrent_offer==0)$fatal(1,"no concurrent offers");
 $display("CONCURRENT seed=%0d three_channel_offer_cycles=%0d",seed_arg,concurrent_offer);
 if(consumed_b+consumed_r-before_count!=256)$fatal(1,"concurrent budget");
 for(n=0;n<5;n=n+1)if(windows[n]==0)$fatal(1,"missing window coverage");
 for(n=0;n<16;n=n+1)if(strobes[n]==0)$fatal(1,"missing WSTRB coverage");
 if(errors[0]==0||errors[2]==0||errors[3]==0||errors[1]!=0||canceled==0)$fatal(1,"error/cancel coverage");
 if(accepted_aw!=write_count+canceled_aw||accepted_w!=write_count+canceled_w||accepted_ar!=read_count+canceled_ar)$fatal(1,"capture/cancel conservation");
 if(commits!=csr_responses+canceled_csr||csr_responses!=consumed_b+consumed_r+canceled_b+canceled_r)$fatal(1,"response/cancel conservation");
 if(canceled_aw==0||canceled_w==0||canceled_ar==0||canceled_csr==0||canceled_b==0||canceled_r==0)$fatal(1,"missing cancel stage");
 $display("CANCEL AW=%0d W=%0d AR=%0d CSR=%0d B=%0d R=%0d read_effects=%0d write_effects=%0d",canceled_aw,canceled_w,canceled_ar,canceled_csr,canceled_b,canceled_r,read_effects,side_effects);
 $display("STRESS PASS seed=%0d random=1000 concurrent=256 cycles=%0d AW=%0d W=%0d AR=%0d commit=%0d leaf=%0d csr_rsp=%0d B=%0d R=%0d cancel=%0d effects=%0d read=%0d write=%0d windows=%0d,%0d,%0d,%0d,%0d errors=%0d,%0d,%0d",seed_arg,cycle_count,accepted_aw,accepted_w,accepted_ar,commits,leaf_commits,csr_responses,consumed_b,consumed_r,canceled,side_effects,read_count,write_count,windows[0],windows[1],windows[2],windows[3],windows[4],errors[0],errors[2],errors[3]);
 $finish;
end
initial begin #10000000;$fatal(1,"watchdog");end
endmodule
