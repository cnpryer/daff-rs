pub use daff::{
    Csv as CsvInner, CsvChanges as CsvChangesInner, Text as TextInner,
    TextChanges as TextChangesInner,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Text {
    inner: TextInner,
}

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(s: String) -> Self {
        Self {
            inner: TextInner::new(s),
        }
    }

    #[wasm_bindgen]
    pub fn compare(&self, other: &Text) -> TextChanges {
        TextChanges {
            inner: self.inner.compare(&other.inner),
        }
    }
}

#[wasm_bindgen]
pub struct TextChanges {
    inner: TextChangesInner,
}

#[wasm_bindgen]
impl TextChanges {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        self.inner.diff
    }
}

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
    pub fn compare(&self, other: &Csv) -> CsvChanges {
        CsvChanges {
            inner: self.inner.compare(&other.inner),
        }
    }
}

#[wasm_bindgen]
pub struct CsvChanges {
    inner: CsvChangesInner,
}

#[wasm_bindgen]
impl CsvChanges {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(self) -> String {
        self.inner.to_string()
    }
}
