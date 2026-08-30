En este archivo vamos a poner los objetivos para la primera entrega del TP y ademas definiremos algunos aspectos de la sintaxis (el que y el porque de las cosas)

# Tipos de datos

**Wyrm** busca ser un lenguaje tipado, por ende es importante estar de acuero en como y que tipos se van a poder instanciar, empecemos por la basico

## Asignacion 

### Como lo hacen los referentes? 

Antes de elegir nuestra asignacion vamos a revisar como se resuelve en lenguajes poulares, hay distintas maneras pero la basico es aclarar el tipo, el nombre de la variable y luego un signo de igual con el valor (aunque esto ultimo es opcional)

podria verse asi:

TIPO NOMBRE SIGNO-IGUAL VALOR

un ejemplo simple es C

```C
int numero = 3;

// sin asginar valor
char caracter;
```

Pero el orden puede cambiar, veamos el ejemplo de Go

```go
var numero int = 2;

// O permite un asignacion especial
caracter := 'a'
```

Observamos un cambio en el orden ademas de una palabra especial para indicar que agisnamos una variable "var"

var NOMBRE TIPO SIGNO-IGUAl VALOR;

Algo muy parecido tenemos en rust
```rs
let numero = 21;

// podemos indica el tipo 
let numero_con_tipo : i32 = 10;

// y tambien hay que indicar si quermos que la variable sea mutable o no
let mut caracter_mutante = 'a'
```

tnemos 
let NOMBRE : TIPO SIGNO-IGUAL VALOR;

Y para revisar algunas más raras tenemos Python

```py
numero : int = 23
#sin evargo python nos permitiria cambiar el tipo de la variable haciendo por ejemplo
numero = "algo"
```

Ejmplo en consola 
```sh
>>> numero :int = 23
>>> print(numero)
23
>>> numero = "sad"
>>> print(numero)
sad
```

Solo por mencionar el caso de R donde existe un operacion de asginacion idicada por "<-"
```r
numero <- 23
#No indicamos el tipo de dato
```

Pasando en limpio hay muchas maneras de istnaciar una variable, y vimos que darle un valor es opcional incluso indicar el tipo es opcional. Como lo veo yo, es una buena forma de controlar y prevenir saber que tipo de dato se usa en cada caso. Obviamnete en la mayoria de los casos a uno no le importa cuantos bytes se reservaron para esa variable y como deberian interprrtarse esso bytes guardados. 

### Aproximacion de **Wyrm**

Hay veneficios en tener reservada una palabra como "let" o "var", ya que a la hora de leer un imput al leer una de esas palabras sabemos que estamos en el caso de asignacion de una nueva variable. En cambio si empezamos por el tipo de dato podria ser cualquier cosa despues. 

Aunque sea un poco más complicado de cara al parset tomaremos la ruta de no reservar una palabra como "var" o "let", la asignacion sera de la siguiente manera 

TIPO NOMBRE = VALOR;

Si el tiempo lo permite me encantari poder inicar variables con el simbolo ":=" de la siguiente manera

NOMBRE := VALOR;

Y en el furuto podemos agregar cosas como "const" o "unmut" para declarar variables que no puedan mutar.

Esta manera permite una declaracion de varibles muy simple y compacta, sin peder el control de tipos

## tipos numericos 

Hay tres tipos bsicos que (a mi entender) un lenguaje basico debier soportar 

- Enteros
- Enetros sin signo (naturales)
- Numeros en punto flotante (a groso modo un sub set de los reales)
- Booleanos (algunos diran que no es numerico, pero a nivel matematico responde de un amanera parecida)

### Eneteros

El tipo de dato generico por exelencia, en general a nivel memoria es un registro de N bytes donde el primero indica si es positivo o negativo y los demas el valor numerico en si. Por ejemplo para registros de 8 bits el numero 0x80 es el -128, el 0x03 es el 3.

Lengujaes como Rust directamente permiten usar i8, i16, i32, etc que permiten total control sobre cuantos bits usamos, una aproxiamcion que me parece muy buena y adaptaremos para *Wrym*

Al final casi todos los lenguajes tienen el int "notmal" y un int especificando cuantos bytes, en el caso de **Wyrm** vamos a ir directamente a los enteros con tamaño indicado. De modo que nuestro leguaje se veria asi

int32 numero = 21;

### Enteros sin signo

Cuando quitamos el sgino positivo o negativo en podemos aprovechar el primer bit para indicar numeros, de modo que ahora el minimo es 0x00 y el maximo es 0xFF osea 255 (para 8 bits). Aunque la mayoria llama a este tipo unsigned int o uint esta arpoximiacion no me parece matematicamente correcta. Por eso en **Wyrm** llamaremos a este tipo como nat o natural number, que representa el conjunto natural de numeros como el int representa los enteros. Investigando hay algunos lenguajes poco conocidaos que ya usaron el tipo nat (uno es motoko que se usa para web3, y otros como Agda pero son poco conocidos).
Usar la la palabra "nat" nos hace ser más criterioso mateamticamente

