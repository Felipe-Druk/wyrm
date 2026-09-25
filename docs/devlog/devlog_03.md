# Fase de Calidad de vida


## Pre-commit

Es importante en un proyecto tener mecanismos que mejoren la calidad de vida, una de las cosas que más me gustan son los **pre-commits**. Pre-commit es uno herramienta que te permite ejecutar linters, formatters incluso tests antes de aplicar un commit. En este caso aproveche para agregar herramientas que revisan los errores de "typos" (al parecer cometo muchos) e instalé un corrector ortográfico en vscode que me ayudara a escribir mejor documentación. 


## Que paso con CSpell? 
Si se estudia el historial de commits se vera que se agrego CSpell y luego se quito, resulta que aunque funciona muy bien para encontrar palabras mal escritas, no me fue posible configurarlo en español e ingles al mismo tiempo.
Fue mejor eliminarlo y como la mayoria de errores son de "typo" (una letra por otra) usamos "typos" para encontrarlos y corregirlos, tiene algunos problemas con palabras como ser o hace que las interpreta como un error de las palbras en inglés set y have, pero se pueden cargar esos casos bordes.

No es perfecto, pero gracias typos y la extensión de vscode espero mejorar bastante la ortografía del proyecto.

## Tests

Al parecer la librería "cc" que usamos para combinar Rust y C nos permite escribir test en Rust y aprovechar el comando "cargo test" para probar funciones de C. Esto es genial porque hacer testing en C siempre fue complicado debio a la ausencia de herramientas nativas.
La idea es escribir una buena pool de test que nos ayuden a garantizar la integridad del codgio a futuro. Y pronto espero integrar valgrind para tener mejor control de los flujos de memoria (vital si trabajamos en el maravilloso C).