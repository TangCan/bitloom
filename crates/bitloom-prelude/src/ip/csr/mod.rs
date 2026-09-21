//! Static CSR descriptions and a composable, single-response RTL leaf (FR196).
use crate::{Diagnostics, ElaborateSession, FrozenHir, Span};
use bitloom_hir::Diagnostic;
use std::collections::BTreeSet;
use std::fmt::Write;

mod codec;
mod rtl;

/// Uniform access policy for every field in one register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsrAccess {
    Rw,
    Ro,
    Wo,
    W1c,
}
/// The sole owner of register storage; WO has no storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsrOwner {
    Leaf,
    External,
    None,
}
/// A nonempty, disjoint 32-bit field mask. Reset must be zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsrField {
    pub name: String,
    pub mask: u64,
    pub reset: u32,
    pub access: CsrAccess,
}
/// A four-byte-aligned local register, with optional dynamic rejection inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsrRegister {
    pub name: String,
    pub offset: u32,
    pub reset: u32,
    pub access: CsrAccess,
    pub owner: CsrOwner,
    pub event: Option<String>,
    pub read_reject: bool,
    pub write_reject: bool,
    pub fields: Vec<CsrField>,
}
/// Host configuration shared by RTL, Markdown, and C local-offset generation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsrBlock {
    pub name: String,
    pub registers: Vec<CsrRegister>,
}

fn error(message: impl Into<String>) -> Diagnostics {
    let message = message.into();
    Diagnostics(vec![Diagnostic {
        span: Span::default(),
        code: "rhdl::E0200".into(),
        en: format!("CSR: {message}"),
        zh: format!("CSR 配置错误：{message}"),
    }])
}

impl CsrAccess {
    fn label(self) -> &'static str {
        match self {
            Self::Rw => "RW",
            Self::Ro => "RO",
            Self::Wo => "WO",
            Self::W1c => "W1C",
        }
    }
    fn readable(self) -> bool {
        self != Self::Wo
    }
    fn writable(self) -> bool {
        self != Self::Ro
    }
}
impl CsrRegister {
    fn mask(&self) -> u32 {
        self.fields.iter().fold(0, |mask, f| mask | f.mask as u32)
    }
    fn ports(&self) -> Vec<String> {
        let mut suffixes = vec!["read_commit", "write_commit"];
        if self.access != CsrAccess::Wo {
            suffixes.push("value");
        }
        if self.access.writable() {
            suffixes.extend(["candidate", "write_mask"]);
        }
        if self.read_reject {
            suffixes.push("read_reject");
        }
        if self.write_reject {
            suffixes.push("write_reject");
        }
        suffixes
            .into_iter()
            .map(|s| format!("{}_{s}", self.name))
            .collect()
    }
}

// The union of C11 and Verilog/SystemVerilog keywords. Original spelling is
// preserved; the separate generated-macro namespace catches case folding.
fn identifier(name: &str) -> Result<(), Diagnostics> {
    const RESERVED: &str = "accept_on alias always always_comb always_ff always_latch and assert assign assume automatic before begin bind bins binsof bit break buf bufif0 bufif1 byte case casex casez cell chandle checker class clocking cmos config const constraint context continue cover covergroup coverpoint cross deassign default defparam design disable dist do edge else end endcase endchecker endclass endclocking endconfig endfunction endgenerate endgroup endinterface endmodule endpackage endprimitive endprogram endproperty endspecify endsequence endtable endtask enum event eventually expect export extends extern final first_match for force foreach forever fork forkjoin function generate genvar global highz0 highz1 if iff ifnone ignore_bins illegal_bins implements implies import incdir include initial inout input inside int integer interconnect interface intersect join join_any join_none large let liblist library local localparam logic longint macromodule matches medium modport module nand negedge nettype new nexttime nmos nor noshowcancelled not notif0 notif1 null or output package packed parameter pmos posedge primitive priority program property protected pull0 pull1 pulldown pullup pulsestyle_ondetect pulsestyle_onevent pure rand randc randcase randsequence rcmos real realtime ref reg reject_on release repeat restrict return rnmos rpmos rtran rtranif0 rtranif1 s_always s_eventually s_nexttime s_until s_until_with scalared sequence shortint shortreal showcancelled signed small soft solve specify specparam static string strong strong0 strong1 struct super supply0 supply1 sync_accept_on sync_reject_on table tagged task this throughout time timeprecision timeunit tran tranif0 tranif1 tri tri0 tri1 triand trior trireg type typedef union unique unique0 unsigned until until_with untyped use uwire var vectored virtual void wait wait_order wand weak weak0 weak1 while wildcard wire with within wor xnor xor auto char double float goto inline long register short sizeof switch volatile _Alignas _Alignof _Atomic _Bool _Complex _Generic _Imaginary _Noreturn _Static_assert _Thread_local";
    if u32::try_from(name.len()).is_err()
        || !name.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
        || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
        || name.contains("__")
        || RESERVED.split_whitespace().any(|word| word == name)
    {
        return Err(error(format!("invalid or reserved identifier {name:?}")));
    }
    Ok(())
}
fn unique(names: &mut BTreeSet<String>, name: String) -> Result<(), Diagnostics> {
    if !names.insert(name.clone()) {
        return Err(error(format!("duplicate generated identifier {name}")));
    }
    Ok(())
}

