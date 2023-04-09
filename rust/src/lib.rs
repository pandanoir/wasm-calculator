mod utils;
extern crate combine;

use utils::parse_expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calc(expr: &str) -> Result<f64, String> {
  parse_expr(expr)
}
