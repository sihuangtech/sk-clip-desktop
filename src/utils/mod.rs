// src/utils/mod.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[allow(dead_code)]
pub struct Interval {
    callback: Closure<dyn FnMut()>,
    handle: i32,
}

impl Interval {
    #[allow(dead_code)]
    pub fn new<F: 'static>(millis: u32, f: F) -> Self
    where
        F: FnMut(),
    {
        let callback = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
        let handle = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                millis as i32,
            )
            .unwrap();

        Interval { callback, handle }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        web_sys::window()
            .unwrap()
            .clear_interval_with_handle(self.handle);
    }
} 