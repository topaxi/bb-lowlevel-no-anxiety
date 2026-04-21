use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn alert(message: &str);
}

#[wasm_bindgen]
pub struct Cat {
    name: String,
    age: u32,
}

#[wasm_bindgen]
impl Cat {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn greet(&self) {
        alert(&format!(
            "Hello, I'm {} and I'm {} years old!",
            self.name, self.age
        ));
    }
}

#[wasm_bindgen]
pub fn run() {
    let winston = Cat::new("Winston".to_string(), 10);
    winston.greet();
}
