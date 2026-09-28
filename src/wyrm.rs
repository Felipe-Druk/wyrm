use crate::interpreter::*;
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
    fn run_wyrm(input: *const c_char, flags: c_int) -> WyrmValue;
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

            let result = self.call_wyrm(&input);

            println!("{}", result);

            if input == EXIT_COMMAND {
                is_running = false;
            }
            input = "".to_string();
        }
        println!("Wyrm se despide :D ...");
    }
}

// solo tests de flujo completo

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}
