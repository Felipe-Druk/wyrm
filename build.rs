// Archivo que concecta la implementacion de Wyrm en C




fn main() { 
    cc::Build::new()
        // Todos los .c
        .file("wyrm_core/wyrm.c")

        .file("wyrm_core/scanner/arigmetic_resolver.c")
        .file("wyrm_core/scanner/scanner.c")
        .file("wyrm_core/utils/token_vector.c")
        


        // Dependencias o .h
        .include("wyrm_core") 
        .file("wyrm_core/scanner/arigmetic_resolver.h")
        .include("wyrm_core/scanner")
        .include("wyrm_core/utils")

        // Compilacion
        .compile("wyrm");
}
