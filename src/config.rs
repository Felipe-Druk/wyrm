use std::fs;

#[derive(Debug)]
pub struct Config {
    verbose: bool,
    file_path: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Self {
        args.next();
        let mut verbose = false;
        let mut file_path = None;
        for arg in args {
            match arg.as_str() {
                "-v" | "--verbose" => verbose = true,
                s if s.starts_with("-") => {
                    eprintln!("Unknown flag: {}", s);
                }
                s if fs::metadata(s).is_ok() => file_path = Some(s.to_string()),
                _ => {} // espacio para nuevos flags
            }
        }
        Config { verbose, file_path }
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }
}
