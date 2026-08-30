
#pragma once

// El tipo del token nos indicara como consumir su "lexme"
typedef enum {
    // Enteros
    T_INT8,
    T_INT16,
    T_INT32,
    T_INT64,

    // Naturales
    T_NAT8,
    T_NAT16,
    T_NAT32,
    T_NAT64,


    // Operadores
    T_ASSIGN,

    // Limitadores
    T_EOF,
    T_SEMICOLON,
} TokenType;    



// Un token es un par de tipo y lexema. 
// El lexema es un string que representa el token en el codigo fuente.
typedef struct {
    TokenType type;
    char* lexeme;
} WyrmToken;