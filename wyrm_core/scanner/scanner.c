#include "scanner.h"
#include "../options.h"
#include "../utils/debuger.h"
#include <string.h> 

#define INITIALCAPACITY 10

#include "../scanner/arigmetic_resolver.h"
#include "../scanner/numerit_resolver.h"

void debug_tokens(token_vector_t* tokens) {
    print_debug("Current tokens: ");
    for (size_t i = 0; i < tokens->size; i++) {
        print_debug("%s \n", tokens->tokens[i].lexeme);
    }
}

token_vector_t* scanner_scan(char* input, scanner_t* scanner) {
    token_vector_t* tokens = create_token_vector(INITIALCAPACITY);
    if (tokens == NULL) {
        return NULL;
    }
    char* word = malloc(sizeof(char) * (strlen(input) + 1));
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
        temp_index = resolver_arigmetic_operator(input, actual_index, tokens);
        if (temp_index != -1) {
            actual_index = temp_index;
        }
        temp_index = resolver_numerit(input, actual_index, tokens);
        if (temp_index != -1 && temp_index != actual_index) {
            actual_index = temp_index;
            continue;
        }
        actual_index++;
    }
    

    scanner->tokens = tokens;
    return tokens;
}