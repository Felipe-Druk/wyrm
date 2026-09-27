# ¿Cómo resolves múltiples tipos? 

Tener múltiples tipos con distintos tamaños en memoria (como nat16 o int64) nos da la ventana de a que pase los siguiente:

nat8 natural = 12;
int32 entero = -20;

al sumar el natural + entero que tipo debería tener el resultado? O deberíamos levantar un error? 
Aparte que pasa si se suman dos int8 y el resultado excede el límite de 2**8-1? 

## Acción general

En general la convección que se va a tener es que el retorno de la operación va a pertenecer al conjunto "más grande" posible, esto porque normalmente si los tipos son distintos (como int64 y nat8), en general uno estará contenido en el otro. De este modo siempre que se sume algo con un número flotante, el resultado será flotante, si se hace entre un entero y un natural el resultado será entero. Solo nos deja un problema que es cuando se alcanza el límite del entero para 64 bits, sería el único momento donde entre natural y entero ganaría el natural.

Los distintos problemas que pueden surgir son en su mayoría de asignación, ahí si quieren cargar un valor que excede el rango del tipo específico el programa debería impedirlo. Por otro lado, se intentará inferior los tipos para acciones como print(1+ 333)o print(1.5 + 41). 

## Suma

La suma tiene dos puntos críticos a destacar, cuando se suman dos números y se excede la el límite de representación y cuando se le suma a un natural un entero negativo.

El excedente de representación puede pasar con esto puede pasar al sumar natx + natx, intx+inx o natx + intx.
En este caso siempre seria mejor movernos int2x y para números muy grandes se podria ir a el maximo de nat64, pero seria mejor lanzar un error, ya que el desarrollador esta usando números positivos muy grandes con el tipo int.


Mientras que la caída por debajo de seria solo puede pasar en natx + intx
Este problema se resuelve con lo de arriba, si el resultado es intx u int2x nunca nos saldremos de representación. 

entonces:

(nat,nat) -> nat
(nat/int, int) -> int 
(nat/int, float) -> float

## Resta

Mismo problema que con la suma, pero esta vez podemos caer de representación tanto con naturales como con enteros. 
En cualquier caso siempre será seguro movernos al intx más grande posible.

entonces:

(nat,nat) -> nat
(nat/int, int) -> int
(nat/int, float) -> float

## Multiplicación

En esto el problema es parecido al de la suma, la representación crece más rapido y en vez de poder caer por debajo de 0 hacer natx *intx donde el entero es negativo. 
Entonces siempre que tengamos un entero deberíamos mantenernos en el.

(nat,nat) -> nat
(nat/int, int) -> int
(nat/int, float) -> float

## División 

Más alla de lo basico aca hay una pregunta de diseño clave, que pasa si hacemos 5/3 siendo estos dos naturales o enteros? lo lógico seria asumir que el ususario querie el valor real entonces deberíamos devolver un flotante. Pero si hace nat/nat ? nat/int ? la pregunta se extiende a si cambiamos el valor del numero o trucamos el resultado (una acción más estándar). Es crítico acá porque tendriamos que ponernos en perspectiva, porque alguien definiria dos datos enteros/naturales para luego dividirlos entre si? 
Aunque no sea lo más riguroso matemáticamente, pero si lo más "lógico" o lo que se esperaría que suceda, es que se trunque el resultado si se trata de dos variables enteras y/o naturales (de paso mantenemos la convención con los otros operadores).

(nat,nat) -> nat
(nat/int, int) -> int
(nat/int, float) -> float

>> Si surge algun problema más se agregara a este archivo