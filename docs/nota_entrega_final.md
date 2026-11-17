## Comentarios

Hoy es el día de la entrega final, y toca ver como llego el Trabajo a este punto. Hay que decir que el resultado final me parece hermoso, no se lograron todas las metas (creo que nunca se llega a ese punto), pero hacer la recreación de mi primer TP, pero esta vez con un lenguaje propio  me parece un cierre bastante poético (hablando de cierre porque es una de las últimas veces que voy a hacer un TP en esta carrera).

No puedo, sino lamentar no haber llegado con todos los planes y estar un poco apurado con la antena (otra vez).

### Peores cosas

Creo que no le dedique el tiempo correcto a todos los componentes, se dejaron algunas cosas como "parches" sin pensar con calma como anexar cada parte del mismo. Por eso no hay orgullo en como quedaron algunas porciones de código, es funcional y bastante perforante y eso es lo importante.

### Mejores

El cambio en las funciones fue "Clave", era uno de los objetivos principales del lenguaje incorporar las "arrow functions" y llegar a la conclusión de que toda función debería ser una "arrow" fue bastante gratificante, aunque un poco trabajo de escribir. Recomiendo leer los [objetivos](./objetivos_02.md) para indagar más.

El reciclaje de toda la arquitectura de `vec` para que pueda comprender `strings` quedo bastante elegante, una vez que de anexo el módulo `writ/read` se pudo ampliar de manera inmensa las utilidades con poco código.  
También creo que el refractor del parser y tomar la estructura como base para los demás componentes también fue exente; obligo a cambiar el build y un poco la estructura del proyecto, pero fue para mejor.

## Que me gustaría agregar

**Wyrm** todavía es muy primitivo, creo que hay tres cosas claves que se podrian agregar para que con el set de herramientas básico se puedan hacer librerías;

- syscall: poder llamar al sistema directamente abré la puerta para jugar mucho más cerca del sistema operativo; se podrían crear procesos, abrir sockets y comunicar esos mismos procesos. Además de poder alojar y liberar memoria. Con el llamado a la operación del sistema **Wyyrm** no tendría límite de ampliación, y sin tener que tocar el código interno, solo construyendo con lo que hay

-  struct: Aunque con el tipo `vec<nat8>` en teoría podríamos hacer cualquier cosa (ya que un struct no es más que una forma especial de leer una tira de byytes); No tener la opcion de declarar estructuras (o clases contendoras) hace que **Wyrm** se quede muy atras en cuanto a funcionalidad, poder definir estructuras propias nos hace independientes de los tipos "nativos" del lenguaje, permitiendo que cada quien lo amplíe según su necesidad (sin tener que agregar un token o tocar el código interno).

- Interfaces: independícenteme de si se agregan strcuts o clases, no estarian siendo aprovechadas del todo sin interfaces. Ya son un estándar en los lenguajes modernos, ofrecen una capacidad de reciclar código y ganar mucha abstracción, además de ser en algunos casos más potentes que las herencias. 

- Inferencia de tipos: El ya discutido":=" no fue implementado, de hecho parece que fue eliminado de la mente del dev mietnras duraba la entrega final.. Es, agregarlo junto a la inferencia de tipos podría elevar la comodidad de **Wyrm** a nivles superiores sin perder el tipado fuerte.

Y como un extra y llegando a las características que no se justifican tanto desde lo funcional, sino que seria un diferencial tenerlo 

-  Colores: Siempre se busca una libreria de colores cuando se quiere hacer un buen debug o simplemente para mejorar la parte visual de un programa, soportar strings coloreados de manera nativa (o con una libreria) seria un gran diferencial que elevaría **Wyrm** y le daria "personalidad"

- Sonidos: Hay mucha teoria de como interactuar directamente con la consola (stdin) y como manejar los píxeles de las pantallas (interactuar con la GPU), pero al parecer es muy poco popular comunicase con la parte sonora del sistema. La idea sería algo simple, una función que envíe una "nota" al sistema y que se reproduzca. No conozco muchos lenguajes que tengan facilidad para interactuar con la placa de sonido y/o parlantes, sería un diferencial enorme.

La mayoria de las cosas que me sería bueno agregar van de la mano con la idea de "*Dejar el set de herramientas básico para construir librerías*" y dejar de ampliar **Wyrm** con base en código, sino que ampliarlo con las mismas herramientas que se tienen de base.
La de colores y sonidos es responde más a una fascinación personal por los minijuegos en consola y como llevarlos a otro nivel.


## Palabras finales

Crear un lenguaje de programación parecía muy complicado al principio, pero creo que no es tan complicado, lo complicado es justificar la decisión de diseño, pensar bien el porqué de las cosas y plasmarlo lo más claro posible en un archivo de texto. Creo que la parte que más disfrute fue explorar ca fondo como resumen la sintaxis los grandes (C, Rust, Go, etc) y darme cuenta cuáles están pensados desde las bases y cuáles se acomodaron con el tiempo. Destaco mucho más a Go que todo lo que describir sobe el tiene tanto sentido y está tan bien resuelto, este proyecto me hizo valorarlo mucho más. 
A nivel de implementación no hay mucho que decir creo, uno podría escribir un programa que lea y ejecute un archivo .py y lo puede implementar como quiera pero si funciona igual que Python3 el lenguaje no cambio, solo la caja negra que lo interpreta. Lo importante es un buen diseño.