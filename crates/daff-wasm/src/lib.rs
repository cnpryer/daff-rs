pub use daff::{Changes as ChangesInner, Csv as CsvInner, Diff as DiffInner};

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
    pub fn compare(&self, other: &Csv) -> Diff {
        Diff {
            inner: self.inner.compare(&other.inner),
        }
    }
}

#[wasm_bindgen]
pub struct Diff {
    inner: DiffInner,
}

#[wasm_bindgen]
impl Diff {
    #[wasm_bindgen]
    pub fn to_string(self) -> String {
        self.inner.to_string()
    }
}
