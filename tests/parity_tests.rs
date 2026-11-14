use serial_test::serial;
use std::ffi::CString;
use std::fs;
use std::os::raw::c_char;
use std::path::Path;
use std::time::{Duration, Instant};
use wyrm_lib::interpreter::{WyrmValue, WyrmValueData, WyrmValueType};
use wyrm_lib::wyrm::Wyrm;

#[repr(C)]
struct WyrmcC {
    input_file_path: *mut c_char,
    output_file_path: *mut c_char,
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

struct TempBytecodeGuard {
    path: String,
}

impl Drop for TempBytecodeGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub struct ParityMetrics {
    pub archivo: String,
    pub tiempo_intérprete: Duration,
    pub tiempo_compilación: Duration,
    pub tiempo_vm: Duration,
    pub resultado: WyrmValue,
}

/// Valida que tanto la vm como el ineterprete funcionen igual, ademas compara los tiempos
fn ejecutar_paridad_wyrm(ruta_wyrm: &str, esperado: WyrmValue) -> ParityMetrics {
    let path = Path::new(ruta_wyrm);
    assert!(
        path.exists(),
        "No se encontró el archivo de prueba: {}",
        ruta_wyrm
    );

    let codigo = fs::read_to_string(path).expect("No se pudo leer el archivo .wyrm");

    // Ejecuta el archivo sin compilar
    let wyrm_interp = Wyrm::new(false, None);
    let t0_interp = Instant::now();
    let res_intérprete = wyrm_interp.call_wyrm(&codigo);
    let tiempo_intérprete = t0_interp.elapsed();
    drop(wyrm_interp);

    assert_eq!(
        res_intérprete.value_type, esperado.value_type,
        "[Intérprete] Tipo incorrecto en '{}'",
        ruta_wyrm
    );
    assert!(
        res_intérprete == esperado,
        "[Intérprete] Valor incorrecto en '{}': obtenido {}, esperado {}",
        ruta_wyrm,
        res_intérprete,
        esperado
    );

    // Compila .wyrm -> .wrc y mide tiempos
    let wrc_path = path.with_extension("wrc").to_string_lossy().into_owned();
    let _guard = TempBytecodeGuard {
        path: wrc_path.clone(),
    };

    let t0_comp = Instant::now();
    unsafe {
        let c_in = CString::new(ruta_wyrm).unwrap();
        let c_out = CString::new(wrc_path.as_str()).unwrap();
        let wyrmc = create_wyrmc(c_in.as_ptr(), c_out.as_ptr(), 0);
        compile_file(wyrmc);
        destroy_wyrmc(wyrmc);
    }
    let tiempo_compilación = t0_comp.elapsed();

    // Ejecuta el bycode con la vm
    let wyrm_vm = Wyrm::new(false, Some(wrc_path.clone()));
    let t0_vm = Instant::now();
    let res_vm = wyrm_vm.call_vm(&wrc_path);
    let tiempo_vm = t0_vm.elapsed();

    assert_eq!(
        res_vm.value_type, esperado.value_type,
        "[VM] Tipo incorrecto en '{}'",
        ruta_wyrm
    );
    assert!(
        res_vm == esperado,
        "[VM] Valor incorrecto en '{}': obtenido {}, esperado {}",
        ruta_wyrm,
        res_vm,
        esperado
    );
    assert!(
        res_intérprete == res_vm,
        "[Paridad] Desfase entre Intérprete ({}) y VM ({}) en '{}'",
        res_intérprete,
        res_vm,
        ruta_wyrm
    );

    let speedup = tiempo_intérprete.as_secs_f64() / tiempo_vm.as_secs_f64().max(1e-9);
    println!(
        "[PARIDAD OK] {:<28} | Interp: {:>9.2?} | Comp: {:>9.2?} | VM: {:>9.2?} | Speedup VM: {:.2}x",
        ruta_wyrm, tiempo_intérprete, tiempo_compilación, tiempo_vm, speedup
    );

    ParityMetrics {
        archivo: ruta_wyrm.to_string(),
        tiempo_intérprete,
        tiempo_compilación,
        tiempo_vm,
        resultado: res_vm,
    }
}

// Constructores auxiliares limpios para valores esperados
fn val_int32(v: i32) -> WyrmValue {
    WyrmValue {
        value_type: WyrmValueType::ValInt32,
        value: WyrmValueData { int32_val: v },
    }
}

fn val_bool(v: bool) -> WyrmValue {
    WyrmValue {
        value_type: WyrmValueType::ValBool,
        value: WyrmValueData { bool_val: v },
    }
}

fn val_void() -> WyrmValue {
    WyrmValue {
        value_type: WyrmValueType::ValVoid,
        value: WyrmValueData { int32_val: 0 }, // para engañar al compilador
    }
}

fn val_nat8(v: u8) -> WyrmValue {
    WyrmValue {
        value_type: WyrmValueType::ValNat8,
        value: WyrmValueData { nat8_val: v },
    }
}

#[test]
#[serial]
fn test_paridad_01_aritmetica() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/01_aritmetica.wyrm", val_int32(10));
}

#[test]
#[serial]
fn test_paridad_02_logica() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/02_logica.wyrm", val_bool(true));
}

#[test]
#[serial]
fn test_paridad_03_if_else() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/03_if_else.wyrm", val_int32(20));
}

#[test]
#[serial]
fn test_paridad_04_while() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/04_while.wyrm", val_int32(5));
}

#[test]
#[serial]
fn test_paridad_05_función_contador() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/05_contador.wyrm", val_int32(5));
}

#[test]
#[serial]
fn test_paridad_06_archivo_vacio() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/06_vacio.wyrm", val_void());
}

#[test]
#[serial]
fn test_paridad_07_benchmarks_contador() {
    ejecutar_paridad_wyrm(
        "tests/wyrm_scripts/07_benchmarks_contador.wyrm",
        val_int32(5000000),
    );
}

#[test]
#[serial]
fn test_paridad_08_chars_como_nat8() {
    ejecutar_paridad_wyrm("tests/wyrm_scripts/08_chars.wyrm", val_nat8(97));
}
