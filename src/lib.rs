extern crate wasm_bindgen;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

mod setup;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str, i: i32);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let canvas = setup::init().unwrap();
    let x = Rc::new(Cell::new(0));
    let y = Rc::new(Cell::new(0));
    let start_x = Rc::new(Cell::new(0));
    let start_y = Rc::new(Cell::new(0));

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    setup::mouse_move(&canvas, x.clone(), y.clone());
    setup::mouse_down(&canvas, start_x.clone(), start_y.clone());
    setup::mouse_up(
        &canvas,
        &context,
        x.clone(),
        y.clone(),
        start_x.clone(),
        start_y.clone(),
    );
    Ok(())
}
