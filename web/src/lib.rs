use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .expect("failed to load");
    let body = document.body().expect("missing body");
    let header = document.get_element_by_id("name").expect("misisng elemnt");
    header.set_inner_html("changed from rust");
    let create_element = document.create_element("created").expect("creation failed");
    create_element.set_inner_html("<p>new element</p>");
    body.append_child(&create_element).expect("append failed");
    Ok(())
}
