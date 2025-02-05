# Task Request

#### Protocol: `0x06`

#### State: `Main`

#### Bound to `Server`

#### Data Sent

| Field           | Type     | Size |
| --------------- | -------- | ---- |
| task_uuid       | `uuid`   | 16   |
| task_type       | `u8`     | 1    |
| parameters_size | `u32`    | 4    |
| parameters      | `String` | Var  |
