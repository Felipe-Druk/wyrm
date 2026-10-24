## Muy justo de tiempo

Obviamente, es culpa del dev, pero calcule mal los tiempos y la entrega intermedia quedo muy corta en cuanto a documentación y testing, se lograron tener test de comportamiento y efectivamente **Wyrm** funciona.

### Peores cosas

Es preferible tener claro que no quedo tan prolijo para que el que lo revise no se assume mucho.

El archivo parser.c quedo increíblemente grande, al querer copiar del repo original no se optó por modularizar como en otros módulos y el resultado es un archivo bastante feo... Es más era el primero en dar problemas cada vez que se agregaba funcionalidad.

Quedaron demasiado TODO regados por ahí, mucho son errores sin disparar o cosas definidas para futuro. El problema fue apurar cuando la entrega estaba a las puertas (de hecho ahora se escribe esto rápisdo).

Intentar tener tantos tipos nativos complico de sobre manera algunos bloques de código, aunque se intentó mitigar con macros y buenas prácticas quedaron regados algunos flujos sin tanta comprobación de por medio.

### Mejores

La verdad **Wyrm** parece un lenguaje (muy primitivo) pero lo parece, ver que se podria escribir un mini programa y que funcione es una algria tremenda y sudar por un trabajo es una experiencia estimulante de vez en cuando.

En general el scanner y el intérprete quedaron geniales, gracias a modularizar son bastante abiertos a agregar funcionalidad y no suelen dar problemas, lastima no haber seguido la misma logica para el parser

La documentación sobre tipos de datos y símbolos quedo como un mini ensayo de como definir e implementar operaciones, creo que leer sobre distintos lenguajes y buscar como definir algunas operaciones fue un gran ejercicio. Lo unico que lamento es que los primeros documentos están mejor escritos que estos ultimo.


## A futuro

Luego de entregar (que quedan menos de 60 minutos) voy a crear una rama donde lo primero será modularizar el parser y luego agregar strings y chars, luego de se vera. Espero que **Wyrm** llegue a buen puerto a futuro.