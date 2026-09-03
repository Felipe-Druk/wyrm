#include "scanner.h"

#include <string.h>

#define INITIALCAPACITY 10

#include "../scanner/arigmetic_resolver.h"

token_vector_t* scanner_scan(char* input){
    token_vector_t* tokens = create_token_vector(INITIALCAPACITY);
    if (tokens == NULL) {
        return NULL;
    }

    char* word = malloc(sizeof(char) * (strlen(input) + 1));
    size_t actual_index = 0;
    size_t word_size = 0;
    int temp_index = 0;
    while (input[actual_index] != '\0')
    {
        if (input[actual_index] == ' ' || input[actual_index] == '\n' || input[actual_index] == '\t') {
            actual_index++;
            continue;
        }
        temp_index = resolver_arigmetic_operator(input, actual_index, tokens);
        if (temp_index != -1) {
            actual_index = temp_index;
            continue;
        }
    }
    


    return tokens;
}