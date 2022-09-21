use wasm_bindgen::prelude::*;
use web_sys::console;
use lynxlang::env::Env;
use lynxlang::evaluator::Evaluator;
use lynxlang::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // todo
    let input = r#"push([1,2,3], { 1: true, "it worked": [1,2,3,4] });"#;
    let program = Parser::get(input).parse_program();
    let mut evaluator = Evaluator::new(Rc::new(RefCell::new(Env::new())));
    
    evaluator.builtin();

    match evaluator.eval_program(program) {
        Some(value) => {
            console::log_1(&JsValue::from_str(format!("{:?}", value).as_str()));
        },
        None => {
            console::log_1(&JsValue::from_str("Null"));
        }
    }

    Ok(())
}
