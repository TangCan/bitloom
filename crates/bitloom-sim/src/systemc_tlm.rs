//! SystemC TLM-2.0 LT product path (FR101 / Epic 46 / Story 46.2).
//!
//! Emits includeable/linkable C++ using IEEE 1666 TLM-2.0 loosely-timed
//! `b_transport` + `tlm_generic_payload`. **Not** host Rust FL (FR47).
//! Does **not** replace cycle-accurate `Sim::tick` (AD-5) and does **not**
//! claim default TLM≡CA (FR100).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use bitloom_hir::{FrozenHir, GroundType, PortDirection};

/// Pinned Accellera SystemC line documented for FR101 MVP (Debian/pkg-config).
pub const SYSTEMC_PIN_VERSION: &str = "2.3.3";

/// Resolved SystemC toolchain (pkg-config).
#[derive(Debug, Clone)]
pub struct SystemcToolchain {
    pub version: String,
    pub cflags: String,
    pub libs: String,
}

/// Emit a SystemC TLM-2.0 **LT-only** fixture under `out_dir` from `FrozenHir`.
///
/// Writes:
/// - `bitloom_tlm_lt.hpp` / `bitloom_tlm_lt.cpp` — target + initiator (D1)
/// - `sc_main.cpp` — runnable smoke (D3)
/// - `Makefile` — links via `pkg-config systemc` (D4)
///
/// Abstraction: **LT-only** (`b_transport`). AT (`nb_transport_*`) is out of MVP.
pub fn emit_systemc_tlm_lt(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
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
            "no non-clock/reset data ports to map into TLM LT register bank",
        ));
    }

    fs::write(
        out_dir.join("bitloom_tlm_lt.hpp"),
        render_header(&mod_name, &regs),
    )?;
    fs::write(
        out_dir.join("bitloom_tlm_lt.cpp"),
        render_impl(&mod_name, &regs),
    )?;
    fs::write(out_dir.join("sc_main.cpp"), render_sc_main(&mod_name))?;
    fs::write(out_dir.join("Makefile"), render_makefile())?;
    fs::write(out_dir.join("README.md"), render_fixture_readme(&mod_name))?;
    Ok(out_dir.to_path_buf())
}

