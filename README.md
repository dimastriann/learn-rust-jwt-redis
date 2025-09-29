# Rust REST API with JWT Authentication + Simple Point of sale

This project demonstrates a simple **REST API** in Rust with **JWT-based authentication**.  
Two implementations are included:

1. **JWT-only** – stateless, no server-side session store.  
2. **JWT + Redis** – JWT stored in Redis for session management (supports logout / revocation).  
3. **CRUD** -- master product, product category, contact and payment method
4. **Transaction** -- order with order detail and payment order

The API is built with [Actix Web](https://actix.rs/), [jsonwebtoken](https://crates.io/crates/jsonwebtoken), and [Redis](https://redis.io/).

---

## Features

- 🔑 **Login** endpoint (`/login`) issues a JWT.  
- 🔒 **Protected** endpoint (`/protected-route`) requires a valid JWT. 
- 🔑 **Login** endpoint (`/redis-login`) issues a JWT + Redis.
- 🔒 **Protected** endpoint (`/redis-protected-route`) requires a valid JWT + check to redis.
- 🚪 **Logout** endpoint (`/logout`) (Redis version only) revokes the token.  
- ⏱ Tokens expire after 1 hour (configurable).  
- 🐳 Redis session support via Docker or WSL (Windows).  
- 📊 Performance testing with [k6](https://k6.io/).  

---

## Requirements

- [Rust](https://www.rust-lang.org/) (edition 2024)  
- [Cargo](https://doc.rust-lang.org/cargo/)  
- [Redis](https://redis.io/) (only for JWT+Redis version)  
- [k6](https://k6.io/) (for load testing, optional)  
- [PostgreSQL](https://www.postgresql.org/) (for database management)
- [Diesel ORM](https://diesel.rs/) (ORM and Query Builder)

---

## Clone Project

```
git clone https://github.com/dimastriann/learn-rust-jwt-redis.git
```

## Database Config
* change user, password, host and port according to your config
```
DATABASE_URL=postgres://user:password@localhost:5432/rust_pos
```

## Run DB migrations
* Ensure postgresql and diesel ORM has been installed on you machine
```
diesel migration run
```

## Run Project

```bash
cd learn-rust-jwt-redis
cargo run
```

## Running redis on windows (Windows OS Only)

* Running on WSL (whatever linux distro)
* Using docker desktop