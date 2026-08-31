
#pragma once

// El tipo del token nos indicara como consumir su "lexme"
typedef enum {
    // Enteros
    T_INT8, // int8 = 8 bits int
    T_INT16, // int16 = 16 bits int
    T_INT32, // int32 = 32 bits int
    T_INT64, // int64 = 64 bits int

    // Naturales
    T_NAT8, // nat8 = 8 bits natural
    T_NAT16, // nat16 = 16 bits natural
    T_NAT32, // nat32 = 32 bits natural
    T_NAT64, // nat64 = 64 bits natural


    // Operadores
    T_ASSIGN, // = para asignacion de variables


    //Importante 
    T_IDENTIFIER, // Identificador etiqutas 

    // Limitadores
    T_EOF, // End of file suele ser 0 char
    T_SEMICOLON, // ; para separar sentencias
} TokenType;    



// Un token es un par de tipo y lexema. 
// El lexema es un string que representa el token en el codigo fuente.
typedef struct {
    TokenType type;
    char* lexeme;
} WyrmToken;

typedef WyrmToken wyrm_token_t;