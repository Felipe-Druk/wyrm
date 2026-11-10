#pragma once
#include "../wyrm_types/ast/ast_node.h"
#include "../utils/token_vector.h"

// Estrutura parser, a partir de un vector de toknes crea un ast
typedef struct {
    token_vector_t *tokens;
    size_t current_index;
    int flags;
} Parser;

typedef Parser parser_t;

// Crea el parser a partir del vector si falla devuelve un NULL pointer
parser_t *create_parser(token_vector_t *tokens);

// Debe devuelve el nodo raiz listo para ejecutar (Puede disparar errores)
ast_node_t *parser_parse(parser_t *parser);

// Actualiza las nuevas banderas
void parser_set_flags(parser_t *parser, int new_flags);

// Libera la memoria del parser
void destroy_parser(parser_t *parser);