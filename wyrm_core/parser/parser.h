#pragma once
#include "../ast_node.h"
#include "../utils/token_vector.h"

typedef struct {
    token_vector_t *tokens;
    size_t current_index;
} Parser;

typedef Parser parser_t;

// Crea el parser a partir del vector si falla devuelve un NULL pointer
parset_t *create_parset(token_vector_t *tokens);

// Debe devuelve el nodo raiz listo para ejecutar (Puede disparar errores)
ast_node_t *parser_parse(parser_t *parser);