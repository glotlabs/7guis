use polyester::page::wasm;
use polyester::page::Page;
use sevenguis_lib::temperature_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TemperaturePage(temperature_page::TemperaturePage);

#[wasm_bindgen]
impl TemperaturePage {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<TemperaturePage, JsValue> {
        Ok(TemperaturePage(temperature_page::TemperaturePage {}))
    }

    #[wasm_bindgen(js_name = id)]
    pub fn id(&self) -> Result<String, JsValue> {
        Ok(self.0.id().to_string())
    }

    #[wasm_bindgen(js_name = init)]
    pub fn initial_model(&self) -> Result<JsValue, JsValue> {
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
