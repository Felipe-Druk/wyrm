En este archivo vamos a enumerar los objetivos para la entrega final, ademas aprovecharemos para documentar algunas decisiones que se tomaron y no se documentaron plenamente

# Que hay ahora 

Asumiendo que no hay bugs (que seguramente los hay) podemos valernos de los tests para saber que tenemos.

Los principales tests están en el archivo [wyrm.rs](../src/wyrm.rs), ya que es una prueba que simula una ejecución "E2E" 

## Tests 1 y retornos

```rs
#[test]
    fn test_wyrm_respeta_punt_y_coma() {
        let input = format!("2+3; 3+6");
        let wyrm = Wyrm::new(false);

        unsafe {
            let resultado = wyrm.call_wyrm(&input);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la suma no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 9,
                "El cálculo matemático de 6 + 3 estuvo al final"
            );
        }
    }
```
Simula que la entrada es la cadena "2+3; 3+6", la prueba se enfoca en que se respeta el orden de ejecución y el retorno final sea 9 (3+6). Hay que notrar que la última sentencia no tiene punto y coma al final. Resulta que este test abrió la puerta a pensar en los retornos.

El retorno sirve para indicar exactamente que devuelve un programa, función y/o bloque. En general todo tiene un retorno, veamos por ejemplo C, en el mismo main se muestra el retorno del ejecutable 

```C
int main(int argc, char *argv[]){
    // codigo ..
    return 0;
}
```

Esto además es importante porque si se llega al retorno 0, se asume que tuvimos una ejecución limpia del programa si se llegara a retornar algo distinto de 0 sabemos que el programa ha tenido algún error. Pero no nos importa tanto eso, sino revisar como indicamos que se retorna cada función y como indicar cuando hacer ese retorno.

Revisemos como lo hacen los lenguajes como C, Rust, Go y Python:

### C
En C ya vimos que la estructura es

TIPO_RETORNO nombre( argumentos espaciados por comas ){
    ...
    return RETORNO;
}

Donde el return es obligatorio a menos que el tipo del retorno sea "void". Pero a su vez existe "void*" que indica una dirección de memoria donde se guardan bytes crudos, lo cual es confuso, ya que hace pensar que ¿void es o no es un tipo de dato? En realidad no lo es, sino que nos indica la ausencia de valor. Por eso C permite funciones de esta manera:

```C
void fn(void){
    // codigo ..
    return;
}
```

Esta función no retorna un valor, tampoco necesita argumentos, aunque la sintaxis es demasiado parecida a un tipo de dato, pero se entiende que como se definió la declaración de funciones el void es casi obligatorio por la esturura de TIPO_RETORNO fn(ars..), puesto que no se podría diferenciar la declaración del llamado

```C
fn(...){ // solo nos quedarian las llaves para saber que esto no es un llamado sino una definición
    // codigo 
}

void fn(...){ // leemos "void nombre(" ya sabemos que es una definición, o incluso solo void 
    // codigo 
}
```

Vemos que C indica el "no retorno" o "no valor" con la palabra void, lo cual tiene sentido, ya que el vacío es simplemente vacío; sin embargo, al incluir "void*" como posible tipo de dato lleva a confusiones sombre el verdadero sentido de ese indicador.

### Rust

Rust tiene una palabra reservada para las funciones "fn" y tienen la particulariad que la ultiam liena sin la palabra "return" y sin ";" sera el valor retornado

```rs
fn dividir(a: i32, b: i32) -> i32 {
    if b == 0{
        return 0;
    }
    a / b // sin return sin ;
}
```

La estructura es:

fn nombre( argumentos espaciados por comas) -> TIPO_RETORNO {
    ...
    RETORNO
}

Tiene alguna semejanza en la declaración de las funciones matematcias, cuando indicamos por ejemplo una función que se mueve entre espacios vectorialas (como sacar la condenada x de un mapa) f(x,y) R² -> R / f(x,y) = x. Tener palabra reservda ayuda a no tener que declarar un "vacio", si no hay retorno simplemente no ponemos el símbolo "->". Y la particulariad de que la ultima linea sin ";" sirve como retorno es clave porque en relidad aun las funciones sin retorno devuelven un "blqoue vacio" o una exprecion vacia "()". Rust permite intenciar bloques como variables, incluso si se quiere indicar un tipo vacio 

