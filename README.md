# Rust REST API with JWT Authentication

This project demonstrates a simple **REST API** in Rust with **JWT-based authentication**.  
Two implementations are included:

1. **JWT-only** – stateless, no server-side session store.  
2. **JWT + Redis** – JWT stored in Redis for session management (supports logout / revocation).  

The API is built with [Actix Web](https://actix.rs/), [jsonwebtoken](https://crates.io/crates/jsonwebtoken), and [Redis](https://redis.io/).

---

## Features

- 🔑 **Login** endpoint (`/login`) issues a JWT.  
- 🔒 **Protected** endpoint (`/protected-route`) requires a valid JWT.  
- 🚪 **Logout** endpoint (`/logout`) (Redis version only) revokes the token.  
- ⏱ Tokens expire after 1 hour (configurable).  
- 🐳 Redis session support via Docker.  
- 📊 Performance testing with [k6](https://k6.io/).  

---

## Requirements

- [Rust](https://www.rust-lang.org/) (edition 2024)  
- [Cargo](https://doc.rust-lang.org/cargo/)  
- [Redis](https://redis.io/) (only for JWT+Redis version)  
- [k6](https://k6.io/) (for load testing, optional)  

---

## Run (JWT-only version)

```bash
cd jwt_only_example
cargo run
```
