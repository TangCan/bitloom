# Story127.2 ATDD 确切 API / 端口契约

2026-09-21；仅收敛 story 建议 API，不修改 AC/范围。以下尚待 build 实现。

```rust
// bitloom_prelude::ip；全部 derive Clone, Debug, PartialEq, Eq；两枚举另 derive Copy。
pub enum CsrAccess { Rw, Ro, Wo, W1c }
pub enum CsrOwner { Leaf, External, None }
pub struct CsrField {
    pub name: String,
    pub mask: u64, // host 配置允许表达 >32-bit 非法负例；合法范围1..=0xffff_ffff
    pub reset: u32,
    pub access: CsrAccess,
}
pub struct CsrRegister {
    pub name: String,
    pub offset: u32, // host 配置；合法0..=0xfffc，四字节对齐
    pub reset: u32,
    pub access: CsrAccess,
    pub owner: CsrOwner,
    pub event: Option<String>,
    pub read_reject: bool,
    pub write_reject: bool,
    pub fields: Vec<CsrField>,
}
pub struct CsrBlock { pub name: String, pub registers: Vec<CsrRegister> }
impl CsrBlock {
    pub fn validate(&self) -> Result<(), bitloom_prelude::Diagnostics>;
    pub fn define_module(&self, session: &mut bitloom_prelude::ElaborateSession,
                         name: impl Into<String>) -> Result<String, bitloom_prelude::Diagnostics>;
    pub fn elaborate(&self, name: impl Into<String>)
        -> Result<bitloom_prelude::FrozenHir, bitloom_prelude::Diagnostics>;
    pub fn emit_markdown(&self) -> Result<String, bitloom_prelude::Diagnostics>;
    pub fn emit_c_header(&self) -> Result<String, bitloom_prelude::Diagnostics>;
}
```

只允许 RW→Leaf/External、RO→External、WO→None、W1C→Leaf；其他组合诊断拒绝。只有 W1C 必须提供非空合法且唯一 event 名称，其他 access 必须 event=None。read_reject 仅可用于可读寄存器，write_reject 仅可用于可写寄存器；不合法配置拒绝。所有 reset 必须0；字段reset并集须等于register reset。块/寄存器/字段列表非空；合法标识符为 ASCII 字母起始，随后 ASCII 字母/数字/单下划线（禁止双下划线），拒绝 HDL/C 保留字、大小写宏碰撞和生成标识符碰撞。原始名称不得自动改名。

规范顺序按 offset 排寄存器、mask 后 name 排字段；名称/所有字段/owner/event/reject 开关纳入完整参数身份。模块名独立于 CsrBlock.name：前者 RTL module，后者软件宏命名空间且也参与参数。预校验失败不调用 session helper；helper 失败保留既有 poison/E0244 语义。内部 codec 私有，用模块内单元测试核验畸形参数，不新增公开测试接口。

固定接口：输入 `clk:Clock, rst:Reset, req_valid:1, write:1, addr:16, wdata:32, wstrb:4, rsp_ready:1`；输出 `req_ready:1, rsp_valid:1, rdata:32, error:2`。

每个寄存器名字记为 R（大小写原样），裁剪接口如下，未列的端口不存在：

| 条件 | 输入 | 输出 |
|---|---|---|
| RW External 或 RO | `R_value:32` | — |
| RW Leaf 或 W1C Leaf | — | `R_value:32` |
| 全部访问类型 | — | `R_read_commit:1`, `R_write_commit:1` |
| RW、WO、W1C | — | `R_candidate:32`, `R_write_mask:32` |
| read_reject=true | `R_read_reject:1` | — |
| write_reject=true | `R_write_reject:1` | — |
| W1C，event=E | `E:32`（直接使用事件身份，无隐含后缀） | — |

不可用权限的 commit 输出恒0。write_mask 为 WSTRB 扩展与有效字段 mask 交集，不依赖 addr/write/valid/reject/commit/rst；RW candidate=`(current & ~write_mask)|(wdata & write_mask)` 后屏蔽有效位，WO candidate=`wdata & write_mask`，W1C candidate=`wdata & write_mask`（clear 请求，未合入 event）。这些候选输出不依赖 reject 或 commit，禁止组合反馈环。RO 不提供 candidate/mask。所有 current/value 只含有效位；RO/external 值由同域 peer 复位，rst 时读提交输出被屏蔽。

read_commit/write_commit 是**上升沿之前的组合成功握手脉冲**，受 rst 屏蔽；read 要合法读，write 还要 write_mask!=0。成功零值写仍 pulse；零有效 mask 无 pulse、忽略动态 write_reject，但权限/地址错误优先。提交沿锁存响应快照，pending 时 req_ready=0（允许消费响应沿不接新请求）；rst 时 req_ready=0。所有写响应 rdata=0；失败读 rdata=0。W1C 自然 event 不依赖访问或背压；reset 优先，其次 `(old & ~successful_clear)|event`，同拍读快照为 old。

C产物：`BITLOOM_<BLOCK_UPPER>_CSR_H` include guard；`<BLOCK>_<REG>_OFFSET`、`_MASK`、`_RESET`；字段 `<BLOCK>_<REG>_<FIELD>_MASK`、`_RESET`，全部 `UINT32_C(0x........)`。访问宏 `<BLOCK>_<REG>_ACCESS` 为字符串 `"RW"/"RO"/"WO"/"W1C"`，字段 `<BLOCK>_<REG>_<FIELD>_ACCESS` 同样。必须注释 local byte offset / 调用者提供 base。这些最终生成名字必须全局检测碰撞（例寄存器 A_OFFSET 与 A 的字段 OFFSET）。Markdown 明确 local offset、寄存器/字段、mask/reset/access，确定性不含时间路径；寄存器行数值依次为 offset、mask、reset，使用0x十六进制（大小写/补零/表格布局不约束）。

测试黄金 bank：name=`Probe`；`control@0 RW Leaf mask=0x80ff00ff`；`status@4 RO External mask=0xff read_reject=true`；`tx@8 WO None mask=0xff write_reject=true`；`events@12 W1C Leaf mask=0x8000000f event=hw_events`；`counter@16 RW External mask=0xffffffff write_reject=true`。全部单字段 `bits`，reset0。测试独立手写这些地址/mask，不能从生成产物反推 oracle。
