//! Builder session: the only public mutation path into HIR (AD-13).

use std::collections::{HashMap, HashSet};

use bitloom_hir::{
    Assign, AssignExpr, AssignTarget, BuilderOwnedHir, Module, Port, PortDirection, Process,
    ProcessKind, Stmt,
};

pub use bitloom_hir::{
    Diagnostic, Diagnostics, Diagnostics as HirDiagnostics, FrozenHir, FrozenHir as Frozen,
    GroundType, SignalKind, Span,
};

mod closures;
pub use closures::*;

#[derive(Debug)]
enum ProcessState {
    Combinational {
        assigns: Vec<Assign>,
        /// Assignment sets along the current path stack for latch analysis.
        path_assigned: Vec<HashSet<String>>,
        /// Open branches: then-set collected so far, optional else placeholder.
        pending_branches: Vec<(HashSet<String>, bool)>,
        span: Span,
    },
    Sequential {
        assigns: Vec<Assign>,
        span: Span,
    },
}

/// Session token holding the unfrozen circuit (AD-13).
pub struct ElaborateSession {
    hir: BuilderOwnedHir,
    current: Option<Module>,
    /// name -> kind for the current module
    signals: HashMap<String, SignalKind>,
    /// name -> bit width for UInt/SInt (Clock/Reset/Bool use 1)
    widths: HashMap<String, u32>,
    /// name -> declared ground type for typed arithmetic validation
    types: HashMap<String, GroundType>,
    /// memory name -> (depth, word width, synchronous read)
    memories: HashMap<String, (u32, u32, bool)>,
    /// Phantom clock-domain id per signal (AD-22); default 0.
    domains: HashMap<String, u32>,
    /// Signals that may legally cross domains (DoubleFlop/SyncFIFO bridges).
    cdc_bridges: HashSet<String>,
    clock_port: Option<String>,
    reset_port: Option<String>,
    process: Option<ProcessState>,
    errors: Diagnostics,
    definitions: HashMap<String, Vec<(String, u32)>>,
    defining_body: bool,
}

impl ElaborateSession {
    pub fn new(circuit_name: impl Into<String>) -> Self {
        Self {
            hir: BuilderOwnedHir::new(circuit_name),
            current: None,
            signals: HashMap::new(),
            widths: HashMap::new(),
            types: HashMap::new(),
            memories: HashMap::new(),
            domains: HashMap::new(),
            cdc_bridges: HashSet::new(),
            clock_port: None,
            reset_port: None,
            process: None,
            errors: Diagnostics::default(),
            definitions: HashMap::new(),
            defining_body: false,
        }
    }

    fn push_err(&mut self, d: Diagnostic) {
        self.errors.push(d);
    }

    pub fn begin_module(&mut self, name: impl Into<String>, span: Span) {
        if self.current.is_some() || self.defining_body {
            self.module_error(
                "rhdl::E0240",
                span,
                "cannot begin a module while another module is active",
                "已有活动模块，不能再次开始模块",
            );
            return;
        }
        self.signals.clear();
        self.widths.clear();
        self.types.clear();
        self.memories.clear();
        self.domains.clear();
        self.cdc_bridges.clear();
        self.clock_port = None;
        self.reset_port = None;
        self.process = None;
        self.current = Some(Module {
            name: name.into(),
            ports: Vec::new(),
            body: Vec::new(),
            span,
        });
    }

    fn module_error(&mut self, code: &str, span: Span, en: &str, zh: &str) {
        self.push_err(Diagnostic {
            span,
            code: code.into(),
            en: en.into(),
            zh: zh.into(),
        });
    }

    /// Define or reuse a module in this session (FR194).
    ///
    /// Parameters are sorted by key before invoking the non-capturing function
    /// pointer. Duplicate keys are rejected. Every request executes the body;
    /// reuse requires equal parameters and the complete same HIR, including
    /// source spans. Function addresses are never used as definition identity.
    /// The callback declares ports, statements and processes only: it must not
    /// begin/end modules or recursively call this helper. Finish the session
    /// once after all definitions. Any failure also poisons the session.
    pub fn define_module(
        &mut self,
        name: impl Into<String>,
        mut params: Vec<(String, u32)>,
        body: fn(&mut ElaborateSession, &[(String, u32)]) -> Result<(), Diagnostics>,
    ) -> Result<String, Diagnostics> {
        let name = name.into();
        if self.current.is_some() || self.defining_body {
            self.module_error(
                "rhdl::E0240",
                Span::default(),
                "define_module requires no active module",
                "define_module 要求当前没有活动模块",
            );
            return Err(self.errors.clone());
        }
        if !valid_module_identifier(&name) {
            self.module_error(
                "rhdl::E0241",
                Span::default(),
                "module name must be a non-reserved ASCII HDL identifier",
                "模块名必须是非保留字的 ASCII HDL 标识符",
            );
        }
        params.sort_by(|a, b| a.0.cmp(&b.0));
        if params.windows(2).any(|p| p[0].0 == p[1].0) {
            self.module_error(
                "rhdl::E0242",
                Span::default(),
                "duplicate module parameter key",
                "模块参数键重复",
            );
        }
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        self.begin_module(&name, Span::default());
        self.defining_body = true;
        if let Err(errors) = body(self, &params) {
            // A nested helper can return diagnostics already recorded in this
            // session. Preserve distinct callback diagnostics without repeating
            // the same code, span and bilingual messages on propagation.
            for diagnostic in errors.0 {
                if !self.errors.0.contains(&diagnostic) {
                    self.errors.push(diagnostic);
                }
            }
            let failure = Diagnostic {
                span: Span::default(),
                code: "rhdl::E0243".into(),
                en: "module definition callback failed".into(),
                zh: "模块定义回调失败".into(),
            };
            if !self.errors.0.contains(&failure) {
                self.errors.push(failure);
            }
        }
        self.defining_body = false;
        if self.process.is_some() {
            self.module_error(
                "rhdl::E0243",
                Span::default(),
                "module definition callback left an open process",
                "模块定义回调留下未结束的过程",
            );
            self.end_process();
        }
        if let Some(candidate) = self.current.take() {
            if let Some(existing) = self
                .hir
                .circuit_mut()
                .modules
                .iter()
                .find(|m| m.name == name)
            {
                if self.definitions.get(&name) != Some(&params) || existing != &candidate {
                    let previous = self.definitions.get(&name).map_or_else(
                        || "<manual module / 手工模块>".to_owned(),
                        |parameters| format!("{parameters:?}"),
                    );
                    self.module_error(
                        "rhdl::E0244", candidate.span,
                        &format!("module '{name}' conflicts with parameters, definition content, or a manual module; previous parameters: {previous}; requested parameters: {params:?}"),
                        &format!("模块 '{name}' 与参数、定义内容或手工模块冲突；原参数：{previous}；请求参数：{params:?}"),
                    );
                }
            } else {
                self.hir.add_module(candidate);
                self.definitions.insert(name.clone(), params);
            }
        } else {
            self.module_error(
                "rhdl::E0243",
                Span::default(),
                "module definition callback changed the active module",
                "模块定义回调改变了活动模块",
            );
        }
        if self.errors.is_empty() {
            Ok(name)
        } else {
            Err(self.errors.clone())
        }
    }

    /// Bind a phantom clock-domain id to a signal (AD-22).
    pub fn bind_domain(&mut self, name: impl Into<String>, domain: u32) {
        self.domains.insert(name.into(), domain);
    }

    /// Mark a CDC bridge signal that may legally cross domains.
    pub fn mark_cdc_bridge(&mut self, name: impl Into<String>) {
        self.cdc_bridges.insert(name.into());
    }

    /// Declare a synthesizable 2-stage CDC synchronizer register pair (FR79 / AD-29).
    ///
    /// Creates `{stem}_ff0` and `{stem}_ff1`, binds both to `dst_domain`, and marks
    /// `{stem}_ff0` as a CDC bridge so [`Self::assign_reg_d_from`] / [`Self::assign_net`]
    /// may sample a source-domain `din`. Call [`Self::connect_double_flop`] inside a
    /// sequential process to wire `ff0 ← din`, `ff1 ← ff0`.
    ///
    /// **Stage count:** 2. **Latency:** 2 destination-domain ticks to `ff1` (see
    /// `DoubleFlop::LATENCY_DST_TICKS` in prelude).
    pub fn declare_double_flop_stages(
        &mut self,
        stem: impl Into<String>,
        ty: GroundType,
        dst_domain: u32,
        span: Span,
    ) -> (String, String) {
        let stem = stem.into();
        let ff0 = format!("{stem}_ff0");
        let ff1 = format!("{stem}_ff1");
        self.declare_reg(ff0.clone(), ty.clone(), span);
        self.declare_reg(ff1.clone(), ty, span);
        self.bind_domain(&ff0, dst_domain);
        self.bind_domain(&ff1, dst_domain);
        self.mark_cdc_bridge(&ff0);
        (ff0, ff1)
    }

    /// Wire a previously declared DoubleFlop pair: `ff0 ← din`, `ff1 ← ff0`.
    ///
    /// Must be called inside a sequential process. `ff0`/`ff1` should come from
    /// [`Self::declare_double_flop_stages`].
    pub fn connect_double_flop(
        &mut self,
        ff0: impl Into<String>,
        ff1: impl Into<String>,
        din: impl Into<String>,
        span: Span,
    ) {
        let ff0 = ff0.into();
        let ff1 = ff1.into();
        let din = din.into();
        self.assign_reg_d_from(&ff0, &din, span);
        self.assign_reg_d_from(&ff1, &ff0, span);
    }

