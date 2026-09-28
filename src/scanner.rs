#![allow(dead_code)]

use crate::token_vector::*;
use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug)]
pub struct Scanner {
    pub tokens: *mut TokenVector,
    pub flags: c_int,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;

    unsafe extern "C" {
        pub fn scanner_scan(input: *mut c_char, scanner: *mut Scanner) -> *mut TokenVector;
    }

    // Test de scanner, si ocurre un bug se agrea un tests

    fn mostrar_diferencias_de_vectors(esperados: &[TokenType], recibidos: &TokenVector) {
        println!("\n=== DIFERENCIAS DE TOKENS ===");
        println!("Esperábamos {} tokens:", esperados.len());
        for (i, t) in esperados.iter().enumerate() {
            println!("  [{}] {:?}", i, t);
        }

        println!("\nEl Scanner de C generó {} tokens:", recibidos.size);
        for i in 0..recibidos.size {
            unsafe {
                let token = &*recibidos.tokens.add(i);
                println!("  [{}] {:?}", i, token.t_type);
            }
        }
        println!("==============================\n");
    }

    /// Función que permite comparar salida esperada con la salida real del scanner
    fn validar_toknes_esperados(
        scanner_state: &mut Scanner,
        esperados: &[TokenType],
        codigo: CString,
    ) {
        unsafe {
            let vector_ptr = scanner_scan(codigo.as_ptr() as *mut c_char, &mut *scanner_state);
            assert!(
                !vector_ptr.is_null(),
                "El scanner falló al crear el vector de tokens"
            );
            let vector = &*vector_ptr;

            if vector.size != esperados.len() {
                mostrar_diferencias_de_vectors(esperados, vector);

                panic!(
                    "Fallo por cantidad: Esperábamos {} pero C devolvió {}",
                    esperados.len(),
                    vector.size
                );
            }

            for (i, tipo_esperado) in esperados.iter().enumerate() {
                let token = &*vector.tokens.add(i);
                assert_eq!(
                    &token.t_type, tipo_esperado,
                    "ERROR en el token {}: esperado {:?}  recibido {:?}",
                    i, tipo_esperado, token.t_type
                );
            }
        }
    }

    #[test]
    fn test_scanner_salta_espacios_y_genera_eof() {
        let codigo = CString::new("   \n \t ").unwrap();

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        unsafe {
            let vector_ptr = scanner_scan(codigo.as_ptr() as *mut c_char, &mut scanner_state);
            assert!(
                !vector_ptr.is_null(),
                "El scanner falló al crear el vector de tokens"
            );
            let vector = &*vector_ptr;
            assert_eq!(vector.size, 1);
            let primer_token = &*vector.tokens.add(0);
            assert_eq!(primer_token.t_type, TokenType::TEof);
        }
    }

    #[test]
    fn test_scanner_detecta_suma_generica() {
        let codigo = CString::new("32 + 1;").unwrap();

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        let esperados = [
            TokenType::TNumber,
            TokenType::TAdd,
            TokenType::TNumber,
            TokenType::TSemicolon,
            TokenType::TEof,
        ];

        validar_toknes_esperados(&mut scanner_state, &esperados, codigo);
    }

    #[test]
    fn test_scanner_ignore_comentarios() {
        let codigo = CString::new("nat64 numero = 312; // comentario a ignorar").unwrap();

        let esperados = [
            TokenType::TNat64,
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TNumber,
            TokenType::TSemicolon,
            TokenType::TEof,
        ];

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        validar_toknes_esperados(&mut scanner_state, &esperados, codigo);
    }

    #[test]
    fn test_sentencia_con_multiples_lineas() {
        let codigo = CString::new(
            "nat64 numero = 312;
     int8 aumento = 2;
     numero = numero + 12;",
        )
        .unwrap();

        let esperados = [
            //nat64 numero = 312;
            TokenType::TNat64,
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TNumber,
            TokenType::TSemicolon,
            //int8 aumento = 2;
            TokenType::TInt8,
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TNumber,
            TokenType::TSemicolon,
            //numero = numero +12;
            TokenType::TIdentifier,
            TokenType::TAssign,
            TokenType::TIdentifier,
            TokenType::TAdd,
            TokenType::TNumber,
            TokenType::TSemicolon,
            TokenType::TEof,
        ];

        let mut scanner_state = Scanner {
            tokens: std::ptr::null_mut(),
            flags: 0,
        };

        validar_toknes_esperados(&mut scanner_state, &esperados, codigo);
    }
}
