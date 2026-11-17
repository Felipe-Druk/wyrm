use std::fs;

#[derive(Debug)]
pub struct Config {
    verbose: bool,
    file_path: Option<String>,
    output_path: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Self {
        args.next();
        let mut verbose = false;
        let mut file_path = None;
        let mut output_path = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-v" | "--verbose" => verbose = true,
                "-o" | "--output" => {
                    if let Some(out) = args.next() {
                        output_path = Some(out);
                    } else {
                        eprintln!("Falta la ruta de salida después de {}", arg);
                    }
                }
                s if s.starts_with("-") => {
                    eprintln!("Unknown flag: {}", s);
                }
                s if fs::metadata(s).is_ok() => file_path = Some(s.to_string()),
                s => {
                    eprintln!("Archivo no encontrado: {}", s);
                }
            }
        }
        Config {
            verbose,
            file_path,
            output_path,
        }
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    pub fn get_output_path(&self) -> Option<&String> {
        self.output_path.as_ref()
    }
}
