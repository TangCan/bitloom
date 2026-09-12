FIRRTL version 6.0.0
circuit Fr170HeadParser :
  module Child :
    input clock: Clock
    input reset: UInt<1>
    input x: UInt<8>
    output y: UInt<8>
    connect y, x

  public module Fr170HeadParser :
    input clock: Clock
    input reset: UInt<1>
    input x: UInt<8>
    output y: UInt<8>
    inst u0 of Child
    connect u0.clock, clock
    connect u0.reset, reset
    connect u0.x, x
    connect y, u0.y