```rs
fn fn_vacia() {
    let _x = 5;
}

fn main() {
    let vacio: () = fn_vacia(); 
}
```
El "tipo" () se lo conoce como "Unit type" el resultado de una sentencia sin retorno, sirve para denotar la unidad básica, similar a nuestro void en **Wyrm**. Incriblemente parecido a el camino que tomo **Wyrm** cuando se crearon los bloques de ejecucion denotados con llaves "{ // codgos varios}".
En particular el enfoque de Rust es increíblemente util si se quiere jugar con las asignaciones, gracias a su diferencia de tipos podemos hacer cosas muy locas como:
```rs
fn fn_vacia() {
    let _x = 5;
}

fn main() {
    let vacio = fn_vacia();  // guardo vacio
    let fn_vacio = fn_vacia; // guardo la función
    let bloque =  || {let y = 5; y}; // guarda un función anonima
    let numero = {let y = 5; y} // guardo el valor numerico 5
}
```
En general este es el enfoque que nos gustaría lograr con **Wyrm** tener libertad de asignación, para eso posiblemente tengamos que modificar como se declaran las funciones y variables para admitir que guardemos estos bloques denotados en {} y de esa menra para las funciones anónimas solo seria cuestión de integar el signo "->" algo como:

"suma = (int64 x, int64 y) -> {x +y}; // posible declaración de funciones lambda en **Wyrm**"

Y con esto se podría hacer por ejemplo (suponiendo que tenemos arrays) map(array, (int64 x) -> {x+1};), y demás operaciones asociadas al paradigma funcional.
Hay varias opocines que se deben tener en cuenta, pero Rust nos muestra algunos caminos tentadores.

### Go

El caso de Go es interesante porque es una de las principales inspiraciones de **Wyrm** para la sintaxis, vamos a revisar una función genérica:

```go
func suma(x int, y int) int {
    return x + y
}
```

La estructura es:
func nombre( argumentos espaciados por comas) TIPO_RETORNO {
    ...
    return RETORNO // El ";" es opcional 
}

Identificamos una palabra clave "func" y sin muchas complicaciones se deja el tipo de retorno a la derecha justo antes de arbir llaves, nada muy distinto de Rust solo que saca el símbolo "->". La palabra "return" es obligatoria en este caso y aunque parezca que ";" no lo es hay un matiz en esto, Go hace automáticamente que algunos '\n' por ';'. Asi quieres escribir en una sola línea puedes poner:

```go
var x = 21; var y = 221; var suma = x + y;
```
pero tambien:
```go
var x = 21
var y = 221
var suma = x + y
```

Internamente, Go "inserta" `;` entre los espacios (una cosa que podiramos incluir en **Wyrm** ya que el error de no poner `;` sule ser muy comun). pero como se hace salvedad, cuando se declara una función de esta manera `func nombre( argumentos espaciados por comas) {..` es obligatorio tener la llave, sin salto de línea (para que se ponga un cartacter `;` antes del cuerpo). Un trade-off acceptable, porque es raro querer abrir llaves con un salto de línea anterior. 

#### Extras Go

Go permite declarar variables de retorno en la firma de la función (cosa que resulta fascinante). En general tener información del nombre de los argumentos es sumamente útil, pero ademas conocer el nombe del retorno suma una capa extra para los que usen la función y para cuando tenemos retornos multiples:

```go
func Descomponer(factor, total int) (descomposicion, resto int) {
    descomposicion = total / factor
    resto = total % factor
    return // Se retornan automáticamente (descomposicion, resto)
}
```
>> Nota: también go permite declarar variables del mismo tipo con name1, name2 TIPO (cosa adaptable en **Wyrm**)

Y con la palabra reservada Go se permite guardarlas como tipo de dato de la forma:
```go
funcmultiplicar := func(a int, b int) int { // podemos guardarla como tipo de dato func(a int, b int) int
        return a * b // esta función no tiene nombre es anonima 
    }
```

También podemos aprovechar la palabra para declarar funciones que devuelvan funciones y las tomen como argumento

```go
func crearAplicador(x int , a_aplicar func(int,int) int ) func(int) int { // la función devuelve otra con argumetno entero 
    return func(y int){ // devolvemmos la función anonima 
        return a_aplicar(x, y)
    }
}
```

