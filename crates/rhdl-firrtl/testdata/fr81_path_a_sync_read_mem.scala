// FR81 Path A / FR71-adjacent Mem contract fixture (compile-only smoke under NFR12).
// Mechanical style matching emit_chisel; pin: Chisel 7.15.0 ↔ firtool 1.158.0 (AD-9).
// Required FR71 gate remains fr28_golden_counter.scala — this file is optional Mem Path A evidence.
import chisel3._

class Fr81PathASyncReadMem extends Module {
  val io = IO(new Bundle {
    val addr = Input(UInt(2.W))
    val wdata = Input(UInt(8.W))
    val we = Input(Bool())
    val rdata = Output(UInt(8.W))
  })
  val ram = SyncReadMem(4, UInt(8.W))
  val q = RegInit(0.U(8.W))
  when (io.we) {
    ram.write(io.addr, io.wdata)
  }
  q := ram.read(io.addr)
  io.rdata := q
}
