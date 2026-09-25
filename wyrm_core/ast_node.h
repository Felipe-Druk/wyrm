#pragma once
#include "wyrm_token.h"

typedef enum {
    AST_NUMBER_LITERAL,
    AST_IDENTIFIER,
    AST_BINARY_EXPR,
    AST_VAR_DECLARATION,
    AST_UNARY_EXPR
} ast_node_type_t;

// Estrutura princiapl del AST, usamos union para "simular" polimorfismo
typedef struct ASTNode {
    ast_node_type_t type;

    union {
        // Almacena literales numericos
        struct {
            char *value; // Bytes del numero {'0','1','2','3','4','5','6','7','8','9','.'} + '\0'
            TokenType numeric_type;
        } number_expr;

        // Etiqueta el valor para el valor de variable
        struct {
            char *name;
        } identifier_expr;

        // Operaciones binarios
        struct {
            struct ASTNode *left;
            TokenType operator;
            struct ASTNode *right;
        } binary_expr;

        // Operaciones unarias com  -5 o ~x
        struct {
            TokenType operator;
            struct ASTNode *right;
        } unary_expr;

        // Declaraciones de variables
        struct {
            TokenType var_type;
            char *identifier;           // Nombre de la variable
            struct ASTNode *expression; // Lo que guardaremos en la variable
        } var_decl_expr;

    } ast_node_value;
} ast_node_t;

ast_node_t *create_number_node(char *value, TokenType numeric_type);
ast_node_t *create_binary_node(ast_node_t *left, TokenType operator, ast_node_t * right);
ast_node_t *create_unary_node(TokenType operator, ast_node_t * right);
ast_node_t *create_var_decl_node(TokenType var_type, char *identifier, ast_node_t *expression);