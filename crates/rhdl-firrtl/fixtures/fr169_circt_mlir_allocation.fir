FIRRTL version 4.0.0
circuit Fr169AllocGate :
  module Child :
    input clock: Clock
    input reset: UInt<1>
    input x: UInt<8>
    output y: UInt<8>
    connect y, x

  public module Fr169AllocGate :
    input clock: Clock
    input reset: UInt<1>
    input x: UInt<8>
    output y: UInt<8>
    inst u0 of Child
    connect u0.clock, clock
    connect u0.reset, reset
    connect u0.x, x
    connect y, u0.y
