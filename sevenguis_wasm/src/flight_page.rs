use polyester::page::wasm;
use polyester::page::Page;
use polyester_macro::impl_wasm_page;
use sevenguis_core::flight_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FlightPage(flight_page::FlightPage);

impl_wasm_page!(FlightPage);

#[wasm_bindgen(js_name = flightPage)]
pub fn flight_page(js_current_time: &JsValue) -> Result<FlightPage, JsValue> {
    let current_time = js_current_time
        .into_serde()
        .map_err(|err| format!("Failed to decode current time: {}", err))?;

    Ok(FlightPage(flight_page::FlightPage {
        initial_time: current_time,
    }))
}
