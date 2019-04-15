# Tarea 3: Web Assembly, Knapsack Problem

## Integrantes
- Francisco Olivares
- Gabriel Valenzuela

### Pre-requisitos
- Node
- Rust Tool Chain
- wasm-pack

Para más info de los requisitos visitar el [siguiente enlace](https://rustwasm.github.io/docs/book/game-of-life/setup.html)

Instrucciones para correr el código

1. Ejecute el comando `wasm-pack build` (En el directorio del proyecto)
2. Dirigase a la carpeta `/pkg` que se acaba de crear y ejecute el siguiente comando `npm link`
3. Dirigase a la carpeta `/www` y ejecute el siguiente comando `npm link knapsack-problem`
4. Ejecute `npm install`
5. Ejecute `npm run start` y dirigase a la url `http://localhost:8080/`


En caso de hacer alguna modificación al código rust (`src/lib.rs`) se deberá volver a compilar el código con el comando:
```
wasm-pack build
```

### Comentarios

El problema que se busca resolver es el Knapsack problem binario. El enfoque de la solución es busqueda sobre todo el espacio de solución, es decir todas las combinaciones posibles.

El código más relevante está en las funciones/métodos
- `solve_it`: Inicia el proceso de busqueda en el espacio de soluciones
- `get_binary`: Transforma un número decimal en un vector de true o false que representa el número en binario
- `evaluate`: Dado un vector de items seleccionados (vector de true o false) calcula cuanto valor aporta la mochila
