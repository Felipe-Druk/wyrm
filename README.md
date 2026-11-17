
```tex
 __        __                    
 \ \      / /   _ _ __ _ __ ___  
  \ \ /\ / / | | | '__| '_ ` _ \ 
   \ V  V /| |_| | |  | | | | | |
    \_/\_/  \__, |_|  |_| |_| |_|
            |___/                
```


## ¿Que es **Wyrm**? 

**Wyrm** es un lenguaje de programación fuertemente tipado pensado para combinar las mejores características de C, Go y algunos extras tomados de distintos lenguajes.
Para la implementación usamos una fuerte base en C (por performance, principalmente) y asistiendo con Rust, en general la combinación fue una idea inicial más tomada por el desafío que por una característica técnica (es una buena oportunidad para aprender como combinar lenguajes).

## ¿Como ejecutar?

Se recomienda ejcutar "make" par ver la lista de comandos, para empezar se recomienda:

```sh
make install
```
Para ver los test:
```sh
make tests
```

Para ver pruebas:
```sh
make run
```

### Expectativas

En el marco de la materia "Lenguajes y Compiladores" me encantaría no solo que el lenguaje cumpla los requisitos mínimos de la materia, sino que también lograr algunas metas extra, las cuales serían:

    - Soportar funciones lambda (X)
    - Escribir un programa "hello Word!" 100% functionals (X)
    - Tener un modulo de string que soporte codigos de color (Incompleto)

En el futuro se pueden pactar algunas metas extra

## Sintaxis

_**Wyrm** quiere ser un lenguaje con pocas sorpresas donde el rigor matemático y lógico sea primordial, bajo **Wyrm** se toma la postura de que darle libertad y conocimiento al desarrollador implica que los productos tengan mejor calidad a largo plazo._

Toda la sintaxis está resumida en el [archivo de gramática ](docs/gramatica.ebnf).
Para ver un archivo funcional hay una recreación de un Trabajo práctico de la ya inexistente mataría "Algoritmos Uno".

Para probar:
```sh
make demo
```

También se puede revisar a fondo el [archivo recreación](docs/recreacion_TP1_ALGO1.wyrm)

### Definida

El control de tamaños de datos es primordial por eso se ha definido manejar con tipos como intXX donde XX representan el número de bits que se quieren usar. También como diferencial a otros lenguajes usaremos el dato "natXX" para represantar a los naturales.
Para no repetirnos hay muchas convenciones tomadas en el siguiente [archivo](docs/objetivos_01.md), donde se relatan que operadores, tipos de datos y operadores.

>Para la entrega parcial se apuraron algunas decisiones, por eso hay menos documentación,

Sin retorno explícito (por ahora), cuando se desarrollaron los "bloques" de ejecución, se eligió que todo tenga retorno (incluso si es vacio), de esta manera cuando uno declara un bloque, sin necesidad de poner una palabra reservada 

```text
{
    nat8 natural = 3;
    natural
}
```
Esta sentencia devuelve un "Wyrm_value" con el valor 3, el comportamiento (que se asemeja a Rust) resulto no solo conveniente, sino que además coherente con la filosofía. Hacer que todo tenga retorno incluso si es vacío nos da más abstracción y nos permite combinar sentencias diversas.

Sin embargo, no tener una palabra reservada como `return` dificulta crear flujos complejos y obliga a tener todo con alguna sentencia final.

Tipos `char` y/o `byte` no existen como tales, toda su lógica es perfectamente encapsulada con el tipo nativo `nat8`, que admite valores de caracteres ASCII. Así mismo podemos denotar strings con el nuevo tipo `vec.

```text
{
    nat8 caracter = 'a';
    vec<nat8> string = "Hola mundo!"
}
```

## Extras

El nombre **Wyrm** fue tomado da una palabra en inglés que podía referiré a un Dragón. También tiene relación con la palabra "Worm" (gusano) una forma despectiva de referirse a los dragones en la fantasía. Los dragones y los lenguajes de programación tienen una relación estrecha, y como fanático de los dragones y los lenguajes de programación me pareció muy acertado el nombre.

>> Más info de la plabra [**Wyrm**](https://en.wikipedia.org/wiki/Germanic_dragon)