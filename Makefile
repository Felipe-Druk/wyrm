# Siempre un Makefile es important para tener ordenado el proyecto, más si vamos a usar C

# Flags estrictos para Valgrind:
# --leak-check=full: Muestra dónde se reservó exactamente cada bloque perdido
# --show-leak-kinds=definite,indirect: Ignore falsos positivos de Rust
# --track-origins=yes: Rastrea de dónde salen las variables no inicializadas
# --error-exitcode=1: Hace que el Makefile falle si algun leak o error de memoria
VALGRIND_FLAGS = --leak-check=full \
                 --show-leak-kinds=definite,indirect \
                 --track-origins=yes \
                 --error-exitcode=1

# Ejecuta directamente el binario de test que se encuentra en target/debug/deps/ y que empieza con "wyrm-"		
TEST_BIN = $$(find target/debug/deps/ -maxdepth 1 -perm -111 -type f -name "wyrm-*" | head -n 1)

.PHONY: install check build run run-v clean format help pre-commit tests memcheck memcheck-one

.DEFAULT_GOAL := help

help:
	@echo ""
	@echo " __        __                    "
	@echo " \ \      / /   _ _ __ _ __ ___  "
	@echo "  \ \ /\ / / | | | '__| '_ \` _ \ "
	@echo "   \ V  V /| |_| | |  | | | | | |"
	@echo "    \_/\_/  \__, |_|  |_| |_| |_|"
	@echo "            |___/  "
	@echo ""
	@echo "- Install: Instala dependencias necesarias, solo se ejecuta una vez"
	@echo "- check: Verfica la integridad del codigo y busca errores"
	@echo "- build: Compila el proyecto como ejecutable"
	@echo "- run: Ejecuta el programa, si no esta compilado lo compila"
	@echo "- run-v: Ejecuta el programa con la flag de seguimiento, si no esta compilado lo compila"
	@echo "- format: Aplica linters al proyecto"
	@echo "- tests: Ejecuta todos los test"
	@echo "- memcheck: Ejecuta Valgrind para detectar leaks de memoria"
	@echo "- memcheck-one: Ejecuta Valgrind en un solo test, debe ser llamdo con TEST=<nombre_test>"
	@echo "- pre-commit: Ejecuta los pre-commits sobre todo el proyecto"
	@echo "- clean: limpia el ejecutable y archivos compilados"

install:
	@echo "Instalando dependencias de Rust..."
	rustup component add clippy rustfmt
	pre-commit install
	pre-commit autoupdate

check:
	@echo "Comprobando el proyecto..."
	cargo check
	cargo clippy -- -D warnings

build:
	@echo "Compilando el proyecto..."
	cargo build -vv

run:
	@echo "Ejecutando el proyecto..."
	@cargo run

run-v:
	@echo "Ejecutando el proyecto en modo verbose..."
	cargo run -- --verbose

format:
	@echo "Formato para Rust..."
	cargo fmt
	find wyrm_core -type f \( -name "*.c" -o -name "*.h" \) -exec clang-format -i {} +

tests: clean
	@echo "Ejecutanto tests..."
	cargo test

memcheck: clean
	@echo "Ejecutando Valgrind..."
	cargo test --no-run
	valgrind $(VALGRIND_FLAGS) $(TEST_BIN) cargo test -- --test-threads=1

memcheck-one: clean
	@echo "Ejecutando Valgrind en un solo test..."
	cargo test --no-run
	valgrind $(VALGRIND_FLAGS) $(TEST_BIN) $(TEST) --test-threads=1 --nocapture

pre-commit:
	@echo "Ejecutando pre-commits..."
	pre-commit run --all-files

clean:
	@echo "Limpiando el proyecto..."
	cargo clean