//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use turbomark::parse_markdown;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn parse_markdown_simple_test() {
    let markdown_input = "Hello world, this is a *very simple* example.";
    let expected_html = "<p>Hello world, this is a <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, parse_markdown(markdown_input));
}