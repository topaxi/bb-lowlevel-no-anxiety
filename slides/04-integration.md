# Integration
### with other languages

---

### JavaScript / WASM

- Integrating Rust with JavaScript is very common, especially for web
development.
  - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), enables high-level interactions between Rust and JavaScript.
- Many JavaScript/TypeScript tools are being rewritten in Rust for performance
  reasons.
  - [Rolldown](https://github.com/rolldown/rolldown) a fully compatible Rollup.js rewrite
  - [swc](https://swc.rs/), a super-fast TypeScript / JavaScript compiler written in Rust.

---
<!-- .element: data-auto-animate="true" -->

### wasm-bindgen Example

```rust
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
struct Cat {
  name: String,
  age: u32,
}

impl Cat {
  pub fn new(name: String, age: u32) -> Self {
    Cat { name, age }
  }

  pub fn greet(&self) {
    alert(&format!("Hello, I'm {} and I'm {} years old!", self.name, self.age));
  }
}
```

---

### wasm-bindgen Example

```javascript
import { Cat } from "./cat";

let winston = new Cat("Winston", 10);
winston.greet();
```

<a href="#" onclick="window.runCatDemo(); return false;">run</a>
