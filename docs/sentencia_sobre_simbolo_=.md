
# Debate sobre la asignacion

Justo al terminar el escáner me encontré con un debate, como tomar el símbolo `=`. No es cualquier cosa, sino que hay un sentido filosófico y técnico detrás.

## ¿Yo igualo o yo defino? 

Posiblemente, el problema más grande es que en matemáticas el `=` es una sentencia, declaro que la igualdad existe y cumple algunas propiedades como la transitividad (a = b, b = c -> a = c). Pero en programación se usan para dos cosas, para igualar una variable a un valor u otra variable y también cuando defino esa variable sé instancia con un valor inicial (a veces). 
Esto hace pensar, ya que al hacer `x = 1` y luego `x = 12` significa que las declaraciones fueron temporales, x efectivamente tenia el valor 1 hasta que le fue cambiado. Esto quiere decir que cuando programamos nunca usamos el símbolo "=" con el rigor matemático que buscamos en **Wyrm**. Y a efectos prácticos pedirle al usuario que siempre instancia una nueva variable cuando quiere cambiar de valor la misma no es lo más eficiente a nivel técnico.

## ¿Qué debería hacer el símbolo? 

En principio todos tenemos claro que el símbolo igual debería asignar el valor a la derecha a el componente de la izquierda y nada más. Esto cumpliría con respetar de alguna manera el concepto matemático (aunque limitado) pero nos limita de hacer declaraciones transitivas como `a = b = 1;` por ejemplo.

Podriamos sino tomar otra aproximación y declarar que una asignación de valor retorne el valor asignado, rompiendo asi el rigor matemático, pero ganado algunas facilidades como la asignación en cadena `a = b =1;` o comprimir líneas del estilo `while( line = read(file) != EOF)` este último solo es posible si el `=` tienen algún retorno. 

Lneguajes como python tienen el igual sin retorno, pero luego añadió el símbolo `:=` para tener un igual con retorno, cosa distinta a Go que no tiene un igual con retorno y reservo el símbolo `:=` para la declaración de variables (algo que nos gustaría copiar a futuro), por último C/C++ tienen el operador igual con retorno por lo que no hay un esntandar en la industria es una definición pura de diseño.

Hay un debate interesante en esta [pagina](https://www.reddit.com/r/ProgrammingLanguages/comments/9qx8kv/assignment_as_an_expression_or_a_statement/)

## Que seria mejor para nuestro diseño?

Principalmente y dejando gustos personales de lado ¿Cuál es la filosofía de **Wyrm**? Para responder veamos que fue escrito como "core" del proyecto:

__"**Wyrm** quiere ser un leguaje con pocas sorpresas donde el rigor matematico y logico sea primordial, bajo **Wyrm** se toma la postuda de que darle libertad y conocimiento al desarrollador implica que los productos tengan mejor calidad a largo plazo. "__ 

Bajo esta sentencia nos debiéramos apegar al rigor matemático y no darle retorno a la asignación... aun si no darle la posibilidad de tener una asignación con retorno y dejar al desarrollador con menos herramientas tendría poco sentido. Por lo que lo mejor seria ir por la opción de Python y tener dos símbolos uno con retorno y otro sin. Ahora solo tenemos que definir cuales...

# Sentencia final

El simbolo `=` no tendrá retorno, sino que simplemente cumplirá con lo que se espera de él, "cargar" el valor en la variable.

Para un igual con retorno vamos a aprovechar un símbolo afín a la asignación, teniendo en cuenta que `:=` lo guardaremos para más tarde, que `==` se reserva en booleanos y que tanto `=>` como `<=` son comparadores. El designado será `<-` un símbolo poco usado en realidad, pero algunos lenguajes lo aprovechan para asignar, y como en el futuro queremos aprovechar `->` para definir funciones, `<-` nos queda perfecto para lo que buscamos.

Ahora un se permitirá `a <- b <- 1;` pero si no queremos tener sentencias complejas podemos declarar como siempre de la manera `x = 1;`

> como extra para mantener el rigor podemos pensar que definimos <-:(a,b) -> b una función que toma a y b asigna el valor b en a y luego retorna b
