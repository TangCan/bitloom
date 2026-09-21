# UartCsr CSR

Local byte offsets; caller supplies the base address.

| Register | Offset | Mask | Reset | Access | Owner | Event | Read reject | Write reject |
|---|---|---|---|---|---|---|---|---|
| ctrl | 0x0000 | 0x00000001 | 0x00000000 | RW | Leaf | — | false | true |
| baud_div | 0x0004 | 0xffffffff | 0x00000000 | RW | Leaf | — | false | true |
| status | 0x0008 | 0x0000000f | 0x00000000 | RO | External | — | false | false |
| tx_data | 0x000c | 0x000000ff | 0x00000000 | WO | None | — | false | true |
| rx_data | 0x0010 | 0x000000ff | 0x00000000 | RO | External | — | true | false |
| EVENT | 0x0014 | 0x0000000f | 0x00000000 | W1C | Leaf | event_bits | false | false |

| Register.Field | Mask | Reset | Access |
|---|---|---|---|
| ctrl.bits | 0x00000001 | 0x00000000 | RW |
| baud_div.bits | 0xffffffff | 0x00000000 | RW |
| status.bits | 0x0000000f | 0x00000000 | RO |
| tx_data.bits | 0x000000ff | 0x00000000 | WO |
| rx_data.bits | 0x000000ff | 0x00000000 | RO |
| EVENT.bits | 0x0000000f | 0x00000000 | W1C |
