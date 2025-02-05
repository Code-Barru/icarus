# Task Response

#### Protocol: `0x07`

#### State: `Main`

#### Bound to `Agent`

#### Data Sent

| Field Name  | Type     | Size |
| ----------- | -------- | ---- |
| task_uuid   | `uuid`   | 16   |
| status      | `u8`     | 1    |
| result_size | `u32`    | 4    |
| result      | `string` | var  |
