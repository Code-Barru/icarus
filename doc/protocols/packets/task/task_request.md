# Task Request

#### Protocol: `0x06`

#### State: `Main`

#### Bound to `Server`

#### Data Sent

| Field           | Type     | Size (bytes) | Description                                         |
| --------------- | -------- | ------------ | --------------------------------------------------- |
| task_uuid       | `uuid`   | 16           | Task UUID                                           |
| task_type       | `u8`     | 1            | Task Type                                           |
| parameters_size | `u32`    | 4            | Size of the parameters                              |
| parameters      | `String` | Var          | The parameters of the task (command for Shell, etc) |
