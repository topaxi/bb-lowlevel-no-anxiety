# Integration
### with other languages

---

### JavaScript / WASM

- <!-- .element class="fragment" -->Integrating Rust with JavaScript is very common, especially for web
development.
  - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), enables high-level interactions between Rust and JavaScript.
  - [NAPI-RS](https://napi.rs/), Building pre-compiled
Node.js addons in Rust
- <!-- .element class="fragment" -->Many JavaScript/TypeScript tools are being rewritten in Rust for performance
  reasons.
  - [Rolldown](https://github.com/rolldown/rolldown) a fully compatible Rollup.js rewrite
  - [swc](https://swc.rs/), a super-fast TypeScript / JavaScript compiler written in Rust.

---
<!-- .element: data-auto-animate="true" -->

### wasm-bindgen Example

```rust [|1-5|7-10|13-14|15-18|20-25]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn alert(message: &str);
}

#[wasm_bindgen]
struct Cat {
  name: String,
  age: u32,
}

#[wasm_bindgen]
impl Cat {
  #[wasm_bindgen(constructor)]
  pub fn new(name: String, age: u32) -> Self {
    Cat { name, age }
  }

  pub fn greet(&self) {
    alert(&format!(
      "Hello, I'm {} and I'm {} years old!",
      self.name, self.age
    ));
  }
}
```

---

### wasm-bindgen Example

```javascript
import { Cat } from "./cat";

// Use Cat class from Rust as if it were a regular JavaScript class.
// Using TypeScript, these will also be fully typed.
let winston = new Cat("Winston", 10);
winston.greet();
```

<a href="#" onclick="event.preventDefault(); event.stopPropagation(); window.runCatDemo(); return false;">run</a>
