# Siempre un Makefile es important para tener ordenado el proyecto, más si vamos a usar C

# Flags estrictos para Valgrind:
# --leak-check=full: Muestra dónde se reservó exactamente cada bloque perdido
# --show-leak-kinds=definite,indirect: Ignore falsos positivos de Rust
# --errors-for-leak-kinds=definite,indirect: Devuelve error si hay leaks de memoria, ignorando falsos positivos de Rust
# --track-origins=yes: Rastrea de dónde salen las variables no inicializadas
# --error-exitcode=1: Hace que el Makefile falle si algun leak o error de memoria
VALGRIND_FLAGS = --leak-check=full \
                 --show-leak-kinds=definite,indirect \
                 --errors-for-leak-kinds=definite,indirect \
                 --track-origins=yes \
                 --error-exitcode=1

VALGRIND_RUNNER = valgrind $(VALGRIND_FLAGS)

# Ejecuta directamente el binario de test que se encuentra en target/debug/deps/ y que empieza con "wyrm-"		
TEST_BIN = $$(find target/debug/deps/ -maxdepth 1 -perm -111 -type f -name "wyrm-*" | head -n 1)

TIME_DIR = tests/time
WYRM_BIN = ./target/release/wyrm
WYRMC_BIN = ./target/release/wyrmc


.PHONY: install check build run run-v clean format help pre-commit tests memcheck memcheck-one tests-time

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
	@echo "- tests-parity: Ejecuta tests de paridad con salida estadnar"
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

tests:
	@echo "Ejecutanto tests..."
	cargo test

tests-parity:
	@echo "Ejecutanto tests de paridad..."
	cargo test --test parity_tests -- --nocapture

tests-time:
	@echo "Compilando Wyrm en modo release..."
	cargo build --release
	@echo "Compilando C y Bytecode (.wrc)..."
	cc -O3 $(TIME_DIR)/loop.c -o $(TIME_DIR)/loop_c
	cc -O3 $(TIME_DIR)/fib.c -o $(TIME_DIR)/fib_c
	rustc -C opt-level=3 $(TIME_DIR)/loop.rs -o $(TIME_DIR)/loop_rs
	rustc -C opt-level=3 $(TIME_DIR)/fib.rs -o $(TIME_DIR)/fib_rs
	go build -o $(TIME_DIR)/loop_go $(TIME_DIR)/loop.go
	go build -o $(TIME_DIR)/fib_go $(TIME_DIR)/fib.go
	javac $(TIME_DIR)/Loop.java $(TIME_DIR)/Fib.java
	$(WYRMC_BIN) $(TIME_DIR)/loop.wyrm -o $(TIME_DIR)/loop.wrc
	$(WYRMC_BIN) $(TIME_DIR)/fib.wyrm -o $(TIME_DIR)/fib.wrc

	@echo "\n=== Bucle de 1.000.000 iteraciones ==="

	hyperfine --warmup 3 \
		--export-markdown $(TIME_DIR)/resultados_loop.md \
		-n "C" "$(TIME_DIR)/loop_c" \
		-n "Rust" "$(TIME_DIR)/loop_rs" \
		-n "Go" "$(TIME_DIR)/loop_go" \
		-n "Java (JVM)" "java -cp $(TIME_DIR) Loop" \
		-n "Wyrm VM (.wrc)" "$(WYRM_BIN) $(TIME_DIR)/loop.wrc" \
		-n "Python3" "python3 $(TIME_DIR)/loop.py" \
		-n "Wyrm Intérprete (.wyrm)" "$(WYRM_BIN) $(TIME_DIR)/loop.wyrm"

	@echo "\n=== Recursion Fibonacci(28) ==="
	
	hyperfine --warmup 2 \
		--export-markdown $(TIME_DIR)/resultados_fib.md \
		-n "C" "$(TIME_DIR)/fib_c" \
		-n "Rust" "$(TIME_DIR)/fib_rs" \
		-n "Go" "$(TIME_DIR)/fib_go" \
		-n "Java (JVM)" "java -cp $(TIME_DIR) Fib" \
		-n "Wyrm VM (.wrc)" "$(WYRM_BIN) $(TIME_DIR)/fib.wrc" \
		-n "Python3" "python3 $(TIME_DIR)/fib.py" \
		-n "Wyrm Intérprete (.wyrm)" "$(WYRM_BIN) $(TIME_DIR)/fib.wyrm"

	@rm -f $(TIME_DIR)/loop_c $(TIME_DIR)/fib_c \
		$(TIME_DIR)/loop_rs $(TIME_DIR)/fib_rs \
		$(TIME_DIR)/loop_go $(TIME_DIR)/fib_go \
		$(TIME_DIR)/*.class $(TIME_DIR)/*.wrc

memcheck:
	@echo "Ejecutando Valgrind..."
	cargo test --no-run
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER="$(VALGRIND_RUNNER)" cargo test -- --test-threads=1

memcheck-one:
	@echo "Ejecutando Valgrind en un solo test..."
	cargo test --no-run
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER="$(VALGRIND_RUNNER)" cargo test $(TEST) -- --test-threads=1 --nocapture
	
pre-commit:
	@echo "Ejecutando pre-commits..."
	pre-commit run --all-files

clean:
	@echo "Limpiando el proyecto..."
	cargo clean
	rm vgcore.* 2> /dev/null || true
	rm core.* 2> /dev/null || true
	rm *.wyrm_history 2> /dev/null || true