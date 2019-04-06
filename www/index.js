import * as wasm from "wasm-game-of-life";

//wasm.greet("Usuario nuevo");

const ITEMS = [
    {
        id: 1,
        name: "Leche",
        value: 600,
        weight: 2
    },
    {
        id: 2,
        name: "Oro",
        value: 1000,
        weight: 10
    },
    {
        id: 3,
        name: "Palta",
        value: 900,
        weight: 5
    }
]

let knapsack = wasm.Knapsack.new()

ITEMS.forEach((item) => {
    knapsack.append_item(wasm.Item.new(item.id, item.name, item.value, item.weight))
})

knapsack.solve_it()