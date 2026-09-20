#!/usr/bin/env python3
"""Tool/API probe against an independent stub, NOT Bitloom product RTL."""
from importlib.metadata import version
from itertools import cycle
from pathlib import Path
import os
import sys
import xml.etree.ElementTree as ET

import cocotb
from cocotb.clock import Clock
from cocotb.triggers import ClockCycles, Timer, with_timeout
from cocotbext.axi import AxiLiteBus, AxiLiteMaster, AxiResp

# Deliberately small one-word, independently buffered test peer. Not a product IP.
STUB = r"""
`timescale 1ns/1ps
module bfm_probe (
 input clk, rst,
 input [7:0] s_axi_awaddr, input s_axi_awvalid, output s_axi_awready,
 input [31:0] s_axi_wdata, input [3:0] s_axi_wstrb,
 input s_axi_wvalid, output s_axi_wready,
 output [1:0] s_axi_bresp, output reg s_axi_bvalid, input s_axi_bready,
 input [7:0] s_axi_araddr, input s_axi_arvalid, output s_axi_arready,
 output reg [31:0] s_axi_rdata, output [1:0] s_axi_rresp,
 output reg s_axi_rvalid, input s_axi_rready
);
reg aw_pending, w_pending;
reg [31:0] data, wdata;
reg [3:0] wstrb;
integer i;
assign s_axi_awready = !aw_pending && !s_axi_bvalid;
assign s_axi_wready = !w_pending && !s_axi_bvalid;
assign s_axi_arready = !s_axi_rvalid;
assign s_axi_bresp = 0;
assign s_axi_rresp = 0;
always @(posedge clk) begin
 if (rst) begin
  aw_pending <= 0; w_pending <= 0; data <= 0;
  wdata <= 0; wstrb <= 0;
  s_axi_bvalid <= 0; s_axi_rvalid <= 0; s_axi_rdata <= 0;
 end else begin
  if (s_axi_awvalid && s_axi_awready) aw_pending <= 1;
  if (s_axi_wvalid && s_axi_wready) begin
   w_pending <= 1; wdata <= s_axi_wdata; wstrb <= s_axi_wstrb;
  end
  if (aw_pending && w_pending && !s_axi_bvalid) begin
   for (i=0; i<4; i=i+1) if (wstrb[i]) data[i*8+:8] <= wdata[i*8+:8];
   aw_pending <= 0; w_pending <= 0; s_axi_bvalid <= 1;
  end
  if (s_axi_bvalid && s_axi_bready) s_axi_bvalid <= 0;
  if (s_axi_arvalid && s_axi_arready) begin
   s_axi_rdata <= data; s_axi_rvalid <= 1;
  end
  if (s_axi_rvalid && s_axi_rready) s_axi_rvalid <= 0;
 end
end
endmodule
"""


@cocotb.test()
async def probe(dut):
    dut.clk.value = 0
    dut.rst.value = 0
    master = AxiLiteMaster(AxiLiteBus.from_prefix(dut, "s_axi"), dut.clk, dut.rst)
    channels = (
        master.write_if.aw_channel, master.write_if.w_channel,
        master.write_if.b_channel, master.read_if.ar_channel,
        master.read_if.r_channel,
    )
    for index, channel in enumerate(channels):
        channel.set_pause_generator(cycle([index % 2, 1, 0, 0]))
        assert callable(channel.assert_reset)
    assert callable(master.write_if.assert_reset)
    assert callable(master.read_if.assert_reset)
    await Timer(1, unit="ns")
    dut.rst.value = 1
    await Timer(1, unit="ns")
    cocotb.start_soon(Clock(dut.clk, 10, unit="ns").start())
    await ClockCycles(dut.clk, 3)
    await Timer(1, unit="ns")
    dut.rst.value = 0
    await ClockCycles(dut.clk, 3)
    result = await with_timeout(master.write(0, b"IP24"), 5, "us")
    assert result is not None and result.resp == AxiResp.OKAY
    result = await with_timeout(master.read(0, 4), 5, "us")
    assert result is not None and result.resp == AxiResp.OKAY
    assert result.data == b"IP24"

    # Stop the B sink to leave a known unfinished operation, then assert reset.
    for channel in channels:
        channel.set_pause_generator(None)
        channel.pause = False
    master.write_if.b_channel.pause = True
    pending = cocotb.start_soon(master.write(0, b"drop"))
    await ClockCycles(dut.clk, 10)
    assert not pending.done(), "write must still await a blocked response"
    await Timer(1, unit="ns")
    dut.rst.value = 1
    cancelled = await with_timeout(pending, 1, "us")
    assert cancelled is None, "reset cancellation is None, not an OKAY response"
    await ClockCycles(dut.clk, 3)
    await Timer(1, unit="ns")
    dut.rst.value = 0
    master.write_if.b_channel.pause = False
    await ClockCycles(dut.clk, 3)
    result = await with_timeout(master.read(0, 4), 5, "us")
    assert result is not None and result.data == bytes(4)
    dut._log.info("BFM_API_PROBE_PASS: five pause APIs, legal write/read, reset cancellation=None")


def main():
    from cocotb_tools.runner import get_runner

    for name, expected in (line.split("==") for line in Path(__file__).with_name(
        "phase24-axi-bfm-requirements.txt").read_text().splitlines()
        if line and not line.startswith("#")):
        actual = version(name)
        assert actual == expected, (name, actual, expected)
        print(f"{name}=={actual}", flush=True)
    print(f"Python {sys.version}", flush=True)
    root = Path(__file__).resolve().parents[1]
    build = root / "target/phase24-axi-bfm-probe"
    build.mkdir(parents=True, exist_ok=True)
    source = build / "stub.sv"
    source.write_text(STUB)
    os.environ["PYTHONPATH"] = str(Path(__file__).resolve().parent) + os.pathsep + os.environ.get("PYTHONPATH", "")
    runner = get_runner("icarus")
    runner.build(sources=[source], hdl_toplevel="bfm_probe", build_dir=build, always=True)
    result = runner.test(test_module="phase24_axi_bfm_probe", hdl_toplevel="bfm_probe",
                         build_dir=build, test_dir=build, results_xml="results.xml")
    report = ET.parse(result).getroot()
    assert len(report.findall(".//testcase")) == 1
    assert not report.findall(".//failure") and not report.findall(".//error")
    assert not report.findall(".//skipped")
    print("BFM probe passed; this is NOT Bitloom product RTL verification.")


if __name__ == "__main__":
    main()
