# Siempre un Makefile es importante para tener ordenado el proyecto, más si vamos a usar C





check:
	@echo "Comprobando el proyecto..."
	@cargo check

build:
	@echo "Compilando el proyecto..."
	@cargo build

run:
	@echo "Ejecutando el proyecto..."
	@cargo run

run-v:
	@echo "Ejecutando el proyecto en modo verbose..."
	@cargo run -- --verbose

make clean:
	@echo "Limpiando el proyecto..."
	@cargo clean