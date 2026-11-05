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
                    let flags = if self.verbose { 1 } else { 0 };
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
