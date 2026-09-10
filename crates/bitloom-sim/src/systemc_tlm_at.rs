//! SystemC TLM-2.0 AT product path (FR107 / Epic 49 / Story 49.2).
//!
//! Parallel to LT (`systemc_tlm.rs` / FR101). Emits AT-style
//! `nb_transport_fw` (documented subset: BEGIN_REQ → TLM_COMPLETED).
//! Does **not** modify LT emit. Not Rust FL (FR47). Not TLM≡CA (FR100).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use bitloom_hir::{FrozenHir, GroundType, PortDirection};

use crate::systemc_tlm::{SYSTEMC_PIN_VERSION, resolve_systemc};

/// Emit a SystemC TLM-2.0 **AT-subset** fixture under `out_dir` from `FrozenHir`.
///
/// Writes `bitloom_tlm_at.hpp/.cpp`, `sc_main.cpp`, `Makefile`, `README.md`.
/// Abstraction: AT documented subset — target `nb_transport_fw` returns
/// `TLM_COMPLETED` on `BEGIN_REQ` (collapsed phase). Full PEQs / quantum /
/// timing family remain out of contract (NFR47).
pub fn emit_systemc_tlm_at(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(out_dir)?;
    let top =
        hir.circuit().modules.first().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "FrozenHir has no modules")
        })?;
    let mod_name = sanitize_cpp_ident(&top.name);
    let regs = collect_data_ports(top);
    if regs.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "no non-clock/reset data ports to map into TLM AT register bank",
        ));
    }

    fs::write(
        out_dir.join("bitloom_tlm_at.hpp"),
        render_header(&mod_name, &regs),
    )?;
    fs::write(
        out_dir.join("bitloom_tlm_at.cpp"),
        render_impl(&mod_name, &regs),
    )?;
    fs::write(out_dir.join("sc_main.cpp"), render_sc_main(&mod_name))?;
    fs::write(out_dir.join("Makefile"), render_makefile())?;
    fs::write(out_dir.join("README.md"), render_fixture_readme(&mod_name))?;
    Ok(out_dir.to_path_buf())
}

/// Alias for product naming symmetry.
pub fn generate_systemc_tlm_at(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    emit_systemc_tlm_at(hir, out_dir)
}

/// Build and run the emitted AT smoke (`make run` in `out_dir`).
pub fn build_and_run_tlm_at_smoke(out_dir: &Path) -> Result<String, String> {
    let tc = resolve_systemc()?;
    let makefile = out_dir.join("Makefile");
    if !makefile.is_file() {
        return Err(format!(
            "missing Makefile under {}; run emit_systemc_tlm_at first",
            out_dir.display()
        ));
    }
    let status = Command::new("make")
        .arg("run")
        .current_dir(out_dir)
        .output()
        .map_err(|e| format!("spawn make: {e}"))?;
    let stdout = String::from_utf8_lossy(&status.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&status.stderr).into_owned();
    if !status.status.success() {
        return Err(format!(
            "SystemC TLM AT smoke failed (SystemC {}, pin {}).\nstdout:\n{stdout}\nstderr:\n{stderr}\n\
             Install: apt install libsystemc-dev (or equivalent). See docs/fr107-systemc-tlm-at.md.",
            tc.version, SYSTEMC_PIN_VERSION
        ));
    }
    if !stdout.contains("BITLOOM_TLM_AT_OK") {
        return Err(format!(
            "smoke ran but missing BITLOOM_TLM_AT_OK marker.\nstdout:\n{stdout}\nstderr:\n{stderr}"
        ));
    }
    Ok(stdout)
}

struct MappedPort {
    name: String,
    width: u32,
    dir: PortDirection,
    offset: u32,
}

fn collect_data_ports(m: &bitloom_hir::Module) -> Vec<MappedPort> {
    let mut out = Vec::new();
    let mut offset = 0u32;
    for p in &m.ports {
        match p.ty {
            GroundType::Clock | GroundType::Reset => continue,
            GroundType::UInt { width } | GroundType::SInt { width } => {
                out.push(MappedPort {
                    name: p.name.clone(),
                    width,
                    dir: p.direction,
                    offset,
                });
                offset = offset.saturating_add(8);
            }
            GroundType::Bool => {
                out.push(MappedPort {
                    name: p.name.clone(),
                    width: 1,
                    dir: p.direction,
                    offset,
                });
                offset = offset.saturating_add(8);
            }
            GroundType::Analog => {}
        }
    }
    out
}

fn sanitize_cpp_ident(s: &str) -> String {
    let mut out: String = s
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if out.is_empty() || out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out = format!("M_{out}");
    }
    out
}

