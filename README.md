# Tarea 3: Web Assembly

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
