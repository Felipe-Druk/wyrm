
#pragma once

/*
Token principal, representan simobolos y palbreas que Wyrm conoce, lo ideal es intentar no agregar dmeadiso tipos
nativos. Queda como todo crear la amenra de isntanciar "structs" de wyrm, y libreiras extra.
*/

// El tipo del token nos indicara como consumir su "lexme"
typedef enum {
    // Enteros
    T_INT8,  // int8 = 8 bits int
    T_INT16, // int16 = 16 bits int
    T_INT32, // int32 = 32 bits int
    T_INT64, // int64 = 64 bits int

    // Naturales
    T_NAT8,  // nat8 = 8 bits natural
    T_NAT16, // nat16 = 16 bits natural
    T_NAT32, // nat32 = 32 bits natural
    T_NAT64, // nat64 = 64 bits natural

    // Flotantes
    T_FLOAT32, // float32 = 32 bits flotante
    T_FLOAT64, // float32 = 64 bits flotante

    // Booleanos
    T_BOOL, // bool 1 byte

    // Palabras para booleanos
    T_TRUE,  // "true" sentencia verdadera
    T_FALSE, // "false" sentencia falsa
    T_NOT,   // "not" niega la sentencia unario
    T_AND,   // "and" compuerta logica donde las dos deben ser veraderas
    T_OR,    // "or" compuerta logica solo una debe ser veradera
    T_XOR,   // "xor" compuerta logica donde las dos deben ser distintas

    // Operadores arigmeticos
    T_ADD, // + para suma
    T_SUB, // - para resta
    T_MUL, // * para multiplicación
    T_DIV, // / para division
    T_POW, // ^ para potencia
    T_MOD, // % para modulo

    // Operadores logicos

    T_EQUAL,         // == para comparar igualdad
    T_NOT_EQUAL,     // ~= para comparar igualdad
    T_LESS,          // < para comparar igualdad
    T_GREATER,       // > para comparar igualdad
    T_LESS_EQUAL,    // <= para comparar igualdad
    T_GREATER_EQUAL, // >= para comparar igualdad

    // Operadores varios
    T_ASSIGN,  // = para asignacion de variables
    T_RASSIGN, // <- para asignar valor y recuperar el retorno

    // Literales
    T_NUMBER,       // Numeros enteros o naturales
    T_STRING,       // Cadenas de texto
    T_FLOAT_NUMBER, // Numeros flotantes

    // Importante
    T_IDENTIFIER, // Identificador etiqutas
    T_COOMA,

    // If/else
    T_IF,
    T_ELSE,
    // loops
    T_WHILE,
    T_FOR,
    // Limitadores
    T_EOF,       // End of file suele ser 0 char
    T_SEMICOLON, // ; para separar sentencias
    T_LPAREN,    // ( encapsula sentencias junto con su cierre
    T_RPAREN,    // )
    T_LBRACE,    // { encapsula "scopes" o bloques de ejecucion
    T_RBRACE,    // }

} TokenType;

#define MIN_TYPE T_INT8
#define MAX_TYPE T_BOOL

// Un token es un par de tipo y lexema.
// El lexema es un string que representa el token en el codigo fuente.
typedef struct {
    TokenType type;
    char *lexeme;
} WyrmToken;

typedef WyrmToken wyrm_token_t;

// No es lo más usual pero voy a definir un to_string es util para debuggear.
const char *token_type_to_string(TokenType type);