# Tarea 3: Web Assembly, Knapsack Problem

## Integrantes
- Francisco Olivares
- Gabriel Valenzuela

### Pre-requisitos
- emsdk

Para más info de los requisitos visitar el [siguiente enlace](https://webassembly.org/getting-started/developers-guide/)

Instrucciones para correr el código

1. Ejecute el comando `emcc -o solver.js solver.c -O3 -s WASM=1 -s NO_EXIT_RUNTIME=1  -s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall"]'` (En el directorio del proyecto)
2. Ejecute el comando `emrun --no_browser --port 8080 .` (En el directorio del proyecto)
3. Dirigase a la url `http://localhost:8080/`

### Comentarios

Lo realizado en este código es el traspaso de código C a JS para su uso en una página HTML. Este crea una función accesible a partir del código JS presente en la página. Se probó otro enfoque con un código de C más complejo, el cual retornaba un arreglo de Strings. Sin embargo, la poca documentación y la dificultad del proceso dejaron esta solución de lado.
