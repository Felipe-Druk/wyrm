
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

### Expectativas

En el marco de de la materia "Lenguajes y Compiladores" me encantaria no solo que el legunaje cumpla los requisitos mínimos de la materia, sino que también lograr algunas metas extra, las cuales serían:

    - Soportar funciones lambda
    - Escribir un programa "hello Word!" 100% functionals
    - Tener un modulo de string que soporte codigos de color

En el futuro se pueden pactar algunas metas extra

## Sintaxis

Algunos componentes de la sintaxis de **Wyrm**  estan bien definidias mientas que otros estan todavia en "veremos", **Wyrm** quiere ser un leguaje con pocas sorpresas donde el rigor matematico y logico sea primordial, bajo **Wyrm** se toma la postuda de que darle libertad y conocimiento al desarrollador implica que los productos tengan mejor calidad a largo plazo.

### Definida

El control de tamaños de datos es primordial por eso se definico manejar con tipos como intXX donde XX representan el número de bits que se quieren usar. También como diferencial a otros lenguajes usaremos el dato "natXX" para represantar a los naturales.
Para no repetirnos hay muchas convenciones tomadas en el siguiente [archivo](docs/objetivos_01.md), donde se relatan que operadoes, tipos de datos y operadores.


### Por definir

    - Los mecanismos de control como "if", "else" o "if else", como manerjar la declarcion de funciones con sus argumentos.
    
    - Tambien queda por definicr como instanciar un dato contendor con un tipo generico (un vector por ejemplo). Se tienene algunas ideas como "vec<Dato>, vec[Dato], Dato[]", pero no se tomo una por ahora.

    - Si agregar o no un tipo de dato char o Byte

    - Si reservar una palabra para "print" tal como hace Rust, o dejarlo para un modulo a parte

## Extras

El nombre **Wyrm** fue tomado da una palabra en ingles que podia referice a un Dragón. Tambien tiene relacion con la palabra "Worm" (gusano) una forma despectiva de referirse a los dragones en la fantasia. Los dragones y los lenguajes de programacion tienen una relacion estrecha, y como fanatico de los dragones y los lenguajes de programacion me parecio muy acertado el nombre.

>> Más info de la plabra [**Wyrm**](https://en.wikipedia.org/wiki/Germanic_dragon)