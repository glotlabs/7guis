use polyester::page::wasm;
use polyester::page::Page;
use polyester_macro::impl_wasm_page;
use sevenguis_lib::temperature_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TemperaturePage(temperature_page::TemperaturePage);

impl_wasm_page!(TemperaturePage);

#[wasm_bindgen(js_name = temperaturePage)]
pub fn temperature_page() -> Result<TemperaturePage, JsValue> {
    Ok(TemperaturePage(temperature_page::TemperaturePage {}))
}
