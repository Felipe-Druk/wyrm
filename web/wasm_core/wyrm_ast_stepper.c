#include <inttypes.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifdef __EMSCRIPTEN__
#include <emscripten/emscripten.h>
#else
#define EMSCRIPTEN_KEEPALIVE
#endif

#include "interpreter/evaluator/evaluator_binary.h"
#include "interpreter/evaluator/evaluator_bool.h"
#include "interpreter/evaluator/evaluator_function.h"
#include "interpreter/evaluator/evaluator_identifier.h"
#include "interpreter/evaluator/evaluator_number.h"
#include "interpreter/evaluator/evaluator_unary.h"
#include "parser/parser.h"
#include "scanner/scanner.h"
#include "utils/error.h"
#include "utils/wyrm_vector.h"
#include "wyrm_token.h"
#include "wyrm_types/ast/ast_node.h"
#include "wyrm_types/environment/environment.h"
#include "wyrm_types/wyrm_value/wyrm_value.h"
#include "wyrm_wasm_internal.h"

#define AST_STEPPER_DEFAULT_MAX_STEPS 1500
#define AST_STEPPER_HARD_MAX_STEPS 20000
#define AST_STEPPER_INIT_JSON_CAP 32768
#define AST_STEPPER_MAX_DEPTH 256
#define AST_STEPPER_MAX_SCOPES 256

typedef struct {
    char *data;
    size_t len;
    size_t cap;
    bool oom;
} ast_jbuf_t;

typedef struct {
    const ast_node_t *ptr;
    int id;
    int parent_id;
    char edge[24];
    char kind[24];
    char label[96];
    char detail[96];
    int *children;
    size_t child_count;
    size_t child_cap;
} ast_node_rec_t;

typedef struct {
    environment_t *env;
    char label[64];
} ast_scope_frame_t;

typedef struct {
    ast_node_rec_t *nodes;
    size_t node_count;
    size_t node_cap;

    ast_jbuf_t jb;
    int step_count;
    int max_steps;
    bool truncated;

    int active_path[AST_STEPPER_MAX_DEPTH];
    size_t active_depth;

    ast_scope_frame_t scopes[AST_STEPPER_MAX_SCOPES];
    size_t scope_count;
    int block_seq;
} ast_trace_ctx_t;

static char *g_ast_trace_json = NULL;

static void jbuf_init(ast_jbuf_t *jb, size_t initial_cap) {
    jb->data = (char *)malloc(initial_cap);
    if (jb->data == NULL) {
        jb->len = 0;
        jb->cap = 0;
        jb->oom = true;
        return;
    }
    jb->data[0] = '\0';
    jb->len = 0;
    jb->cap = initial_cap;
    jb->oom = false;
}

static void jbuf_free(ast_jbuf_t *jb) {
    free(jb->data);
    jb->data = NULL;
    jb->len = 0;
    jb->cap = 0;
}

static bool jbuf_reserve(ast_jbuf_t *jb, size_t extra) {
    if (jb->oom) {
        return false;
    }
    size_t needed = jb->len + extra + 1;
    if (needed <= jb->cap) {
        return true;
    }
    size_t next_cap = jb->cap > 0 ? jb->cap : AST_STEPPER_INIT_JSON_CAP;
    while (next_cap < needed) {
        if (next_cap > (SIZE_MAX / 2)) {
            jb->oom = true;
            return false;
        }
        next_cap *= 2;
    }
    char *resized = (char *)realloc(jb->data, next_cap);
    if (resized == NULL) {
        jb->oom = true;
        return false;
    }
    jb->data = resized;
    jb->cap = next_cap;
    return true;
}

static void jbuf_append_raw(ast_jbuf_t *jb, const char *bytes, size_t n) {
    if (n == 0 || !jbuf_reserve(jb, n)) {
        return;
    }
    memcpy(jb->data + jb->len, bytes, n);
    jb->len += n;
    jb->data[jb->len] = '\0';
}

static void jbuf_append_str(ast_jbuf_t *jb, const char *s) {
    if (s == NULL) {
        return;
    }
    jbuf_append_raw(jb, s, strlen(s));
}

static void jbuf_append_char(ast_jbuf_t *jb, char c) {
    jbuf_append_raw(jb, &c, 1);
}

static void jbuf_append_fmt(ast_jbuf_t *jb, const char *fmt, ...) {
    if (jb->oom) {
        return;
    }
    va_list args;
    va_start(args, fmt);
    va_list args_copy;
    va_copy(args_copy, args);
    int needed = vsnprintf(NULL, 0, fmt, args);
    va_end(args);

    if (needed <= 0) {
        va_end(args_copy);
        return;
    }
    if (!jbuf_reserve(jb, (size_t)needed)) {
        va_end(args_copy);
        return;
    }
    vsnprintf(jb->data + jb->len, jb->cap - jb->len, fmt, args_copy);
    va_end(args_copy);
    jb->len += (size_t)needed;
}

static void jbuf_append_json_string(ast_jbuf_t *jb, const char *s) {
    jbuf_append_char(jb, '"');
    if (s != NULL) {
        for (const unsigned char *p = (const unsigned char *)s; *p != '\0'; p++) {
            unsigned char ch = *p;
            switch (ch) {
            case '"':
                jbuf_append_raw(jb, "\\\"", 2);
                break;
            case '\\':
                jbuf_append_raw(jb, "\\\\", 2);
                break;
            case '\b':
                jbuf_append_raw(jb, "\\b", 2);
                break;
            case '\f':
                jbuf_append_raw(jb, "\\f", 2);
                break;
            case '\n':
                jbuf_append_raw(jb, "\\n", 2);
                break;
            case '\r':
                jbuf_append_raw(jb, "\\r", 2);
                break;
            case '\t':
                jbuf_append_raw(jb, "\\t", 2);
                break;
            default:
                if (ch < 0x20) {
                    jbuf_append_fmt(jb, "\\u%04x", (unsigned int)ch);
                } else {
                    jbuf_append_char(jb, (char)ch);
                }
                break;
            }
        }
    }
    jbuf_append_char(jb, '"');
}

static const char *token_op_symbol(token_type_t op) {
    switch (op) {
    case T_ADD:
        return "+";
    case T_SUB:
        return "-";
    case T_MUL:
        return "*";
    case T_DIV:
        return "/";
    case T_MOD:
        return "%";
    case T_POW:
        return "^";
    case T_AND:
        return "and";
    case T_OR:
        return "or";
    case T_XOR:
        return "xor";
    case T_NOT:
        return "not";
    case T_EQUAL:
        return "==";
    case T_NOT_EQUAL:
        return "~=";
    case T_LESS:
        return "<";
    case T_LESS_EQUAL:
        return "<=";
    case T_GREATER:
        return ">";
    case T_GREATER_EQUAL:
        return ">=";
    case T_ASSIGN:
        return "=";
    case T_LARROW:
        return "<-";
    default:
        return "?";
    }
}

