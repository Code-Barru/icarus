# Handshake

The connection sequence between Agent & Server.

**A->S - Agent to Server**

**S->A - Server to Agent**

1. **A->S** [Login Request](./packets/handshake/login_request.md)
2. **S->A** [Encryption Request](./packets/handshake/encryption_request.md)
3. **A->S** [Encryption Response](./packets/handshake/encryption_response.md)

**The connection is now encrypted using [AES-GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode)**.