impl CsrBlock {
    /// Validate all products before modifying an elaboration session.
    pub fn validate(&self) -> Result<(), Diagnostics> {
        identifier(&self.name)?;
        if self.registers.is_empty() {
            return Err(error("empty register list"));
        }
        let mut offsets = BTreeSet::new();
        let mut registers = BTreeSet::new();
        let mut ports: BTreeSet<String> = [
            "clk",
            "rst",
            "req_valid",
            "write",
            "addr",
            "wdata",
            "wstrb",
            "rsp_ready",
            "req_ready",
            "rsp_valid",
            "rdata",
            "error",
        ]
        .map(str::to_owned)
        .into();
        let mut macros = BTreeSet::new();
        unique(
            &mut macros,
            format!("BITLOOM_{}_CSR_H", self.name.to_ascii_uppercase()),
        )?;
        for r in &self.registers {
            identifier(&r.name)?;
            unique(&mut registers, r.name.to_ascii_uppercase())?;
            if r.offset > 0xfffc || r.offset % 4 != 0 || !offsets.insert(r.offset) {
                return Err(error(format!(
                    "invalid/duplicate local offset for {}",
                    r.name
                )));
            }
            if !matches!(
                (r.access, r.owner),
                (CsrAccess::Rw, CsrOwner::Leaf | CsrOwner::External)
                    | (CsrAccess::Ro, CsrOwner::External)
                    | (CsrAccess::Wo, CsrOwner::None)
                    | (CsrAccess::W1c, CsrOwner::Leaf)
            ) {
                return Err(error(format!("invalid owner for {}", r.name)));
            }
            if (r.access == CsrAccess::W1c) != r.event.is_some() {
                return Err(error(format!("invalid event ownership for {}", r.name)));
            }
            if (r.read_reject && !r.access.readable()) || (r.write_reject && !r.access.writable()) {
                return Err(error(format!("invalid rejection input for {}", r.name)));
            }
            for port in r.ports() {
                identifier(&port)?;
                unique(&mut ports, port)?;
            }
            if let Some(event) = &r.event {
                identifier(event)?;
                unique(&mut ports, event.clone())?;
            }
            if r.reset != 0 || r.fields.is_empty() {
                return Err(error(format!(
                    "reset must be zero and fields nonempty for {}",
                    r.name
                )));
            }
            let base = format!("{}_{}", self.name, r.name).to_ascii_uppercase();
            for suffix in ["OFFSET", "MASK", "RESET", "ACCESS"] {
                unique(&mut macros, format!("{base}_{suffix}"))?;
            }
            let mut fields = BTreeSet::new();
            let mut mask = 0u64;
            let mut reset = 0u32;
            for f in &r.fields {
                identifier(&f.name)?;
                unique(&mut fields, f.name.to_ascii_uppercase())?;
                if f.mask == 0
                    || f.mask > u32::MAX as u64
                    || mask & f.mask != 0
                    || f.reset != 0
                    || f.access != r.access
                {
                    return Err(error(format!(
                        "invalid field {}.{} mask/reset/access",
                        r.name, f.name
                    )));
                }
                mask |= f.mask;
                reset |= f.reset;
                for suffix in ["MASK", "RESET", "ACCESS"] {
                    unique(
                        &mut macros,
                        format!("{base}_{}_{suffix}", f.name.to_ascii_uppercase()),
                    )?;
                }
            }
            if reset != r.reset {
                return Err(error("field/register reset mismatch"));
            }
        }
        // Private RTL nets begin with `_csr_`, impossible for a user identifier
        // or any generated public port (all must start with an ASCII letter).
        Ok(())
    }
    fn canonical(&self) -> Result<Self, Diagnostics> {
        self.validate()?;
        let mut block = self.clone();
        block.registers.sort_by_key(|r| r.offset);
        for r in &mut block.registers {
            r.fields
                .sort_by(|a, b| (a.mask, &a.name).cmp(&(b.mask, &b.name)));
        }
        Ok(block)
    }
    /// Define/reuse the leaf in a caller-owned session, without freezing it.
    pub fn define_module(
        &self,
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        let config = self.canonical()?;
        session.define_module(name, codec::encode(&config), rtl::define_body)
    }
    /// Elaborate a standalone leaf with exactly one finish/freeze.
    pub fn elaborate(&self, name: impl Into<String>) -> Result<FrozenHir, Diagnostics> {
        let name = name.into();
        let mut session = ElaborateSession::new(&name);
        self.define_module(&mut session, name)?;
        session.finish()
    }
    /// Emit deterministic local offsets; the consumer supplies the physical base.
    pub fn emit_c_header(&self) -> Result<String, Diagnostics> {
        let block = self.canonical()?;
        let upper = block.name.to_ascii_uppercase();
        let guard = format!("BITLOOM_{upper}_CSR_H");
        let mut text = format!(
            "/* Local byte offsets: caller supplies the base address. */\n#ifndef {guard}\n#define {guard}\n#include <stdint.h>\n\n"
        );
        // Different block spellings can still flatten to the same complete
        // macro name (A_B/C versus A/B_C). Reject preexisting definitions even
        // when the replacement tokens are identical and C would allow them.
        fn collision_guard(text: &mut String, name: &str) {
            writeln!(
                text,
                "#ifdef {name}\n#error \"Bitloom CSR macro collision: {name}\"\n#endif"
            )
            .unwrap();
        }
        for r in &block.registers {
            let base = format!("{upper}_{}", r.name.to_ascii_uppercase());
            for suffix in ["OFFSET", "MASK", "RESET", "ACCESS"] {
                collision_guard(&mut text, &format!("{base}_{suffix}"));
            }
            for f in &r.fields {
                for suffix in ["MASK", "RESET", "ACCESS"] {
                    collision_guard(
                        &mut text,
                        &format!("{base}_{}_{suffix}", f.name.to_ascii_uppercase()),
                    );
                }
            }
            for (suffix, value) in [("OFFSET", r.offset), ("MASK", r.mask()), ("RESET", r.reset)] {
                writeln!(text, "#define {base}_{suffix} UINT32_C(0x{value:08x})").unwrap();
            }
            writeln!(text, "#define {base}_ACCESS \"{}\"", r.access.label()).unwrap();
            for f in &r.fields {
                let field = format!("{base}_{}", f.name.to_ascii_uppercase());
                for (suffix, value) in [("MASK", f.mask as u32), ("RESET", f.reset)] {
                    writeln!(text, "#define {field}_{suffix} UINT32_C(0x{value:08x})").unwrap();
                }
                writeln!(text, "#define {field}_ACCESS \"{}\"", f.access.label()).unwrap();
            }
            text.push('\n');
        }
        writeln!(text, "#endif /* {guard} */").unwrap();
        Ok(text)
    }
    /// Emit a deterministic register/field address table with local byte offsets.
    pub fn emit_markdown(&self) -> Result<String, Diagnostics> {
        let block = self.canonical()?;
        let mut text = format!(
            "# {} CSR\n\nLocal byte offsets; caller supplies the base address.\n\n| Register | Offset | Mask | Reset | Access | Owner | Event | Read reject | Write reject |\n|---|---|---|---|---|---|---|---|---|\n",
            block.name
        );
        for r in &block.registers {
            writeln!(
                text,
                "| {} | 0x{:04x} | 0x{:08x} | 0x{:08x} | {} | {:?} | {} | {} | {} |",
                r.name,
                r.offset,
                r.mask(),
                r.reset,
                r.access.label(),
                r.owner,
                r.event.as_deref().unwrap_or("—"),
                r.read_reject,
                r.write_reject
            )
            .unwrap();
        }
        text.push_str("\n| Register.Field | Mask | Reset | Access |\n|---|---|---|---|\n");
        for r in &block.registers {
            for f in &r.fields {
                writeln!(
                    text,
                    "| {}.{} | 0x{:08x} | 0x{:08x} | {} |",
                    r.name,
                    f.name,
                    f.mask,
                    f.reset,
                    f.access.label()
                )
                .unwrap();
            }
        }
        Ok(text)
    }
}
