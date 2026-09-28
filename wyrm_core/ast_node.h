#pragma once
#include "wyrm_token.h"
#include <stddef.h>
#include <stdbool.h>

typedef enum {
    AST_NUMBER_LITERAL,
    AST_BOOL_LITERAL,
    AST_IDENTIFIER,
    AST_BINARY_EXPR,
    AST_VAR_DECLARATION,
    AST_UNARY_EXPR,
    AST_BLOCK
} ast_node_type_t;

typedef struct ASTNode ast_node_t; // declaración para prevenir recursividad

// Almacena literales numericos
typedef struct {
    char *value; // Bytes del numero {'0','1','2','3','4','5','6','7','8','9','.'} + '\0'
    TokenType numeric_type;
} AstNumberLiteral;

typedef AstNumberLiteral ast_number_literal_t;

// Almacena literales booleanos
typedef struct {
    bool value; // Byte de 0 o 1
} AstBoolLiteral;

typedef AstBoolLiteral ast_bool_literal_t;

// Etiqueta el valor para el valor de variable
typedef struct {
    char *name;
} AstIdentifier;
typedef AstIdentifier ast_identifier_t;

// Operaciones binarios
typedef struct {
    struct ASTNode *left;
    TokenType operator;
    struct ASTNode *right;
} AstBinaryExpr;

typedef AstBinaryExpr ast_binary_expr_t;

// Operaciones unarias com  -5 o ~x
typedef struct {
    TokenType operator;
    struct ASTNode *right;
} AstUnaryExpr;

typedef AstUnaryExpr ast_unary_expr_t;

// Declaraciones de variables
typedef struct {
    TokenType var_type;
    char *identifier;           // Nombre de la variable
    struct ASTNode *expression; // Lo que guardaremos en la variable
    TokenType assign_op;        // Con que operador se isntancio
} AstVarDecl;

typedef AstVarDecl ast_var_decl_t;

typedef struct {
    size_t size;     // Cantidad actual de sentencias (separadas por ";")
    size_t capacity; // Tamañio del "array"
    ast_node_t **nodes;
} AstBlock;

typedef AstBlock ast_block_t;

// Estrutura princiapl del AST, usamos union para "simular" polimorfismo
struct ASTNode {
    ast_node_type_t type;

    union {
        ast_number_literal_t number_expr;

        ast_bool_literal_t bool_expr;

        ast_identifier_t identifier_expr;

        ast_binary_expr_t binary_expr;

        ast_unary_expr_t unary_expr;

        ast_var_decl_t var_decl_expr;

        ast_block_t block_expr;

    } ast_node_value;
};

ast_node_t *create_number_node(char *value, TokenType numeric_type);
ast_node_t *create_bool_node(bool value);
ast_node_t *create_identifier_node(char *value);
ast_node_t *create_binary_node(ast_node_t *left, TokenType operator, ast_node_t * right);
ast_node_t *create_unary_node(TokenType operator, ast_node_t * right);
ast_node_t *create_var_decl_node(TokenType var_type, char *identifier, ast_node_t *expression, TokenType assign_op);
ast_node_t *create_block_node(size_t capacity);

int block_is_full(const ast_block_t *block_node);

void resize_block(ast_block_t *block_node);