fn render_header(mod_name: &str, regs: &[MappedPort]) -> String {
    let mut decls = String::new();
    for r in regs {
        decls.push_str(&format!(
            "  // port {} width={} dir={:?} @0x{:x}\n",
            r.name, r.width, r.dir, r.offset
        ));
    }
    format!(
        r#"#pragma once
// Generated by Bitloom emit_systemc_tlm_at (FR107).
// AT documented subset: nb_transport_fw (BEGIN_REQ → TLM_COMPLETED).
// Parallel to LT emit_systemc_tlm_lt (FR101). Not Rust FL (FR47).
// Product: Bitloom. Unrelated to samitbasu/rhdl.
// AD-5 (revised 2026-09-10 / Phase 13): AT / nb_transport product path allowed.

#include <systemc>
#include <tlm>
#include <tlm_utils/simple_initiator_socket.h>
#include <tlm_utils/simple_target_socket.h>
#include <cstdint>
#include <cstring>
#include <vector>

namespace bitloom_tlm_at {{

struct {mod_name}AtTarget : sc_core::sc_module {{
  tlm_utils::simple_target_socket<{mod_name}AtTarget> socket;
  SC_HAS_PROCESS({mod_name}AtTarget);
  explicit {mod_name}AtTarget(sc_core::sc_module_name name);
  tlm::tlm_sync_enum nb_transport_fw(tlm::tlm_generic_payload& trans,
                                     tlm::tlm_phase& phase,
                                     sc_core::sc_time& delay);

private:
  std::vector<std::uint8_t> mem_;
}};

struct {mod_name}AtInitiator : sc_core::sc_module {{
  tlm_utils::simple_initiator_socket<{mod_name}AtInitiator> socket;
  SC_HAS_PROCESS({mod_name}AtInitiator);
  explicit {mod_name}AtInitiator(sc_core::sc_module_name name);
  void run();
}};

{decls}
}}  // namespace bitloom_tlm_at
"#,
        mod_name = mod_name,
        decls = decls,
    )
}

fn render_impl(mod_name: &str, regs: &[MappedPort]) -> String {
    let mem_size = regs
        .last()
        .map(|r| r.offset.saturating_add(8))
        .unwrap_or(8)
        .max(16);
    format!(
        r#"#include "bitloom_tlm_at.hpp"
#include <iostream>

namespace bitloom_tlm_at {{

{mod_name}AtTarget::{mod_name}AtTarget(sc_core::sc_module_name name)
    : sc_module(name), socket("socket"), mem_({mem_size}, 0) {{
  socket.register_nb_transport_fw(this, &{mod_name}AtTarget::nb_transport_fw);
}}

tlm::tlm_sync_enum {mod_name}AtTarget::nb_transport_fw(
    tlm::tlm_generic_payload& trans, tlm::tlm_phase& phase,
    sc_core::sc_time& delay) {{
  (void)delay;
  if (phase != tlm::BEGIN_REQ) {{
    return tlm::TLM_ACCEPTED;
  }}
  auto cmd = trans.get_command();
  auto addr = static_cast<std::uint64_t>(trans.get_address());
  auto* ptr = trans.get_data_ptr();
  auto len = trans.get_data_length();
  if (addr + len > mem_.size()) {{
    trans.set_response_status(tlm::TLM_ADDRESS_ERROR_RESPONSE);
    phase = tlm::END_RESP;
    return tlm::TLM_COMPLETED;
  }}
  if (cmd == tlm::TLM_WRITE_COMMAND) {{
    std::memcpy(mem_.data() + addr, ptr, len);
  }} else if (cmd == tlm::TLM_READ_COMMAND) {{
    std::memcpy(ptr, mem_.data() + addr, len);
  }} else {{
    trans.set_response_status(tlm::TLM_COMMAND_ERROR_RESPONSE);
    phase = tlm::END_RESP;
    return tlm::TLM_COMPLETED;
  }}
  trans.set_response_status(tlm::TLM_OK_RESPONSE);
  phase = tlm::END_RESP;
  return tlm::TLM_COMPLETED;
}}

{mod_name}AtInitiator::{mod_name}AtInitiator(sc_core::sc_module_name name)
    : sc_module(name), socket("socket") {{
  SC_THREAD(run);
}}

void {mod_name}AtInitiator::run() {{
  unsigned char buf[4] = {{0x11, 0x22, 0x33, 0x44}};
  tlm::tlm_generic_payload trans;
  sc_core::sc_time delay = sc_core::SC_ZERO_TIME;
  tlm::tlm_phase phase = tlm::BEGIN_REQ;
  trans.set_command(tlm::TLM_WRITE_COMMAND);
  trans.set_address(0);
  trans.set_data_ptr(buf);
  trans.set_data_length(4);
  trans.set_streaming_width(4);
  trans.set_byte_enable_ptr(nullptr);
  trans.set_dmi_allowed(false);
  trans.set_response_status(tlm::TLM_INCOMPLETE_RESPONSE);
  auto st = socket->nb_transport_fw(trans, phase, delay);
  if (st != tlm::TLM_COMPLETED || !trans.is_response_ok()) {{
    std::cerr << "BITLOOM_TLM_AT_FAIL write\n";
    sc_core::sc_stop();
    return;
  }}
  unsigned char rbuf[4] = {{0, 0, 0, 0}};
  phase = tlm::BEGIN_REQ;
  trans.set_command(tlm::TLM_READ_COMMAND);
  trans.set_data_ptr(rbuf);
  trans.set_response_status(tlm::TLM_INCOMPLETE_RESPONSE);
  st = socket->nb_transport_fw(trans, phase, delay);
  if (st != tlm::TLM_COMPLETED || !trans.is_response_ok()
      || std::memcmp(buf, rbuf, 4) != 0) {{
    std::cerr << "BITLOOM_TLM_AT_FAIL readback\n";
    sc_core::sc_stop();
    return;
  }}
  std::cout << "BITLOOM_TLM_AT_OK\n";
  sc_core::sc_stop();
}}

}}  // namespace bitloom_tlm_at
"#,
        mod_name = mod_name,
        mem_size = mem_size,
    )
}

