extern crate wyrm_lib;

use std::path::Path;

pub struct Wyrmc {
    verbose: bool,
    input_files_path: String,
    output_files_path: Option<String>,
}

#[repr(C)]
pub struct WyrmcC {
    pub input_files_path: *mut std::os::raw::c_char,
    pub output_files_path: *mut std::os::raw::c_char,
    pub flags: i32,
}

const DEFAULT_OUTPUT_FILE_NAME: &str = "output.wrc";
const INPUT_FILE_EXTENSION: &str = "wyrm";

unsafe extern "C" {
    fn create_wyrmc(
        input_file_path: *const std::os::raw::c_char,
        output_file_path: *const std::os::raw::c_char,
        flags: i32,
    ) -> *mut WyrmcC;
    fn compile_file(wyrmc: *mut WyrmcC);
    fn destroy_wyrmc(wyrmc: *mut WyrmcC);
}

impl Wyrmc {
    pub fn new(verbose: bool, input_files_path: String, output_files_path: Option<String>) -> Self {
        Wyrmc {
            verbose,
            input_files_path,
            output_files_path,
        }
    }

    pub fn compile(&self) {
        let file_path = &self.input_files_path;
        let wr_path = Path::new(file_path);

        if let Some(extension) = wr_path.extension() {
            if extension == INPUT_FILE_EXTENSION {
                unsafe {
                    let input_file_path = std::ffi::CString::new(file_path.as_str()).unwrap();
                    let output_file_path = match &self.output_files_path {
                        Some(output_path) => std::ffi::CString::new(output_path.as_str()).unwrap(),
                        None => std::ffi::CString::new(DEFAULT_OUTPUT_FILE_NAME).unwrap(),
                    };
                    let flags = if self.verbose { 0xFFFF } else { 0 };
                    let wyrmc =
                        create_wyrmc(input_file_path.as_ptr(), output_file_path.as_ptr(), flags);
                    compile_file(wyrmc);
                    destroy_wyrmc(wyrmc);
                }
            } else {
                eprintln!("El archivo {} no tiene la extensión .wyrm", file_path);
            }
        }
    }
}

fn main() {
    // leer arguementos
    // let wyrmc = Wyrmc::new(...);
    // wyrmc.compile();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::path::Path;

    const TEST_INPUT_FILE: &str = "test_input.wyrm";
    const TEST_OUTPUT_FILE: &str = "test_output.wrc";

    struct TempFileGuard<'a> {
        paths: Vec<&'a str>,
    }

    impl<'a> Drop for TempFileGuard<'a> {
        fn drop(&mut self) {
            for path in &self.paths {
                let _ = fs::remove_file(path);
            }
        }
    }

    #[test]
    fn test_wyrmc_compila_archivo_con_magic_number() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        let mut source_file =
            File::create(TEST_INPUT_FILE).expect("No se pudo crear el archivo temporal .wyrm");
        source_file
            .write_all(b"nat8 x = 1;\nnat8 xplus = x + 1;")
            .expect("Fallo al escribir en el archivo de prueba");
        drop(source_file);

        let wyrmc = Wyrmc::new(
            false,
            TEST_INPUT_FILE.to_string(),
            Some(TEST_OUTPUT_FILE.to_string()),
        );
        wyrmc.compile();

        let out_path = Path::new(TEST_OUTPUT_FILE);

        assert!(
            out_path.exists(),
            "El archivo .wrc de salida no fue generado por wyrmc"
        );

        let metadata =
            fs::metadata(TEST_OUTPUT_FILE).expect("No se pudo leer la metadata de salida");
        assert!(
            metadata.len() > 0,
            "El archivo compilado .wrc se generó con 0 bytes"
        );

        let mut compiled_file =
            File::open(TEST_OUTPUT_FILE).expect("No se pudo abrir el .wrc generado");
        let mut magic = [0u8; 4];
        compiled_file
            .read_exact(&mut magic)
            .expect("No se pudieron leer los primeros 4 bytes del header");
        assert_eq!(
            &magic, b"WYRM",
            "El archivo binario no inicia con el Magic Number WYRM"
        );
    }
}