Go es un lenguaje inteligente y da múltiples opciones los dev para declarar funciones y retornar valores, no necesita que todo tenga retorno, sino que el foco está en tener declaraciones limpias de escribir.


Has hecho un gran trabajo capturando la esencia visual de Python, pero efectivamente hay un par de conceptos técnicos que son falsos y que debemos corregir para que tu documento sea preciso.

### Python

Python no se sale demasiado de lo que vimos anteriormente (con la salvedad de que no tenemos por qué aclarar tipos de antemano), revisemos una función básica:

```python
def suma(x, y):
    resultado = x + y
    return resultado

```

Python toma la palabra reservada `def` para declarar funciones. A diferencia de lo que podría parecer, la palabra `return` no es estrictamente obligatoria al final; si se omite, la función retornará implícitamente el valor `None` (el equivalente a la ausencia de valor).

A diferencia de los lenguajes anteriormente mencionados, Python no necesita del carácter `;` para separar sentencias, ni de llaves `{}` para encerrar cuerpos de funciones. Python utiliza los saltos de línea para diferenciar instrucciones y se vale de la indentación (típicamente 4 espacios) para indicar a qué bloque pertenece el código, precedido siempre por el símbolo `:`.

```python
if c > 10: # el simbolo ':' indica que lo que sigue indentado pertenece a este bloque
    def aux(x):
        # ahora estamos dentro de la declaración de la función, a otro nivel de indentación
        return x * 2
    fun = aux

c = c + 1 # ya estamos fuera de los bloques porque volvimos al nivel de indentación original
```

Este enfoque es bastante declarativo y nos ata a una estructura rígida en cuanto a los espacios. Es tan estricto con su diseño visual que, por ejemplo, si queremos usar expresiones lambda (funciones anónimas), estas están limitadas por diseño a contener **una única expresión**. No se permiten sentencias, asignaciones o múltiples líneas de lógica dentro de ellas:

```python
sumar = lambda x, y: x + y # válido, es una sola expresión que se retorna automáticamente

sumar = lambda x, y: 
    x = x + 1
    x + y # inválido: las lambdas en Python no admiten bloques de múltiples sentencias

```

Al no tener que aclarar tipos de datos ni retornos Python hace un alarde de comodidad y simplicidad a la hora de declarar funciones y sus retornos. y como cuenta con el valor `None` simplemente asigna como retorno default a toda función y luego si aparece un `return` devuelve el valor que tenga, y como una variable pude mutar de tipo la libertad es gigantesca. 
El enfoque de Python es útil, en su condición de lenguaje de alto nivel se permiten escribir líneas que van a funcionar en la mayorai de los casos sin tener que preocuparse por el macheo de tipos o la declaración de retornos. Pero para permitir esos grados de libertad el costo de de ejecucion es alto, ya que casi todo en realidad es un objeto en memoria con un estado mutable y atributos propios. No se pone en duda que el enfoque es fructífero, pero se aleja mucho de la filosofía que queremos tener en **Wyrm** 


#### Funciones vs lambda

Como indicamos casi todo en Python es un objeto, y esto aplica para las funciones y para las expresiones lambda, las dos son objetos que implementan en la interfaz "callable" (o sea que se le puede llamar con paréntesis), y las dos son del tipo/clase `function`. Y con lo último podemos hacer dos preguntas:

    - ¿Una expresioon lambda es lo mismo que una función?
    - ¿Tiene sentido tener lambda exprecion si en ralidad se instancia lo mismo que en la función? 

La única diferencia entre una función normal y una lambda a nivel interno es que la función cuando se declara tiene un identificador (el nombre) mientras que la expresión lambda no cuenta con ese identificar. Da la sensación de que python maquilla la las funciones anónimas aplicando una palabra clave nueva `lambda` y una sintaxis levemente distinta a las funciones, lo que crea (a criterio personal) una falla en la coherencia del lenguaje.
Esto es algo que no queremos en **Wyrm**, queremos alta cohetería, por lo que intentaremos que nuestras expresiones lambda sean parecidas a nuestras declaraciones de funciones

### Funciones, Retorno y Lambdas en **Wyrm**

Quién diría que el análisis de un test tan simple nos llevaría a hacer esta reflexión. La buena noticia es que los demás tests no van a requerir un re-análisis si dejamos planteadas nuestras convenciones desde el principio.
Evaluar el retorno sin evaluar las funciones no tiene sentido, como tampoco tiene sentido no analizar las funciones Lambdas sin analizar las funciones. Luego de revisar comentar las distintas aproximaciones de los lenguajes de referencia tenemos en limpio algunos ítems.

