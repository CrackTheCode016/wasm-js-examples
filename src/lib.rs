use wasm_bindgen::prelude::*;

/// Extern "C" here really denotes that we wish to expose host functions
/// This is specifically when targeting WebAssembly as our build target
#[wasm_bindgen]
extern "C" {
    /// A host function - as WebAssembly is a format, it does not have the ability to 
    /// call web APIs natively (unless we supply them!)
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greeting_from_rust(name: &str) {
    // We can call the host function - whatever that may be - and write normal Rust code here as well
    alert(&format!("Salutations from Rust, {}!", name));
}