static void format_decl_type(token_type_t var_type, token_type_t elem_type, char *buf, size_t cap) {
    if (var_type == T_VEC) {
        wyrm_value_type_t et = token_to_value_type(elem_type);
        snprintf(buf, cap, "vec<%s>", wyrm_web_type_name(et));
        return;
    }
    snprintf(buf, cap, "%s", wyrm_web_type_name(token_to_value_type(var_type)));
}

static bool is_ast_string_literal(const ast_node_t *node) {
    if (node == NULL || node->type != AST_VEC_LITERAL) {
        return false;
    }
    ast_block_t blk = node->ast_node_value.block_expr;
    if (blk.size < 2 || blk.nodes == NULL) {
        return false;
    }
    const ast_node_t *last = blk.nodes[blk.size - 1];
    if (last == NULL || last->type != AST_NUMBER_LITERAL ||
        last->ast_node_value.number_expr.numeric_type != T_NUMBER ||
        last->ast_node_value.number_expr.value == NULL ||
        strcmp(last->ast_node_value.number_expr.value, "0") != 0) {
        return false;
    }
    for (size_t i = 0; i + 1 < blk.size; i++) {
        const ast_node_t *el = blk.nodes[i];
        if (el == NULL || el->type != AST_NUMBER_LITERAL ||
            el->ast_node_value.number_expr.numeric_type != T_NUMBER ||
            el->ast_node_value.number_expr.value == NULL) {
            return false;
        }
        long byte = strtol(el->ast_node_value.number_expr.value, NULL, 10);
        bool printable = (byte >= 32 && byte <= 126) || byte == '\n' || byte == '\t' || byte == '\r';
        if (!printable) {
            return false;
        }
    }
    return true;
}

static void format_ast_string_preview(const ast_node_t *node, char *buf, size_t cap) {
    if (cap == 0) {
        return;
    }
    ast_block_t blk = node->ast_node_value.block_expr;
    size_t content_len = blk.size > 0 ? blk.size - 1 : 0;
    size_t pos = 0;
    if (pos + 1 < cap) {
        buf[pos++] = '"';
    }
    for (size_t i = 0; i < content_len && pos + 6 < cap; i++) {
        long byte = strtol(blk.nodes[i]->ast_node_value.number_expr.value, NULL, 10);
        if (byte == '\n') {
            buf[pos++] = '\\';
            buf[pos++] = 'n';
        } else if (byte == '\t') {
            buf[pos++] = '\\';
            buf[pos++] = 't';
        } else if (byte == '\r') {
            buf[pos++] = '\\';
            buf[pos++] = 'r';
        } else if (byte == '"' || byte == '\\') {
            buf[pos++] = '\\';
            buf[pos++] = (char)byte;
        } else {
            buf[pos++] = (char)byte;
        }
    }
    if (content_len > 0 && pos + 6 >= cap && pos + 4 < cap) {
        buf[pos++] = '.';
        buf[pos++] = '.';
        buf[pos++] = '.';
    }
    if (pos + 1 < cap) {
        buf[pos++] = '"';
    }
    buf[pos] = '\0';
}

static int alloc_node_rec(ast_trace_ctx_t *ctx, const ast_node_t *ptr, int parent_id, const char *edge) {
    if (ctx->node_count >= ctx->node_cap) {
        size_t next_cap = ctx->node_cap == 0 ? 64 : ctx->node_cap * 2;
        ast_node_rec_t *grown = (ast_node_rec_t *)realloc(ctx->nodes, next_cap * sizeof(ast_node_rec_t));
        if (grown == NULL) {
            ctx->jb.oom = true;
            return -1;
        }
        ctx->nodes = grown;
        ctx->node_cap = next_cap;
    }

    int id = (int)ctx->node_count++;
    ast_node_rec_t *rec = &ctx->nodes[id];
    memset(rec, 0, sizeof(*rec));
    rec->ptr = ptr;
    rec->id = id;
    rec->parent_id = parent_id;
    snprintf(rec->edge, sizeof(rec->edge), "%s", edge != NULL ? edge : "");
    return id;
}

static void add_child_id(ast_trace_ctx_t *ctx, int parent_id, int child_id) {
    if (parent_id < 0 || child_id < 0) {
        return;
    }
    ast_node_rec_t *parent = &ctx->nodes[parent_id];
    if (parent->child_count >= parent->child_cap) {
        size_t next_cap = parent->child_cap == 0 ? 4 : parent->child_cap * 2;
        int *grown = (int *)realloc(parent->children, next_cap * sizeof(int));
        if (grown == NULL) {
            ctx->jb.oom = true;
            return;
        }
        parent->children = grown;
        parent->child_cap = next_cap;
    }
    parent->children[parent->child_count++] = child_id;
}

static int lookup_node_id(const ast_trace_ctx_t *ctx, const ast_node_t *ptr) {
    if (ptr == NULL) {
        return -1;
    }
    for (size_t i = 0; i < ctx->node_count; i++) {
        if (ctx->nodes[i].ptr == ptr) {
            return ctx->nodes[i].id;
        }
    }
    return -1;
}