nat16 natural = 89;

### Numeros en punto flotante

Lamentablemente el criterio matematico no se puede mantener en los flotantes, aunque son una represetnacion de los reales tienen bastntes errores de representacion, me encantaria llamarlos real pero a diferencia de int o nat que representan todos los conjuntos dentro de un rango, como los reales entre cualquier rango tenemos infinitos elementos no es posible representar bien este conjunto. Por lo tanto nos quedaremos con float32 y float64

float32 pi = 3.1415;

### Booleanos

Aunque no es muy comentado, tecnicamente un booleano puede escapat de su represantacion 1 y 0, una algebra de bool puede tener varios elementos mientras se cumplan algunas propiedades. Sin embargo nos quedaremos con el esntadar {1,0}, {True.False}, {T,F} o {true, false}. Para ser consitentes usaremos con minusculas bool 

bool flag = true;

# Operadores

Hay varios operadores a tener en cuenta no solo los basicos {+,-,*,/} tambien tenemos operadores dobles como ++, --, ** y combinados como !=, <= y otros como ?,%,& que son menos comunes. Asi que separaremos en tres, argimeticos, logicos y especiales.

## Artmeticos

Los aritmeticos sirven para representar operaciones matematicas comunes, listemos los comunes que si o si implementaremos 

-   "+" Suma o adicion
-   "-" Resta o sustracción
-   "*" Multiplicacion (el famoso "por" solemos denotar en papel como una X o un punto)
-   "/" Division operacion contraria a la multiplicacion

Luego para denotar el exponente tenemos algunas opciones como "**" o "^" el simbolo "^" suele estar resevado para XOR, pero usar el simbol "^" para XOR nos obligaria a usar {& | ~} basicamente. Sin embargo vamos a arriesgarnos a utilizar el simbolo "^" para los exponentes, luego lidiaremos con como representar XOR para operadores logicos. 

Otro operador excelente a tener en cuenta es "%" que retorna el resto de una division, es increiblemente util para contadores circulares, y sera incluido en **Wyrm**

## Logicos

### Evaluar condicion 
Principalmente son aquellos que devuelven un booleano como respuesta, ejemplo si evaluamos 2 > 4 nos devolvera false, ya que 2 es menor que 4. De los más comunes son los que nos sirven para comparar magnitudes, aqui entran los primeros dobles el "==","<=" y">="  ya que el simbolo "=" esta reservado para ala asigncacion, seria complicado tenerlo para evaluar la igualdad.
De este modo tendriamos

-   ">" Mayor que 
-   "<" Menor que
-   ">=" Mayor igual que
-   "<=" menor igual que
-   "==" Es igual que 

Todos son basicos a la hora de comparar sobre todo los valores numericos y son un gran estandar que no vamos a modificar.

### Compuertas logicas

Ya que tenemos palabras reservadas para true y false, vamos a aplicar la misma logica para las compuertas logicas que se aplican sobre sentencias booleanas, asi que nuestro pool va a ser {and, or, not, xor} que cubriria las compuertas más importantes.
Tengo como evalucion si agregar los operadores logicos "bit a bit" (cuyo equivalente en C es &,|,~ y ^) pero al reservar ^ para el exponente podria crear un problema. En todo caso si en el futuro se agrea el soporte al tipo Byte se agrgaran de alguna manera o considerando que el XOR sea "><" o "|&"

### Especiales

Es importante detectar algunos operadores como () {} []

() sirven para englobar operaciones (2+1) / 2, tambien en la mayoria de lenguajes se usa para denotar funciones y sus argumentos, ejemplos:

C
```C

int int_to_char(int x){
    return (char)x;
}
```
Python
```py
def int_to_char(x : int):
    return chr(x)
```

Go
```go
func int_to_rune(x int) rune {
	return rune(x)
}
```

Del mismo modo las llaves {} suelen eglobar el cuerpo de las funciones, o bien son un simbolo para representar mapas. En concreto **Wyrm** tomara las llaves para cuertpos de funcion similar a como trabaja C, pero quizas podriamos aprovecharlos apra separar stacks.

Por ultimo los corchetes [] son perfectos para que sean el simbolo de indexar, donde a algun contenedor lo podemos indexar de la manera contenedor[indice]

# Conclusiones 

Ya definimos nuestros primero objetivos con los tipos de datos numericos y que simbolos deberiamos reconocer, actualmente nuestro lenguaje **Wyrm** sera más una calculadora glorificada y luego de tener esto funcinado debemos pasar con lo poderoso, las funciones, if else, for y while.

Pero si logramos reconcoer todos los simbolos en este archivo y resolver las operaciones de manera correcta, tendremos una buena base para ampliar.