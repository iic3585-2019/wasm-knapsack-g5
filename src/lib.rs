mod utils;
extern crate web_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.
/*
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}*/

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Item {
    id: u8,
    name: char,
    value: u8,
    weight: u8,
}

#[wasm_bindgen]
impl Item {
    pub fn new(new_id: u8, new_name: char, new_value: u8, new_weight: u8) -> Item {
        Item {
            id: new_id,
            name: new_name,
            value: new_value,
            weight: new_weight,
        }
    }
}

#[wasm_bindgen]
pub struct Knapsack {
    maximun_weight: u8,
    items: Vec<Item>,
    selected_items: Vec<Item>,
}

#[wasm_bindgen]
impl Knapsack {
    pub fn new(weight: u8) -> Knapsack {
        Knapsack {
            maximun_weight: weight,
            items: Vec::new(),
            selected_items: Vec::new(),
        }
    }

    pub fn append_item(&mut self, new_item: Item) {
        // log!("Nombre de lo que veo: {}", &new_item.name);
        self.items.push(new_item);
    }

    pub fn solve_it(&mut self) {
        // i es posición, x es el item
        for (i, x) in self.items.iter().enumerate() {
            log!("El id del item que veo es: {}", x.id)
        }
    }
}