static int index_ast_tree(ast_trace_ctx_t *ctx, const ast_node_t *node, int parent_id, const char *edge) {
    if (node == NULL) {
        return -1;
    }

    int id = alloc_node_rec(ctx, node, parent_id, edge);
    if (id < 0) {
        return -1;
    }
    if (parent_id >= 0) {
        add_child_id(ctx, parent_id, id);
    }

    // Llenar kind / label / detail antes de recurrir (el arreglo puede reubicarse al agregar hijos)
    char kind[24];
    char label[96];
    char detail[96];
    kind[0] = '\0';
    label[0] = '\0';
    detail[0] = '\0';

    switch (node->type) {
    case AST_PROGRAM:
        snprintf(kind, sizeof(kind), "Program");
        snprintf(label, sizeof(label), "Program");
        snprintf(detail, sizeof(detail), "%zu stmts", node->ast_node_value.block_expr.size);
        break;
    case AST_BLOCK:
        snprintf(kind, sizeof(kind), "Block");
        snprintf(label, sizeof(label), "{ ... }");
        snprintf(detail, sizeof(detail), "%zu stmts", node->ast_node_value.block_expr.size);
        break;
    case AST_NUMBER_LITERAL:
        snprintf(kind, sizeof(kind), "Number");
        snprintf(
            label,
            sizeof(label),
            "%s",
            node->ast_node_value.number_expr.value != NULL ? node->ast_node_value.number_expr.value : "0"
        );
        snprintf(
            detail,
            sizeof(detail),
            "%s",
            node->ast_node_value.number_expr.numeric_type == T_FLOAT_NUMBER ? "float64" : "int64"
        );
        break;
    case AST_BOOL_LITERAL:
        snprintf(kind, sizeof(kind), "Bool");
        snprintf(label, sizeof(label), "%s", node->ast_node_value.bool_expr.value ? "true" : "false");
        snprintf(detail, sizeof(detail), "bool");
        break;
    case AST_IDENTIFIER:
        snprintf(kind, sizeof(kind), "Ident");
        snprintf(
            label,
            sizeof(label),
            "%s",
            node->ast_node_value.identifier_expr.name != NULL ? node->ast_node_value.identifier_expr.name : "?"
        );
        snprintf(detail, sizeof(detail), "var");
        break;
    case AST_BINARY_EXPR:
        snprintf(kind, sizeof(kind), "Binary");
        snprintf(label, sizeof(label), "%s", token_op_symbol(node->ast_node_value.binary_expr.operator));
        snprintf(detail, sizeof(detail), "binop");
        break;
    case AST_UNARY_EXPR:
        snprintf(kind, sizeof(kind), "Unary");
        snprintf(label, sizeof(label), "%s", token_op_symbol(node->ast_node_value.unary_expr.operator));
        snprintf(detail, sizeof(detail), "unop");
        break;
    case AST_VAR_DECLARATION: {
        ast_var_decl_t decl = node->ast_node_value.var_decl_expr;
        snprintf(kind, sizeof(kind), "VarDecl");
        snprintf(label, sizeof(label), "%s", decl.identifier != NULL ? decl.identifier : "?");
        format_decl_type(decl.var_type, decl.element_type, detail, sizeof(detail));
        break;
    }
    case AST_ASSIGNMENT: {
        ast_assignment_t asg = node->ast_node_value.assignment_expr;
        snprintf(kind, sizeof(kind), "Assign");
        if (asg.index != NULL) {
            snprintf(label, sizeof(label), "%s[...]", asg.name != NULL ? asg.name : "?");
        } else {
            snprintf(label, sizeof(label), "%s", asg.name != NULL ? asg.name : "?");
        }
        snprintf(detail, sizeof(detail), "%s", token_op_symbol(asg.operator));
        break;
    }
    case AST_IF_EXPR:
        snprintf(kind, sizeof(kind), "If");
        snprintf(label, sizeof(label), "if");
        snprintf(
            detail,
            sizeof(detail),
            "%s",
            node->ast_node_value.if_expr.else_branch != NULL ? "if / else" : "if"
        );
        break;
    case AST_WHILE_EXPR:
        snprintf(kind, sizeof(kind), "While");
        snprintf(label, sizeof(label), "while");
        snprintf(detail, sizeof(detail), "loop");
        break;
    case AST_FOR_EXPR: {
        const ast_node_t *hdr = node->ast_node_value.while_expr.condition;
        const char *iter_name = (hdr != NULL && hdr->type == AST_ASSIGNMENT &&
                                 hdr->ast_node_value.assignment_expr.name != NULL)
            ? hdr->ast_node_value.assignment_expr.name
            : "item";
        snprintf(kind, sizeof(kind), "For");
        snprintf(label, sizeof(label), "for (%s <- ...)", iter_name);
        snprintf(detail, sizeof(detail), "iter");
        break;
    }
    case AST_FUNCTION_DECL: {
        ast_function_decl_t fdecl = node->ast_node_value.function_decl;
        snprintf(kind, sizeof(kind), "FnDecl");
        snprintf(label, sizeof(label), "%s", fdecl.name != NULL ? fdecl.name : "fn");
        snprintf(
            detail,
            sizeof(detail),
            "(%zu) -> %s",
            fdecl.function.arg_count,
            wyrm_web_type_name(fdecl.function.returned)
        );
        break;
    }
    case AST_CALL: {
        ast_call_expr_t call = node->ast_node_value.call_expr;
        const char *cname = call.name != NULL ? call.name : "call";
        bool is_builtin = (strcmp(cname, "write") == 0 || strcmp(cname, "read") == 0);
        snprintf(kind, sizeof(kind), "Call");
        snprintf(label, sizeof(label), "%s()", cname);
        snprintf(detail, sizeof(detail), "%s · %zu arg", is_builtin ? "builtin" : "fn", call.arg_count);
        break;
    }
    case AST_VEC_LITERAL:
        if (is_ast_string_literal(node)) {
            snprintf(kind, sizeof(kind), "StringLit");
            format_ast_string_preview(node, label, sizeof(label));
            snprintf(detail, sizeof(detail), "vec<nat8> (%zu B)", node->ast_node_value.block_expr.size);
        } else {
            snprintf(kind, sizeof(kind), "VecLit");
            snprintf(label, sizeof(label), "[ ... ]");
            snprintf(detail, sizeof(detail), "%zu elems", node->ast_node_value.block_expr.size);
        }
        break;
    case AST_VEC_ACCESS:
        snprintf(kind, sizeof(kind), "Index");
        snprintf(label, sizeof(label), "vec[i]");
        snprintf(detail, sizeof(detail), "access");
        break;
    default:
        snprintf(kind, sizeof(kind), "Node");
        snprintf(label, sizeof(label), "unknown");
        detail[0] = '\0';
        break;
    }

    snprintf(ctx->nodes[id].kind, sizeof(ctx->nodes[id].kind), "%s", kind);
    snprintf(ctx->nodes[id].label, sizeof(ctx->nodes[id].label), "%s", label);
    snprintf(ctx->nodes[id].detail, sizeof(ctx->nodes[id].detail), "%s", detail);

    char edge_buf[40];
    switch (node->type) {
    case AST_PROGRAM:
    case AST_BLOCK: {
        ast_block_t blk = node->ast_node_value.block_expr;
        for (size_t i = 0; i < blk.size; i++) {
            snprintf(edge_buf, sizeof(edge_buf), "stmt[%zu]", i);
            index_ast_tree(ctx, blk.nodes[i], id, edge_buf);
        }
        break;
    }
    case AST_BINARY_EXPR:
        index_ast_tree(ctx, node->ast_node_value.binary_expr.left, id, "left");
        index_ast_tree(ctx, node->ast_node_value.binary_expr.right, id, "right");
        break;
    case AST_UNARY_EXPR:
        index_ast_tree(ctx, node->ast_node_value.unary_expr.right, id, "expr");
        break;
    case AST_VAR_DECLARATION:
        if (node->ast_node_value.var_decl_expr.expression != NULL) {
            index_ast_tree(ctx, node->ast_node_value.var_decl_expr.expression, id, "init");
        }
        break;
    case AST_ASSIGNMENT:
        if (node->ast_node_value.assignment_expr.index != NULL) {
            index_ast_tree(ctx, node->ast_node_value.assignment_expr.index, id, "index");
        }
        index_ast_tree(ctx, node->ast_node_value.assignment_expr.value, id, "expr");
        break;
    case AST_IF_EXPR:
        index_ast_tree(ctx, node->ast_node_value.if_expr.condition, id, "cond");
        index_ast_tree(ctx, node->ast_node_value.if_expr.then_branch, id, "then");
        if (node->ast_node_value.if_expr.else_branch != NULL) {
            index_ast_tree(ctx, node->ast_node_value.if_expr.else_branch, id, "else");
        }
        break;
    case AST_WHILE_EXPR:
        index_ast_tree(ctx, node->ast_node_value.while_expr.condition, id, "cond");
        index_ast_tree(ctx, node->ast_node_value.while_expr.body, id, "body");
        break;
    case AST_FOR_EXPR: {
        const ast_node_t *hdr = node->ast_node_value.while_expr.condition;
        if (hdr != NULL && hdr->type == AST_ASSIGNMENT) {
            index_ast_tree(ctx, hdr->ast_node_value.assignment_expr.value, id, "vec");
        }
        index_ast_tree(ctx, node->ast_node_value.while_expr.body, id, "body");
        break;
    }
    case AST_FUNCTION_DECL:
        if (node->ast_node_value.function_decl.function.body != NULL) {
            index_ast_tree(ctx, node->ast_node_value.function_decl.function.body, id, "body");
        }
        break;
    case AST_CALL: {
        ast_call_expr_t call = node->ast_node_value.call_expr;
        for (size_t i = 0; i < call.arg_count; i++) {
            snprintf(edge_buf, sizeof(edge_buf), "arg[%zu]", i);
            index_ast_tree(ctx, call.args[i], id, edge_buf);
        }
        break;
    }
    case AST_VEC_LITERAL:
        if (!is_ast_string_literal(node)) {
            ast_block_t blk = node->ast_node_value.block_expr;
            for (size_t i = 0; i < blk.size; i++) {
                snprintf(edge_buf, sizeof(edge_buf), "elem[%zu]", i);
                index_ast_tree(ctx, blk.nodes[i], id, edge_buf);
            }
        }
        break;
    case AST_VEC_ACCESS:
        index_ast_tree(ctx, node->ast_node_value.binary_expr.left, id, "vec");
        index_ast_tree(ctx, node->ast_node_value.binary_expr.right, id, "index");
        break;
    default:
        break;
    }

    return id;
}

