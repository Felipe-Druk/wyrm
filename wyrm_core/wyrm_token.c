#include "wyrm_token.h"


const char* token_type_to_string(TokenType type){
    // codigo bastante feo, en le futuro intentare mejorarlo.
    switch (type) {
        case T_INT8: return "T_INT8";
        case T_INT16: return "T_INT16";
        case T_INT32: return "T_INT32";
        case T_INT64: return "T_INT64";
        case T_NAT8: return "T_NAT8";
        case T_NAT16: return "T_NAT16";
        case T_NAT32: return "T_NAT32";
        case T_NAT64: return "T_NAT64";
        case T_ADD: return "T_ADD";
        case T_SUB: return "T_SUB";
        case T_MUL: return "T_MUL";
        case T_DIV: return "T_DIV";
        case T_POW: return "T_POW";
        case T_MOD: return "T_MOD";
        case T_ASSIGN: return "T_ASSIGN";
        case T_NUMBER: return "T_NUMBER";
        case T_STRING: return "T_STRING";
        case T_FLOAT_NUMBER: return "T_FLOAT_NUMBER";
        case T_IDENTIFIER: return "T_IDENTIFIER";
        case T_EOF: return "T_EOF";
        case T_SEMICOLON: return "T_SEMICOLON";
        default: return "UNKNOWN_TOKEN_TYPE";
    }
}