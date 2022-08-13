use polyester::page::wasm;
use polyester::page::Page;
use sevenguis_lib::timer_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TimerPage(timer_page::TimerPage);

#[wasm_bindgen]
impl TimerPage {
    #[wasm_bindgen(constructor)]
    pub fn new(js_current_time: &JsValue) -> Result<TimerPage, JsValue> {
        let current_time = js_current_time
            .into_serde()
            .map_err(|err| format!("Failed to decode current time: {}", err))?;

        Ok(TimerPage(timer_page::TimerPage {
            initial_time: current_time,
        }))
    }

    #[wasm_bindgen(js_name = id)]
    pub fn id(&self) -> Result<String, JsValue> {
        Ok(self.0.id().to_string())
    }

    #[wasm_bindgen(js_name = init)]
    pub fn init(&self) -> Result<JsValue, JsValue> {
        wasm::init(&self.0)
    }

    #[wasm_bindgen(js_name = viewBody)]
    pub fn view_body(&self, js_model: &JsValue) -> Result<String, JsValue> {
        wasm::view_body(&self.0, js_model)
    }

    #[wasm_bindgen(js_name = getSubscriptions)]
    pub fn get_subscriptions(&self, js_model: &JsValue) -> Result<JsValue, JsValue> {
        wasm::get_subscriptions(&self.0, js_model)
    }

    #[wasm_bindgen(js_name = update)]
    pub fn update(&self, js_msg: &JsValue, js_model: &JsValue) -> Result<JsValue, JsValue> {
        wasm::update(&self.0, js_msg, js_model)
    }
}
