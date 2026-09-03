use std::ffi::CString;
use std::os::raw::{c_char, c_int};

const VERSION: &str = "0.0.1";
const EXIT_COMMAND: &str = "exit";
const C_VERBOSE_MODE: c_int = 1 << 0;
const C_DEBUG_MODE: c_int = 1 << 1;

/// Aplicacion de consola que lee entradas del usuario, y llama a la implementacion de Wyrm en C
pub struct Wyrm {
    verbose: bool,
}

unsafe extern "C" {
    fn run_wyrm(input: *const c_char,flags:  c_int);
}

impl Wyrm {
    /// Crea un nuevo objeto Wyrm
    pub fn new(verbose: bool) -> Self {
        Wyrm { verbose : verbose }
    }

    fn input_verbose(&self, input: &str) {
        println!("Entrada recibida: {}", input);
    }

    fn call_scanner(&self, input: &str) {
        let c_input = CString::new(input).expect("Error al convertir a CString");
        unsafe {
            run_wyrm(c_input.as_ptr(), if self.verbose { C_VERBOSE_MODE | C_DEBUG_MODE } else { C_DEBUG_MODE });
        }
    }

    /// While infinto que lee entradas del usuario y llama a la implementacion de Wyrm en C, en principio se sale con el comando "exit"
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
            std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");

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