#include "environment.h"
#include "stdlib.h"
#include "utils/error.h"
#include <string.h>

#define INITIAL_SCOPE_CAPACITY 5

environment_t *create_environment(environment_t *parent) {
    environment_t *env = calloc(1, sizeof(environment_t));
    if (env == NULL) {
        panic(ERR_OUT_MEMORY, "Fatal error: out of memory of environment");
    }

    env->capacity = INITIAL_SCOPE_CAPACITY;
    env->count = 0;
    env->entries = calloc(env->capacity, sizeof(env_entry_t));
    if (env->entries == NULL) {
        panic(ERR_OUT_MEMORY, "Fatal error: out of memory of entry environment");
    }

    env->parent = parent;
    return env;
}

void environment_set(environment_t *env, const char *name, wyrm_value_t value, TokenType type) {
    for (size_t i = 0; i < env->count; i++) {
        if (strcmp(env->entries[i].name, name) == 0) {
            env->entries[i].value = value;
            env->entries[i].type = type;
            return;
        }
    }

    if (env->count >= env->capacity) {
        env->capacity *= 2;
        env->entries = realloc(env->entries, sizeof(env_entry_t) * env->capacity);
        if (env->entries == NULL) {
            panic(ERR_OUT_MEMORY, "Fatal error: out of realloc environment");
        }
    }
    env->entries[env->count].name = strdup(name);
    env->entries[env->count].value = value;
    env->entries[env->count].type = type;
    env->count++;
}

wyrm_value_t *environment_get(environment_t *env, const char *name) {
    environment_t *current = env;

    while (current != NULL) {
        for (size_t i = 0; i < current->count; i++) {
            if (strcmp(current->entries[i].name, name) == 0) {
                return &current->entries[i].value;
            }
        }
        current = current->parent;
    }

    return NULL;
}

void destroy_environment(environment_t *env) {
    if (env == NULL)
        return;
    for (size_t i = 0; i < env->count; i++) {
        free(env->entries[i].name);
    }
    free(env->entries);
    free(env);
}