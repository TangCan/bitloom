FIRRTL version 6.0.0
circuit WaveCounter :
  module WaveCounter :
    input clk: Clock
    input rst: Reset
    output data_out: UInt<8>
    regreset count: UInt<8>, clk, rst, UInt(0)
    count <= add(count, UInt<8>(1))
    data_out <= count
