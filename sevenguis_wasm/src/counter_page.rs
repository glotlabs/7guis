use polyester::page::wasm;
use polyester::page::Page;
use polyester_macro::impl_wasm_page;
use sevenguis_core::counter_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CounterPage(counter_page::CounterPage);

impl_wasm_page!(CounterPage);

#[wasm_bindgen(js_name = counterPage)]
pub fn counter_page() -> Result<CounterPage, JsValue> {
    Ok(CounterPage(counter_page::CounterPage {}))
}
