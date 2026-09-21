# GpioCsr CSR

Local byte offsets; caller supplies the base address.

| Register | Offset | Mask | Reset | Access | Owner | Event | Read reject | Write reject |
|---|---|---|---|---|---|---|---|---|
| dir | 0x0000 | 0xffffffff | 0x00000000 | RW | Leaf | — | false | false |
| out | 0x0004 | 0xffffffff | 0x00000000 | RW | External | — | false | false |
| in | 0x0008 | 0xffffffff | 0x00000000 | RO | External | — | false | false |
| set | 0x000c | 0xffffffff | 0x00000000 | WO | None | — | false | false |
| clear | 0x0010 | 0xffffffff | 0x00000000 | WO | None | — | false | false |
| rise_event | 0x0014 | 0xffffffff | 0x00000000 | W1C | Leaf | rise_bits | false | false |

| Register.Field | Mask | Reset | Access |
|---|---|---|---|
| dir.bits | 0xffffffff | 0x00000000 | RW |
| out.bits | 0xffffffff | 0x00000000 | RW |
| in.bits | 0xffffffff | 0x00000000 | RO |
| set.bits | 0xffffffff | 0x00000000 | WO |
| clear.bits | 0xffffffff | 0x00000000 | WO |
| rise_event.bits | 0xffffffff | 0x00000000 | W1C |
