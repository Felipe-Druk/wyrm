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
}