En principio que se desea:

    - Tratar a las funciones como otro tipo de dato
    - Poder guardar funciones como en variables
    - Queremos expresiones de funciones anonimas 
    - La forma de declarar funciones debe ser parecida a la de declarar variables
    - Sintasis clara

Que no se quiere:

    - Maquillar las declaraciones de las funciones anonimas 
    - Tener una sintaxis de distintas para las mismas cosas 
    - Tener sentencias demasido laragas 

Bajo estas primicias e ideando como deberían verse las fusiones se presenta un problema, una decisión sobre como declara alguna cosa influye directamente en las otras declaraciones. Como ya decidimos declarar variables de la forma `TIPO nombre IGUAL valor` no podemos movernos mucho de esta forma para las funciones, lo que nos obliga a no difinir una palabra reservada (como `def`, `fn`, etc) para la declaración de funciones, si lo hacemos nos obliga a declarar la palabra reservada para variables. También como nuestra definición el `TIPO` va a la izquierda sería extraño declarar de la manera:
`nombre(TIPO1 ARG_1, TIPO2 ARG_2,..., TIPON ARG_N) RETORNO {codigo ....}`
Es decir para mantener la coherencia tenemos que dejar la dedición de "tipo variables" o "retorno" a la izquierda y sin palabra reservada, al menos si no queremos cambiar la forma en la que declaramos variables. 

>> NOTA: desde ahora usaremos una función suma "suma(x,y) = x+y" como ejeplo para todas las declaraciones

Eso nos reduce unas pocas formas para las funciones:

```C
int64 suma(int64 x, int64 y) {x +y } // E1 enfoque igual a C 
(int64, int64)  int64 suma(int64 x, int64 y) {return x+y} // E2 enfoque ultra declarativo 
(int64, int64) -> int64 suma(int64 x, int64 y) -> {return x+y} //E2 enfoque ultra declarativo con flechas para poder ser explictio en fn1 -> fn2 -> ...
```
E1 es clasico y facil de implemtnar, tiene mucha coherencia con la declaración de varaibeles. Su problema es que para funciones que devuelvan funciones las firmas se tranforman en algo poco legible:

```C
int64 (int64, int64)  crear_operador(OPERADOR_TYPE op) {
    if(op = ADD){
        return suma;
    }...
 } 
```
Y ni hablar de funciones con varios niveles de anidación..
Uno pensaría entonces que E2 no es mejor que E1, pero E2 nos es más legible, ya que tiene el flujo de transformación de datos (de argumentos hacia retorno) y si agregamos el símbolo `->` como vemos en E3 resolvemos el problema de la anidación de funciones.

```C
// supongamos que tenemos anidacion de 3 niveles:
(int64) (int64) (int64) int64 suma_triple(int64 a) {
    return (int64 b) {
        return (int64 c) {
            return a + b + c;
        };
    };
}

// Con E3 las flechas nos maracan el camino argumento -> retorno 
(int64) -> (int64) -> (int64) -> int64 suma_triple(int64 a) -> { // función que devuelve una función que devuelve una función que devuelve entero
    return (int64 b) -> {
        return (int64 c) -> {
            return a + b + c;
        };
    };
}
```

Nótese que mágicamente surgió la necedad de crear funciones anónimas y que estas tengan una sintaxis parecida a las funciones (si es que no idénticas). Asi que hablemos de como declararíamos funciones anónimas antes de llegar al resultado final.

```C
// Alternativas 
lambda suma = int64 (int64 x, int64 y) -> {x + y}; // lambda como tipo de dato 
suma = int64 (int64 x, int64 y) -> {x + y}; // sin firma 
int64 (int64, int64) suma = int64 (int64 x, int64 y) -> {x + y}; // ultra declarativo 

```
Podemos observar que el símbolo `->` es super importante, sin él no sabríamos cunado empieza una asignación "nombre=función" y cuando se establece el flujo "(args)->retorno".
De estas alternativas para lambda la primera usaría lambda como tipo de dato, aunque parece provechoso tener un identificador como si fuera tipo de dato surge un problema de coherencia al ver las declaraciones de funciones y de lambda juntas

