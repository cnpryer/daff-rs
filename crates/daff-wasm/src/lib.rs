pub use daff::{Changes as ChangesInner, Csv as CsvInner};

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
    pub fn compare(&self, other: &Csv) -> Changes {
        Changes {
            inner: self.inner.compare(&other.inner),
        }
    }
}

#[wasm_bindgen]
pub struct Changes {
    inner: ChangesInner,
}

#[wasm_bindgen]
impl Changes {
    #[wasm_bindgen]
    pub fn to_string(self) -> String {
        self.inner.to_string()
    }
}
