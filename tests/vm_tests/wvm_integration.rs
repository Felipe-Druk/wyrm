use serial_test::serial;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::Write;
use std::os::raw::c_char;
use wyrm_lib::interpreter::{WyrmValue, WyrmValueType};
use wyrm_lib::wyrm::Wyrm;

#[repr(C)]
struct WyrmcC {
    input_files_path: *mut c_char,
    output_files_path: *mut c_char,
    flags: i32,
}

unsafe extern "C" {
    fn create_wyrmc(
        input_file_path: *const c_char,
        output_file_path: *const c_char,
        flags: i32,
    ) -> *mut WyrmcC;
    fn compile_file(wyrmc: *mut WyrmcC);
    fn destroy_wyrmc(wyrmc: *mut WyrmcC);
}

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

/// Compila código fuente .wyrm a bytecode .wrc y lo ejecuta en la Máquina Virtual (wvm)
fn compilar_y_ejecutar_wvm(nombre_test: &str, codigo: &[u8], verbose: bool) -> WyrmValue {
    let input_path = format!("test_wvm_{}.wyrm", nombre_test);
    let output_path = format!("test_wvm_{}.wrc", nombre_test);
    let _guard = TempFileGuard {
        paths: vec![&input_path, &output_path],
    };

    let mut source_file =
        File::create(&input_path).expect("No se pudo crear el archivo temporal .wyrm");
    source_file
        .write_all(codigo)
        .expect("Fallo al escribir el código fuente");
    drop(source_file);

    // Compilación (.wyrm -> .wrc)
    unsafe {
        let c_in = CString::new(input_path.as_str()).unwrap();
        let c_out = CString::new(output_path.as_str()).unwrap();
        let flags = if verbose { 0xFFFF } else { 0 };
        let wyrmc = create_wyrmc(c_in.as_ptr(), c_out.as_ptr(), flags);
        compile_file(wyrmc);
        destroy_wyrmc(wyrmc);
    }

    // Ejecusion VM (.wrc -> WyrmValue)
    let wyrm = Wyrm::new(verbose, Some(output_path.clone()));
    wyrm.call_vm(&output_path)
}

#[test]
#[serial]
fn test_wvm_01_instancia_y_lee_variable() {
    let resultado = compilar_y_ejecutar_wvm("var_local", b"int32 entero = 54;\nentero;", false);

    unsafe {
        assert_eq!(
            resultado.value_type,
            WyrmValueType::ValInt32,
            "La VM debe devolver ValInt32 tras ejecutar OP_CONSTANT, OP_CAST y OP_GET_LOCAL"
        );
        assert_eq!(
            resultado.value.int32_val, 54,
            "El valor devuelto por la VM no coincide con el esperado (54)"
        );
    }
}
#[test]
#[serial]
fn test_wvm_02_operadores_arigmeticos() {
    let codigo = b"int32 entero = 10;\n\
                   entero = entero + 1\n\
                   entero = entero - 1\n\
                   entero = entero / 2\n\
                   entero = entero * 2;\n\
                   entero;";

    let resultado = compilar_y_ejecutar_wvm("aritmetica", codigo, false);

    unsafe {
        assert_eq!(
            resultado.value_type,
            WyrmValueType::ValInt32,
            "La VM debe devolver ValInt32"
        );
        assert_eq!(
            resultado.value.int32_val, 10,
            "El valor devuelto por la VM no coincide con el esperado (10)"
        );
    }
}
