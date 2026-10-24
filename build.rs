use std::fs;
use std::path::Path;

// Archivo que concecta la implementation de Wyrm en C

fn collect_c_sources(dir: &Path, build: &mut cc::Build) {
    build.include(dir);
    let entries = fs::read_dir(dir).expect("No se pudo leer el directorio de wyrm_core");

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            collect_c_sources(&path, build);
        } else if let Some(ext) = path.extension() {
            if ext == "c" {
                build.file(&path);
                println!("cargo:rerun-if-changed={}", path.display()); // solo compila si hay cambios
            } else if ext == "h" {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}

fn main() {
    let mut build = cc::Build::new();

    collect_c_sources(Path::new("wyrm_core"), &mut build);

    build.compile("wyrm");

    println!("cargo:rustc-link-lib=m");
}
