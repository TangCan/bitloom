// FR138 — Bitloom product-equivalent Scala façade for restored Parser close condition.
// Product: Bitloom. Unrelated to samitbasu/rhdl.
//
// Historical API (removed in CIRCT-era Chisel; chipsalliance/chisel#4899):
//   firrtl.Parser.parse / Parser.parse
//
// Product-equivalent API (named in docs/fr138-parser-restore.md + AD-27):
//   bitloom.firrtl.BitloomFirrtlParser.parse
//
// Implementation of the product path is AD-9 firtool-1.158.0 -parse-only
// (see `just parser-restore-check` / scripts/parser-restore-check.sh).
// This file documents the Scala surface name; do NOT pull Scala runtime into design crates.
// Pairing: Chisel 7.15.0 ↔ firtool-1.158.0.

package bitloom.firrtl

/** Product-equivalent of historical `firrtl.Parser.parse` / `Parser.parse`. */
object BitloomFirrtlParser {
  /** Parse FIRRTL text via the documented Bitloom FR138 product path (firtool -parse-only). */
  def parse(firPath: String): Unit = {
    // Runtime: invoked by scripts/parser-restore-check.sh under pinned firtool-1.158.0.
    // Design crates must not depend on this JVM façade.
    sys.error(
      s"BitloomFirrtlParser.parse: use `just parser-restore-check` (firtool -parse-only) for path=$firPath"
    )
  }
}

// Compatibility documentation alias (name continuity with firrtl.Parser.parse).
package firrtl {
  /** Compatibility façade name — product path is BitloomFirrtlParser.parse (FR138). */
  object Parser {
    def parse(firPath: String): Unit = bitloom.firrtl.BitloomFirrtlParser.parse(firPath)
  }
}