fn render_sc_main(mod_name: &str) -> String {
    format!(
        r#"#include "bitloom_tlm_at.hpp"

int sc_main(int, char*[]) {{
  bitloom_tlm_at::{mod_name}AtInitiator init("init");
  bitloom_tlm_at::{mod_name}AtTarget tgt("tgt");
  init.socket.bind(tgt.socket);
  sc_core::sc_start();
  return 0;
}}
"#,
        mod_name = mod_name
    )
}

fn render_makefile() -> String {
    format!(
        r#"# Bitloom FR107 SystemC TLM-2.0 AT smoke (pin {pin}).
# Requires: pkg-config systemc + g++ (libsystemc-dev).
CXX ?= g++
CXXFLAGS ?= -std=c++17 -Wall -O0 $(shell pkg-config --cflags systemc)
LDFLAGS ?= $(shell pkg-config --libs systemc) -pthread

.PHONY: all run clean

all: bitloom_tlm_at_smoke

bitloom_tlm_at_smoke: sc_main.cpp bitloom_tlm_at.cpp bitloom_tlm_at.hpp
	@if ! pkg-config --exists systemc; then \
	  echo "error: SystemC not found. Install libsystemc-dev (pinned {pin}). See docs/fr107-systemc-tlm-at.md." >&2; \
	  exit 1; \
	fi
	$(CXX) $(CXXFLAGS) sc_main.cpp bitloom_tlm_at.cpp -o $@ $(LDFLAGS)

run: bitloom_tlm_at_smoke
	./bitloom_tlm_at_smoke

clean:
	rm -f bitloom_tlm_at_smoke
"#,
        pin = SYSTEMC_PIN_VERSION
    )
}

fn render_fixture_readme(mod_name: &str) -> String {
    format!(
        r#"# Bitloom SystemC TLM-2.0 AT fixture ({mod_name})

Generated by `bitloom_sim::emit_systemc_tlm_at` / `cargo bitloom gen-tlm-at` (**FR107**).

- Abstraction: **AT documented subset** (`nb_transport_fw` + `tlm_generic_payload`)
- Parallel to LT (`gen-tlm` / FR101); LT regression must stay green
- Pin: SystemC **{pin}** (`pkg-config systemc`)
- Not host Rust functional sim (**FR47**); not LT-only closeout
- Cross-link: revised **AD-5**; docs/fr107-systemc-tlm-at.md

```bash
make run
# expects: BITLOOM_TLM_AT_OK
```
"#,
        mod_name = mod_name,
        pin = SYSTEMC_PIN_VERSION
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    fn tiny_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("TlmAtTiny");
        s.begin_module("TlmAtTiny", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("data", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_lit("data", 0, Span::default());
        s.end_process();
        s.end_module();
        s.finish().expect("TlmAtTiny")
    }

    #[test]
    fn emit_writes_at_sources() {
        let hir = tiny_hir();
        let dir = std::env::temp_dir().join(format!("bitloom-tlm-at-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let out = emit_systemc_tlm_at(&hir, &dir).unwrap();
        let hpp = fs::read_to_string(out.join("bitloom_tlm_at.hpp")).unwrap();
        assert!(hpp.contains("nb_transport_fw"));
        assert!(hpp.contains("Bitloom"));
        assert!(!hpp.contains("register_b_transport"));
        assert!(!hpp.contains("->b_transport"));
        let mk = fs::read_to_string(out.join("Makefile")).unwrap();
        assert!(mk.contains("systemc") && mk.contains(SYSTEMC_PIN_VERSION));
    }
}
