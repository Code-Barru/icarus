# Task Response

#### Protocol: `0x07`

#### State: `Main`

#### Bound to `Agent`

#### Data Sent

| Field Name  | Type     | Size (bytes) | Description                                              |
| ----------- | -------- | ------------ | -------------------------------------------------------- |
| task_uuid   | `uuid`   | 16           | Task UUID                                                |
| status      | `u8`     | 1            | Task Status (Queued, Running, Completed, Failed)         |
| result_size | `u32`    | 4            | Size of the result                                       |
| result      | `string` | var          | The result of the task (Output of the Shellcommand, etc) |
