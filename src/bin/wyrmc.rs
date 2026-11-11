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
    // Opcodes de variables
    WYRM_OP_GET_LOCAL = 0x11,
    WYRM_OP_SET_LOCAL = 0x12,
    // Opcodes relacionales
    WYRM_OP_LESS = 0x13,
    WYRM_OP_GREATER = 0x14,
    WYRM_OP_EQUAL = 0x15,
    WYRM_OP_NOT_EQUAL = 0x16,
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

    #[test]
    #[serial]
    fn test_wyrmc_compila_función_contador() {
        let _guard = TempFileGuard {
            paths: vec![TEST_INPUT_FILE, TEST_OUTPUT_FILE],
        };

        let codigo = b"
        (nat32) -> int32 contador = (x) -> {
            int32 actual = 0;
            while (actual < x) {
                actual = actual + 1;
            }
            actual
        }
        contador(5);
        ";

        compilar_codigo_prueba(codigo, true);

        let bytes = fs::read(TEST_OUTPUT_FILE).expect("No se pudo leer el archivo .wrc generado");
        let mut cursor = HEADER_SIZE;

        // constantes
        let const_count = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;
        assert_eq!(
            const_count, 3,
            "Deben existir 3 constantes literales (0, 1 y 5)"
        );

        let valores_esperados = [0i64, 1i64, 5i64];
        for (i, &esperado) in valores_esperados.iter().enumerate() {
            let const_type = bytes[cursor];
            cursor += 1;
            assert_eq!(const_type, 8, "La constante {} debe ser ValInt64 (8)", i);

            let val = i64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
            cursor += 8;
            assert_eq!(val, esperado, "Valor incorrecto en la constante {}", i);
        }

        // funciones
        let func_count = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        cursor += 2;
        assert_eq!(func_count, 1, "Debe haber exactamente 1 función registrada");

        let arg_count = bytes[cursor];
        let return_type = bytes[cursor + 1];
        let code_offset = u32::from_le_bytes(bytes[cursor + 2..cursor + 6].try_into().unwrap());
        let code_length = u32::from_le_bytes(bytes[cursor + 6..cursor + 10].try_into().unwrap());
        cursor += 10; // 1B arg_count + 1B return_type + 4B code_offset + 4B code_length

        assert_eq!(arg_count, 1, "contador recibe 1 parámetro (x)");
        assert_eq!(return_type, 7, "contador retorna ValInt32 (7)");
        assert_eq!(
            code_offset, 3,
            "El cuerpo de contador inicia en el byte 3 tras el OP_JUMP inicial"
        );
        assert_eq!(code_length, 32, "El cuerpo de contador debe medir 32 bytes");

        // Bytecode generado:
        let code_size = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        cursor += 4;
        assert_eq!(
            bytes.len() - cursor,
            code_size,
            "El tamaño declarado de code[] no coincide con los bytes restantes del archivo"
        );

        let code = &bytes[cursor..];

        // Bytecode esperado:
        // OP_JUMP 35          -> Salta el cuerpo de 'contador' directo al byte 35 (contador(5))
        // OP_CONSTANT 0       -> Literal 0
        // OP_CAST 7           -> Casteo a  ValInt32
        // OP_GET_LOCAL 1      -> Lee "actual"
        // OP_GET_LOCAL 0      -> Lee "x"
        // OP_LESS             ->  "actual < x"
        // OP_JUMP_IF_NOT 31   -> false = salta a byte 31
        // OP_GET_LOCAL 1      -> Lee "actual"
        // OP_CONSTANT 1       -> Literal 1
        // OP_ADD              -> "actual + 1"
        // OP_SET_LOCAL 1      -> Guarda el resultado en "actual"
        // OP_JUMP 8           -> Vuelve al principio del "while"
        // OP_GET_LOCAL 1      -> Lee "actual"
        // OP_RETURN           -> Termina la función (offset 3 + 32 bytes = 35)
        // OP_CONSTANT 2       -> Argumento 5 (constante índice 2)
        // OP_CALL 0           -> Llama a la función 0 (contador)
        // OP_RETURN           -> Fin del programa

        // inicia la función
        assert_eq!(code[0], WyrmOpcode::WYRM_OP_JUMP as u8);
        assert_eq!(u16::from_le_bytes([code[1], code[2]]), 35);

        // int32 actual = 0;
        assert_eq!(code[3], WyrmOpcode::WYRM_OP_CONSTANT as u8);
        assert_eq!(u16::from_le_bytes([code[4], code[5]]), 0);
        assert_eq!(code[6], WyrmOpcode::WYRM_OP_CAST as u8);
        assert_eq!(code[7], 7); // ValInt32

        // Condición while (actual < x)
        assert_eq!(code[8], WyrmOpcode::WYRM_OP_GET_LOCAL as u8);
        assert_eq!(u16::from_le_bytes([code[9], code[10]]), 1); // actual
        assert_eq!(code[11], WyrmOpcode::WYRM_OP_GET_LOCAL as u8);
        assert_eq!(u16::from_le_bytes([code[12], code[13]]), 0); // x
        assert_eq!(code[14], WyrmOpcode::WYRM_OP_LESS as u8);
        assert_eq!(code[15], WyrmOpcode::WYRM_OP_JUMP_IF_NOT as u8);
        assert_eq!(u16::from_le_bytes([code[16], code[17]]), 31);

        //Cuerpo del while: actual = actual + 1;
        assert_eq!(code[18], WyrmOpcode::WYRM_OP_GET_LOCAL as u8);
        assert_eq!(u16::from_le_bytes([code[19], code[20]]), 1); // actual
        assert_eq!(code[21], WyrmOpcode::WYRM_OP_CONSTANT as u8);
        assert_eq!(u16::from_le_bytes([code[22], code[23]]), 1); // constante 1
        assert_eq!(code[24], WyrmOpcode::WYRM_OP_ADD as u8);
        assert_eq!(code[25], WyrmOpcode::WYRM_OP_SET_LOCAL as u8);
        assert_eq!(u16::from_le_bytes([code[26], code[27]]), 1); // actual
        assert_eq!(code[28], WyrmOpcode::WYRM_OP_JUMP as u8);
        assert_eq!(u16::from_le_bytes([code[29], code[30]]), 8); // vuelve a la condición

        // Retorno de "actual"
        assert_eq!(code[31], WyrmOpcode::WYRM_OP_GET_LOCAL as u8);
        assert_eq!(u16::from_le_bytes([code[32], code[33]]), 1);
        assert_eq!(code[34], WyrmOpcode::WYRM_OP_RETURN as u8);

        // Llamada contador(5) y fin
        assert_eq!(code[35], WyrmOpcode::WYRM_OP_CONSTANT as u8);
        assert_eq!(u16::from_le_bytes([code[36], code[37]]), 2); // constante 5
        assert_eq!(code[38], WyrmOpcode::WYRM_OP_CALL as u8);
        assert_eq!(u16::from_le_bytes([code[39], code[40]]), 0); // func_id 0
        assert_eq!(code[41], WyrmOpcode::WYRM_OP_RETURN as u8);
    }
}
