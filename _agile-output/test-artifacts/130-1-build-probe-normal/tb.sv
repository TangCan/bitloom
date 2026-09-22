module tb; reg clk=0, rst=0; reg [7:0] data_in=0; wire [7:0] data_out;
vendor_ext_ip dut(.clk(clk),.rst(rst),.data_in(data_in),.data_out(data_out));
initial begin $display("BLACKBOX BINDING MECHANISM PASS behavior=absent"); #1 $finish; end
endmodule
