module ResetTop(input clk, aresetn, enable, input [7:0] data0,data1, output [7:0] out0,out1);
wire core_reset = ~aresetn;
CorePorts core(.clk(clk),.rst(core_reset),.enable(enable),.data0(data0),.data1(data1),.out0(out0),.out1(out1));
endmodule
