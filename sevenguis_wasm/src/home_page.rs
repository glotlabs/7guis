use polyester::page::wasm;
use polyester::page::Page;
use polyester_macro::impl_wasm_page;
use sevenguis_core::home_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HomePage(home_page::HomePage);

impl_wasm_page!(HomePage);

#[wasm_bindgen(js_name = homePage)]
pub fn home_page() -> Result<HomePage, JsValue> {
    Ok(HomePage(home_page::HomePage {}))
}
