# Low-Level Programming without Anxiety
## using Rust

by Damian Senn

---

### Low-Level is fun and exciting!

```cpp [|7|8|7-8]
#include <iostream>
#include <vector>

int main() {
    std::vector<int> xs = {1, 2, 3};
    int& x = xs[0];
    xs.push_back(4);
    std::cout << x << "\n";
}
```

```sh
$ ./example
1450893592
```
<!-- .element: class="fragment fade-down" data-fragment-index="1" -->

---
<!-- .element: data-auto-animate="true" -->

### Low-Level can be fun without anxiety!

```rust
fn main() {
  let mut xs = vec![1, 2, 3];
  let x: &isize = &xs[0];
  xs.push(4);
  println!("{}", *x);
}
```

---
<!-- .element: data-auto-animate="true" -->

### Low-Level can be fun without anxiety!

```txt [|1-3|5-6|7-8|9-10]
error[E0502]: cannot borrow `xs` as mutable because it is also
borrowed as immutable
 --> src/main.rs:4:3
  |
3 |   let x: &isize = &xs[0];
  |                  -- immutable borrow occurs here
4 |   xs.push(4);
  |   ^^^^^^^^^^^ mutable borrow occurs here
5 |   println!("{}", *x);
  |                  -- immutable borrow later used here
```

---

# Agenda

1. Introduction
1. What is Rust?
1. The Rust Ecosystem
1. Integrating with Java or JavaScript
1. How can we benefit from this?
1. Questions

---

## Who am I?

- At TI&M for about two years
- Mostly Frontend Engineer
- Currently working for TWINT
- Pet Dad of a dog and two cats
- Please ask questions right away or at the end of the presentation :)

---

## What is Low-Level Programming?

- **High-level** languages (Java, Python, JavaScript) manage memory automatically
  - Easier to write, but less control and usually slower
- **Low-level** languages (C, C++, Rust, Assembly) give you direct control over memory and hardware
  - Faster and more efficient, but traditionally harder to write safely
- Think of it like driving: **high-level is automatic**, **low-level is manual**
