:Directory "downloads"
:Directory "uploads"

task 'FileDownload'
pass path of file as input
agent uploads file to `POST /explorer/:agent_id/files/:task_id`
site download it to `GET /explorer/:agent_id/files/:task_id`

```json
"FileDownload": {
    "name": string,
    "task_id": UUID
}
```

task 'FileUpload'
site uploads file to `POST /explorer/:agent_id/files/:task_id`
agent downloads it by querying `GET /explorer/:agent_id/files/:task_id`

```json
"FileUpload":  {
    "name": string,
    "path_to_upload": string,
    "task_id": UUID
}
```
