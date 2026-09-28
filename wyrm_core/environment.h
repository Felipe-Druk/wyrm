#pragma once

#include "wyrm_token.h"
#include "wyrm_value.h"
#include <stddef.h>

// las variable del scope
typedef struct {
    char *name;         // El nombre de la variable
    wyrm_value_t value; // El valor enccerado en un wyrm_value
    TokenType type;     // El tipo de la misma
} EnvEntry;

typedef EnvEntry env_entry_t;

// escope que guarda las entradas de las variables
typedef struct Environment {
    env_entry_t *entries;
    size_t count;
    size_t capacity;
    struct Environment *parent;
} environment_t; // como es autoreferencial el typedef es compacto

environment_t *create_environment(environment_t *parent);

// carga una nueva variable o la reescribe
void environment_set(environment_t *env, const char *name, wyrm_value_t value, TokenType type);

// devuelve un puntero al valor de la variable, o NULL si no existe
wyrm_value_t *environment_get(environment_t *env, const char *name);

// carga el valor en una variable ya existente, sino devuelve 0
int environment_assign(environment_t *env, const char *name, wyrm_value_t value);

// limpia el scope
void destroy_environment(environment_t *env);