    fn reject_illegal_cdc(&mut self, from: &str, to: &str, span: Span) -> bool {
        let src_dom = self.domains.get(from).copied().unwrap_or(0);
        let dst_dom = self.domains.get(to).copied().unwrap_or(0);
        if src_dom != dst_dom && !self.cdc_bridges.contains(to) && !self.cdc_bridges.contains(from)
        {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0220".into(),
                en: format!(
                    "illegal clock-domain crossing '{from}'(D{src_dom}) → '{to}'(D{dst_dom}); use DoubleFlop/SyncFIFO"
                ),
                zh: format!(
                    "非法跨时钟域：'{from}'(D{src_dom}) → '{to}'(D{dst_dom})；请用 DoubleFlop/SyncFIFO"
                ),
            });
            return true;
        }
        false
    }

    fn record_width(&mut self, name: &str, ty: &GroundType) {
        let w = match ty {
            GroundType::UInt { width } | GroundType::SInt { width } => *width,
            GroundType::Clock | GroundType::Reset | GroundType::Bool | GroundType::Analog => 1,
        };
        self.widths.insert(name.to_string(), w);
        self.types.insert(name.to_string(), ty.clone());
    }

    /// Fail-before-emit when a flattened leaf / port name collides (FR51).
    fn ensure_fresh_signal_name(&mut self, name: &str, span: Span) -> bool {
        if self.signals.contains_key(name) {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0152".into(),
                en: format!(
                    "flattened leaf/port name '{name}' collides with an existing signal (rename Bundle members or fields so `{{field}}_{{member}}` / `{{field}}_{{i}}` stay unique)"
                ),
                zh: format!(
                    "展平叶/端口名 '{name}' 与已有信号冲突（请调整 Bundle 成员或字段名，保证 `{{field}}_{{member}}` / `{{field}}_{{i}}` 唯一）"
                ),
            });
            return false;
        }
        true
    }

    pub fn add_input(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        if !self.ensure_fresh_signal_name(&name, span) {
            return;
        }
        if matches!(ty, GroundType::Clock) {
            self.clock_port = Some(name.clone());
        }
        if matches!(ty, GroundType::Reset) {
            self.reset_port = Some(name.clone());
        }
        self.signals.insert(name.clone(), SignalKind::Input);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.ports.push(Port {
                name,
                direction: PortDirection::Input,
                ty,
                span,
            });
        }
    }

    pub fn add_output(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        if !self.ensure_fresh_signal_name(&name, span) {
            return;
        }
        self.signals.insert(name.clone(), SignalKind::Output);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.ports.push(Port {
                name,
                direction: PortDirection::Output,
                ty,
                span,
            });
        }
    }

    /// Top-level InOut / Analog IO (FR27). Non-top uses are rejected at freeze.
    pub fn add_inout(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        if !self.ensure_fresh_signal_name(&name, span) {
            return;
        }
        self.signals.insert(name.clone(), SignalKind::Wire);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.ports.push(Port {
                name,
                direction: PortDirection::InOut,
                ty,
                span,
            });
        }
    }

    pub fn declare_wire(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        self.signals.insert(name.clone(), SignalKind::Wire);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::WireDecl { name, ty, span });
        }
    }

    pub fn declare_reg(&mut self, name: impl Into<String>, ty: GroundType, span: Span) {
        let name = name.into();
        let (Some(clock), Some(reset)) = (self.clock_port.clone(), self.reset_port.clone()) else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0124".into(),
                en: "cannot declare Reg before Clock and Reset ports are declared".into(),
                zh: "声明寄存器前必须先有 Clock 与 Reset 端口".into(),
            });
            return;
        };
        self.signals.insert(name.clone(), SignalKind::Reg);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::RegDecl {
                name,
                ty,
                clock,
                reset,
                async_reset: false,
                has_enable: false,
                span,
            });
        }
    }

    /// Declare a register with optional async reset / clock enable (AD-23).
    pub fn declare_reg_ex(
        &mut self,
        name: impl Into<String>,
        ty: GroundType,
        async_reset: bool,
        has_enable: bool,
        span: Span,
    ) {
        let name = name.into();
        let (Some(clock), Some(reset)) = (self.clock_port.clone(), self.reset_port.clone()) else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0124".into(),
                en: "cannot declare Reg before Clock and Reset ports are declared".into(),
                zh: "声明寄存器前必须先有 Clock 与 Reset 端口".into(),
            });
            return;
        };
        self.signals.insert(name.clone(), SignalKind::Reg);
        self.record_width(&name, &ty);
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::RegDecl {
                name,
                ty,
                clock,
                reset,
                async_reset,
                has_enable,
                span,
            });
        }
    }

    /// Declare SyncReadMem (CHIRRTL-friendly; sync_read=true).
    pub fn declare_sync_read_mem(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        span: Span,
    ) {
        self.declare_mem_inner(name, depth, width, true, None, span);
    }

    /// Declare Mem (async-read / reg-file style; sync_read=false).
    pub fn declare_mem(&mut self, name: impl Into<String>, depth: u32, width: u32, span: Span) {
        self.declare_mem_inner(name, depth, width, false, None, span);
    }

    /// Declare Mem with elaborate-time init words (FR73). Prefer
    /// [`Self::declare_mem_with_init_fn`] for generator closures.
    pub fn declare_mem_with_init(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        init: Vec<u64>,
        span: Span,
    ) {
        self.declare_mem_inner(name, depth, width, false, Some(init), span);
    }

    /// Declare SyncReadMem with elaborate-time init words (FR73).
    pub fn declare_sync_read_mem_with_init(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        init: Vec<u64>,
        span: Span,
    ) {
        self.declare_mem_inner(name, depth, width, true, Some(init), span);
    }

    /// Elaborate-time Mem init generator (FR73 / AD-18).
    ///
    /// Runs `f(addr)` for each address inside the session, masks to `width` bits,
    /// and stores plain `Vec<u64>` on the HIR mem node. The closure does **not**
    /// enter FrozenHir (NFR36). Only **non-capturing** `Fn` is legal; capturing
    /// Wire/Reg/signal refs must be rejected via
    /// [`Self::assert_no_hw_capture`] (`rhdl::E0142`).
    pub fn declare_mem_with_init_fn<F>(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        f: F,
        span: Span,
    ) where
        F: Fn(usize) -> u64,
    {
        let init = generate_mem_init_words(depth, width, f);
        self.declare_mem_inner(name, depth, width, false, Some(init), span);
    }

    /// SyncReadMem variant of [`Self::declare_mem_with_init_fn`] (FR73).
    pub fn declare_sync_read_mem_with_init_fn<F>(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        f: F,
        span: Span,
    ) where
        F: Fn(usize) -> u64,
    {
        let init = generate_mem_init_words(depth, width, f);
        self.declare_mem_inner(name, depth, width, true, Some(init), span);
    }

    fn declare_mem_inner(
        &mut self,
        name: impl Into<String>,
        depth: u32,
        width: u32,
        sync_read: bool,
        init: Option<Vec<u64>>,
        span: Span,
    ) {
        let name = name.into();
        if depth == 0 || width == 0 {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0210".into(),
                en: "Mem depth and width must be non-zero".into(),
                zh: "Mem 的 depth 与 width 必须非零".into(),
            });
            return;
        }
        let init = match init {
            None => None,
            Some(words) => {
                if width > 64 {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0211".into(),
                        en: "Mem init path supports width ≤ 64 for this MVP".into(),
                        zh: "本 MVP 的 Mem 初值路径仅支持 width ≤ 64".into(),
                    });
                    return;
                }
                if words.len() != depth as usize {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0212".into(),
                        en: format!(
                            "Mem init length {} does not match depth {depth}",
                            words.len()
                        ),
                        zh: format!("Mem 初值长度 {} 与 depth {depth} 不一致", words.len()),
                    });
                    return;
                }
                Some(words.into_iter().map(|w| mask_mem_word(w, width)).collect())
            }
        };
        self.signals.insert(name.clone(), SignalKind::Wire);
        self.widths.insert(name.clone(), width);
        self.types.insert(name.clone(), GroundType::UInt { width });
        self.memories
            .insert(name.clone(), (depth, width, sync_read));
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::MemDecl {
                name,
                depth,
                width,
                sync_read,
                init,
                span,
            });
        }
    }

    /// Same-width binary add. Returns Err diagnostic via session if widths differ.
    pub fn check_add(&mut self, lhs: &str, rhs: &str, span: Span) -> Option<u32> {
        let lw = self.widths.get(lhs).copied();
        let rw = self.widths.get(rhs).copied();
        match (lw, rw) {
            (Some(a), Some(b)) if a == b => Some(a),
            (Some(a), Some(b)) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0130".into(),
                    en: format!(
                        "add requires same width; '{lhs}' is {a}, '{rhs}' is {b} (use pad/trunc)"
                    ),
                    zh: format!("加法要求同位宽；'{lhs}' 为 {a}，'{rhs}' 为 {b}（请用 pad/trunc）"),
                });
                None
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0113".into(),
                    en: format!("unknown signal in add ('{lhs}', '{rhs}')"),
                    zh: format!("加法中有未知信号（'{lhs}', '{rhs}'）"),
                });
                None
            }
        }
    }

    pub fn check_connect(&mut self, lhs: &str, rhs: &str, span: Span) -> Option<u32> {
        let lw = self.widths.get(lhs).copied();
        let rw = self.widths.get(rhs).copied();
        match (lw, rw) {
            (Some(a), Some(b)) if a == b => Some(a),
            (Some(a), Some(b)) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0131".into(),
                    en: format!(
                        "connect requires same width; '{lhs}' is {a}, '{rhs}' is {b} (use pad/trunc)"
                    ),
                    zh: format!(
                        "连接要求同位宽；'{lhs}' 为 {a}，'{rhs}' 为 {b}（请用 pad/trunc）"
                    ),
                });
                None
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0113".into(),
                    en: format!("unknown signal in connect ('{lhs}', '{rhs}')"),
                    zh: format!("连接中有未知信号（'{lhs}', '{rhs}'）"),
                });
                None
            }
        }
    }

    /// Explicit zero-extend / sign-pad to a wider width; records result as a temp wire name.
    pub fn pad_to(
        &mut self,
        src: &str,
        to_width: u32,
        dest: impl Into<String>,
        span: Span,
    ) -> bool {
        let Some(from) = self.widths.get(src).copied() else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0113".into(),
                en: format!("unknown signal '{src}' in pad"),
                zh: format!("pad 中未知信号 '{src}'"),
            });
            return false;
        };
        if to_width <= from {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0132".into(),
                en: format!("pad requires to_width > from_width ({to_width} <= {from})"),
                zh: format!("pad 要求目标位宽大于源位宽（{to_width} <= {from}）"),
            });
            return false;
        }
        let dest = dest.into();
        self.declare_wire(dest, GroundType::UInt { width: to_width }, span);
        let _ = bitloom_hir::Expr::Pad {
            from_width: from,
            to_width,
            span,
        };
        true
    }

    pub fn trunc_to(
        &mut self,
        src: &str,
        to_width: u32,
        dest: impl Into<String>,
        span: Span,
    ) -> bool {
        let Some(from) = self.widths.get(src).copied() else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0113".into(),
                en: format!("unknown signal '{src}' in trunc"),
                zh: format!("trunc 中未知信号 '{src}'"),
            });
            return false;
        };
        if to_width >= from {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0133".into(),
                en: format!("trunc requires to_width < from_width ({to_width} >= {from})"),
                zh: format!("trunc 要求目标位宽小于源位宽（{to_width} >= {from}）"),
            });
            return false;
        }
        let dest = dest.into();
        self.declare_wire(dest, GroundType::UInt { width: to_width }, span);
        let _ = bitloom_hir::Expr::Trunc {
            from_width: from,
            to_width,
            span,
        };
        true
    }

    pub fn begin_combinational(&mut self, span: Span) {
        if self.process.is_some() {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0101".into(),
                en: "nested processes are not allowed".into(),
                zh: "不允许嵌套硬件过程".into(),
            });
            return;
        }
        self.process = Some(ProcessState::Combinational {
            assigns: Vec::new(),
            path_assigned: vec![HashSet::new()],
            pending_branches: Vec::new(),
            span,
        });
    }

    pub fn begin_sequential(&mut self, span: Span) {
        if self.process.is_some() {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0101".into(),
                en: "nested processes are not allowed".into(),
                zh: "不允许嵌套硬件过程".into(),
            });
            return;
        }
        self.process = Some(ProcessState::Sequential {
            assigns: Vec::new(),
            span,
        });
    }

    /// Start an if-branch inside a combinational process (latch analysis).
    pub fn begin_then(&mut self, span: Span) {
        let err = match self.process.as_mut() {
            Some(ProcessState::Combinational {
                path_assigned,
                pending_branches,
                ..
            }) => {
                pending_branches.push((HashSet::new(), false));
                path_assigned.push(HashSet::new());
                None
            }
            Some(ProcessState::Sequential { .. }) => Some(Diagnostic {
                span,
                code: "rhdl::E0102".into(),
                en: "branch tracking for latch checks is only valid in combinational processes"
                    .into(),
                zh: "仅组合过程支持 if/else 赋值完整性检查".into(),
            }),
            None => Some(Diagnostic {
                span,
                code: "rhdl::E0103".into(),
                en: "assignment control outside a marked combinational/sequential process".into(),
                zh: "在未标注的 comb/seq 过程外使用分支".into(),
            }),
        };
        if let Some(d) = err {
            self.push_err(d);
        }
    }

    pub fn begin_else(&mut self, span: Span) {
        let err = match self.process.as_mut() {
            Some(ProcessState::Combinational {
                path_assigned,
                pending_branches,
                ..
            }) => {
                let then_set = path_assigned.pop().unwrap_or_default();
                if let Some(last) = pending_branches.last_mut() {
                    last.0 = then_set;
                    last.1 = true;
                    path_assigned.push(HashSet::new());
                    None
                } else {
                    Some(Diagnostic {
                        span,
                        code: "rhdl::E0102".into(),
                        en: "else without an open combinational then-branch".into(),
                        zh: "else 没有对应的组合 then 分支".into(),
                    })
                }
            }
            _ => Some(Diagnostic {
                span,
                code: "rhdl::E0102".into(),
                en: "else without an open combinational then-branch".into(),
                zh: "else 没有对应的组合 then 分支".into(),
            }),
        };
        if let Some(d) = err {
            self.push_err(d);
        }
    }

    pub fn end_if(&mut self, span: Span) {
        let mut latch_errs = Vec::new();
        let err = match self.process.as_mut() {
            Some(ProcessState::Combinational {
                path_assigned,
                pending_branches,
                ..
            }) => {
                let current = path_assigned.pop().unwrap_or_default();
                let Some((stored_then, had_else)) = pending_branches.pop() else {
                    latch_errs.push(Diagnostic {
                        span,
                        code: "rhdl::E0102".into(),
                        en: "end_if without begin_then".into(),
                        zh: "end_if 缺少 begin_then".into(),
                    });
                    for d in latch_errs {
                        self.push_err(d);
                    }
                    return;
                };

                let (then_set, else_set) = if had_else {
                    (stored_then, current)
                } else {
                    (current, HashSet::new())
                };

                let union: HashSet<_> = then_set.union(&else_set).cloned().collect();
                let inter: HashSet<_> = then_set.intersection(&else_set).cloned().collect();
                for name in union.difference(&inter) {
                    latch_errs.push(Diagnostic {
                        span,
                        code: "rhdl::E0110".into(),
                        en: format!(
                            "incomplete combinational assignment to '{name}' (would infer a latch)"
                        ),
                        zh: format!("组合赋值不完整：'{name}'（会推断成 latch）"),
                    });
                }

                if let Some(parent) = path_assigned.last_mut() {
                    for n in inter {
                        parent.insert(n);
                    }
                }
                None
            }
            _ => Some(Diagnostic {
                span,
                code: "rhdl::E0102".into(),
                en: "end_if outside combinational process".into(),
                zh: "end_if 不在组合过程中".into(),
            }),
        };
        for d in latch_errs {
            self.push_err(d);
        }
        if let Some(d) = err {
            self.push_err(d);
        }
    }

    /// Combinational assign `dst = lhs + rhs` (same-width required).
    pub fn assign_add(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        let dst = dst.into();
        let lhs = lhs.into();
        let rhs = rhs.into();
        if self.check_add(&lhs, &rhs, span).is_none() {
            return;
        }
        // Reuse assign_net permission checks by temporarily pushing Add.
        let kind = self.signals.get(&dst).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };
        match process_kind {
            Some(ProcessKind::Combinational) => {
                match kind {
                    Some(SignalKind::Wire | SignalKind::Output) => {}
                    Some(SignalKind::Reg) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0111".into(),
                            en: format!("combinational process must not drive Reg '{dst}'"),
                            zh: format!("组合过程不能驱动寄存器 '{dst}'"),
                        });
                        return;
                    }
                    Some(SignalKind::Input) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0112".into(),
                            en: format!("cannot assign to input port '{dst}'"),
                            zh: format!("不能给输入端口 '{dst}' 赋值"),
                        });
                        return;
                    }
                    None => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0113".into(),
                            en: format!("unknown signal '{dst}'"),
                            zh: format!("未知信号 '{dst}'"),
                        });
                        return;
                    }
                }
                if let Some(ProcessState::Combinational {
                    assigns,
                    path_assigned,
                    ..
                }) = self.process.as_mut()
                {
                    assigns.push(Assign {
                        target: AssignTarget::Net(dst.clone()),
                        expr: AssignExpr::Add(lhs, rhs),
                        span,
                    });
                    if let Some(path) = path_assigned.last_mut() {
                        path.insert(dst);
                    }
                }
            }
            Some(ProcessKind::Sequential) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0114".into(),
                    en: format!("sequential process must not drive combinational net '{dst}'"),
                    zh: format!("时序过程不能驱动组合网 '{dst}'"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Combinational assign `dst = lit`.
    pub fn assign_lit(&mut self, dst: impl Into<String>, lit: u64, span: Span) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Lit(lit), span);
    }

    /// Combinational assign `dst = (lhs == rhs)` (0/1).
    pub fn assign_eq(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Eq(lhs.into(), rhs.into()), span);
    }

    /// Combinational assign `dst = sel ? t : f` (`sel != 0` is true).
    pub fn assign_mux(
        &mut self,
        dst: impl Into<String>,
        sel: impl Into<String>,
        t: impl Into<String>,
        f: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(
            dst.into(),
            AssignExpr::Mux {
                sel: sel.into(),
                t: t.into(),
                f: f.into(),
            },
            span,
        );
    }

    /// Combinational `dst = lhs - rhs`.
    pub fn assign_sub(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Sub(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs & rhs`.
    pub fn assign_and(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::And(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs | rhs`.
    pub fn assign_or(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Or(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs ^ rhs`.
    pub fn assign_xor(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Xor(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs << rhs`, truncated to the destination width; large shifts yield zero.
    pub fn assign_shl(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Shl(lhs.into(), rhs.into()), span);
    }

    /// Combinational `dst = lhs >> rhs` (logical); large shifts yield zero.
    pub fn assign_shr(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.push_comb_net_expr(dst.into(), AssignExpr::Shr(lhs.into(), rhs.into()), span);
    }

    fn assign_typed_compare(
        &mut self,
        dst: String,
        lhs: String,
        rhs: String,
        signed: bool,
        span: Span,
    ) {
        let required = if signed { "SInt" } else { "UInt" };
        let valid_operands = matches!(self.types.get(&lhs), Some(GroundType::SInt { .. }))
            && matches!(self.types.get(&rhs), Some(GroundType::SInt { .. }));
        let valid_operands = if signed {
            valid_operands
        } else {
            matches!(self.types.get(&lhs), Some(GroundType::UInt { .. }))
                && matches!(self.types.get(&rhs), Some(GroundType::UInt { .. }))
        };
        let width = self
            .widths
            .get(&lhs)
            .copied()
            .zip(self.widths.get(&rhs).copied())
            .and_then(|(lhs_width, rhs_width)| (lhs_width == rhs_width).then_some(lhs_width));
        if !valid_operands || width.is_none() {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0137".into(),
                en: format!("comparison requires same-width {required} operands"),
                zh: format!("比较要求同位宽 {required} 操作数"),
            });
            return;
        }
        if !matches!(self.types.get(&dst), Some(GroundType::Bool)) {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0138".into(),
                en: format!("comparison destination '{dst}' must be Bool"),
                zh: format!("比较目标 '{dst}' 必须为 Bool"),
            });
            return;
        }
        let expr = if signed {
            AssignExpr::Slt {
                lhs,
                rhs,
                width: width.unwrap(),
            }
        } else {
            AssignExpr::Ult {
                lhs,
                rhs,
                width: width.unwrap(),
            }
        };
        self.push_comb_net_expr(dst, expr, span);
    }

    /// Combinational unsigned comparison of same-width `UInt` operands.
    pub fn assign_ult(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.assign_typed_compare(dst.into(), lhs.into(), rhs.into(), false, span);
    }

    /// Combinational two's-complement comparison of same-width `SInt` operands.
    pub fn assign_slt(
        &mut self,
        dst: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
        span: Span,
    ) {
        self.assign_typed_compare(dst.into(), lhs.into(), rhs.into(), true, span);
    }

    /// Combinational arithmetic right shift of a same-width `SInt` value.
    pub fn assign_sar(
        &mut self,
        dst: impl Into<String>,
        value: impl Into<String>,
        shamt: impl Into<String>,
        span: Span,
    ) {
        let dst = dst.into();
        let value = value.into();
        let shamt = shamt.into();
        let width = self.widths.get(&value).copied();
        let valid = matches!(self.types.get(&value), Some(GroundType::SInt { .. }))
            && self.widths.contains_key(&shamt)
            && matches!(self.types.get(&dst), Some(GroundType::SInt { .. }))
            && self.widths.get(&dst).copied() == width;
        let Some(width) = width else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0139".into(),
                en: format!("arithmetic right shift references unknown value '{value}'"),
                zh: format!("算术右移引用未知值 '{value}'"),
            });
            return;
        };
        if !valid {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0139".into(),
                en: format!(
                    "arithmetic right shift requires SInt value/destination with matching width"
                ),
                zh: "算术右移要求 SInt 输入和同位宽 SInt 目标".into(),
            });
            return;
        }
        self.push_comb_net_expr(
            dst,
            AssignExpr::Sar {
                value,
                shamt,
                width,
            },
            span,
        );
    }

    /// Combinational extraction of `width` bits starting at LSB bit `lo`.
    pub fn assign_slice(
        &mut self,
        dst: impl Into<String>,
        src: impl Into<String>,
        lo: u32,
        width: u32,
        span: Span,
    ) {
        let dst = dst.into();
        let src = src.into();
        let src_width = self.widths.get(&src).copied();
        let dst_width = self.widths.get(&dst).copied();
        if width == 0 || src_width.is_none_or(|w| lo.checked_add(width).is_none_or(|end| end > w)) {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0133".into(),
                en: format!("slice [{lo} +: {width}] is outside source '{src}'"),
                zh: format!("slice [{lo} +: {width}] 超出源 '{src}' 范围"),
            });
            return;
        }
        if dst_width != Some(width) {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0134".into(),
                en: format!("slice destination '{dst}' must have width {width}"),
                zh: format!("slice 目标 '{dst}' 必须为 {width} 位"),
            });
            return;
        }
        self.push_comb_net_expr(dst, AssignExpr::Slice { src, lo, width }, span);
    }

    /// Combinational concatenation with `high` placed above `low`.
    pub fn assign_concat(
        &mut self,
        dst: impl Into<String>,
        high: impl Into<String>,
        low: impl Into<String>,
        span: Span,
    ) {
        let dst = dst.into();
        let high = high.into();
        let low = low.into();
        let low_width = self.widths.get(&low).copied();
        let expected = self
            .widths
            .get(&high)
            .copied()
            .zip(low_width)
            .and_then(|(a, b)| a.checked_add(b));
        if expected.is_none() || self.widths.get(&dst).copied() != expected {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0135".into(),
                en: format!("concat destination '{dst}' must equal widths of '{high}' and '{low}'"),
                zh: format!("concat 目标 '{dst}' 必须等于 '{high}' 与 '{low}' 的位宽之和"),
            });
            return;
        }
        self.push_comb_net_expr(
            dst,
            AssignExpr::Concat {
                high,
                low,
                low_width: low_width.unwrap(),
            },
            span,
        );
    }

    fn assign_extend(&mut self, dst: String, src: String, to_width: u32, sign: bool, span: Span) {
        let src_width = self.widths.get(&src).copied();
        if src_width.is_none_or(|w| to_width <= w)
            || self.widths.get(&dst).copied() != Some(to_width)
        {
            self.push_err(Diagnostic { span, code: "rhdl::E0136".into(), en: format!("extension from '{src}' to {to_width} bits requires a wider matching destination '{dst}'"), zh: format!("从 '{src}' 扩展到 {to_width} 位要求目标 '{dst}' 更宽且位宽匹配") });
            return;
        }
        let from_width = src_width.unwrap();
        let expr = if sign {
            AssignExpr::SignExtend {
                src,
                from_width,
                to_width,
            }
        } else {
            AssignExpr::ZeroExtend {
                src,
                from_width,
                to_width,
            }
        };
        self.push_comb_net_expr(dst, expr, span);
    }

    pub fn assign_zero_extend(
        &mut self,
        dst: impl Into<String>,
        src: impl Into<String>,
        to_width: u32,
        span: Span,
    ) {
        self.assign_extend(dst.into(), src.into(), to_width, false, span);
    }

    pub fn assign_sign_extend(
        &mut self,
        dst: impl Into<String>,
        src: impl Into<String>,
        to_width: u32,
        span: Span,
    ) {
        self.assign_extend(dst.into(), src.into(), to_width, true, span);
    }

    /// Combinational read from an async-read `Mem`.
    pub fn assign_mem_read(
        &mut self,
        dst: impl Into<String>,
        mem: impl Into<String>,
        addr: impl Into<String>,
        span: Span,
    ) {
        let dst = dst.into();
        let mem = mem.into();
        let addr = addr.into();
        let Some((depth, width, sync_read)) = self.memories.get(&mem).copied() else {
            self.push_err(Diagnostic {
                span,
                code: "rhdl::E0213".into(),
                en: format!("unknown memory '{mem}'"),
                zh: format!("未知 memory '{mem}'"),
            });
            return;
        };
        let addr_width = self.widths.get(&addr).copied();
        let required = (32 - (depth - 1).leading_zeros()).max(1);
        if sync_read
            || self.widths.get(&dst).copied() != Some(width)
            || addr_width != Some(required)
        {
            self.push_err(Diagnostic { span, code: "rhdl::E0214".into(), en: format!("combinational read of '{mem}' requires async memory, {required}-bit address, and {width}-bit destination"), zh: format!("组合读取 '{mem}' 要求异步 memory、{required} 位地址和 {width} 位目标") });
            return;
        }
        self.push_comb_net_expr(dst, AssignExpr::MemRead { mem, addr }, span);
    }

    fn push_comb_net_expr(&mut self, dst: String, expr: AssignExpr, span: Span) {
        let kind = self.signals.get(&dst).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };
        match process_kind {
            Some(ProcessKind::Combinational) => {
                match kind {
                    Some(SignalKind::Wire | SignalKind::Output) => {}
                    Some(SignalKind::Reg) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0111".into(),
                            en: format!("combinational process must not drive Reg '{dst}'"),
                            zh: format!("组合过程不能驱动寄存器 '{dst}'"),
                        });
                        return;
                    }
                    Some(SignalKind::Input) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0112".into(),
                            en: format!("cannot assign to input port '{dst}'"),
                            zh: format!("不能给输入端口 '{dst}' 赋值"),
                        });
                        return;
                    }
                    None => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0113".into(),
                            en: format!("unknown signal '{dst}'"),
                            zh: format!("未知信号 '{dst}'"),
                        });
                        return;
                    }
                }
                if let Some(ProcessState::Combinational {
                    assigns,
                    path_assigned,
                    ..
                }) = self.process.as_mut()
                {
                    assigns.push(Assign {
                        target: AssignTarget::Net(dst.clone()),
                        expr,
                        span,
                    });
                    if let Some(path) = path_assigned.last_mut() {
                        path.insert(dst);
                    }
                }
            }
            Some(ProcessKind::Sequential) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0114".into(),
                    en: format!("sequential process must not drive combinational net '{dst}'"),
                    zh: format!("时序过程不能驱动组合网 '{dst}'"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Assign a combinational net / output / wire from `from`.
    pub fn assign_net(&mut self, name: impl Into<String>, from: impl Into<String>, span: Span) {
        let name = name.into();
        let from = from.into();
        if self.reject_illegal_cdc(&from, &name, span) {
            return;
        }
        // Width gate (E0131) before emit — FR51 / FR22 same-width connects.
        if self.check_connect(&name, &from, span).is_none() {
            return;
        }
        let kind = self.signals.get(&name).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };

        match process_kind {
            Some(ProcessKind::Combinational) => {
                match kind {
                    Some(SignalKind::Reg) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0111".into(),
                            en: format!(
                                "combinational process must not drive Reg '{name}' (use Reg.d in sequential)"
                            ),
                            zh: format!(
                                "组合过程不能驱动寄存器 '{name}'（请在时序过程写 Reg.d）"
                            ),
                        });
                        return;
                    }
                    Some(SignalKind::Input) => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0112".into(),
                            en: format!("cannot assign to input port '{name}'"),
                            zh: format!("不能给输入端口 '{name}' 赋值"),
                        });
                        return;
                    }
                    Some(SignalKind::Wire | SignalKind::Output) => {}
                    None => {
                        self.push_err(Diagnostic {
                            span,
                            code: "rhdl::E0113".into(),
                            en: format!("unknown signal '{name}'"),
                            zh: format!("未知信号 '{name}'"),
                        });
                        return;
                    }
                }
                if let Some(ProcessState::Combinational {
                    assigns,
                    path_assigned,
                    ..
                }) = self.process.as_mut()
                {
                    assigns.push(Assign {
                        target: AssignTarget::Net(name.clone()),
                        expr: AssignExpr::Ref(from.clone()),
                        span,
                    });
                    if let Some(path) = path_assigned.last_mut() {
                        path.insert(name);
                    }
                }
            }
            Some(ProcessKind::Sequential) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0114".into(),
                    en: format!("sequential process must not drive combinational net '{name}'"),
                    zh: format!("时序过程不能驱动组合网 '{name}'"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Assign `Reg.d` next-state as `name + 1` (wrapping).
    pub fn assign_reg_d_inc(&mut self, name: impl Into<String>, span: Span) {
        self.assign_reg_d_expr(name, None, span);
    }

    /// Assign `Reg.d` from another signal.
    pub fn assign_reg_d_from(
        &mut self,
        name: impl Into<String>,
        from: impl Into<String>,
        span: Span,
    ) {
        self.assign_reg_d_expr(name, Some(from.into()), span);
    }

    /// Assign `Reg.d` as a 2:1 mux (same-width `t`/`f` as the register).
    ///
    /// Used for handwritten seq goldens alongside [`Self::inline_seq_fn`].
    pub fn assign_reg_d_mux(
        &mut self,
        name: impl Into<String>,
        sel: impl Into<String>,
        t: impl Into<String>,
        f: impl Into<String>,
        span: Span,
    ) {
        let name = name.into();
        let t = t.into();
        let f = f.into();
        if self.check_connect(&name, &t, span).is_none()
            || self.check_connect(&name, &f, span).is_none()
        {
            return;
        }
        self.push_reg_d_assign(
            name,
            AssignExpr::Mux {
                sel: sel.into(),
                t,
                f,
            },
            span,
        );
    }

    /// Sequential SyncReadMem / Mem write: `mem[addr] <= data` (always enabled).
    pub fn assign_mem_write(
        &mut self,
        mem: impl Into<String>,
        addr: impl Into<String>,
        data: impl Into<String>,
        span: Span,
    ) {
        self.assign_mem_write_inner(mem.into(), addr.into(), data.into(), None, span);
    }

    /// Sequential mem write gated by `we` (`we != 0`).
    pub fn assign_mem_write_en(
        &mut self,
        mem: impl Into<String>,
        addr: impl Into<String>,
        data: impl Into<String>,
        we: impl Into<String>,
        span: Span,
    ) {
        self.assign_mem_write_inner(mem.into(), addr.into(), data.into(), Some(we.into()), span);
    }

    fn assign_mem_write_inner(
        &mut self,
        mem: String,
        addr: String,
        data: String,
        we: Option<String>,
        span: Span,
    ) {
        match &self.process {
            Some(ProcessState::Sequential { .. }) => {
                if let Some(ProcessState::Sequential { assigns, .. }) = self.process.as_mut() {
                    assigns.push(Assign {
                        target: AssignTarget::MemWrite { mem, addr, we },
                        expr: AssignExpr::Ref(data),
                        span,
                    });
                }
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0211".into(),
                    en: "mem write must be inside a sequential process".into(),
                    zh: "mem 写必须在 sequential 过程内".into(),
                });
            }
        }
    }

    /// Sample a memory read into a destination register.
    ///
    /// For `Mem`, the register samples the pre-edge bank on this edge. For
    /// `SyncReadMem`, the memory read stage samples on every edge and the
    /// destination register receives the previous read-stage value: an address
    /// presented before edge N becomes visible in the register at edge N+1.
    /// The destination's current-edge enable/reset controls this register update,
    /// not memory-read sampling. Reset clears the register but preserves the bank;
    /// writes are suppressed during reset. Internal same-address read/write uses
    /// the pre-write bank; FIRRTL collision values remain undefined.
    pub fn assign_reg_d_mem_read(
        &mut self,
        reg: impl Into<String>,
        mem: impl Into<String>,
        addr: impl Into<String>,
        span: Span,
    ) {
        let reg = reg.into();
        let mem = mem.into();
        let addr = addr.into();
        match &self.process {
            Some(ProcessState::Sequential { .. }) => {
                if let Some(ProcessState::Sequential { assigns, .. }) = self.process.as_mut() {
                    assigns.push(Assign {
                        target: AssignTarget::RegD(reg),
                        expr: AssignExpr::MemRead { mem, addr },
                        span,
                    });
                }
            }
            _ => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0212".into(),
                    en: "sync mem read into Reg must be inside a sequential process".into(),
                    zh: "SyncReadMem 读入寄存器必须在 sequential 过程内".into(),
                });
            }
        }
    }

    fn assign_reg_d_expr(&mut self, name: impl Into<String>, from: Option<String>, span: Span) {
        let name = name.into();
        if let Some(ref src) = from {
            if self.reject_illegal_cdc(src, &name, span) {
                return;
            }
            if self.check_connect(&name, src, span).is_none() {
                return;
            }
        }
        let expr = match from {
            Some(src) => AssignExpr::Ref(src),
            None => AssignExpr::Inc(name.clone()),
        };
        self.push_reg_d_assign(name, expr, span);
    }

    pub fn end_process(&mut self) {
        let Some(state) = self.process.take() else {
            return;
        };
        match state {
            ProcessState::Combinational {
                assigns,
                pending_branches,
                span,
                ..
            } => {
                if !pending_branches.is_empty() {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0102".into(),
                        en: "unclosed if/else in combinational process".into(),
                        zh: "组合过程中有未关闭的 if/else".into(),
                    });
                }
                if let Some(m) = self.current.as_mut() {
                    m.body.push(Stmt::Process(Process {
                        kind: ProcessKind::Combinational,
                        assigns,
                        span,
                    }));
                }
            }
            ProcessState::Sequential { assigns, span } => {
                if let Some(m) = self.current.as_mut() {
                    m.body.push(Stmt::Process(Process {
                        kind: ProcessKind::Sequential,
                        assigns,
                        span,
                    }));
                }
            }
        }
    }

    pub fn end_module(&mut self) {
        if self.defining_body {
            self.module_error(
                "rhdl::E0243",
                Span::default(),
                "module definition callback must not call end_module",
                "模块定义回调不得调用 end_module",
            );
            return;
        }
        if self.process.is_some() {
            self.end_process();
        }
        if let Some(m) = self.current.take() {
            self.hir.add_module(m);
        }
        self.signals.clear();
        self.widths.clear();
        self.clock_port = None;
        self.reset_port = None;
    }

    pub fn finish(mut self) -> Result<FrozenHir, Diagnostics> {
        if self.current.is_some() || self.defining_body {
            self.module_error(
                "rhdl::E0240",
                Span::default(),
                "cannot finish with an active module; call end_module first",
                "不能冻结活动模块，请先调用 end_module",
            );
        }
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        bitloom_hir::seal_from_builder(self.hir)
    }

    /// Record a synthesizable-path violation (heap, threads, f64, capturing closure, …)
    /// as a structured diagnostic (`rhdl::E0141` / FR16).
    pub fn reject_unsynthesizable(&mut self, construct: &str, span: Span) {
        self.push_err(Diagnostic {
            span,
            code: "rhdl::E0141".into(),
            en: format!(
                "unsynthesizable construct '{construct}' is not allowed on the cycle-accurate path"
            ),
            zh: format!("周期精确路径不允许不可综合构造 '{construct}'"),
        });
    }

    /// Reject capturing a hardware Wire/Reg/signal ref into an elaborate-time
    /// generator or factory closure (FR73 / NFR35 / AD-18).
    ///
    /// Stable code: **`rhdl::E0142`**. Distinct from FR16 cycle-accurate
    /// [`Self::reject_unsynthesizable`] (`rhdl::E0141`).
    pub fn reject_hw_capture(&mut self, capture: &HwCaptureRef, span: Span) {
        let kind = capture.kind_label();
        self.push_err(Diagnostic {
            span,
            code: "rhdl::E0142".into(),
            en: format!(
                "illegal capture of hardware {kind} '{}' into elaborate-time generator closure; \
                 only non-capturing Fn that dissolves before freeze is allowed (FR73 / NFR35 / AD-18)",
                capture.name
            ),
            zh: format!(
                "不允许将硬件 {kind} '{}' 捕获进 elaborate-time 生成器闭包；\
                 仅允许冻前消解的非捕获 Fn（FR73 / NFR35 / AD-18）",
                capture.name
            ),
        });
    }

    /// Assert that no hardware Wire/Reg/signal refs were captured into a
    /// generator / factory context. Empty slice is a no-op (legal non-capturing path).
    ///
    /// Call when a documented illegal capture is present (ATDD / macros / typed
    /// handles). Any token → `rhdl::E0142` at [`Self::finish`].
    pub fn assert_no_hw_capture(&mut self, captures: &[HwCaptureRef], span: Span) {
        for c in captures {
            self.reject_hw_capture(c, span);
        }
    }

    /// Reject a documented SynthesizableClosure constraint breach (FR74 / Cap-R-60).
    ///
    /// Stable codes: **`rhdl::E0143`** (heap), **`rhdl::E0144`** (runtime capture
    /// state), **`rhdl::E0145`** (impure). Does **not** emit closure IR into HIR
    /// (NFR36) — diagnostics only. Distinct from E0141 / E0142.
    pub fn reject_unsynthesizable_closure(
        &mut self,
        violation: &SynthesizableClosureViolation,
        span: Span,
    ) {
        for d in diagnose_synthesizable_closure_violations(std::slice::from_ref(violation), span).0
        {
            self.push_err(d);
        }
    }

    /// Cap-R-60 check hook: record all documented SynthesizableClosure violations.
    ///
    /// Empty `violations` is a no-op (legal empty/simple closure — paves 28.2).
    /// Reachable from design crates via prelude and from future
    /// `cargo bitloom check` (or equivalent) wrappers.
    pub fn check_synthesizable_closure(
        &mut self,
        violations: &[SynthesizableClosureViolation],
        span: Span,
    ) {
        for v in violations {
            self.reject_unsynthesizable_closure(v, span);
        }
    }

    /// Cap-R-60 helper: run [`SynthesizableClosure::synthesizable_closure_violations`]
    /// and record any tokens (empty = pass).
    pub fn check_synthesizable_closure_marker<C: SynthesizableClosure>(
        &mut self,
        closure: &C,
        span: Span,
    ) {
        let vs = closure.synthesizable_closure_violations();
        self.check_synthesizable_closure(&vs, span);
    }

    /// Inline a synthesizable combinational transform (FR75 / Cap-R-55).
    ///
    /// Must be called inside an open [`Self::begin_combinational`] process
    /// (same rules as `assign_*`). Sequence:
    /// 1. Cap-R-60 [`Self::check_synthesizable_closure`] on `violations`
    /// 2. If any violation token was supplied, **do not** expand (finish fails)
    /// 3. Otherwise invoke `f(args)` once and lower [`CombInline`] through
    ///    existing Wire/`assign_*` paths — FrozenHir holds only ordinary
    ///    [`AssignExpr`] (NFR36; no `Fn` objects after freeze)
    ///
    /// Incomplete-assign / latch analysis (AD-18) still applies to `dst`.
    /// Sequential-block inline: [`Self::inline_seq_fn`] (Cap-R-56 / Cap-R-70).
    ///
    /// ```ignore
    /// session.begin_combinational(span);
    /// session.inline_comb_fn("y", &["a", "b"], &[], span, |args| {
    ///     CombInline::Add(args[0].into(), args[1].into())
    /// });
    /// session.end_process();
    /// ```
    pub fn inline_comb_fn<F>(
        &mut self,
        dst: impl Into<String>,
        args: &[&str],
        violations: &[SynthesizableClosureViolation],
        span: Span,
        f: F,
    ) where
        F: FnOnce(&[&str]) -> CombInline,
    {
        self.check_synthesizable_closure(violations, span);
        if !violations.is_empty() {
            return;
        }
        let inline = f(args);
        self.apply_comb_inline(dst.into(), inline, span);
    }

    /// Cap-R-55 helper: run marker [`SynthesizableClosure`] check then inline.
    pub fn inline_comb_fn_marker<C, F>(
        &mut self,
        dst: impl Into<String>,
        args: &[&str],
        marker: &C,
        span: Span,
        f: F,
    ) where
        C: SynthesizableClosure,
        F: FnOnce(&[&str]) -> CombInline,
    {
        let vs = marker.synthesizable_closure_violations();
        self.inline_comb_fn(dst, args, &vs, span, f);
    }

    fn apply_comb_inline(&mut self, dst: String, inline: CombInline, span: Span) {
        match inline {
            CombInline::Ref(src) => self.assign_net(dst, src, span),
            CombInline::Lit(v) => self.assign_lit(dst, v, span),
            CombInline::Add(l, r) => self.assign_add(dst, l, r, span),
            CombInline::Sub(l, r) => self.assign_sub(dst, l, r, span),
            CombInline::And(l, r) => self.assign_and(dst, l, r, span),
            CombInline::Or(l, r) => self.assign_or(dst, l, r, span),
            CombInline::Xor(l, r) => self.assign_xor(dst, l, r, span),
            CombInline::Eq(l, r) => self.assign_eq(dst, l, r, span),
            CombInline::Mux { sel, t, f } => self.assign_mux(dst, sel, t, f, span),
        }
    }

    /// Cap-R-70: record documented seq-ownership violations (`rhdl::E0146`).
    pub fn reject_seq_ownership_violation(
        &mut self,
        violation: &SeqOwnershipViolation,
        span: Span,
    ) {
        for d in diagnose_seq_ownership_violations(std::slice::from_ref(violation), span).0 {
            self.push_err(d);
        }
    }

    /// Cap-R-70 check hook: record all documented seq-ownership violations.
    /// Empty `violations` is a no-op.
    pub fn check_seq_ownership(&mut self, violations: &[SeqOwnershipViolation], span: Span) {
        for v in violations {
            self.reject_seq_ownership_violation(v, span);
        }
    }

    /// Inline a synthesizable sequential transform onto `Reg.d` (FR75 / Cap-R-56).
    ///
    /// Must be called inside an open [`Self::begin_sequential`] process.
    /// Sequence:
    /// 1. Cap-R-60 [`Self::check_synthesizable_closure`] on `synth_violations`
    /// 2. Cap-R-70 [`Self::check_seq_ownership`] on `ownership_violations`
    /// 3. Cap-R-70 auto-detect: if `dst_reg.d` is already assigned in this
    ///    sequential process, diagnose `rhdl::E0146` and **do not** expand
    /// 4. If any Cap-R-60 / Cap-R-70 token was supplied, **do not** expand
    /// 5. Otherwise invoke `f(args)` once and lower [`SeqInline`] to ordinary
    ///    sequential [`AssignExpr`] / `Reg.d` — FrozenHir holds no `Fn` (NFR36)
    ///
    /// Cross-process multi-drive / undriven after expand still use AD-4 freeze
    /// checks (`rhdl::E0140`, …).
    ///
    /// ```ignore
    /// session.begin_sequential(span);
    /// session.inline_seq_fn("count", &[], &[], &[], span, |_args| SeqInline::Inc);
    /// session.end_process();
    /// ```
    pub fn inline_seq_fn<F>(
        &mut self,
        dst_reg: impl Into<String>,
        args: &[&str],
        synth_violations: &[SynthesizableClosureViolation],
        ownership_violations: &[SeqOwnershipViolation],
        span: Span,
        f: F,
    ) where
        F: FnOnce(&[&str]) -> SeqInline,
    {
        self.check_synthesizable_closure(synth_violations, span);
        self.check_seq_ownership(ownership_violations, span);
        let dst = dst_reg.into();
        let already = self.seq_reg_d_already_assigned(&dst);
        if already {
            self.reject_seq_ownership_violation(
                &SeqOwnershipViolation::illegal_mutable_borrow(format!(
                    "Reg.d '{dst}' already assigned in this sequential process"
                )),
                span,
            );
        }
        if !synth_violations.is_empty() || !ownership_violations.is_empty() || already {
            return;
        }
        let inline = f(args);
        self.apply_seq_inline(dst, inline, span);
    }

    /// Cap-R-56 helper: marker Cap-R-60 check then seq inline (Cap-R-70 ownership args).
    pub fn inline_seq_fn_marker<C, F>(
        &mut self,
        dst_reg: impl Into<String>,
        args: &[&str],
        marker: &C,
        ownership_violations: &[SeqOwnershipViolation],
        span: Span,
        f: F,
    ) where
        C: SynthesizableClosure,
        F: FnOnce(&[&str]) -> SeqInline,
    {
        let vs = marker.synthesizable_closure_violations();
        self.inline_seq_fn(dst_reg, args, &vs, ownership_violations, span, f);
    }

    fn seq_reg_d_already_assigned(&self, name: &str) -> bool {
        match &self.process {
            Some(ProcessState::Sequential { assigns, .. }) => assigns
                .iter()
                .any(|a| matches!(&a.target, AssignTarget::RegD(n) if n == name)),
            _ => false,
        }
    }

    fn apply_seq_inline(&mut self, dst: String, inline: SeqInline, span: Span) {
        let expr = match inline {
            SeqInline::Inc => AssignExpr::Inc(dst.clone()),
            SeqInline::Comb(CombInline::Ref(src)) => {
                if self.check_connect(&dst, &src, span).is_none() {
                    return;
                }
                AssignExpr::Ref(src)
            }
            SeqInline::Comb(CombInline::Lit(v)) => AssignExpr::Lit(v),
            SeqInline::Comb(CombInline::Add(l, r)) => {
                if self.check_add(&l, &r, span).is_none() {
                    return;
                }
                AssignExpr::Add(l, r)
            }
            SeqInline::Comb(CombInline::Sub(l, r)) => {
                if self.check_add(&l, &r, span).is_none() {
                    return;
                }
                AssignExpr::Sub(l, r)
            }
            SeqInline::Comb(CombInline::And(l, r)) => {
                if self.check_add(&l, &r, span).is_none() {
                    return;
                }
                AssignExpr::And(l, r)
            }
            SeqInline::Comb(CombInline::Or(l, r)) => {
                if self.check_add(&l, &r, span).is_none() {
                    return;
                }
                AssignExpr::Or(l, r)
            }
            SeqInline::Comb(CombInline::Xor(l, r)) => {
                if self.check_add(&l, &r, span).is_none() {
                    return;
                }
                AssignExpr::Xor(l, r)
            }
            SeqInline::Comb(CombInline::Eq(l, r)) => {
                if self.check_add(&l, &r, span).is_none() {
                    return;
                }
                AssignExpr::Eq(l, r)
            }
            SeqInline::Comb(CombInline::Mux { sel, t, f }) => {
                if self.check_connect(&dst, &t, span).is_none()
                    || self.check_connect(&dst, &f, span).is_none()
                {
                    return;
                }
                AssignExpr::Mux { sel, t, f }
            }
        };
        self.push_reg_d_assign(dst, expr, span);
    }

    /// Push a sequential `Reg.d` assign (shared by `assign_reg_d_*` and seq inline).
    fn push_reg_d_assign(&mut self, name: String, expr: AssignExpr, span: Span) {
        let kind = self.signals.get(&name).copied();
        let process_kind = match &self.process {
            Some(ProcessState::Combinational { .. }) => Some(ProcessKind::Combinational),
            Some(ProcessState::Sequential { .. }) => Some(ProcessKind::Sequential),
            None => None,
        };

        match process_kind {
            Some(ProcessKind::Sequential) => match kind {
                Some(SignalKind::Reg) => {
                    if let Some(ProcessState::Sequential { assigns, .. }) = self.process.as_mut() {
                        assigns.push(Assign {
                            target: AssignTarget::RegD(name),
                            expr,
                            span,
                        });
                    }
                }
                Some(_) => {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0115".into(),
                        en: format!("'{name}' is not a Reg; Reg.d requires a register"),
                        zh: format!("'{name}' 不是寄存器，不能写 Reg.d"),
                    });
                }
                None => {
                    self.push_err(Diagnostic {
                        span,
                        code: "rhdl::E0113".into(),
                        en: format!("unknown signal '{name}'"),
                        zh: format!("未知信号 '{name}'"),
                    });
                }
            },
            Some(ProcessKind::Combinational) => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0116".into(),
                    en: format!("combinational process must not write Reg.d for '{name}'"),
                    zh: format!("组合过程不能写 '{name}' 的 Reg.d"),
                });
            }
            None => {
                self.push_err(Diagnostic {
                    span,
                    code: "rhdl::E0103".into(),
                    en: "assignment outside a marked combinational/sequential process".into(),
                    zh: "在未标注的 comb/seq 过程外赋值".into(),
                });
            }
        }
    }

    /// Hierarchical instance (Story 2.2); not flattened at elaborate.
    pub fn add_instance(
        &mut self,
        name: impl Into<String>,
        module: impl Into<String>,
        connects: Vec<(String, String)>,
        params: Vec<(String, u32)>,
        span: Span,
    ) {
        use bitloom_hir::{Instance, PortConnect};
        let connects = connects
            .into_iter()
            .map(|(child_port, parent_net)| PortConnect {
                child_port,
                parent_net,
                span,
                dangling: false,
            })
            .collect();
        if let Some(m) = self.current.as_mut() {
            m.body.push(Stmt::Instance(Instance {
                name: name.into(),
                module: module.into(),
                connects,
                params,
                span,
            }));
        }
    }

    /// Elaborate-time module factory (FR73 / Cap-R-53).
    ///
    /// Invokes `f(i, session)` for each `i` in `0..count`. The factory should
    /// call [`Self::add_instance`] (and optional wire decls) so each iteration
    /// records a child instance plus type-safe port connects. Only ordinary
    /// `Stmt::Instance` / `PortConnect` remain after the loop — the closure
    /// does **not** enter FrozenHir (NFR36). Width/dir checks still run at
    /// `finish` (FR8 / existing instance validation). Capturing Wire/Reg into
    /// the factory is illegal — use [`Self::assert_no_hw_capture`] (`rhdl::E0142`).
    ///
    /// ```ignore
    /// session.generate_instances(4, |i, s| {
    ///     s.add_instance(
    ///         format!("u{i}"),
    ///         "Lane",
    ///         vec![
    ///             ("clk".into(), "clk".into()),
    ///             ("rst".into(), "rst".into()),
    ///             ("x".into(), format!("x{i}")),
    ///             ("y".into(), format!("y{i}")),
    ///         ],
    ///         vec![],
    ///         Span::default(),
    ///     );
    /// });
    /// ```
    pub fn generate_instances<F>(&mut self, count: usize, mut f: F)
    where
        F: FnMut(usize, &mut Self),
    {
        for i in 0..count {
            f(i, self);
        }
    }

    /// Pure-return factory variant (FR73 / Cap-R-53): `f(i)` returns an
    /// [`GeneratedInstance`] that is immediately recorded as HIR via
    /// [`Self::add_instance`]. Prefer when the factory needs no extra session
    /// side effects (wires, dangling marks).
    pub fn generate_instances_from<F>(&mut self, count: usize, f: F, span: Span)
    where
        F: Fn(usize) -> GeneratedInstance,
    {
        for i in 0..count {
            let g = f(i);
            self.add_instance(g.name, g.module, g.connects, g.params, span);
        }
    }

    pub fn add_dangling_input(
        &mut self,
        instance: &str,
        child_port: impl Into<String>,
        span: Span,
    ) {
        use bitloom_hir::PortConnect;
        if let Some(m) = self.current.as_mut() {
            for stmt in &mut m.body {
                if let Stmt::Instance(inst) = stmt {
                    if inst.name == instance {
                        inst.connects.push(PortConnect {
                            child_port: child_port.into(),
                            parent_net: String::new(),
                            span,
                            dangling: true,
                        });
                        return;
                    }
                }
            }
        }
        self.push_err(Diagnostic {
            span,
            code: "rhdl::E0201".into(),
            en: format!("unknown instance '{instance}' for dangling mark"),
            zh: format!("悬空标记找不到实例 '{instance}'"),
        });
    }
}

