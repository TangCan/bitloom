# Timer CSR

Local byte offsets; caller supplies the base address.

| Register | Offset | Mask | Reset | Access | Owner | Event | Read reject | Write reject |
|---|---|---|---|---|---|---|---|---|
| ctrl | 0x0000 | 0x00000003 | 0x00000000 | RW | External | — | false | false |
| count | 0x0004 | 0xffffffff | 0x00000000 | RW | External | — | false | false |
| compare | 0x0008 | 0xffffffff | 0x00000000 | RW | Leaf | — | false | false |
| EVENT | 0x000c | 0x00000001 | 0x00000000 | W1C | Leaf | match_bits | false | false |

| Register.Field | Mask | Reset | Access |
|---|---|---|---|
| ctrl.bits | 0x00000003 | 0x00000000 | RW |
| count.bits | 0xffffffff | 0x00000000 | RW |
| compare.bits | 0xffffffff | 0x00000000 | RW |
| EVENT.bits | 0x00000001 | 0x00000000 | W1C |
