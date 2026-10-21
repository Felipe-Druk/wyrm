## Memoria

Al fin logramos no tener errores de memoria, gracias a valgrind tenemos 0 memoria perdida. Aunque al no sé 100% nativo C se cuentan los "posibles" errores de Rust aunque se confía en que simplemente no se puede saber si son reales o no. El código fue facil de "arreglar", eso habla bien de las estructuras, cuando tenemos el parser refactorizado será fácil ampliar funcionalidad.

## Siguiente paso
Ahora pasaremos un rato organizado el parser para que deje de ser un archivo gigante complicado de leer, a varios archivos pequeños
Cambiar el test de funciones al nuevo formato, y escribir una de varios agumentos

## Ideas 
Como limpiar estos errores llevo menos tiempo del que esperaba, intentaré agregar algunas pequeñas cosas de calidad de vida al interpretar, como poder usar las flechas "Up" "Dow" para volver al comando anterior y si puedo "left" "rignt" para controlar mejor lo que el usuario escribe.    