# The Ecosystem

---
<!-- .element: data-auto-animate="true" -->

### Rust in Production

- **Mozilla** - where Rust was born. Used in Firefox's rendering engine
- **AWS** - Firecracker microVMs powering Lambda and Fargate
- **Microsoft** - Windows kernel components, Azure services
- **Google** - Chromium browser, Android OS codebase

---
<!-- .element: data-auto-animate="true" -->

### Rust in Production

- **Cloudflare** - edge proxy services
- **Discord** - switched from Go, dramatically reducing latency spikes
- **Linux Kernel** - first new language supported alongside C (since 6.1)

---

### Internationalization APIs

- Some Browser APIs are being implemented in Rust for performance and security reasons.
  - [icu4x](https://github.com/unicode-org/icu4x), provides internationalization support for Rust and WebAssembly.
    - Used by Firefox to build their internationalization features
  - [temporal_rs](https://github.com/boa-dev/temporal), the JavaScript Temporal date/time API.
    - Builds on top of icu4x
    - Used by Boa, Kiesel, V8 (Chrome, Node.js, Deno), Yavashark

---

### HTTP Frameworks

- [Actix Web](https://actix.rs/), a powerful, pragmatic, and extremely fast web framework for Rust.
- [Axum](https://github.com/tokio-rs/axum), is an HTTP routing and request-handling library that focuses on ergonomics and modularity
- [Loco](https://loco.rs/), It’s Like Ruby on Rails, but for Rust.
- [Rocket](https://rocket.rs/),  A web framework for Rust that makes it simple to write fast, type-safe, secure web applications with incredible usability, productivity and performance.

---

### Frontend Frameworks

- [Dioxus](https://dioxuslabs.com/), Dioxus is *the* Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time.
- [egui](https://www.egui.rs/), is an immediate mode GUI library written in Rust.
- [iced](https://iced.rs/), A cross-platform GUI library for Rust focused on simplicity and type-safety.
- [Leptos](https://leptos.dev/), Full stack, fully typed. A cutting-edge Rust framework for the modern web.

---

### Database Libraries

- [Diesel](https://diesel.rs/), Diesel is a safe, extensible ORM and Query Builder for Rust.
- [SeaORM](https://www.sea-ql.org/SeaORM/), is a powerful ORM for building web services in Rust
- [SQLx](https://github.com/launchbadge/sqlx), The Rust SQL Toolkit

---

### Rust Embedded

- https://github.com/rust-embedded
- The embedded Rust ecosystem is growing rapidly, with support for a wide range
of microcontrollers and platforms.
- If you own a Raspberry Pi, Arduino, STM32 or ESP32, you can easily get
started with using Rust for development.
