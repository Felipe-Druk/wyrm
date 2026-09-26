# Apurando el paso

Tenemos un parser funcional, no cumple todas las características mínimas para la entrega inicial, pero hay un buen motivo para avanzar al intérprete ahora.
Si tenemos el flujo "E2E" completo será mucho más fácil testearlo, encontrar bugs y agregar funcionalidad, el plan es copiar los tests del repositorio plox y tenerlos funcionando.

## Que tendremos en la entrega parcial

El compromiso fue:

    - Asignación de variables
    - Ciclos For y While
    - Estructuras If else 
    - Testing automático
    - Funciones 

Y se llegará con todo, pero veremos reducidos nuestros tipos de dato. Probablemente, no tengamos soporte de tipo char y string o el tipo "vector".
Muy a mi pesar aunque tengamos la versión funcional es 100% seguro de que tengamos algunos bugs y problemas de memoria, asumo el riego, ya que prefiero entregar algo que funcioné y cumpla los requisitos mínimos para luego pulir a futuro. 

## Notas sobre el intérprete

No estaba muy seguro de que podría retonar cuando se evalúa toda una expresión, si el usuario pone una sentencia simple como "3+1" espera la respuesta por consola como "4", pero si hace algo como "int8 i = 2;" no deberíamos tener retorno, sino asignar la variable "i". 
Investigando como lidiar con esto, ya que C no permite el retorno variable, se resolverá con un "union", es una manera en la que ahorramos memoria y podemos agrupar distintos tipos de datos. El polimorfismo en C no es precisamente "lindo", pero seguramente logramos una buena velocidad de ejecución.
Cuando tengamos siclos podremos comparar con python un conteo grande.