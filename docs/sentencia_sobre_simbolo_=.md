
# Debate sobre la asignacion

Justo al terminar el escaner me encontre con un debate, como tomar el simbolo "=". No es cualquier cosa sino que hay un sentido filosofico y tecnico detraz.

## Yo igualo o yo defino? 

Posiblemente el problema más grande es que en matematicas el "=" es una sentencia, declaro que la igualdad existe y cumple algunas propiedades como la transitividad (a = b, b = c -> a = c). Pero en programcion se suan para dos cosas, para igualar una variable a un valor u otra variable y tambien cuando defino esa variable se instancia con un valor incial (aveces). 
Esto hace pensar, ya que al hacer x = 1 y luego x = 12 significa que las declaracines fueron temporales, x efectivamente tenia el valor 1 hasta que le fue cambiado. Esto quiere decir que cuando programamos nunca usamos el simbolo "=" con el rigor matematico que buscamos en **Wyrm**. Y a efectos practicos pedirle al usuario que siempre instancie una nvuea varible cuando quiere cambiar de valor la misma no es lo más eficinete a nivel tecnico.

## Que deberia hacer el simbolo? 

En principio todos tenemos claro que el simbolo igual deberia asignar el valor a la derecha a el componente de la izquierda y nada más. Esto cumpliria con respetar de alguna manera el concepto matematico (aunque limitado) pero nos limita de hacer declaraciones transitivas como a = b = 1; por ejemplo.

Podriamos sino tomar otra aproximacion y declarar que una asignacion de valor retorne el valor asignado, rompiendo asi el rigor matemativo pero ganado algunas facilidades como la asignacion en candena "a = b =1;" o comprimier lineas del estilo "while( line = read(file) != EOF)" este ultomo solo es posible si el = teien algun retorno. 

Lneguajes como python tienen el igual sin retorno pero luego añadio el simbolo ":=" para tener un igual con retorno, cosa disitinta a Go que no tiene un igual con retorno y reservo el sombolo ":=" para la declaracion de variables (algo que nos gustaria copiar a futuro), por ultimo C/C++ tienen el operador igual con retorno por lo que no hay un esntandar en la industria es una definicion pura de diseño.

Hay un debate interesante en esta [pagina](https://www.reddit.com/r/ProgrammingLanguages/comments/9qx8kv/assignment_as_an_expression_or_a_statement/)

## Que seria mejor para nuestro diseño?

Principalmente y dejando gustos personales de lado ¿Cual es la filosofia de **Wyrm**? Para responder veamos que fue escrito como "core" del proyecto:

__"**Wyrm** quiere ser un leguaje con pocas sorpresas donde el rigor matematico y logico sea primordial, bajo **Wyrm** se toma la postuda de que darle libertad y conocimiento al desarrollador implica que los productos tengan mejor calidad a largo plazo. "__ 

Bajo esta sentencia nos debiermos apegar al rigor matematico y no darle retorno a la asignacion... aun si no darle la posiblidad de tener una asignacion con retorno y dejar al desarrollador con menos herramietnas tendria poco snetido. Por lo que lo mejor seria ir por la opcion de Python y tener dos simbolos uno con retorno y otro sin. Ahora solo tenemos que definir cuales...

# Sentencia final

El simbolo = no tendra retorno sino que simplemente cumplira con lo que se espera de el, "cargar" el valor en la variable.

Para un igual con retorno vamos a aprovechar un simbolo afin a la asignacion, teniedo en cuenta que := lo guardaremos para más tarde, que == se reserva en booleanos y que tanto => como <= son comparadores. El desginado sera "<-" un simbolo poco usado en realidad, pero algunos lenguajes lo aprocehcan para asginar, y como en el futuro queremos aprovechar "->" para definir funciones, "<-" nos queda perfecto para lo que buscamos.

Ahora un se permitira a <- b <- 1; pero si no queremos tener sentencias complejas podemos declarar como siempre de la manera x = 1;

> como extra para mantener el rigor podemos pensar que definimos <-:(a,b) -> b una funcion que toma a y b asigna el valor b en a y luego retorna b
