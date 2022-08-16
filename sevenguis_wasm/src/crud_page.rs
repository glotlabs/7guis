use polyester::page::wasm;
use polyester::page::Page;
use polyester_macro::impl_wasm_page;
use sevenguis_core::crud_page;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CrudPage(crud_page::CrudPage);

impl_wasm_page!(CrudPage);

#[wasm_bindgen(js_name = crudPage)]
pub fn crud_page() -> Result<CrudPage, JsValue> {
    Ok(CrudPage(crud_page::CrudPage {}))
}