/// Alias for product naming symmetry with `emit_functional_crate`.
pub fn generate_systemc_tlm_lt(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf> {
    emit_systemc_tlm_lt(hir, out_dir)
}

/// Resolve SystemC via `pkg-config`. Missing install → readable `Err` (never silent OK).
pub fn resolve_systemc() -> Result<SystemcToolchain, String> {
    let version = pkg_config_stdout(&["--modversion", "systemc"]).map_err(|e| {
        format!(
            "SystemC not found ({e}). Install libsystemc-dev (pinned {SYSTEMC_PIN_VERSION}) \
             or ensure `pkg-config --exists systemc`. See docs/fr101-systemc-tlm.md."
        )
    })?;
    let cflags = pkg_config_stdout(&["--cflags", "systemc"])
        .map_err(|e| format!("pkg-config --cflags systemc failed: {e}"))?;
    let libs = pkg_config_stdout(&["--libs", "systemc"])
        .map_err(|e| format!("pkg-config --libs systemc failed: {e}"))?;
    Ok(SystemcToolchain {
        version: version.trim().to_string(),
        cflags: cflags.trim().to_string(),
        libs: libs.trim().to_string(),
    })
}

/// Build and run the emitted LT smoke (`make run` in `out_dir`).
///
/// Requires SystemC; failures include install/version hints (D4).
pub fn build_and_run_tlm_lt_smoke(out_dir: &Path) -> Result<String, String> {
    let tc = resolve_systemc()?;
    if !tc.version.starts_with(SYSTEMC_PIN_VERSION)
        && !tc.version.starts_with("2.3.")
        && std::env::var_os("BITLOOM_SYSTEMC_ALLOW_ANY").is_none()
    {
        // Soft warn path: still try build, but surface pin in message on failure.
        let _ = &tc;
    }
    let makefile = out_dir.join("Makefile");
    if !makefile.is_file() {
        return Err(format!(
            "missing Makefile under {}; run emit_systemc_tlm_lt first",
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
            "SystemC TLM LT smoke failed (SystemC {}, pin {}).\nstdout:\n{stdout}\nstderr:\n{stderr}\n\
             Install: apt install libsystemc-dev (or equivalent). See docs/fr101-systemc-tlm.md.",
            tc.version, SYSTEMC_PIN_VERSION
        ));
    }
    if !stdout.contains("BITLOOM_TLM_LT_OK") {
        return Err(format!(
            "smoke ran but missing BITLOOM_TLM_LT_OK marker.\nstdout:\n{stdout}\nstderr:\n{stderr}"
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
                offset = offset.saturating_add(8); // 8-byte aligned slots
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
// Generated by Bitloom emit_systemc_tlm_lt (FR101).
// LT-only TLM-2.0 (b_transport + tlm_generic_payload). Not Rust FL (FR47).
// Product: Bitloom. Unrelated to samitbasu/rhdl.
// AD-5 (revised): SystemC TLM-2.0 product path allowed; does not replace Sim::tick.

#include <systemc>
#include <tlm>
#include <tlm_utils/simple_initiator_socket.h>
#include <tlm_utils/simple_target_socket.h>
#include <cstdint>
#include <cstring>
#include <vector>

namespace bitloom_tlm {{

struct {mod_name}LtTarget : sc_core::sc_module {{
  tlm_utils::simple_target_socket<{mod_name}LtTarget> socket;
  SC_HAS_PROCESS({mod_name}LtTarget);
  explicit {mod_name}LtTarget(sc_core::sc_module_name name);
  void b_transport(tlm::tlm_generic_payload& trans, sc_core::sc_time& delay);

private:
  std::vector<std::uint8_t> mem_;
}};

struct {mod_name}LtInitiator : sc_core::sc_module {{
  tlm_utils::simple_initiator_socket<{mod_name}LtInitiator> socket;
  SC_HAS_PROCESS({mod_name}LtInitiator);
  explicit {mod_name}LtInitiator(sc_core::sc_module_name name);
  void run();
}};

{decls}
}}  // namespace bitloom_tlm
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
        r#"#include "bitloom_tlm_lt.hpp"
#include <iostream>

namespace bitloom_tlm {{

{mod_name}LtTarget::{mod_name}LtTarget(sc_core::sc_module_name name)
    : sc_module(name), socket("socket"), mem_({mem_size}, 0) {{
  socket.register_b_transport(this, &{mod_name}LtTarget::b_transport);
}}

void {mod_name}LtTarget::b_transport(tlm::tlm_generic_payload& trans,
                                     sc_core::sc_time& delay) {{
  (void)delay;
  auto cmd = trans.get_command();
  auto addr = static_cast<std::uint64_t>(trans.get_address());
  auto* ptr = trans.get_data_ptr();
  auto len = trans.get_data_length();
  if (addr + len > mem_.size()) {{
    trans.set_response_status(tlm::TLM_ADDRESS_ERROR_RESPONSE);
    return;
  }}
  if (cmd == tlm::TLM_WRITE_COMMAND) {{
    std::memcpy(mem_.data() + addr, ptr, len);
  }} else if (cmd == tlm::TLM_READ_COMMAND) {{
    std::memcpy(ptr, mem_.data() + addr, len);
  }} else {{
    trans.set_response_status(tlm::TLM_COMMAND_ERROR_RESPONSE);
    return;
  }}
  trans.set_response_status(tlm::TLM_OK_RESPONSE);
}}

{mod_name}LtInitiator::{mod_name}LtInitiator(sc_core::sc_module_name name)
    : sc_module(name), socket("socket") {{
  SC_THREAD(run);
}}

void {mod_name}LtInitiator::run() {{
  unsigned char buf[4] = {{0x11, 0x22, 0x33, 0x44}};
  tlm::tlm_generic_payload trans;
  sc_core::sc_time delay = sc_core::SC_ZERO_TIME;
  trans.set_command(tlm::TLM_WRITE_COMMAND);
  trans.set_address(0);
  trans.set_data_ptr(buf);
  trans.set_data_length(4);
  trans.set_streaming_width(4);
  trans.set_byte_enable_ptr(nullptr);
  trans.set_dmi_allowed(false);
  trans.set_response_status(tlm::TLM_INCOMPLETE_RESPONSE);
  socket->b_transport(trans, delay);
  if (!trans.is_response_ok()) {{
    std::cerr << "BITLOOM_TLM_LT_FAIL write\n";
    sc_core::sc_stop();
    return;
  }}
  unsigned char rbuf[4] = {{0, 0, 0, 0}};
  trans.set_command(tlm::TLM_READ_COMMAND);
  trans.set_data_ptr(rbuf);
  trans.set_response_status(tlm::TLM_INCOMPLETE_RESPONSE);
  socket->b_transport(trans, delay);
  if (!trans.is_response_ok() || std::memcmp(buf, rbuf, 4) != 0) {{
    std::cerr << "BITLOOM_TLM_LT_FAIL readback\n";
    sc_core::sc_stop();
    return;
  }}
  std::cout << "BITLOOM_TLM_LT_OK\n";
  sc_core::sc_stop();
}}

}}  // namespace bitloom_tlm
"#,
        mod_name = mod_name,
        mem_size = mem_size,
    )
}

fn render_sc_main(mod_name: &str) -> String {
    format!(
        r#"#include "bitloom_tlm_lt.hpp"

int sc_main(int, char*[]) {{
  bitloom_tlm::{mod_name}LtInitiator init("init");
  bitloom_tlm::{mod_name}LtTarget tgt("tgt");
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
        r#"# Bitloom FR101 SystemC TLM-2.0 LT smoke (pin {pin}).
# Requires: pkg-config systemc + g++ (libsystemc-dev).
CXX ?= g++
CXXFLAGS ?= -std=c++17 -Wall -O0 $(shell pkg-config --cflags systemc)
LDFLAGS ?= $(shell pkg-config --libs systemc) -pthread

.PHONY: all run clean

all: bitloom_tlm_lt_smoke

bitloom_tlm_lt_smoke: sc_main.cpp bitloom_tlm_lt.cpp bitloom_tlm_lt.hpp
	@if ! pkg-config --exists systemc; then \
	  echo "error: SystemC not found. Install libsystemc-dev (pinned {pin}). See docs/fr101-systemc-tlm.md." >&2; \
	  exit 1; \
	fi
	$(CXX) $(CXXFLAGS) sc_main.cpp bitloom_tlm_lt.cpp -o $@ $(LDFLAGS)

run: bitloom_tlm_lt_smoke
	./bitloom_tlm_lt_smoke

clean:
	rm -f bitloom_tlm_lt_smoke
"#,
        pin = SYSTEMC_PIN_VERSION
    )
}

fn render_fixture_readme(mod_name: &str) -> String {
    format!(
        r#"# Bitloom SystemC TLM-2.0 LT fixture ({mod_name})

Generated by `bitloom_sim::emit_systemc_tlm_lt` / `cargo bitloom gen-tlm` (**FR101**).

- Abstraction: **LT-only** (`b_transport` + `tlm_generic_payload`)
- Pin: SystemC **{pin}** (`pkg-config systemc`)
- Not host Rust functional sim (**FR47**)
- Cross-link: revised **AD-5** (ARCHITECTURE-SPINE); docs/fr101-systemc-tlm.md

```bash
make run
# expects: BITLOOM_TLM_LT_OK
```
"#,
        mod_name = mod_name,
        pin = SYSTEMC_PIN_VERSION
    )
}

fn pkg_config_stdout(args: &[&str]) -> Result<String, String> {
    let out = Command::new("pkg-config")
        .args(args)
        .output()
        .map_err(|e| format!("spawn pkg-config: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_builder::{ElaborateSession, GroundType, Span};

    fn tiny_hir() -> FrozenHir {
        let mut s = ElaborateSession::new("TlmTiny");
        s.begin_module("TlmTiny", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("data", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_lit("data", 0, Span::default());
        s.end_process();
        s.end_module();
        s.finish().expect("TlmTiny")
    }

    #[test]
    fn emit_writes_lt_sources() {
        let hir = tiny_hir();
        let dir = std::env::temp_dir().join(format!("bitloom-tlm-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let out = emit_systemc_tlm_lt(&hir, &dir).unwrap();
        let hpp = fs::read_to_string(out.join("bitloom_tlm_lt.hpp")).unwrap();
        assert!(hpp.contains("tlm_generic_payload") || hpp.contains("b_transport"));
        assert!(hpp.contains("b_transport"));
        assert!(hpp.contains("Bitloom"));
        assert!(!hpp.to_lowercase().contains("functional_model"));
        let mk = fs::read_to_string(out.join("Makefile")).unwrap();
        assert!(mk.contains("systemc") && mk.contains(SYSTEMC_PIN_VERSION));
    }
}