static void emit_static_nodes_json(ast_trace_ctx_t *ctx) {
    jbuf_append_str(&ctx->jb, "\"root_id\":0,\"nodes\":[");
    for (size_t i = 0; i < ctx->node_count; i++) {
        const ast_node_rec_t *rec = &ctx->nodes[i];
        if (i > 0) {
            jbuf_append_char(&ctx->jb, ',');
        }
        jbuf_append_fmt(&ctx->jb, "{\"id\":%d,\"parent_id\":", rec->id);
        if (rec->parent_id >= 0) {
            jbuf_append_fmt(&ctx->jb, "%d", rec->parent_id);
        } else {
            jbuf_append_str(&ctx->jb, "null");
        }
        jbuf_append_str(&ctx->jb, ",\"edge\":");
        jbuf_append_json_string(&ctx->jb, rec->edge);
        jbuf_append_str(&ctx->jb, ",\"kind\":");
        jbuf_append_json_string(&ctx->jb, rec->kind);
        jbuf_append_str(&ctx->jb, ",\"label\":");
        jbuf_append_json_string(&ctx->jb, rec->label);
        jbuf_append_str(&ctx->jb, ",\"detail\":");
        jbuf_append_json_string(&ctx->jb, rec->detail);
        jbuf_append_str(&ctx->jb, ",\"children\":[");
        for (size_t c = 0; c < rec->child_count; c++) {
            if (c > 0) {
                jbuf_append_char(&ctx->jb, ',');
            }
            jbuf_append_fmt(&ctx->jb, "%d", rec->children[c]);
        }
        jbuf_append_str(&ctx->jb, "]}");
    }
    jbuf_append_str(&ctx->jb, "],");
}

static void push_scope(ast_trace_ctx_t *ctx, environment_t *env, const char *label) {
    if (ctx->scope_count < AST_STEPPER_MAX_SCOPES) {
        ctx->scopes[ctx->scope_count].env = env;
        snprintf(ctx->scopes[ctx->scope_count].label, sizeof(ctx->scopes[ctx->scope_count].label), "%s", label);
        ctx->scope_count++;
    }
}

static void pop_scope(ast_trace_ctx_t *ctx) {
    if (ctx->scope_count > 0) {
        ctx->scope_count--;
    }
}

static void emit_scopes_json(ast_trace_ctx_t *ctx) {
    char type_buf[128];
    char val_buf[512];
    jbuf_append_char(&ctx->jb, '[');
    for (size_t s = 0; s < ctx->scope_count; s++) {
        if (s > 0) {
            jbuf_append_char(&ctx->jb, ',');
        }
        const ast_scope_frame_t *sf = &ctx->scopes[s];
        jbuf_append_fmt(&ctx->jb, "{\"depth\":%zu,\"label\":", s);
        jbuf_append_json_string(&ctx->jb, sf->label);
        jbuf_append_str(&ctx->jb, ",\"vars\":[");
        if (sf->env != NULL) {
            for (size_t e = 0; e < sf->env->count; e++) {
                if (e > 0) {
                    jbuf_append_char(&ctx->jb, ',');
                }
                const env_entry_t *ent = &sf->env->entries[e];
                wyrm_web_format_type(ent->value, type_buf, sizeof(type_buf));
                wyrm_web_format_value(ent->value, val_buf, sizeof(val_buf));
                jbuf_append_str(&ctx->jb, "{\"name\":");
                jbuf_append_json_string(&ctx->jb, ent->name != NULL ? ent->name : "");
                jbuf_append_str(&ctx->jb, ",\"type\":");
                jbuf_append_json_string(&ctx->jb, type_buf);
                jbuf_append_str(&ctx->jb, ",\"value\":");
                jbuf_append_json_string(&ctx->jb, val_buf);
                jbuf_append_char(&ctx->jb, '}');
            }
        }
        jbuf_append_str(&ctx->jb, "]}");
    }
    jbuf_append_char(&ctx->jb, ']');
}

static bool emit_ast_step(
    ast_trace_ctx_t *ctx,
    int node_id,
    const char *phase,
    const wyrm_value_t *result_val,
    const char *note
) {
    if (ctx->truncated) {
        return false;
    }
    if (ctx->step_count >= ctx->max_steps) {
        ctx->truncated = true;
        return false;
    }

    if (ctx->step_count > 0) {
        jbuf_append_char(&ctx->jb, ',');
    }

    int step_num = ctx->step_count++;
    size_t depth = ctx->active_depth > 0 ? ctx->active_depth - 1 : 0;

    jbuf_append_fmt(&ctx->jb, "{\"step\":%d,\"node_id\":%d,\"phase\":", step_num, node_id);
    jbuf_append_json_string(&ctx->jb, phase);
    jbuf_append_fmt(&ctx->jb, ",\"depth\":%zu,\"active_path\":[", depth);
    for (size_t i = 0; i < ctx->active_depth; i++) {
        if (i > 0) {
            jbuf_append_char(&ctx->jb, ',');
        }
        jbuf_append_fmt(&ctx->jb, "%d", ctx->active_path[i]);
    }
    jbuf_append_str(&ctx->jb, "],\"result_type\":");
    if (result_val != NULL) {
        char type_buf[128];
        char val_buf[512];
        wyrm_web_format_type(*result_val, type_buf, sizeof(type_buf));
        wyrm_web_format_value(*result_val, val_buf, sizeof(val_buf));
        jbuf_append_json_string(&ctx->jb, type_buf);
        jbuf_append_str(&ctx->jb, ",\"result_value\":");
        jbuf_append_json_string(&ctx->jb, val_buf);
    } else {
        jbuf_append_str(&ctx->jb, "null,\"result_value\":null");
    }
    jbuf_append_str(&ctx->jb, ",\"note\":");
    jbuf_append_json_string(&ctx->jb, note != NULL ? note : "");
    jbuf_append_str(&ctx->jb, ",\"scopes\":");
    emit_scopes_json(ctx);
    jbuf_append_char(&ctx->jb, '}');
    return true;
}

