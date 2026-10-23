use crate::interpreter::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

const VERSION: &str = "0.0.3";
const EXIT_COMMAND: &str = "exit";
const C_VERBOSE_MODE: c_int = 1 << 0;
const C_DEBUG_MODE: c_int = 1 << 1;

/// Aplicacion de consola que lee entradas del usuario, y llama a la implementación de Wyrm en C
pub struct Wyrm {
    verbose: bool,
}

unsafe extern "C" {
    fn run_wyrm(input: *const c_char, flags: c_int) -> WyrmValue;
    fn terminate_wyrm();
}

impl Wyrm {
    /// Crea un nuevo objeto Wyrm
    pub fn new(verbose: bool) -> Self {
        Wyrm { verbose }
    }

    fn input_verbose(&self, input: &str) {
        println!("Entrada recibida: {}", input);
    }

    fn call_wyrm(&self, input: &str) -> WyrmValue {
        let c_input = CString::new(input).expect("Error al convertir a CString");
        unsafe {
            run_wyrm(
                c_input.as_ptr(),
                if self.verbose {
                    C_VERBOSE_MODE | C_DEBUG_MODE
                } else {
                    0
                },
            )
        }
    }

    fn terminate(&self) {
        unsafe {
            terminate_wyrm();
        }
    }

    /// While infinto que lee entradas del usuario y llama a la implementación de Wyrm en C, en principio se sale con el comando "exit"
    pub fn run(&self) {
        println!("Wyrm Version {}", VERSION);
        println!("Power by DrukDev");
        if self.verbose {
            println!("Modo Verbose activado");
        }

        let mut rl = DefaultEditor::new().expect("No se pudo inicializar la terminal interactiva");
        let _ = rl.load_history(".wyrm_history");

        loop {
            match rl.readline("wyrm> ") {
                Ok(line) => {
                    let input = line.trim();
                    if self.verbose {
                        self.input_verbose(input);
                    }
                    if input.is_empty() {
                        continue;
                    }
                    if input == EXIT_COMMAND {
                        println!("Wyrm se despide :D ...");
                        break;
                    }
                    let _ = rl.add_history_entry(input);
                    let result = self.call_wyrm(input);
                    println!("{}", result);
                }
                Err(ReadlineError::Eof) => {
                    println!("Wyrm se despide :D ...");
                    break;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    println!("Wyrm se despide :D ...");
                    break;
                }
                Err(err) => {
                    eprintln!("Error al leer la entrada: {:?}", err);
                    break;
                }
            }
        }
        let _ = rl.save_history(".wyrm_history");
    }
}

// Nos aseguramos de que al salir del scope de Wyrm, se llame a terminate_wyrm para liberar la memoria del scope global y terminar el intérprete
impl Drop for Wyrm {
    fn drop(&mut self) {
        self.terminate();
    }
}

