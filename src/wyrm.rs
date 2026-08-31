

const VERSION: &str = "0.0.1";
const EXIT_COMMAND: &str = "exit";

/// Aplicacion de consola que lee entradas del usuario, y llama a la implementacion de Wyrm en C
pub struct Wyrm {
    verbose: bool,
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
        // Llamar a la funcion de scanner en C
    }

    /// While infinto que lee entradas del usuario y llama a la implementacion de Wyrm en C, en principio se sale con el comando "exit"
    pub fn run(&self) {
        println!("Wyrm Version {}", VERSION);
        println!("Power by DrukDev");
        if self.verbose {
            println!("Modo Verbose activado");
        }

        let mut input = String::new();

        while true {

            println!("Wyrm >: ");
            std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");

            if self.verbose {
                self.input_verbose(&input);
            }

            self.call_scanner(&input);

            if input == EXIT_COMMAND {
                println!("Wyrm se despide :D ...");
                break;
            }
            // Llamar a parser
            
            input = "".to_string();
        }
    }
}