static wyrm_value_t trace_eval_node(ast_trace_ctx_t *ctx, ast_node_t *node, environment_t *scope);

static wyrm_value_t trace_eval_native_read(
    ast_trace_ctx_t *ctx,
    size_t arg_count,
    ast_node_t **args,
    environment_t *scope
) {
    if (arg_count != 2) {
        panic(ERR_TYPE, "Runtime error: 'read' expects 2 arguments (fd, count), got %zu", arg_count);
    }

    wyrm_value_t fd_val = trace_eval_node(ctx, args[0], scope);
    if (ctx->truncated) {
        return (wyrm_value_t){.type = VAL_VOID};
    }
    wyrm_value_t count_val = trace_eval_node(ctx, args[1], scope);
    if (ctx->truncated) {
        return (wyrm_value_t){.type = VAL_VOID};
    }

    int fd = (int)get_cast_int64(fd_val);
    int64_t count = get_cast_int64(count_val);
    if (count < 0) {
        panic(ERR_OVERFLOW, "Runtime error: 'read' byte count cannot be negative");
    }

    uint8_t *raw_buf = NULL;
    ssize_t bytes_read = 0;
    if (count > 0) {
        raw_buf = (uint8_t *)malloc((size_t)count);
        if (raw_buf == NULL) {
            panic(ERR_OUT_MEMORY, "Could not allocate buffer for 'read'");
        }
        bytes_read = read(fd, raw_buf, (size_t)count);
        if (bytes_read < 0) {
            bytes_read = 0;
        }
    }

    size_t initial_cap = (bytes_read > 0) ? (size_t)bytes_read : 1;
    wyrm_vec_t *vec = create_wyrm_vec(VAL_NAT8, initial_cap);
    for (ssize_t i = 0; i < bytes_read; i++) {
        wyrm_value_t item = {.type = VAL_NAT8, .value.nat8_val = raw_buf[i]};
        vector_push(&vec->items, &item);
    }
    free(raw_buf);

    return (wyrm_value_t){.type = VAL_VEC, .value.vec_val = vec};
}

static wyrm_value_t trace_eval_native_write(
    ast_trace_ctx_t *ctx,
    size_t arg_count,
    ast_node_t **args,
    environment_t *scope
) {
    if (arg_count != 2) {
        panic(ERR_TYPE, "Runtime error: 'write' expects 2 arguments (fd, buffer), got %zu", arg_count);
    }

    wyrm_value_t fd_val = trace_eval_node(ctx, args[0], scope);
    if (ctx->truncated) {
        return (wyrm_value_t){.type = VAL_VOID};
    }
    wyrm_value_t buf_val = trace_eval_node(ctx, args[1], scope);
    if (ctx->truncated) {
        return (wyrm_value_t){.type = VAL_VOID};
    }

    if (buf_val.type != VAL_VEC || buf_val.value.vec_val == NULL) {
        panic(ERR_TYPE, "Runtime error: Second argument to 'write' must be a vector");
    }

    int fd = (int)get_cast_int64(fd_val);
    wyrm_vec_t *vec = buf_val.value.vec_val;
    size_t len = vec->items.size;

    uint8_t *raw_buf = NULL;
    if (len > 0) {
        raw_buf = (uint8_t *)malloc(len);
        for (size_t i = 0; i < len; i++) {
            wyrm_value_t item = *(wyrm_value_t *)vector_get(&vec->items, i);
            raw_buf[i] = (uint8_t)get_cast_int64(item);
        }
        if (raw_buf[len - 1] == 0) {
            len--;
        }
    }

    ssize_t written = (len > 0) ? write(fd, raw_buf, len) : 0;
    free(raw_buf);
    if (fd == 1) {
        fflush(stdout);
    }

    return (wyrm_value_t){.type = VAL_INT64, .value.int64_val = (int64_t)written};
}

