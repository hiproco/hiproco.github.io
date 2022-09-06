use wasm_bindgen::{prelude::*, JsCast};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .expect("failed to load");
    let canvas = document
        .get_element_by_id("firstCanvas")
        .and_then(|ce| ce.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .expect("missing canvas");
    let context = canvas
        .get_context("2d")
        .expect("failed getting context")
        .and_then(|c| c.dyn_into::<web_sys::CanvasRenderingContext2d>().ok())
        .expect("missing context");
    context.stroke_rect(10.0, 10.0, 10.0, 10.0);
    // let on_key_down : Closure<_> = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |k:web_sys::KeyboardEvent| {
    //     (k.code() == "KeyQ").then(|| web_sys::window().unwrap().alert());
    // });
    // document.set_onkeydown(Some(on_key_down.as_ref().unchecked_ref()));
    Ok(())
}