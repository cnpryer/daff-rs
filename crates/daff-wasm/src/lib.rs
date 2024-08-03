// expose library
pub use daff::Csv as DaffCsv;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// temporary wrapper struct
pub struct Diff {}

#[wasm_bindgen]
// temporary wrapper struct
impl Diff {
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        String::from("hello world")
    }
}

#[wasm_bindgen]
// temporary wrapper struct
pub struct Csv {
    inner: DaffCsv,
}

#[wasm_bindgen]
impl Csv {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: String) -> Self {
        Self {
            inner: DaffCsv::new(buffer),
        }
    }

    #[wasm_bindgen]
    pub fn compare(&self, other: &Csv) -> Diff {
        Diff {}
    }
}
