use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str, x: i32, y: i32, start_x: i32, start_y: i32);
}

pub fn init() -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("stage").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    Ok(canvas)
}

pub fn attach_event_listener(
    canvas: &HtmlCanvasElement,
    event_type: &str,
    boxed_listener: Box<dyn FnMut(web_sys::MouseEvent)>,
) {
    let handler = Closure::wrap(boxed_listener);
    canvas
        .add_event_listener_with_callback(event_type, handler.as_ref().unchecked_ref())
        .unwrap();
    handler.forget()
}

pub fn mouse_move(canvas: &HtmlCanvasElement, x: Rc<Cell<i32>>, y: Rc<Cell<i32>>) {
    let move_handler = Box::new(move |event: web_sys::MouseEvent| {
        x.set(event.offset_x());
        y.set(event.offset_y());
    });
    attach_event_listener(&canvas, "mousemove", move_handler);
}

pub fn mouse_down(canvas: &HtmlCanvasElement, x: Rc<Cell<i32>>, y: Rc<Cell<i32>>) {
    let move_handler = Box::new(move |event: web_sys::MouseEvent| {
        x.set(event.offset_x());
        y.set(event.offset_y());
    });
    attach_event_listener(&canvas, "mousemove", move_handler);
}

pub fn mouse_up(
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
    x: Rc<Cell<i32>>,
    y: Rc<Cell<i32>>,
    start_x: Rc<Cell<i32>>,
    start_y: Rc<Cell<i32>>,
) {
    let context = context.clone();
    let canvas = canvas.clone();
    let move_handler = Box::new(move |event: web_sys::MouseEvent| {
        context.clear_rect(0 as f64, 0 as f64, 500 as f64, 500 as f64);
        context.fill_rect(
            x.get() as f64,
            y.get() as f64,
            start_x.get() as f64 - x.get() as f64,
            start_y.get() as f64 - y.get() as f64,
        );
        log(
            "move up called",
            x.get(),
            y.get(),
            start_x.get(),
            start_y.get(),
        )
    });
    attach_event_listener(&canvas, "mouseup", move_handler);
}
