use std::ffi::CString;
use std::os::raw::{c_char, c_int};

const VERSION: &str = "0.0.2";
const EXIT_COMMAND: &str = "exit";
const C_VERBOSE_MODE: c_int = 1 << 0;
const C_DEBUG_MODE: c_int = 1 << 1;

/// Aplicacion de consola que lee entradas del usuario, y llama a la implementación de Wyrm en C
pub struct Wyrm {
    verbose: bool,
}

unsafe extern "C" {
    fn run_wyrm(input: *const c_char, flags: c_int);
}

impl Wyrm {
    /// Crea un nuevo objeto Wyrm
    pub fn new(verbose: bool) -> Self {
        Wyrm { verbose }
    }

    fn input_verbose(&self, input: &str) {
        println!("Entrada recibida: {}", input);
    }

    fn call_scanner(&self, input: &str) {
        let c_input = CString::new(input).expect("Error al convertir a CString");
        unsafe {
            run_wyrm(
                c_input.as_ptr(),
                if self.verbose {
                    C_VERBOSE_MODE | C_DEBUG_MODE
                } else {
                    C_DEBUG_MODE
                },
            );
        }
    }

    /// While infinto que lee entradas del usuario y llama a la implementación de Wyrm en C, en principio se sale con el comando "exit"
    pub fn run(&self) {
        println!("Wyrm Version {}", VERSION);
        println!("Power by DrukDev");
        if self.verbose {
            println!("Modo Verbose activado");
        }

        let mut input = String::new();
        let mut is_running = true;

        while is_running {
            println!("Wyrm >: ");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error al leer la entrada");

            if self.verbose {
                self.input_verbose(&input);
            }

            self.call_scanner(&input);

            if input == EXIT_COMMAND {
                is_running = false;
            }
            input = "".to_string();
        }
        println!("Wyrm se despide :D ...");
    }
}

// Los tests de scanner, parser y intérprete se declaran en este archivo ya que son "unitarios"

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[repr(C)]
    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        TInt8 = 0,
        TInt16,
        TInt32,
        TInt64,
        TNat8,
        TNat16,
        TNat32,
        TNat64,
        TAdd,
        TSub,
        TMul,
        TDiv,
        TPow,
        TMod,
        TAssign,
        TRassign,
        TNumber,
        TString,
        TFloatNumber,
        TIdentifier,
        TEof,
        TSemicolon,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct WyrmToken {
        pub t_type: TokenType,
        pub lexeme: *mut c_char,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct TokenVector {
        pub size: usize,
        pub capacity: usize,
        pub tokens: *mut WyrmToken,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Scanner {
        pub tokens: *mut TokenVector,
        pub flags: c_int,
    }

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
