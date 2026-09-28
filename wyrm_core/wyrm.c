#include "wyrm.h"
#include "options.h"
#include "scanner/scanner.h"
#include "parser/parser.h"
#include "interpreter/interpreter.h"
#include "utils/error.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

environment_t *global_scope = NULL; // global scope escondido para storage

wyrm_value_t run_wyrm(const char *input, int flags) {

    if (global_scope == NULL) {
        global_scope = create_environment(NULL);
    }

    char *input_copy = malloc(sizeof(char) * (strlen(input) + 1));
    if (input_copy == NULL) {
        panic(1, "Could not allocate memory for the input copy");
    }

    strcpy(input_copy, input);
    scanner_t scanner;
    scanner.flags = flags;

    token_vector_t *tokens = scanner_scan(input_copy, &scanner);
    if (tokens == NULL) {
        free(input_copy);
        panic(1, "Could not create the token vector");
    }

    parser_t *parser = create_parser(tokens);
    parser->flags = flags;
    ast_node_t *root_ast = parser_parse(parser);
    interpreter_t *interpreter = create_interpreter(root_ast, global_scope);
    return interpreter_interpret(interpreter);
}