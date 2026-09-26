## Info general

Como el parser es una estructura más compleja va a ser muy parecida a la implemntacion del repositorio ["plox"](https://github.com/FdelMazo/plox/blob/main/plox/Parser.py). En esa implementación el orden de llamado defini el "peso" del arbol y es crucial tenerlo bien definidos para que la ejecucion sea limpia. 

## Nuestro orden

Puede estar atado a cambios, y si esos cambios existen se reflejaran en este archivo para tener mejor documentación y seguimiento del proyecto

**Orden 1**

statement -> var_decl

var_decl -> "TIPO" T_IDENTIFIER T_ASSIGN expression T_SEMICOLON 

> TIPO es cualquier tipo de dato

expression -> term

term -> factor ( ( T_ADD | T_SUB ) factor )

factor -> power ( ( T_MUL | T_DIV | T_MOD ) power )

power -> primary ( T_POW primary )

primary -> T_NUMBER | T_FLOAT_NUMBER | T_IDENTIFIER


**Orden 2**

El orden anterior ignoraba operadores unarios como "-"

statement -> var_decl

var_decl -> "TIPO" T_IDENTIFIER T_ASSIGN expression T_SEMICOLON 

> TIPO es cualquier tipo de dato

expression -> term

term -> factor ( ( T_ADD | T_SUB ) factor )

factor -> unary ( ( T_MUL | T_DIV | T_MOD ) unary )

unary -> ( T_SUB ) power

power -> primary ( T_POW primary )

primary -> T_NUMBER | T_FLOAT_NUMBER | T_IDENTIFIER