use polyester::page::wasm;
use polyester::page::Page;
use polyester_macro::impl_wasm_page;
use sevenguis_lib::timer_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TimerPage(timer_page::TimerPage);

impl_wasm_page!(TimerPage);

#[wasm_bindgen(js_name = timerPage)]
pub fn timer_page(js_current_time: &JsValue) -> Result<TimerPage, JsValue> {
    let current_time = js_current_time
        .into_serde()
        .map_err(|err| format!("Failed to decode current time: {}", err))?;

    Ok(TimerPage(timer_page::TimerPage {
        initial_time: current_time,
    }))
}
