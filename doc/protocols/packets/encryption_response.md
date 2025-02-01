# Encryption Response

#### Protocol: `0x03`

#### State: `Handshake`

#### Bound to `Agent`

| Field         | Type   | Size | Description                                                                 |
| ------------- | ------ | ---- | --------------------------------------------------------------------------- |
| Shared Secret | byte[] | 256  | A secret key shared between client and server                               |
| Verify Token  | byte[] | 256  | Verify token value, encrypted with the same public key as the shared secret |
