extern crate web_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Item {
    name: char,
    value: u32,
    weight: u32,
}

#[wasm_bindgen]
impl Item {
    pub fn new(new_name: char, new_value: u32, new_weight: u32) -> Item {
        Item {
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
    selected_items: Vec<u32>,
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

    // Getter de los selected_items, que viene a ser el resultado final
    pub fn get_selected_items(&self) -> Vec<u32> {
        return self.selected_items.clone();
    }

    pub fn solve_it(&mut self) {
        // Knapsack::show_items(&self.items);

        let n = self.items.len();
        let mut max = 0;
        let mut best_items = Vec::new();
        let mut eval;

        for i in 0..(i32::pow(2, n as u32) + 1) {
            //log!("Voy en la iteración de {}", i);
            let current_items = get_binary(i as u32, n as u32);

            eval = Knapsack::evaluate(self, &current_items);
            if eval > max {
                max = eval;
                best_items = current_items.clone();
            }
        }

        // Se asignan los items
        for i in 0..best_items.len() {
            if best_items[i] {
                self.selected_items.push(1);
            } else {
                self.selected_items.push(0);
            }
        }
    }
}

// Dado un número decimal retorna un vector con true o false
// que lo representa en binario
pub fn get_binary(input: u32, n_bits: u32) -> Vec<bool> {
    let mut flag = true;
    let mut number = input.clone();
    let mut i = 0;
    let mut binary = Vec::new();

    while flag {
        if number / 2 == 0 {
            flag = false;
        }

        if number % 2 > 0 {
            binary.push(true)
        } else {
            binary.push(false)
        }

        number = number / 2;

        i += 1;
    }

    while (binary.len() as u32) < n_bits {
        binary.push(false);
    }

    // Importante darlo vuelta para que los bits menos significativos queden a la derecha
    binary.reverse();
    return binary;
}

impl Knapsack {
    // Evalua el arreglo de items dado un vector de true o false que dice si el item
    // es seleccionado o no
    pub fn evaluate(&self, selected_items: &[bool]) -> u32 {
        let mut current_weight = 0;
        let mut current_value = 0;
        for i in 0..selected_items.len() {
            // Se ha seleccionado el item
            if selected_items[i] {
                // Se suma el peso
                current_weight += self.items[i].weight;

                // Se ve si el peso es válido
                if current_weight <= self.maximun_weight {
                    current_value += self.items[i].value;
                // Si 'rompe' la mochila se retorna altiro
                } else {
                    return 0;
                }
            }
        }
        return current_value;
    }

    // Función que muestra un arreglo de items
    pub fn show_items(arr: &[Item]) {
        for i in 0..arr.len() {
            log!(
                "Indice: {}, Valor: {}, peso {}",
                i,
                arr[i].value,
                arr[i].weight
            );
        }
    }
}
