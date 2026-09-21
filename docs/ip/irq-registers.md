# Irq CSR

Local byte offsets; caller supplies the base address.

| Register | Offset | Mask | Reset | Access | Owner | Event | Read reject | Write reject |
|---|---|---|---|---|---|---|---|---|
| pending | 0x0000 | 0x0000001f | 0x00000000 | W1C | Leaf | event_bits | false | false |
| enable | 0x0004 | 0x0000001f | 0x00000000 | RW | Leaf | — | false | false |
| test | 0x0008 | 0x0000001f | 0x00000000 | WO | None | — | false | false |
| raw | 0x000c | 0x0000001f | 0x00000000 | RO | External | — | false | false |

| Register.Field | Mask | Reset | Access |
|---|---|---|---|
| pending.timer | 0x00000001 | 0x00000000 | W1C |
| pending.uart_rx | 0x00000002 | 0x00000000 | W1C |
| pending.uart_tx | 0x00000004 | 0x00000000 | W1C |
| pending.uart_error | 0x00000008 | 0x00000000 | W1C |
| pending.gpio | 0x00000010 | 0x00000000 | W1C |
| enable.timer | 0x00000001 | 0x00000000 | RW |
| enable.uart_rx | 0x00000002 | 0x00000000 | RW |
| enable.uart_tx | 0x00000004 | 0x00000000 | RW |
| enable.uart_error | 0x00000008 | 0x00000000 | RW |
| enable.gpio | 0x00000010 | 0x00000000 | RW |
| test.timer | 0x00000001 | 0x00000000 | WO |
| test.uart_rx | 0x00000002 | 0x00000000 | WO |
| test.uart_tx | 0x00000004 | 0x00000000 | WO |
| test.uart_error | 0x00000008 | 0x00000000 | WO |
| test.gpio | 0x00000010 | 0x00000000 | WO |
| raw.timer | 0x00000001 | 0x00000000 | RO |
| raw.uart_rx | 0x00000002 | 0x00000000 | RO |
| raw.uart_tx | 0x00000004 | 0x00000000 | RO |
| raw.uart_error | 0x00000008 | 0x00000000 | RO |
| raw.gpio | 0x00000010 | 0x00000000 | RO |