// solo tests de flujo completo

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial] // Como el scope es global, nos es bueno que exista más de un wyrm corriendo al mismo tiempo, serial fuerza a que se ejecuten de a uno
    fn test_wyrm_respeta_punt_y_coma() {
        let input = format!("2+3; 3+6");
        let wyrm = Wyrm::new(false);

        unsafe {
            let resultado = wyrm.call_wyrm(&input);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la suma no es int64"
            );
            assert_eq!(
                resultado.value.int64_val, 9,
                "El cálculo matemático de 6 + 3 estuvo al final"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_instancia_variables() {
        let input = format!("int32 entero = 54;");
        let input2 = format!("entero;");
        let wyrm = Wyrm::new(false);

        unsafe {
            let resultado = wyrm.call_wyrm(&input);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValVoid,
                "El no es vacio"
            );
            let resultado2 = wyrm.call_wyrm(&input2);
            assert_eq!(
                resultado2.value_type,
                WyrmValueType::ValInt32,
                "El la variable no es int32"
            );
            assert_eq!(
                resultado2.value.int64_val, 54,
                "La variable no tiene el valor correcto"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_trabaja_booleanos() {
        let input = format!("bool falso = false; falso ~= true");
        let wyrm = Wyrm::new(false);

        unsafe {
            let resultado = wyrm.call_wyrm(&input);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValBool,
                "El tipo resultantado  no es booleano"
            );
            assert_eq!(resultado.value.bool_val, true, "El cálculo no es true");
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_trabaja_bloques() {
        let input = "int32 x = (5 + 3) * 2;
                    { 
                        bool y = true; 
                    } 
                    int32 z = 10;";
        let wyrm = Wyrm::new(false);

        let resultado = wyrm.call_wyrm(&input);
        assert_eq!(
            resultado.value_type,
            WyrmValueType::ValVoid,
            "El tipo resultantado vacio"
        );
    }

    #[test]
    #[serial]
    fn test_wyrm_respeta_parentecis() {
        let input = format!("(2+2) / 2");
        let wyrm = Wyrm::new(false);

        unsafe {
            let resultado = wyrm.call_wyrm(&input);
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante de la suma no es int64"
            );
            assert_eq!(resultado.value.int64_val, 2, "El resultado no es 2");
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_aisla_variables_en_scope_local() {
        let input = "
            int32 x = 5;
            {
                int32 x = 100;
            }
            x;
        ";
        let wyrm = Wyrm::new(false);

        let resultado = wyrm.call_wyrm(input);

        unsafe {
            assert_eq!(
                resultado.value_type,
                WyrmValueType::ValInt32,
                "El tipo resultante debe ser int32"
            );
            assert_eq!(
                resultado.value.int32_val, 5,
                "La variable global fue sobreescrita"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_control_de_flujo_if_else() {
        let input = "
            int32 valor = 15;
            int32 resultado = 0;

            if (valor < 10) {
                resultado = 10;
            } else if (valor == 15) {
                resultado = 20;
            } else {
                resultado = 30;
            }
            
            resultado;
        ";

        let wyrm = Wyrm::new(false);
        let output = wyrm.call_wyrm(input);

        unsafe {
            assert_eq!(
                output.value_type,
                WyrmValueType::ValInt32,
                "El tipo resultante debe ser int32"
            );
            assert_eq!(
                output.value.int32_val, 20,
                "El control de flujo no entró en el 'else if' correcto"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_bucle_while_conteo() {
        let input = "int32 contador = 0;
         while (contador < 5) {
             contador = contador + 1;
         }
        contador";

        let wyrm = Wyrm::new(false);
        let output = wyrm.call_wyrm(input);

        unsafe {
            assert_eq!(
                output.value_type,
                WyrmValueType::ValInt32,
                "El tipo resultante debe ser int32"
            );
            assert_eq!(
                output.value.int32_val, 5,
                "El contador debería haber sumado hasta 5"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_función_conteo() {
        let input = "
        (nat32) -> int32 contador = (x) -> {
            int32 actual = 0;
            while (actual < x) {
                actual = actual + 1;
            }
            actual
        }        
        contador(5)";

        let wyrm = Wyrm::new(false);
        let output = wyrm.call_wyrm(input);

        unsafe {
            assert_eq!(
                output.value_type,
                WyrmValueType::ValInt32,
                "El tipo resultante debe ser int32"
            );
            assert_eq!(
                output.value.int32_val, 5,
                "El contador debería haber sumado hasta 5"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_inserta_punto_coma() {
        let input = "
        (nat32) -> int32 contador = (x) -> {
            int32 actual = 0
            while (actual < x) {
                actual = actual + 1
            }
            actual
        }        
        nat16 y = 5 + contador(5); y";

        let wyrm = Wyrm::new(false);
        let output = wyrm.call_wyrm(input);

        unsafe {
            assert_eq!(
                output.value_type,
                WyrmValueType::ValNat16,
                "El tipo resultante debe ser nat16"
            );
            assert_eq!(
                output.value.nat16_val, 10,
                "El contador debería haber sumado hasta 10"
            );
        }
    }

    #[test]
    #[serial]
    fn test_wyrm_función_multiples_argumetnos() {
        let input = "
        (int32, int32) -> int64 suma = (x,y) -> { x+y }        
        int64 resultado = suma(2,suma(1,2)); resultado";

        let wyrm = Wyrm::new(false);
        let output = wyrm.call_wyrm(input);

        unsafe {
            assert_eq!(
                output.value_type,
                WyrmValueType::ValInt64,
                "El tipo resultante debe ser int64"
            );
            assert_eq!(
                output.value.int64_val, 5,
                "El contador debería haber sumado hasta 5"
            );
        }
    }
}
