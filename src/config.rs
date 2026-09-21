#[derive(Debug)]
pub struct Config {
    verbose: bool,
}


impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Self {
        args.next();
        let mut verbose = false;
        for arg in args {
            match arg.as_str() {
                "-v" | "--verbose" => verbose = true,
                _ => {} // espacio para nuevos flags
            }
        }
        Config { verbose }
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }
}