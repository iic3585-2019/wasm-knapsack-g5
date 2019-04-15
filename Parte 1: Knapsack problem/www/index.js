//import * as wasm from "wasm-game-of-life";
import * as wasm from "knapsack-problem";

const ITEMS = [
    {
        name: "Leche",
        weight: 23,
        value: 92
    },
    {
        name: "Oro",
        weight: 31,
        value: 57
    },
    {
        name: "Palta",
        weight: 29,
        value: 49
    },
    {
        name: "Tomate",
        weight: 44,
        value: 68
    },
    {
        name: "Yogurt",
        weight: 53,
        value: 60
    },
    {
        name: "Empanada",
        weight: 38,
        value: 43
    },
    {
        name: "Palta",
        weight: 63,
        value: 67
    },
    {
        name: "Palta",
        weight: 85,
        value: 84
    },
    {
        name: "Palta",
        weight: 89,
        value: 87
    },
    {
        name: "Palta",
        weight: 82,
        value: 72
    }
]

for (let i = 0; i < 10; i++) {
    ITEMS.push({ name: "Fake", weight: 1, value: 1 })
}

const MAX_VALUE = 165

let knapsack = wasm.Knapsack.new(MAX_VALUE)

ITEMS.forEach((item) => {
    knapsack.append_item(wasm.Item.new(item.name, item.value, item.weight))
})

const timer = name => {
    let start = new Date();
    return {
        stop: function () {
            let end = new Date();
            let time = end.getTime() - start.getTime();
            console.log('Timer:', name, 'finished in', time, 'ms');
        }
    }
}

let a = timer("Resolución")

knapsack.solve_it()

a.stop()
console.log(knapsack.get_selected_items())