/// Trait implemented by `#[rhdl::top]` / design modules (AD-19 partial for 1.1).
pub trait Elaboratable {
    fn elaborate() -> Result<FrozenHir, Diagnostics>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_ports(s: &mut ElaborateSession) {
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    }

    #[test]
    fn complete_comb_assign_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn incomplete_branch_is_latch_error() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.begin_then(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.begin_else(Span::default());
        // else does not assign data_out
        s.end_if(Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0110"),
            "expected latch diagnostic, got {err}"
        );
    }

    #[test]
    fn both_branches_assign_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.begin_then(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.begin_else(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_if(Span::default());
        s.end_process();
        s.end_module();
        let r = s.finish();
        assert!(r.is_ok(), "{:?}", r.err());
    }

    #[test]
    fn comb_cannot_write_reg_d() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0116"));
    }

    #[test]
    fn seq_cannot_drive_comb_net() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_sequential(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0114"));
    }

    #[test]
    fn assign_outside_process_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.assign_net("data_out", "data_in", Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0103"));
    }

    #[test]
    fn seq_reg_d_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn missing_clock_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0120"));
    }

    #[test]
    fn missing_reset_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0121"));
    }

    #[test]
    fn mismatched_add_width_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_wire("a", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("b", GroundType::UInt { width: 16 }, Span::default());
        assert!(s.check_add("a", "b", Span::default()).is_none());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0130"));
    }

    #[test]
    fn mismatched_assign_net_width_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.add_output("narrow", GroundType::UInt { width: 4 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("narrow", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0131"),
            "expected E0131, got {err}"
        );
    }

    #[test]
    fn mismatched_assign_reg_d_width_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("q_narrow", GroundType::UInt { width: 4 }, Span::default());
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q_narrow", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0131"),
            "expected E0131 on Reg.d path, got {err}"
        );
    }

    #[test]
    fn pad_then_add_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_wire("a", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("b", GroundType::UInt { width: 16 }, Span::default());
        assert!(s.pad_to("a", 16, "a_pad", Span::default()));
        assert_eq!(s.check_add("a_pad", "b", Span::default()), Some(16));
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn multi_drive_rejected() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0140"));
    }

    #[test]
    fn parameterized_widths_w8_and_w16() {
        fn elaborate_w(w: u32) -> bitloom_hir::FrozenHir {
            let mut s = ElaborateSession::new("t");
            s.begin_module(format!("Add{w}"), Span::default());
            s.add_input("clk", GroundType::Clock, Span::default());
            s.add_input("rst", GroundType::Reset, Span::default());
            s.add_input("a", GroundType::UInt { width: w }, Span::default());
            s.add_input("b", GroundType::UInt { width: w }, Span::default());
            s.add_output("y", GroundType::UInt { width: w }, Span::default());
            s.begin_combinational(Span::default());
            s.assign_net("y", "a", Span::default());
            s.end_process();
            s.end_module();
            s.finish().unwrap()
        }
        let h8 = elaborate_w(8);
        let h16 = elaborate_w(16);
        assert!(matches!(
            h8.circuit().modules[0].ports[2].ty,
            GroundType::UInt { width: 8 }
        ));
        assert!(matches!(
            h16.circuit().modules[0].ports[2].ty,
            GroundType::UInt { width: 16 }
        ));
    }

    #[test]
    fn hierarchy_instance_preserved() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.end_module();

        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "x".into()),
                ("y".into(), "y".into()),
            ],
            vec![("W".into(), 8)],
            Span::default(),
        );
        s.end_module();
        let frozen = s.finish().unwrap();
        assert_eq!(frozen.circuit().modules.len(), 2);
        assert!(frozen.circuit().modules[1].body.iter().any(|st| matches!(
            st,
            bitloom_hir::Stmt::Instance(i) if i.name == "u0" && i.module == "Child"
        )));
    }

    #[test]
    fn generate_instances_factory_batches_children() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Lane", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "x", Span::default());
        s.end_process();
        s.end_module();

        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        for i in 0..3 {
            s.add_input(
                format!("x{i}"),
                GroundType::UInt { width: 8 },
                Span::default(),
            );
            s.add_output(
                format!("y{i}"),
                GroundType::UInt { width: 8 },
                Span::default(),
            );
        }
        s.generate_instances(3, |i, sess| {
            sess.add_instance(
                format!("u{i}"),
                "Lane",
                vec![
                    ("clk".into(), "clk".into()),
                    ("rst".into(), "rst".into()),
                    ("x".into(), format!("x{i}")),
                    ("y".into(), format!("y{i}")),
                ],
                vec![],
                Span::default(),
            );
        });
        s.end_module();
        let frozen = s.finish().unwrap();
        let parent = frozen
            .circuit()
            .modules
            .iter()
            .find(|m| m.name == "Parent")
            .unwrap();
        let instances: Vec<_> = parent
            .body
            .iter()
            .filter_map(|st| match st {
                bitloom_hir::Stmt::Instance(i) => Some(i.name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(instances, ["u0", "u1", "u2"]);
    }

    #[test]
    fn generate_instances_from_returns_plain_specs() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Lane", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x0", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y0", GroundType::UInt { width: 8 }, Span::default());
        s.generate_instances_from(
            1,
            |_| {
                GeneratedInstance::new(
                    "u0",
                    "Lane",
                    vec![
                        ("clk".into(), "clk".into()),
                        ("rst".into(), "rst".into()),
                        ("x".into(), "x0".into()),
                        ("y".into(), "y0".into()),
                    ],
                    vec![],
                )
            },
            Span::default(),
        );
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn undriven_child_input_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0202"));
    }

    #[test]
    fn sync_read_mem_declares_and_emits() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("MemTop", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_sync_read_mem("ram", 16, 8, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "ram", Span::default());
        s.end_process();
        s.end_module();
        let frozen = s.finish().unwrap();
        assert!(frozen.circuit().modules[0].body.iter().any(|st| matches!(
            st,
            bitloom_hir::Stmt::MemDecl {
                sync_read: true,
                ..
            }
        )));
    }

    #[test]
    fn mem_with_init_fn_stores_plain_words() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Lut", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_mem_with_init_fn("rom", 4, 8, |i| ((i * i) & 0xff) as u64, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "rom", Span::default());
        s.end_process();
        s.end_module();
        let frozen = s.finish().unwrap();
        let init = frozen.circuit().modules[0]
            .body
            .iter()
            .find_map(|st| match st {
                bitloom_hir::Stmt::MemDecl {
                    name,
                    init: Some(words),
                    ..
                } if name == "rom" => Some(words.clone()),
                _ => None,
            })
            .expect("rom init present");
        assert_eq!(init, vec![0, 1, 4, 9]);
    }

    #[test]
    fn mem_init_len_mismatch_fails() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Bad", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.declare_mem_with_init("rom", 4, 8, vec![1, 2], Span::default());
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0212"));
    }

    #[test]
    fn async_reset_and_enable_flags() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("M", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg_ex(
            "q",
            GroundType::UInt { width: 8 },
            true,
            true,
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("y", "q", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("q", Span::default());
        s.end_process();
        s.end_module();
        let frozen = s.finish().unwrap();
        assert!(frozen.circuit().modules[0].body.iter().any(|st| matches!(
            st,
            bitloom_hir::Stmt::RegDecl {
                async_reset: true,
                has_enable: true,
                ..
            }
        )));
    }

    #[test]
    fn illegal_domain_crossing_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Cdc", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.bind_domain("a", 0);
        s.bind_domain("y", 1);
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0220"), "{err}");
    }

    #[test]
    fn cdc_bridge_allows_crossing() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("CdcOk", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.bind_domain("a", 0);
        s.bind_domain("y", 1);
        s.mark_cdc_bridge("y");
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn double_flop_stages_allow_reg_d_crossing() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Df", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("din", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("dout", GroundType::UInt { width: 1 }, Span::default());
        s.bind_domain("din", 0);
        s.bind_domain("dout", 1);
        let (ff0, ff1) =
            s.declare_double_flop_stages("sync", GroundType::UInt { width: 1 }, 1, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("dout", &ff1, Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.connect_double_flop(&ff0, &ff1, "din", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn assign_reg_d_cross_domain_without_bridge_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Bad", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("din", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("q", GroundType::UInt { width: 1 }, Span::default());
        s.bind_domain("din", 0);
        s.bind_domain("q", 1);
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q", "din", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0220"), "{err}");
    }

    #[test]
    fn unknown_parent_net_rejected() {
        let mut s = ElaborateSession::new("t");
        s.begin_module("Child", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.end_module();
        s.begin_module("Parent", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.add_instance(
            "u0",
            "Child",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), "no_such_net".into()),
                ("y".into(), "y".into()),
            ],
            vec![],
            Span::default(),
        );
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0204"), "{err}");
    }

    #[test]
    fn hw_capture_wire_rejected_e0142() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
        // Documented illegal capture of Wire into generator context.
        s.assert_no_hw_capture(&[HwCaptureRef::wire("w")], Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0142"),
            "expected E0142, got {err}"
        );
        assert!(
            err.0
                .iter()
                .any(|d| d.en.contains("Wire") && d.en.contains("w")),
            "diagnostic should name Wire 'w': {err}"
        );
    }

    #[test]
    fn hw_capture_reg_rejected_e0142() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("r", GroundType::UInt { width: 8 }, Span::default());
        s.reject_hw_capture(&HwCaptureRef::reg("r"), Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0
                .iter()
                .any(|d| d.code == "rhdl::E0142" && d.en.contains("Reg")),
            "expected E0142 Reg, got {err}"
        );
    }

    #[test]
    fn assert_no_hw_capture_empty_ok() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.assert_no_hw_capture(&[], Span::default());
        s.declare_mem_with_init_fn("rom", 2, 8, |i| i as u64, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok());
    }

    #[test]
    fn fr16_capturing_closure_still_e0141() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.reject_unsynthesizable("capturing closure", Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0141"),
            "FR16 capturing closure must stay E0141, got {err}"
        );
    }

    #[test]
    fn synthesizable_closure_heap_e0143() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.reject_unsynthesizable_closure(
            &SynthesizableClosureViolation::heap("Box<u8> in body"),
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0143"),
            "expected E0143, got {err}"
        );
    }

    #[test]
    fn synthesizable_closure_capture_state_e0144() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.check_synthesizable_closure(
            &[SynthesizableClosureViolation::runtime_capture_state(
                "captures local threshold",
            )],
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0144"),
            "expected E0144, got {err}"
        );
    }

    #[test]
    fn synthesizable_closure_impure_e0145() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.reject_unsynthesizable_closure(
            &SynthesizableClosureViolation::impure("file I/O"),
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0145"),
            "expected E0145, got {err}"
        );
    }

    #[test]
    fn legal_empty_and_simple_synthesizable_closure_pass() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.check_synthesizable_closure(&[], Span::default());
        s.check_synthesizable_closure_marker(&LegalEmptyClosure, Span::default());
        s.check_synthesizable_closure_marker(&LegalSimpleClosure, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "data_in", Span::default());
        s.end_process();
        s.end_module();
        assert!(s.finish().is_ok(), "legal empty/simple must pass");
    }

    #[test]
    fn diagnose_free_fn_cap_r60() {
        let diags = diagnose_synthesizable_closure_violations(
            &[SynthesizableClosureViolation::heap("String")],
            Span::default(),
        );
        assert!(diags.0.iter().any(|d| d.code == "rhdl::E0143"));
    }

    #[test]
    fn inline_comb_fn_expands_to_ordinary_assign() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("sum", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.inline_comb_fn("sum", &["data_in", "b"], &[], Span::default(), |args| {
            CombInline::Add(args[0].into(), args[1].into())
        });
        s.inline_comb_fn_marker(
            "data_out",
            &["sum"],
            &LegalSimpleClosure,
            Span::default(),
            |args| CombInline::Ref(args[0].into()),
        );
        s.end_process();
        s.end_module();
        let hir = s.finish().expect("legal inline must finish");
        let body = &hir.circuit().modules[0].body;
        let procs: Vec<_> = body
            .iter()
            .filter_map(|st| match st {
                Stmt::Process(p) => Some(p),
                _ => None,
            })
            .collect();
        assert_eq!(procs.len(), 1);
        assert_eq!(procs[0].assigns.len(), 2);
        assert!(matches!(
            &procs[0].assigns[0].expr,
            AssignExpr::Add(l, r) if l == "data_in" && r == "b"
        ));
        assert!(matches!(
            &procs[0].assigns[1].expr,
            AssignExpr::Ref(n) if n == "sum"
        ));
        // NFR36: no Fn / closure types in Debug of frozen module body.
        let dump = format!("{body:?}");
        assert!(!dump.contains("CombInline"));
        assert!(!dump.to_lowercase().contains("closure"));
    }

    #[test]
    fn inline_comb_fn_violation_skips_expand() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.inline_comb_fn(
            "data_out",
            &["data_in"],
            &[SynthesizableClosureViolation::heap("Box in transform")],
            Span::default(),
            |_args| CombInline::Ref("data_in".into()),
        );
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("heap must fail");
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0143"));
    }

    #[test]
    fn inline_comb_fn_incomplete_branch_still_latch() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.begin_combinational(Span::default());
        s.begin_then(Span::default());
        s.inline_comb_fn("data_out", &["data_in"], &[], Span::default(), |args| {
            CombInline::Ref(args[0].into())
        });
        s.begin_else(Span::default());
        // else does not assign data_out — AD-18 latch
        s.end_if(Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().unwrap_err();
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0110"),
            "expected latch diagnostic after inline, got {err}"
        );
    }

    #[test]
    fn inline_seq_fn_expands_to_ordinary_reg_d() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_sequential(Span::default());
        s.inline_seq_fn("count", &[], &[], &[], Span::default(), |_args| {
            SeqInline::Inc
        });
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.end_module();
        let hir = s.finish().expect("legal seq inline must finish");
        let body = &hir.circuit().modules[0].body;
        let seq = body.iter().find_map(|st| match st {
            Stmt::Process(p) if matches!(p.kind, ProcessKind::Sequential) => Some(p),
            _ => None,
        });
        let seq = seq.expect("sequential process");
        assert_eq!(seq.assigns.len(), 1);
        assert!(matches!(
            &seq.assigns[0],
            Assign {
                target: AssignTarget::RegD(n),
                expr: AssignExpr::Inc(i),
                ..
            } if n == "count" && i == "count"
        ));
        let dump = format!("{body:?}");
        assert!(!dump.contains("SeqInline"));
        assert!(!dump.to_lowercase().contains("closure"));
    }

    #[test]
    fn inline_seq_fn_cap_r70_blocks_second_reg_d() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.inline_seq_fn("count", &["data_in"], &[], &[], Span::default(), |args| {
            CombInline::Ref(args[0].into()).into()
        });
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("second Reg.d must fail Cap-R-70");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0146"),
            "expected E0146, got {err}"
        );
    }

    #[test]
    fn inline_seq_fn_ownership_token_skips_expand() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_sequential(Span::default());
        s.inline_seq_fn(
            "count",
            &[],
            &[],
            &[SeqOwnershipViolation::illegal_mutable_borrow(
                "&mut count captured",
            )],
            Span::default(),
            |_args| SeqInline::Inc,
        );
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("ownership token must fail");
        assert!(err.0.iter().any(|d| d.code == "rhdl::E0146"));
    }

    #[test]
    fn inline_seq_fn_multi_drive_still_e0140() {
        let mut s = ElaborateSession::new("t");
        base_ports(&mut s);
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_sequential(Span::default());
        s.inline_seq_fn("count", &[], &[], &[], Span::default(), |_args| {
            SeqInline::Inc
        });
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("count", "data_in", Span::default());
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("cross-process multi-drive");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0140"),
            "expected E0140, got {err}"
        );
    }

    #[test]
    fn diagnose_seq_ownership_free_fn() {
        let diags = diagnose_seq_ownership_violations(
            &[SeqOwnershipViolation::illegal_mutable_borrow("x")],
            Span::default(),
        );
        assert!(diags.0.iter().any(|d| d.code == "rhdl::E0146"));
    }
}

