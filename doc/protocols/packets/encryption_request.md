# Encryption Request

#### Protocol: `0x02`

#### State: `Handshake`

#### Bound to `Server`

#### Data Sent

| Field          | Type   | Size (bytes) | Description           |
| -------------- | ------ | ------------ | --------------------- |
| RSA Key Length | `u16`  | 2            | Length of the RSA key |
| Pub RSA Key    | `byte` | Variable     | Public RSA key        |
| Verify Token   | `u64`  | 8            | A u64 of random bytes |
