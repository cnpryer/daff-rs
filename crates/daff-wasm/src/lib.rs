// expose library
pub use daff::{Csv as CsvInner, Diff};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Csv {
    inner: CsvInner,
}

#[wasm_bindgen]
impl Csv {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: String) -> Self {
        Self {
            inner: CsvInner::new(buffer),
        }
    }

    #[wasm_bindgen]
    pub fn compare(&self, other: &Csv) -> String {
        self.inner.compare(&other.inner).to_string()
    }
}