fn valid_module_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == b'_')
        || !bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
    {
        return false;
    }
    !HDL_RESERVED
        .split_ascii_whitespace()
        .any(|word| word == name)
}

const HDL_RESERVED: &str = "accept_on alias always always_comb always_ff always_latch and assert assign assume automatic before begin bind bins binsof bit break buf bufif0 bufif1 byte case casex casez cell chandle checker class clocking cmos config const constraint context continue cover covergroup coverpoint cross deassign default defparam design disable dist do edge else end endcase endchecker endclass endclocking endconfig endfunction endgenerate endgroup endinterface endmodule endpackage endprimitive endprogram endproperty endspecify endsequence endtable endtask enum event eventually expect export extends extern final first_match for force foreach forever fork forkjoin function generate genvar global highz0 highz1 if iff ifnone ignore_bins illegal_bins implements implies import incdir include initial inout input inside int integer interconnect interface intersect join join_any join_none large let liblist library local localparam logic longint macromodule matches medium modport module nand negedge nettype new nexttime nmos nor noshowcancelled not notif0 notif1 null or output package packed parameter pmos posedge primitive priority program property protected pull0 pull1 pulldown pullup pulsestyle_ondetect pulsestyle_onevent pure rand randc randcase randsequence rcmos real realtime ref reg reject_on release repeat restrict return rnmos rpmos rtran rtranif0 rtranif1 s_always s_eventually s_nexttime s_until s_until_with scalared sequence shortint shortreal showcancelled signed small soft solve specify specparam static string strong strong0 strong1 struct super supply0 supply1 sync_accept_on sync_reject_on table tagged task this throughout time timeprecision timeunit tran tranif0 tranif1 tri tri0 tri1 triand trior trireg type typedef union unique unique0 unsigned until until_with untyped use uwire var vectored virtual void wait wait_order wand weak weak0 weak1 while wildcard wire with within wor xnor xor";
