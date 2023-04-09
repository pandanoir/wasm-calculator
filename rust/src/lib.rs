mod utils;

use utils::parse_expr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calc(expr: &str) -> Result<f64, String> {
    parse_expr(expr)
}