```C
int64 suma(int64 x, int64 y){ return x+y}

lambda suma_l = (int64 x, int64 y) int64 -> {x + y}; 

// como guardo "suma"?

func suma_v = suma: //?
lambda suma_v = suma: //repito lambda?
(int64, int64) int64 suma_v = suma // explicito el retrono como en lambda?
int64 (int64, int64)  suma_v = suma // explicito el retrono como en las funciones?
```
El problema de Python se nos presenta, tenemos dos elementos que son iguales internamente, pero reservamos palabras distintas y/o sintaxis alejadas.

Para la opcion sin firma aunque es cómoda, no respeta para nada la coherencia con la declaración de variables, le estamos dando un character especial a las funciones anonimas que aunque serviría en casos de funciones como argumentos, se pierde totalmente la coherencia con los demás tipos:

```C
// Supongamos que tenemos un arreglo y la función map ;

// entoces podriamos llamar con
arr_map = map(arr, (int64 x) int64 -> {x *2}) // forma comoda

//como definimoas map? 
arr<int64> map(arr<int64> arr, int64 (int64 x, int64 y) operando ) // el nombre a la derecha como tipo de dato normal
arr<int64> map(arr<int64> arr, operando int64 (int64 x, int64 y)  ) // el nombre a la izquieda ?
arr<int64> map(arr<int64> arr, int64 (int64 x, int64 y)  ) // sin nombre porque al ser anonima ?

// si no ponemos nada a la izquierda a la hora de definir se complica demacia tratar a las funciones como un tipo de dato más
int64 operando_1 = 1;
int64 operando_2 = 2;

suma = int64 (int64 x, int64 y) -> {x + y};  // el unico sin "dato"?
```

Nos queda por descarte ser super declarativos que además de ser similar a las firmas de las funciones tiene el "tipo" a la izquierda...

Todo parece correcto **Wyrm** queda con:

```C

int64 suma(int64 x, int64 y){ return x+y}

int64 (int64, int64) suma_l = int64 (int64 x, int64 y) -> {x + y}; 

// como guardo "suma"?

int64 (int64, int64) suma_v = suma;

/// y como argumento?
arr<int64> map_int(arr<int64>, int64 (int64) operando);
```

Pero esta aproximación tiene dos problemas grandes:

La primera existe en como declaramos las funciones lambda donde repetimos los tipos de datos dos veces;

```C
/*
bloque con los tiposd de datos = mismo blouqe solo agregamos nombres*/
int64 (int64, int64) suma_l = int64 (int64 x, int64 y) -> {x + y}; 
```
Nos encantaría poder reducir la redundancia, pero como podemos hacerlo si somos fuertemente tipeados? Por suerte el lengauje Java nos trae la solucion, revisemos rapidamente lo que es, a criterio personalas, una de las mejores sintaxis de funciones anónimas

```Java

@FunctionalInterface
interface Operacion {
    int calculator(int a, int b);
} // se define una interfas

// la interfas se usa como dato
Operacion suma = (a, b) -> a + b;
```

Sin que nos importe la interfaz fijémonos que sus expresiones no necesitan indicar los tipos del argumento, Java es inteligente y aprovecha la información de la izquierda para asignar tipo al retorno y a los argumentos. Java logra mantener su tipado fuerte, pero sin caer en redundancia a la hora de declarar funciones anónimas

```C
// usamos la información de la izquierda para asignar los tipos a nuestros argumentos.
int64 (int64, int64) suma = (x, y) -> { x+y };

// firmas
arr<int64> map_int(arr<int64>, int64 (int64) operando);


// cuando llamaos no hace falta declarar tipos, porque la firma se resolvio el tipo
map_int(arr, (x) -> { x*2 });
```

Una sintaxis fantástica, **Wyrm** debería aplicarla para reducir la redundancia y cumplir el objetivo de tener funciones anónimas de tipado fuerte. Existe una mejora más y es agregar el simbolo `->` del lado izquierdo también de esta manera:
```C
// empatamos perfecto el lado izquierdo 
(int64, int64) -> int64 suma = (x, y) -> { x+y };
```

