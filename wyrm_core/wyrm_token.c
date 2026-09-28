#include "wyrm_token.h"

const char *token_type_to_string(TokenType type) {
    // codigo bastante feo, en le futuro intentare mejorarlo.
    switch (type) {
        // tipos
    case T_INT8:
        return "T_INT8";
    case T_INT16:
        return "T_INT16";
    case T_INT32:
        return "T_INT32";
    case T_INT64:
        return "T_INT64";
    case T_NAT8:
        return "T_NAT8";
    case T_NAT16:
        return "T_NAT16";
    case T_NAT32:
        return "T_NAT32";
    case T_NAT64:
        return "T_NAT64";
    case T_FLOAT32:
        return "T_FLOAT32";
    case T_FLOAT64:
        return "T_FLOAT64";
    case T_BOOL:
        return "T_BOOL";
        // sentencias boolenas
    case T_TRUE:
        return "T_TRUE";
    case T_FALSE:
        return "T_FALSE";
    case T_NOT:
        return "T_NOT";
    case T_AND:
        return "T_AND";
    case T_OR:
        return "T_OR";
    case T_XOR:
        return "T_XOR";
        // operadores arigmeticos
    case T_ADD:
        return "T_ADD";
    case T_SUB:
        return "T_SUB";
    case T_MUL:
        return "T_MUL";
    case T_DIV:
        return "T_DIV";
    case T_POW:
        return "T_POW";
    case T_MOD:
        return "T_MOD";
        // operadores logicos
    case T_EQUAL:
        return "T_EQUAL";
    case T_NOT_EQUAL:
        return "T_NOT_EQUAL";
    case T_LESS:
        return "T_LESS";
    case T_GREATER:
        return "T_GREATER";
    case T_LESS_EQUAL:
        return "T_LESS_EQUAL";
    case T_GREATER_EQUAL:
        return "T_GREATER_EQUAL";
    case T_ASSIGN:
        return "T_ASSIGN";
    case T_NUMBER:
        return "T_NUMBER";
    case T_STRING:
        return "T_STRING";
    case T_FLOAT_NUMBER:
        return "T_FLOAT_NUMBER";
    case T_IDENTIFIER:
        return "T_IDENTIFIER";
    case T_EOF:
        return "T_EOF";
    case T_SEMICOLON:
        return "T_SEMICOLON";
    default:
        return "UNKNOWN_TOKEN_TYPE";
    }
}