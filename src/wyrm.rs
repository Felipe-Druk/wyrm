


/// Aplicacion de consola que lee entradas del usuario, y llama a la implementacion de Wyrm en C
pub struct Wyrm {

}


impl Wyrm {
    /// Crea un nuevo objeto Wyrm
    pub fn new() -> Self {
        Wyrm {}
    }

    /// While infinto que lee entradas del usuario y llama a la implementacion de Wyrm en C, en principio se sale con el comando "exit"
    pub fn run(&self) {
        println!("Wyrm Version 0.0.1");

        let mut input = String::new();

        while true {

            println!("Wyrm >: ");
            std::io::stdin().read_line(&mut input).expect("Error al leer la entrada");
            let input_trim = input.trim();

            if input_trim == "exit" {
                println!("Wyrm se despide :D ...");
                break;
            }
            // Llamar a parser
            
            input = "".to_string();
        }
    }
}