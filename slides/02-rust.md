<!-- .element: data-auto-animate="true" -->
# What is Rust?

- Rust is a systems programming language focused on safety, speed, and
concurrency.
- Rust compiles to native code and does not have a garbage collector nor
a runtime.
- <!-- .element: class="fragment wave" --><span>And</span> <span style="--animation-start: 0.1s">your</span> <span style="--animation-start: 0.2s">future</span> <span style="--animation-start: 0.3s">favorite</span> <span style="--animation-start: 0.4s">language</span>

---
<!-- .element: data-auto-animate="true" -->

# What is Rust?

- What is the promise of Rust?
- How does Rust guarantee memory safety?
- What are key features of Rust?
- Why aren't we using Rust for everything?

---

## What is the promise of Rust?

- "A language empowering everyone to build reliable and efficient software."
  — rust-lang.org
- Fast, reliable, stable
- Memory safety without garbage collection
- Thread safety

---
<!-- .element: data-auto-animate="true" -->

### Errors Rust Prevents
##### (at Compile Time)

- <!-- .element: class="fragment" --><strong>Memory Safety</strong><br>
  Use-after-free, double free, dangling pointers, null pointer dereference
- <!-- .element: class="fragment" --><strong>Aliasing & Mutability</strong><br>
  Data races, concurrent modification, mutable aliasing, iterator invalidation

---
<!-- .element: data-auto-animate="true" -->

### Errors Rust Prevents
##### (at Compile Time)

- <strong>Initialization & State Correctness</strong><br>
  Uninitialized and partially initialized variables, invalid enum states
- <!-- .element: class="fragment" --><strong>Type & API Misuse</strong><br>
  Type coercions, implicit conversions, nulls, unhandled errors

---
<!-- .element: data-auto-animate="true" -->

### Errors Rust Prevents
##### (at Compile Time)

- **Resource Management**<br>
  `Drop` trait to implement RAII guarantees cleanup of resources (files,
  network connections, etc.)
- <!-- .element: class="fragment" --><strong>Mismatched allocation/deallocation</strong><br>
  No mixing of memory allocators, no manual memory management
- <!-- .element: class="fragment" --><strong>Concurrency Correctness</strong><br>
  No non-thread-safe sharing, types must implement `Send` / `Sync` traits to cross threads.

---

## What are key features of Rust?

- Very strong type system
  - Generics, traits, lifetimes
  - Algebraic data types (enums, structs)
  - No NULL <span class="fragment wave"><span>😍</span></span>
- Ownership and borrowing
- Zero-cost abstractions
- Pattern matching
- Hygienic and procedural macros

---
<!-- .element: data-auto-animate="true" -->

### Example Rust Code

```rust [|1-4|6-8|10-16]
struct Cat {
    name: String,
    age: u32,
}

fn oldest(x: &Cat, y: &Cat) -> &Cat {
    if x.age > y.age { x } else { y }
}

fn main() {
    let a = Cat { name: String::from("Winston"), age: 10 };
    let b = Cat { name: String::from("Lou"), age: 7 };

    let o = oldest(&a, &b);
    println!("{} is older", o.name);
}
```

---
<!-- .element: data-auto-animate="true" -->

### Example Rust Code

```txt [|1-2|4-11|13-14]
error[E0106]: missing lifetime specifier
 --> src/main.rs:6:32
  |
6 | fn oldest(x: &Cat, y: &Cat) -> &Cat {
  |              ----     ----     ^ expected named lifetime
  |                                  parameter
  |
  = help: this function's return type contains a borrowed value,
but the signature does not say whether it is borrowed from `x`
or `y`
help: consider introducing a named lifetime parameter
  |
6 | fn oldest<'a>(x: &'a Cat, y: &'a Cat) -> &'a Cat {
  |          ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
```

---
<!-- .element: data-auto-animate="true" -->

### Example Rust Code

```rust [6]
struct Cat {
    name: String,
    age: u32,
}

fn oldest<'a>(x: &'a Cat, y: &'a Cat) -> &'a Cat {
    if x.age > y.age { x } else { y }
}

fn main() {
    let a = Cat { name: String::from("Winston"), age: 10 };
    let b = Cat { name: String::from("Lou"), age: 7 };

    let o = oldest(&a, &b);
    println!("{} is older", o.name);
}
```

```txt
Winston is older
```
<!-- .element: class="fragment fade-down" -->

---
<!-- .element: data-auto-animate="true" -->

### But I like dogs too! I need inheritance and polymorphism!

#### No!
<!-- .element: class="fragment fade-down" style="font-size: 4rem; color: red;" -->

---

### Enums and trait objects to the rescue!

```rust [|6-10|12-15|17|18-24|26-31|34-36|38-48|50-56]
struct Cat {
    name: String,
    age: u32,
}

struct Dog {
    name: String,
    age: u32,
    is_barky: bool,
}

enum Pet {
    Cat(Cat),
    Dog(Dog),
}

impl Pet {
  pub fn new_cat(name: String, age: u32) -> Self {
    Pet::Cat(Cat { name, age })
  }

  pub fn new_dog(name: String, age: u32, is_barky: bool) -> Self {
    Pet::Dog(Dog { name, age, is_barky })
  }

  pub fn age(&self) -> u32 {
    match self {
      Pet::Cat(cat) => cat.age,
      Pet::Dog(dog) => dog.age,
    }
  }
}

fn oldest<'a>(x: &'a Pet, y: &'a Pet) -> &'a Pet {
    if x.age() > y.age() { x } else { y }
}

// implementing the Display trait allows us to print our Pet
// using the {} format specifier as well as using `to_string()`
// to convert it to a String
impl std::fmt::Display for Pet {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Pet::Cat(cat) => write!(f, "🐶 {}", cat.name),
      Pet::Dog(dog) => write!(f, "😸 {}", dog.name),
    }
  }
}

fn main() {
    let a = Pet::new_cat(String::from("Winston"), 10);
    let b = Pet::new_dog(String::from("Upsi"), 4, false);

    println!("{} is older", oldest(&a, &b));
    // 🐶 Winston is older
}
```

---

## Why aren't we using Rust for everything?

- Steep learning curve <!-- .element: class="fragment fade-down" -->
- Less mature ecosystem compared to Java/C#/JavaScript/Python <!-- .element: class="fragment fade-down" -->
- Slower compilation times <!-- .element: class="fragment fade-down" -->
- Knowledge gap in the industry <!-- .element: class="fragment fade-down" -->
- Other high level languages are good enough for many use cases <!-- .element: class="fragment fade-down" -->
