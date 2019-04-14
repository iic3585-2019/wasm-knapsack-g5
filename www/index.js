import * as wasm from "wasm-game-of-life";

const ITEMS = [
    {
        id: 0,
        name: "Leche",
        weight: 23,
        value: 92
    },
    {
        id: 1,
        name: "Oro",
        weight: 31,
        value: 57
    },
    {
        id: 2,
        name: "Palta",
        weight: 29,
        value: 49
    },
    {
        id: 3,
        name: "Tomate",
        weight: 44,
        value: 68
    },
    {
        id: 4,
        name: "Yogurt",
        weight: 53,
        value: 60
    },
    {
        id: 5,
        name: "Empanada",
        weight: 38,
        value: 43
    },
    {
        id: 6,
        name: "Palta",
        weight: 63,
        value: 67
    },
    {
        id: 7,
        name: "Palta",
        weight: 85,
        value: 84
    },
    {
        id: 8,
        name: "Palta",
        weight: 89,
        value: 87
    },
    {
        id: 9,
        name: "Palta",
        weight: 82,
        value: 72
    }
]

const MAX_VALUE = 165

let knapsack = wasm.Knapsack.new(MAX_VALUE)

ITEMS.forEach((item) => {
    knapsack.append_item(wasm.Item.new(item.name, item.value, item.weight))
})

knapsack.solve_it()
console.log(knapsack.get_selected_items())