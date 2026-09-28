// Archivo que concecta la implementation de Wyrm en C

fn main() {
    cc::Build::new()
        // Todos los .c
        .file("wyrm_core/wyrm.c")
        .file("wyrm_core/wyrm_token.c")
        .file("wyrm_core/ast_node.c")
        .file("wyrm_core/wyrm_value.c")
        .file("wyrm_core/environment.c")
        // Scanner
        .file("wyrm_core/scanner/arithmetic_resolver.c")
        .file("wyrm_core/scanner/numeric_resolver.c")
        .file("wyrm_core/scanner/types_resolver.c")
        .file("wyrm_core/scanner/scanner.c")
        .file("wyrm_core/scanner/identifier_resolver.c")
        .file("wyrm_core/scanner/limiter_resolver.c")
        .file("wyrm_core/scanner/logical_resolver.c")
        .file("wyrm_core/scanner/boolean_resolver.c")
        // Parser
        .file("wyrm_core/parser/parser.c")
        .file("wyrm_core/parser/debugger_parser.c")
        // Interpreter
        .file("wyrm_core/interpreter/interpreter.c")
        .file("wyrm_core/interpreter/evaluator_number.c")
        .file("wyrm_core/interpreter/evaluator_binary.c")
        .file("wyrm_core/interpreter/evaluator_unary.c")
        .file("wyrm_core/interpreter/evaluator_identifier.c")
        .file("wyrm_core/interpreter/evaluator_bool.c")
        //utils
        .file("wyrm_core/utils/debugger.c")
        .file("wyrm_core/utils/token_vector.c")
        .file("wyrm_core/utils/error.c")
        // Dependencias o .h
        .include("wyrm_core")
        .include("wyrm_core/scanner")
        .include("wyrm_core/utils")
        .include("wyrm_core/parser")
        .include("wyrm_core/interpreter")
        // Compilación
        .compile("wyrm");

    println!("cargo:rustc-link-lib=m");
}
