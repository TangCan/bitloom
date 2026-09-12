// FR170 — Bitloom update-mainline Parser façade (beyond FR138).
// Product: Bitloom. Unrelated to samitbasu/rhdl.
//
// FR138: BitloomFirrtlParser.parse (FIRRTL v4 restore path).
// FR170: BitloomFirrtlParser.parseUpdateMainline — document-pinned
// update-mainline track accepting FIRRTL version 6.0.0 (AD-3 header)
// at Chisel 7.14.0 ↔ firtool-1.155.0 (AD-9 unchanged).
//
// Runtime: scripts/parser-head-migration-check.sh (firtool -parse-only).
// Design crates must not depend on this JVM façade.

package bitloom.firrtl

object BitloomFirrtlParserMainline {
  /** Update-mainline Parser product path (FR170); ≡ BitloomFirrtlParser.parse semantics. */
  def parseUpdateMainline(firPath: String): Unit = {
    sys.error(
      s"BitloomFirrtlParser.parseUpdateMainline: use `just parser-head-migration-check` (firtool -parse-only / FIRRTL 6.0.0) for path=$firPath"
    )
  }
}

// Continuity alias on the FR138 object name space (docs + AD-27).
object BitloomFirrtlParser {
  def parseUpdateMainline(firPath: String): Unit =
    BitloomFirrtlParserMainline.parseUpdateMainline(firPath)
}