static wyrm_value_t trace_eval_node(ast_trace_ctx_t *ctx, ast_node_t *node, environment_t *scope) {
    wyrm_value_t result = {.type = VAL_VOID};
    if (node == NULL || ctx->truncated) {
        return result;
    }

    int node_id = lookup_node_id(ctx, node);
    bool pushed_path = false;
    if (node_id >= 0 && ctx->active_depth < AST_STEPPER_MAX_DEPTH) {
        ctx->active_path[ctx->active_depth++] = node_id;
        pushed_path = true;
    }

    char note_buf[512];
    char lhs_buf[96];
    char rhs_buf[96];
    char res_buf[128];
    char type_buf[64];

    switch (node->type) {
    case AST_NUMBER_LITERAL: {
        result = eval_number(node);
        wyrm_web_format_value(result, res_buf, sizeof(res_buf));
        snprintf(note_buf, sizeof(note_buf), "Literal numérico => %s", res_buf);
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    case AST_BOOL_LITERAL: {
        result = eval_bool(node);
        snprintf(note_buf, sizeof(note_buf), "Literal booleano => %s", result.value.bool_val ? "true" : "false");
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    case AST_IDENTIFIER: {
        const char *name = node->ast_node_value.identifier_expr.name;
        result = eval_identifier(node, scope);
        wyrm_web_format_value(result, res_buf, sizeof(res_buf));
        snprintf(note_buf, sizeof(note_buf), "Lee '%s' => %s", name != NULL ? name : "?", res_buf);
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    case AST_PROGRAM: {
        ast_block_t blk = node->ast_node_value.block_expr;
        snprintf(note_buf, sizeof(note_buf), "Inicia evaluación del programa (%zu sentencias)", blk.size);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }
        for (size_t i = 0; i < blk.size && !ctx->truncated; i++) {
            result = trace_eval_node(ctx, blk.nodes[i], scope);
        }
        if (!ctx->truncated) {
            wyrm_web_format_value(result, res_buf, sizeof(res_buf));
            snprintf(note_buf, sizeof(note_buf), "Fin de programa => %s", res_buf);
            emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        }
        break;
    }
    case AST_BLOCK: {
        ast_block_t blk = node->ast_node_value.block_expr;
        environment_t *local_scope = create_environment(scope);
        char scope_label[64];
        snprintf(scope_label, sizeof(scope_label), "BLOCK #%d", ++ctx->block_seq);
        push_scope(ctx, local_scope, scope_label);

        snprintf(note_buf, sizeof(note_buf), "Abre ámbito %s (%zu sentencias)", scope_label, blk.size);
        if (emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            for (size_t i = 0; i < blk.size && !ctx->truncated; i++) {
                result = trace_eval_node(ctx, blk.nodes[i], local_scope);
            }
            if (!ctx->truncated) {
                wyrm_web_format_value(result, res_buf, sizeof(res_buf));
                snprintf(note_buf, sizeof(note_buf), "Cierra %s => %s", scope_label, res_buf);
                emit_ast_step(ctx, node_id, "exit", &result, note_buf);
            }
        }

        pop_scope(ctx);
        destroy_environment(local_scope);
        break;
    }
    case AST_BINARY_EXPR: {
        const char *op_sym = token_op_symbol(node->ast_node_value.binary_expr.operator);
        snprintf(note_buf, sizeof(note_buf), "Evalúa operación binaria '%s'", op_sym);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }
        wyrm_value_t left = trace_eval_node(ctx, node->ast_node_value.binary_expr.left, scope);
        if (ctx->truncated) {
            break;
        }
        wyrm_value_t right = trace_eval_node(ctx, node->ast_node_value.binary_expr.right, scope);
        if (ctx->truncated) {
            break;
        }
        result = eval_binary(node, left, right);
        wyrm_web_format_value(left, lhs_buf, sizeof(lhs_buf));
        wyrm_web_format_value(right, rhs_buf, sizeof(rhs_buf));
        wyrm_web_format_value(result, res_buf, sizeof(res_buf));
        snprintf(note_buf, sizeof(note_buf), "%s %s %s => %s", lhs_buf, op_sym, rhs_buf, res_buf);
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    case AST_UNARY_EXPR: {
        const char *op_sym = token_op_symbol(node->ast_node_value.unary_expr.operator);
        snprintf(note_buf, sizeof(note_buf), "Evalúa operador unario '%s'", op_sym);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }
        wyrm_value_t right = trace_eval_node(ctx, node->ast_node_value.unary_expr.right, scope);
        if (ctx->truncated) {
            break;
        }
        result = eval_unary(node, right);
        wyrm_web_format_value(right, rhs_buf, sizeof(rhs_buf));
        wyrm_web_format_value(result, res_buf, sizeof(res_buf));
        snprintf(note_buf, sizeof(note_buf), "%s %s => %s", op_sym, rhs_buf, res_buf);
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    case AST_VAR_DECLARATION: {
        ast_var_decl_t decl = node->ast_node_value.var_decl_expr;
        format_decl_type(decl.var_type, decl.element_type, type_buf, sizeof(type_buf));
        snprintf(
            note_buf,
            sizeof(note_buf),
            "Declarando %s %s",
            type_buf,
            decl.identifier != NULL ? decl.identifier : "?"
        );
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }

        wyrm_value_t casted_value = {0};
        if (decl.expression == NULL) {
            casted_value.type = token_to_value_type(decl.var_type);
            if (decl.var_type == T_VEC) {
                wyrm_value_type_t elem_type = token_to_value_type(decl.element_type);
                casted_value.value.vec_val = create_wyrm_vec(elem_type, 4);
            }
        } else {
            wyrm_value_t expr_value = trace_eval_node(ctx, decl.expression, scope);
            if (ctx->truncated) {
                break;
            }
            if (decl.var_type == T_VEC && expr_value.type == VAL_VEC && expr_value.value.vec_val != NULL) {
                wyrm_value_type_t elem_type = token_to_value_type(decl.element_type);
                wyrm_vec_t *vec = expr_value.value.vec_val;
                if (elem_type != VAL_VOID) {
                    vec->element_type = elem_type;
                    for (size_t i = 0; i < vec->items.size; i++) {
                        wyrm_value_t *item_ptr = (wyrm_value_t *)vector_get(&vec->items, i);
                        *item_ptr = cast_to_value_type(*item_ptr, elem_type);
                    }
                }
                casted_value = expr_value;
            } else {
                casted_value = cast_value(expr_value, decl.var_type);
            }
        }

        result = eval_val_decl(node, scope, casted_value);
        wyrm_web_format_value(casted_value, res_buf, sizeof(res_buf));
        snprintf(
            note_buf,
            sizeof(note_buf),
            "Declara %s %s = %s",
            type_buf,
            decl.identifier != NULL ? decl.identifier : "?",
            res_buf
        );
        emit_ast_step(ctx, node_id, "exit", &casted_value, note_buf);
        break;
    }
    case AST_ASSIGNMENT: {
        ast_assignment_t asg = node->ast_node_value.assignment_expr;
        const char *vname = asg.name != NULL ? asg.name : "?";
        snprintf(note_buf, sizeof(note_buf), "Evalúa asignación sobre '%s'", vname);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }

        wyrm_value_t new_val = trace_eval_node(ctx, asg.value, scope);
        if (ctx->truncated) {
            break;
        }

        if (asg.index != NULL) {
            wyrm_value_t *vec_var = environment_get(scope, vname);
            if (vec_var == NULL) {
                panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: '%s'", vname);
            }
            if (vec_var->type != VAL_VEC || vec_var->value.vec_val == NULL) {
                panic(ERR_TYPE, "Runtime error: '%s' is not a vector", vname);
            }

            wyrm_value_t idx_val = trace_eval_node(ctx, asg.index, scope);
            if (ctx->truncated) {
                break;
            }
            int64_t idx = get_cast_int64(idx_val);
            wyrm_vec_t *vec = vec_var->value.vec_val;
            if (idx < 0 || (size_t)idx >= vec->items.size) {
                panic(
                    ERR_OVERFLOW,
                    "Runtime error: Vector index %lld out of bounds (size: %zu)",
                    (long long)idx,
                    vec->items.size
                );
            }

            wyrm_value_t casted_elem = cast_to_value_type(new_val, vec->element_type);
            wyrm_value_t *slot = (wyrm_value_t *)vector_get(&vec->items, (size_t)idx);
            *slot = casted_elem;

            result = (asg.operator == T_LARROW) ? casted_elem : (wyrm_value_t){.type = VAL_VOID};
            wyrm_web_format_value(casted_elem, res_buf, sizeof(res_buf));
            snprintf(note_buf, sizeof(note_buf), "Muta %s[%" PRId64 "] = %s", vname, idx, res_buf);
            emit_ast_step(ctx, node_id, "exit", &casted_elem, note_buf);
        } else {
            if (!environment_assign(scope, vname, new_val)) {
                panic(ERR_UNDEFINED_VAR, "Error UNDEFINED_VAR: Cannot assign to undeclared variable '%s'", vname);
            }
            result = (asg.operator == T_LARROW) ? new_val : (wyrm_value_t){.type = VAL_VOID};
            wyrm_value_t *stored = environment_get(scope, vname);
            wyrm_value_t shown = (stored != NULL) ? *stored : new_val;
            wyrm_web_format_value(shown, res_buf, sizeof(res_buf));
            snprintf(note_buf, sizeof(note_buf), "Asigna %s = %s", vname, res_buf);
            emit_ast_step(ctx, node_id, "exit", &shown, note_buf);
        }
        break;
    }
    case AST_IF_EXPR: {
        if (!emit_ast_step(ctx, node_id, "enter", NULL, "Evalúa condición de if")) {
            break;
        }
        wyrm_value_t cond = trace_eval_node(ctx, node->ast_node_value.if_expr.condition, scope);
        if (ctx->truncated) {
            break;
        }
        if (cond.type != VAL_BOOL) {
            panic(ERR_TYPE, "Type error: The 'if' condition must evaluate to a bool.");
        }
        const char *branch_taken = "sin rama";
        if (cond.value.bool_val) {
            branch_taken = "rama then";
            result = trace_eval_node(ctx, node->ast_node_value.if_expr.then_branch, scope);
        } else if (node->ast_node_value.if_expr.else_branch != NULL) {
            branch_taken = "rama else";
            result = trace_eval_node(ctx, node->ast_node_value.if_expr.else_branch, scope);
        }
        if (!ctx->truncated) {
            wyrm_web_format_value(result, res_buf, sizeof(res_buf));
            snprintf(note_buf, sizeof(note_buf), "If (%s) => %s", branch_taken, res_buf);
            emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        }
        break;
    }
    case AST_WHILE_EXPR: {
        if (!emit_ast_step(ctx, node_id, "enter", NULL, "Inicia bucle while")) {
            break;
        }
        size_t iters = 0;
        while (!ctx->truncated) {
            wyrm_value_t cond = trace_eval_node(ctx, node->ast_node_value.while_expr.condition, scope);
            if (ctx->truncated) {
                break;
            }
            if (cond.type != VAL_BOOL) {
                panic(ERR_TYPE, "Type error: La condición del 'while' debe resultar en un booleano");
            }
            if (!cond.value.bool_val) {
                break;
            }
            iters++;
            trace_eval_node(ctx, node->ast_node_value.while_expr.body, scope);
        }
        if (!ctx->truncated) {
            snprintf(note_buf, sizeof(note_buf), "Finaliza while tras %zu iteración(es)", iters);
            emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        }
        break;
    }
    case AST_FOR_EXPR: {
        ast_node_t *hdr = node->ast_node_value.while_expr.condition;
        ast_node_t *body = node->ast_node_value.while_expr.body;
        if (hdr == NULL || hdr->type != AST_ASSIGNMENT ||
            hdr->ast_node_value.assignment_expr.operator != T_LARROW) {
            panic(ERR_SYNTAX, "Runtime error: 'for' loop requires extraction syntax 'item <- vector'");
        }
        const char *iter_name = hdr->ast_node_value.assignment_expr.name;
        snprintf(note_buf, sizeof(note_buf), "Inicia bucle for (%s <- vec)", iter_name);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }
        wyrm_value_t vec_val = trace_eval_node(ctx, hdr->ast_node_value.assignment_expr.value, scope);
        if (ctx->truncated) {
            break;
        }
        if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
            panic(ERR_TYPE, "Runtime error: 'for' loop can only iterate over a 'vec'");
        }

        wyrm_vec_t *vec = vec_val.value.vec_val;
        environment_t *for_scope = create_environment(scope);
        char scope_label[64];
        snprintf(scope_label, sizeof(scope_label), "FOR (%s)", iter_name);
        push_scope(ctx, for_scope, scope_label);

        for (size_t i = 0; i < vec->items.size && !ctx->truncated; i++) {
            wyrm_value_t elem = *(wyrm_value_t *)vector_get(&vec->items, i);
            if (i == 0) {
                environment_set(for_scope, iter_name, elem, (token_type_t)vec->element_type);
            } else {
                wyrm_value_t *slot = environment_get(for_scope, iter_name);
                if (slot != NULL) {
                    *slot = elem;
                }
            }
            result = trace_eval_node(ctx, body, for_scope);
        }

        pop_scope(ctx);
        destroy_environment(for_scope);
        if (!ctx->truncated) {
            snprintf(note_buf, sizeof(note_buf), "Finaliza for (%zu elementos)", vec->items.size);
            emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        }
        break;
    }
    case AST_FUNCTION_DECL: {
        const char *fname = node->ast_node_value.function_decl.name;
        result = eval_function_decl(node, scope);
        snprintf(note_buf, sizeof(note_buf), "Registra función '%s'", fname != NULL ? fname : "fn");
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    case AST_CALL: {
        ast_call_expr_t call = node->ast_node_value.call_expr;
        const char *fname = call.name != NULL ? call.name : "fn";
        snprintf(note_buf, sizeof(note_buf), "Invoca '%s' (%zu arg)", fname, call.arg_count);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }

        if (strcmp(fname, "write") == 0) {
            result = trace_eval_native_write(ctx, call.arg_count, call.args, scope);
            if (!ctx->truncated) {
                wyrm_web_format_value(result, res_buf, sizeof(res_buf));
                snprintf(note_buf, sizeof(note_buf), "write(...) escribió %s byte(s)", res_buf);
                emit_ast_step(ctx, node_id, "exit", &result, note_buf);
            }
            break;
        }
        if (strcmp(fname, "read") == 0) {
            result = trace_eval_native_read(ctx, call.arg_count, call.args, scope);
            if (!ctx->truncated) {
                wyrm_web_format_value(result, res_buf, sizeof(res_buf));
                snprintf(note_buf, sizeof(note_buf), "read(...) => %s", res_buf);
                emit_ast_step(ctx, node_id, "exit", &result, note_buf);
            }
            break;
        }

        wyrm_value_t *func_val = environment_get(scope, fname);
        if (func_val == NULL) {
            panic(ERR_TYPE, "Runtime error: Function '%s' is not defined", fname);
        }
        if (func_val->type != VAL_FUNCTION) {
            panic(ERR_TYPE, "Runtime error: '%s' is not a function", fname);
        }

        wyrm_function_t func_def = func_val->value.wyrm_function_val;
        if (func_def.arg_count != call.arg_count) {
            panic(
                ERR_TYPE,
                "Runtime error: Function '%s' expects %zu arguments, but got %zu",
                fname,
                func_def.arg_count,
                call.arg_count
            );
        }

        environment_t *local_env = create_environment(scope);
        for (size_t i = 0; i < call.arg_count && !ctx->truncated; i++) {
            wyrm_value_t arg_val = trace_eval_node(ctx, call.args[i], scope);
            if (ctx->truncated) {
                break;
            }
            environment_set(
                local_env,
                func_def.args[i].name,
                arg_val,
                (token_type_t)func_def.args[i].type
            );
        }

        if (!ctx->truncated) {
            char scope_label[64];
            snprintf(scope_label, sizeof(scope_label), "FN %s", fname);
            push_scope(ctx, local_env, scope_label);

            wyrm_value_t body_res = trace_eval_node(ctx, func_def.body, local_env);

            pop_scope(ctx);

            if (!ctx->truncated) {
                if (func_def.returned == VAL_VOID) {
                    result = (wyrm_value_t){.type = VAL_VOID};
                } else {
                    result = cast_to_value_type(body_res, func_def.returned);
                    if (result.type != func_def.returned) {
                        destroy_environment(local_env);
                        panic(ERR_TYPE, "Runtime error: Function '%s' returned wrong type", fname);
                    }
                }
                wyrm_web_format_value(result, res_buf, sizeof(res_buf));
                snprintf(note_buf, sizeof(note_buf), "%s(...) => %s", fname, res_buf);
                emit_ast_step(ctx, node_id, "exit", &result, note_buf);
            }
        }
        destroy_environment(local_env);
        break;
    }
    case AST_VEC_LITERAL: {
        ast_block_t blk = node->ast_node_value.block_expr;
        if (is_ast_string_literal(node)) {
            wyrm_vec_t *vec = create_wyrm_vec(VAL_NAT8, blk.size);
            for (size_t i = 0; i < blk.size; i++) {
                wyrm_value_t raw_item = eval_number(blk.nodes[i]);
                wyrm_value_t byte_item = cast_to_value_type(raw_item, VAL_NAT8);
                vector_push(&vec->items, &byte_item);
            }
            result.type = VAL_VEC;
            result.value.vec_val = vec;
            wyrm_web_format_value(result, res_buf, sizeof(res_buf));
            snprintf(note_buf, sizeof(note_buf), "Cadena => %s", res_buf);
            emit_ast_step(ctx, node_id, "exit", &result, note_buf);
            break;
        }

        snprintf(note_buf, sizeof(note_buf), "Construye vector literal (%zu elems)", blk.size);
        if (!emit_ast_step(ctx, node_id, "enter", NULL, note_buf)) {
            break;
        }
        wyrm_vec_t *vec = create_wyrm_vec(VAL_INT64, blk.size);
        for (size_t i = 0; i < blk.size && !ctx->truncated; i++) {
            wyrm_value_t item = trace_eval_node(ctx, blk.nodes[i], scope);
            if (ctx->truncated) {
                break;
            }
            if (i == 0) {
                vec->element_type = item.type;
            }
            vector_push(&vec->items, &item);
        }
        result.type = VAL_VEC;
        result.value.vec_val = vec;
        if (!ctx->truncated) {
            wyrm_web_format_value(result, res_buf, sizeof(res_buf));
            snprintf(note_buf, sizeof(note_buf), "Vector => %s", res_buf);
            emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        }
        break;
    }
    case AST_VEC_ACCESS: {
        if (!emit_ast_step(ctx, node_id, "enter", NULL, "Evalúa acceso por índice vec[i]")) {
            break;
        }
        wyrm_value_t vec_val = trace_eval_node(ctx, node->ast_node_value.binary_expr.left, scope);
        if (ctx->truncated) {
            break;
        }
        if (vec_val.type != VAL_VEC || vec_val.value.vec_val == NULL) {
            panic(ERR_TYPE, "Runtime error: Attempting to index a non-vector value");
        }
        wyrm_value_t idx_val = trace_eval_node(ctx, node->ast_node_value.binary_expr.right, scope);
        if (ctx->truncated) {
            break;
        }
        int64_t idx = get_cast_int64(idx_val);
        wyrm_vec_t *vec = vec_val.value.vec_val;
        if (idx < 0 || (size_t)idx >= vec->items.size) {
            panic(
                ERR_OVERFLOW,
                "Runtime error: Vector index %lld out of bounds (size: %zu)",
                (long long)idx,
                vec->items.size
            );
        }
        result = *(wyrm_value_t *)vector_get(&vec->items, (size_t)idx);
        wyrm_web_format_value(result, res_buf, sizeof(res_buf));
        snprintf(note_buf, sizeof(note_buf), "vec[%" PRId64 "] => %s", idx, res_buf);
        emit_ast_step(ctx, node_id, "exit", &result, note_buf);
        break;
    }
    default:
        break;
    }

    if (pushed_path && ctx->active_depth > 0) {
        ctx->active_depth--;
    }

    return result;
}

