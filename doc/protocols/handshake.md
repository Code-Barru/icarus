# Handshake

The connection sequence between Agent & Server.

**A->S - Agent to Server**

**S->A - Server to Agent**

1. **A->S** [Login Request](./packets/login_request.md)
2. **S->A** [Encryption Request](./packets/encryption_request.md)
3. **A->S** [Encryption Response](./packets/encryption_response.md)

**The connection is now encrypted using [AES-GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode)**.
