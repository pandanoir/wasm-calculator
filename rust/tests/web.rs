//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rust::calc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(calc("3.2"), Ok(3.2));
    assert_eq!(calc("1.2+3.4*5.6"), Ok(20.24));
    assert_eq!(calc("32"), Ok(32.0));
    assert_eq!(calc("2*3"), Ok(6.0));
    assert_eq!(calc("3*(2*2)"), Ok(12.0));
    assert_eq!(calc("(64)"), Ok(64.0));
    assert_eq!(calc("123+456"),Ok(579.0));
    assert_eq!(calc("1+2*(3+4)"), Ok(15.0));
}