static void free_ast_index(ast_trace_ctx_t *ctx) {
    for (size_t i = 0; i < ctx->node_count; i++) {
        free(ctx->nodes[i].children);
    }
    free(ctx->nodes);
    ctx->nodes = NULL;
    ctx->node_count = 0;
    ctx->node_cap = 0;
}

EMSCRIPTEN_KEEPALIVE
const char *wyrm_web_ast_trace_json(void) {
    return (g_ast_trace_json != NULL) ? g_ast_trace_json : "";
}

EMSCRIPTEN_KEEPALIVE
int wyrm_web_ast_trace(const char *source, int max_steps) {
    free(g_ast_trace_json);
    g_ast_trace_json = NULL;
    wyrm_web_clear_last_return();

    if (source == NULL) {
        fprintf(stderr, "Error: código fuente nulo.\n");
        fflush(stdout);
        fflush(stderr);
        return 1;
    }

    if (max_steps <= 0) {
        max_steps = AST_STEPPER_DEFAULT_MAX_STEPS;
    } else if (max_steps > AST_STEPPER_HARD_MAX_STEPS) {
        max_steps = AST_STEPPER_HARD_MAX_STEPS;
    }

    free_all_wyrm_vecs();

    char *input_copy = strdup(source);
    if (input_copy == NULL) {
        fprintf(stderr, "Error: sin memoria para copiar el código fuente.\n");
        fflush(stderr);
        return 1;
    }

    scanner_t scanner = {.tokens = NULL, .flags = 0};
    token_vector_t *tokens = scanner_scan(input_copy, &scanner);
    if (tokens == NULL) {
        free(input_copy);
        fprintf(stderr, "Error: no se pudo escanear el código fuente.\n");
        fflush(stderr);
        return 1;
    }

    parser_t *parser = create_parser(tokens);
    ast_node_t *root_ast = parser_parse(parser);

    ast_trace_ctx_t ctx;
    memset(&ctx, 0, sizeof(ctx));
    ctx.max_steps = max_steps;
    jbuf_init(&ctx.jb, AST_STEPPER_INIT_JSON_CAP);

    index_ast_tree(&ctx, root_ast, -1, "");

    jbuf_append_char(&ctx.jb, '{');
    emit_static_nodes_json(&ctx);
    jbuf_append_str(&ctx.jb, "\"steps\":[");

    environment_t *global_env = create_environment(NULL);
    push_scope(&ctx, global_env, "GLOBAL");

    wyrm_value_t final_result = trace_eval_node(&ctx, root_ast, global_env);

    pop_scope(&ctx);

    jbuf_append_fmt(
        &ctx.jb,
        "],\"truncated\":%s,\"max_steps\":%d,\"total_steps\":%d}",
        ctx.truncated ? "true" : "false",
        ctx.max_steps,
        ctx.step_count
    );

    // Capturar valor y tipo de retorno ANTES de liberar entorno y vectores
    wyrm_web_set_last_return(final_result);

    destroy_environment(global_env);
    free_all_wyrm_vecs();
    free_ast_index(&ctx);
    destroy_ast_node(root_ast);
    destroy_parser(parser);
    destroy_token_vector(tokens);
    free(input_copy);

    if (ctx.jb.oom) {
        jbuf_free(&ctx.jb);
        fprintf(stderr, "Error: sin memoria al serializar la traza AST JSON.\n");
        fflush(stderr);
        return 1;
    }

    g_ast_trace_json = ctx.jb.data;
    fflush(stdout);
    fflush(stderr);
    return 0;
}
