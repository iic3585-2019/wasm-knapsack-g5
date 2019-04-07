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
#[derive(Clone)]
pub struct Item {
    id: u32,
    name: char,
    value: u32,
    weight: u32,
}

#[wasm_bindgen]
impl Item {
    pub fn new(new_id: u32, new_name: char, new_value: u32, new_weight: u32) -> Item {
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
    maximun_weight: u32,
    items: Vec<Item>,
    selected_items: Vec<Item>,
}

#[wasm_bindgen]
impl Knapsack {
    pub fn new(weight: u32) -> Knapsack {
        Knapsack {
            maximun_weight: weight,
            items: Vec::new(),
            selected_items: Vec::new(),
        }
    }

    pub fn append_item(&mut self, new_item: Item) {
        self.items.push(new_item);
    }

    pub fn solve_it(&mut self) {
        // Knapsack::show_items(&self.items);

        let n = self.items.len();
        let mut max = 0;
        let mut best_subset = Vec::new();
        let mut eval;

        for i in 0..n {
            let mut sub_set = Vec::new();
            sub_set.push(self.items[i].clone());

            eval = Knapsack::evaluate(self, &sub_set);
            // log!("[Afuera] El eval de {}, dio {}", self.items[i].id, eval);
            if eval > max {
                max = eval;
                best_subset = sub_set.clone();
            }

            for j in (i + 1)..n {
                sub_set.push(self.items[j].clone());
                eval = Knapsack::evaluate(self, &sub_set);
                // log!("[Dentro] El eval de {}, dio {}", self.items[j].id, eval);
                if eval > max {
                    max = eval;
                    best_subset = sub_set.clone();
                }
            }
        }

        log!("Se van a mostrar los id de los mejores items");
        for i in 0..best_subset.len() {
            log!("ID: {}", best_subset[i].id)
        }
    }
}

impl Knapsack {
    pub fn evaluate(&self, arr: &[Item]) -> u32 {
        let mut current_weight = 0;
        let mut current_value = 0;
        for i in 0..arr.len() {
            current_weight += arr[i].weight;

            if current_weight <= self.maximun_weight {
                current_value += arr[i].value;
            } else {
                return 0;
            }
        }
        return current_value;
    }

    pub fn show_items(arr: &[Item]) {
        for i in 0..arr.len() {
            log!(
                "ID: {}, valor: {}, peso {}",
                arr[i].id,
                arr[i].value,
                arr[i].weight
            )
        }
    }
}
