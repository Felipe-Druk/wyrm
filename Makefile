# Siempre un Makefile es important para tener ordenado el proyecto, más si vamos a usar C


.PHONY: install check build run run-v clean format help pre-commit tests

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

pre-commit:
	@echo "Ejecutando pre-commits..."
	pre-commit run --all-files

make clean:
	@echo "Limpiando el proyecto..."
	cargo clean