#include "scanner.h"
#include "../options.h"
#include "../utils/debuger.h"
#include <string.h> 

#define INITIALCAPACITY 10

#include "arigmetic_resolver.h"
#include "numerit_resolver.h"
#include "types_resolver.h"
#include "identifier_resolver.h"


void debug_tokens(token_vector_t* tokens) {
    print_debug("Current tokens:\n  ");
    for (size_t i = 0; i < tokens->size; i++) {
        print_debug("type: %s  lexeme: %s\n", token_type_to_string(tokens->tokens[i].type), tokens->tokens[i].lexeme);
    }
}

size_t call_resolver(int (*resolver)(char *, int, token_vector_t *), char* input, size_t actual_index, token_vector_t* tokens){
    size_t temp_index = resolver(input, actual_index, tokens);
    return (temp_index != -1 && temp_index != actual_index) ? temp_index : actual_index;
}

token_vector_t* scanner_scan(char* input, scanner_t* scanner) {
    token_vector_t* tokens = create_token_vector(INITIALCAPACITY);
    if (tokens == NULL) {
        return NULL;
    }
    size_t actual_index = 0;
    int temp_index = 0;

    int debug = scanner->flags & (DEBUG_MODE | SCANNER_MODE);
    while (input[actual_index] != '\0')
    {
        if (debug) {
            print_debug("actual_index: %zu, current_char: '%c'\n", actual_index, input[actual_index]);
            debug_tokens(tokens);
        }
        if (input[actual_index] == ' ' || input[actual_index] == '\n' || input[actual_index] == '\t') {
            actual_index++;
            continue;
        }
        actual_index = call_resolver(resolver_arigmetic_operator, input, actual_index, tokens);

        temp_index = resolver_numerit(input, actual_index, tokens);
        if (temp_index != -1 && temp_index != actual_index) {
            actual_index = temp_index;
        }

        temp_index = resolver_types(input, actual_index, tokens);
        if (temp_index != -1 && temp_index != actual_index) {
            actual_index = temp_index;
        }

        // Importante que este al final para no confundir con algun tipo de dato
        temp_index = resolver_identidier(input, actual_index, tokens);
            if (temp_index != -1 && temp_index != actual_index) {
            actual_index = temp_index;
            continue;
        }
        actual_index++;
    }
    
    if(debug){
        print_debug("== End Scanner ==\n");
        debug_tokens(tokens);
    }
    scanner->tokens = tokens;
    return tokens;
}