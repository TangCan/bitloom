// FR28 / FR71 golden fixture: mechanical Chisel 7.14.0 Module (compile-only smoke).
// Generated-style; pin: Chisel 7.14.0 ↔ firtool 1.155.0 (AD-9).
import chisel3._

class Fr28GoldenCounter extends Module {
  val io = IO(new Bundle {
    val out = Output(UInt(8.W))
  })
  val r = RegInit(0.U(8.W))
  r := r + 1.U
  io.out := r
}
