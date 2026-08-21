FIRRTL version 6.0.0
circuit ExternalTop :
  module Child :
    input clk: Clock
    input rst: Reset
    input x: UInt<8>
    output y: UInt<8>
    y <= x
  module ExternalTop :
    input clk: Clock
    input rst: Reset
    input x: UInt<8>
    output y: UInt<8>
    inst u0 of Child
    u0.clk <= clk
    u0.rst <= rst
    u0.x <= x
    y <= u0.y
