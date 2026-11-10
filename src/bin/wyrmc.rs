#![allow(dead_code, non_camel_case_types)]

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

#[repr(C)]
pub enum WyrmOpcode {
    // Opcodes para el stack
    WYRM_OP_PUSH = 0x00,
    WYRM_OP_POP = 0x01,
    // Opcodes operadores aritméticos
    WYRM_OP_ADD = 0x02,
    WYRM_OP_SUB = 0x03,
    WYRM_OP_MUL = 0x04,
    WYRM_OP_DIV = 0x05,
    WYRM_OP_MOD = 0x06,
    // Opcodes operadores lógicos
    WYRM_OP_AND = 0x07,
    WYRM_OP_OR = 0x08,
    WYRM_OP_NOT = 0x09,
    // Opcodes de control de flujo
    WYRM_OP_JUMP = 0x0A,
    WYRM_OP_JUMP_IF = 0x0B,
    WYRM_OP_JUMP_IF_NOT = 0x0C,
    // Opcodes de valores
    WYRM_OP_CONSTANT = 0x0D,
    WYRM_OP_CAST = 0x0E,
    // Opcodes de funciones
    WYRM_OP_CALL = 0x0F,
    WYRM_OP_RETURN = 0x10,
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
    use serial_test::serial;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    const TEST_INPUT_FILE: &str = "test_input.wyrm";
    const TEST_OUTPUT_FILE: &str = "test_output.wrc";
    const HEADER_SIZE: usize = 6; // 4 bytes para "WYRM" + 2 bytes para la versión
    const CONSTANTS_SECTION_SIZE: usize = 2 + 1 + 8; // 2 bytes count + 1 byte type + 8 bytes valor
    const FUNCTIONS_SECTION_SIZE: usize = 2; // 2 bytes func_count (0 funciones)
    const WYRMC_VERSION: u16 = 0x0100;

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

    fn compilar_codigo_prueba(codigo: &[u8], verbose: bool) {
        let mut source_file =
            File::create(TEST_INPUT_FILE).expect("No se pudo crear el archivo temporal .wyrm");
        source_file
            .write_all(codigo)
            .expect("Fallo al escribir en el archivo de prueba");
        drop(source_file);

        let wyrmc = Wyrmc::new(
            verbose,
            TEST_INPUT_FILE.to_string(),
            Some(TEST_OUTPUT_FILE.to_string()),
        );
        wyrmc.compile();
    }

    #[test]
    #[serial]
    fn test_wyrmc_compila_archivo_con_declaraciones() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        compilar_codigo_prueba(b"nat8 x = 1;\nnat8 xplus = x + 1;", false);

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
    }

    #[test]
    #[serial]
    fn test_wyrmc_compila_archivo_con_encabezado() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        compilar_codigo_prueba(b"nat8 x = 1;", true);

        let bytes = fs::read(TEST_OUTPUT_FILE).expect("No se pudo leer el archivo .wrc generado");

        // encabezado
        assert!(
            bytes.len() >= HEADER_SIZE,
            "El binario es más corto que el header"
        );
        assert_eq!(&bytes[0..4], b"WYRM", "Magic Number incorrecto");
        assert_eq!(
            &bytes[4..6],
            &WYRMC_VERSION.to_le_bytes(),
            "Versión del formato debe ser 1.0 (Little-Endian)"
        );
    }

    #[test]
    #[serial]
    fn test_wyrmc_compila_archivo_con_constantes() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        compilar_codigo_prueba(b"nat8 x = 1;", true);

        let bytes = fs::read(TEST_OUTPUT_FILE).expect("No se pudo leer el archivo .wrc generado");
        let mut cursor = HEADER_SIZE;

        // constantes
        let const_count = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;
        assert_eq!(
            const_count, 1,
            "Debe haber exactamente 1 constante en el pool"
        );
        let const_type = bytes[cursor];
        cursor += 1;
        assert_eq!(
            const_type, 8,
            "El literal numérico inicial debe ser ValInt64"
        );
        let const_val = i64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
        assert_eq!(const_val, 1, "El valor de la constante 0 debe ser 1");
    }

    #[test]
    #[serial]
    fn test_wyrmc_compila_archivo_con_funciones() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        compilar_codigo_prueba(b"nat8 x = 1;", true);

        let bytes = fs::read(TEST_OUTPUT_FILE).expect("No se pudo leer el archivo .wrc generado");
        let cursor = HEADER_SIZE + CONSTANTS_SECTION_SIZE;

        // funciones
        let func_count = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        assert_eq!(func_count, 0, "No debería haber funciones declaradas");
    }

    #[test]
    #[serial]
    fn test_wyrmc_compila_archivo_con_opcodes() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        compilar_codigo_prueba(b"nat8 x = 1;", true);

        let bytes = fs::read(TEST_OUTPUT_FILE).expect("No se pudo leer el archivo .wrc generado");
        let mut cursor = HEADER_SIZE + CONSTANTS_SECTION_SIZE + FUNCTIONS_SECTION_SIZE;

        // codigo esperado
        let code_size = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;
        assert_eq!(
            bytes.len() - cursor,
            code_size,
            "El tamaño declarado de code[] no coincide con el archivo"
        );

        // Bytecode generado:
        // OP_CONSTANT 0     -> [0x0D, 0x00, 0x00]
        // OP_CAST VAL_NAT8  -> [0x0E, 0x01]
        // OP_RETURN         -> [0x10]
        let code = &bytes[cursor..];

        assert_eq!(
            code[0],
            WyrmOpcode::WYRM_OP_CONSTANT as u8,
            "La primera instrucción debe ser OP_CONSTANT"
        );
        let const_idx = u16::from_le_bytes([code[1], code[2]]);
        assert_eq!(
            const_idx, 0,
            "OP_CONSTANT debe referenciar el índice 0 del pool"
        );
        assert_eq!(
            code[3],
            WyrmOpcode::WYRM_OP_CAST as u8,
            "La segunda instrucción debe ser OP_CAST"
        );
        assert_eq!(code[4], 1, "El cast debe ser hacia ValNat8");
        assert_eq!(
            code[5],
            WyrmOpcode::WYRM_OP_RETURN as u8,
            "La tercera instrucción debe ser OP_RETURN"
        );
    }
}