Ahora el nivel de coherencia es enorme (más de 9000), si observamos la redundancia es practicamente nula y separamos del lado izquiedo el "tipo" y del lado derecho su valor real.
Obviamente tenemos que preguntarnos como resolves si la función no tiene argumentos o tiene retorno vacio, podemos definir el vacio como `()` o bien como palabra `void`, la palabra aunque funcionalmente identica, es preferible darle al "vacio" un character especial y denotarlo con algun simbolo. Podriamos usar `∅` que seria perfecto, pero pedir un character especial seria demasido complicado trabajar con el, `(/)` seria otra opcion valida, incluso `(\)`, pero es preferible mantener la simplesa y "tomar" el enfoque Rust.

```C
// definimos () para funciones sin argumetnos 
() -> int64 uno_int = () -> {1};

// analogamente vamos de arg -> vacio indicado por ()
(int64, int64) -> () print_suma = (x,y) -> {pritn(x+y)};
```

Listo, tenemos las funciones anónimas ideales para el marco de **Wyrm**. 

Nos queda el último problema y es la clave para llegar a la resolución final, y es la coherencia.
Ahora que tenemos una forma ideal de funciones anónima ¿Deberíamos darle un carácter especial a las funciones genéricas? ¿Como ser veria una función generia luego de todo lo evaluado?

```C
// el enfoque clasico ?
// es demasico distitno 
int64 suma(int64 x, int64 y) { return x+y}

// con ->?
// mismo problema que antes 
int64 suma(int64 x, int64 y) -> { return x+y}

// dejamos los argumentos con su tipo separado?
// similar a las anonimas pero con los argumentos ivnertidos, no podemos poner "->"
int64 (int64, int64) suma( x,  y) { return x+y}

// identica a la anonima pero sin ?
(int64, int64) -> int64 suma(x, y) -> { x+y };
```
Claro si es identica para ganar coherencia y ademas aprovechar el asignado de tipos.... ¿Es necesraio?
Desde **Wyrm** decimos que no, si en síntesis se comportan igual, deberíamos declararlos igual. **Wyrmm** no va a hacer distinciones entre las funciones anonimas y las "normales". Ya que en sí queremos que las funciones se puedan usar como valores de variables, que se puedan guardar, recibir como argumento y devolver.
El enfoque definitivo es no tener distinción, aprovechar la declaración de funciones lambda y funciones normales, toda función será anónima y si se la desea etiquetar se la debe guardar en una variable.

Para este **Enfoque definitvio** vamos a agregar un parámetro opcinal luego de `(arg..) ->` tenemos el tipo del retorno a la izquierda podemos darle un nombre del lado derecho e imitar a Go, sumando una capa de comodidad (de alguna manera para compensar la diferencia con el estándar). 

```C
// Enfoque definitivo
(int64, int64) -> int64 suma = -> (x,y) -> {
    return x+y // return opcional, mientras este en la ultima liena 
}

// Opcinal declarar los nombres del retorno
(int64) -> (int64, int64) dividir = (x) -> (cociente, resto) {
    cociente = x / 2;
    resto = x % 2;
}; // retorno automático 
```

**Estructura base:**
```c
//                                           el = podria ser <- o :=
(TIPO_1, TIPO_2, ...) -> (TIPO_RETORNO)  nombre = (arg_1, arg_2, ...) -> {
    // código terminando en return o devolviendo el último valor evaluado 
}

```

**Estructura opcional (retornos nombrados):**

```c
(TIPO_1, TIPO_2) -> (TIPO_RETORNO_1, TIPO_RETORNO_2) nombre = (arg_1, arg_2) -> (retorno_1, retorno_2) {
    // se devuelven automáticamente las variables de retorno (como en Go)
}

```

Para indicar una función sin retorno, Rust nos dio la respuesta con `()`: su retorno es vacío, algo que ya tenemos integrado en **Wyrm**.

```c
() -> (TIPO_ARG) nombre = (arg) -> { 
    // código sin retorno
}
```

Esta sintaxis es útil, pero no está exenta de problemas. Por ejemplo, a nivel léxico, al leer el carácter `(` no sabemos de inmediato si va a comenzar una expresión matemática al estilo `(4/5 - 0) + 1;` o si va a empezar la firma de una función. También, aunque la sintaxis es bastante limpia, al ser ligeramente distinta a los estándares habituales de C, puede que al principio sea complicado acostumbrarse.

El mayor desafío técnico es que implementar esto requiere cambiar internamente los tests y, casi con total seguridad, aplicar una refactorización profunda en el parser.

## Tests 2