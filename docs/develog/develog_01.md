
# Incia el viaje

Como es el primer develog real vamos a expliar un poco como sera la tematica, la idea es expliar que espero hacer en  una branch nueva, documentar lo más dficil y dar una conclucion.

Por ahora el plan es tener construido una estrutra de tokens que surgan desde la entrada estandar. La idea de combinar Ruts y C es que la logica prfunda este contenida en el mismo C, pero la parte de leer por consola y controlar argumentos se pare en rust, esto porque en el futuro es probable que tengamos que leer archivos y controlar multiples archivos y rust tiene algunas optimizaciones para trabajar. Todavia no se como respondera la libreri "cc" voy a ponerme como objetivo secundario investigar como se compila y se maneja el llamado a funciones de C dentro de codigo rust. 

Por lo demas veremos que problemas nos trae el camino y como lo sorteamos.


## Problemas inicales

Evidenteme hace mucho no programaba en rust y tampoco a mano, asi que el proceso esta siendo un poco lento. Espero que con el tiempo vuelva a tener rapidez.
Ya tenemos un programa simple que lee por pantalla, ahora agregaremos argumentos para el dev y posiblemente mejoremos las cosntantes y mensajes.