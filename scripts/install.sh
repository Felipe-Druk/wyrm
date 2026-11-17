#!/usr/bin/env bash
set -e

echo "  Instalando dependencias para Wyrm.."


echo "Verificando paquetes del sistema..."
if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get install -y \
        build-essential \
        clang-format \
        valgrind \
        python3 \
        python3-pip \
        pre-commit \
        golang-go \
        default-jdk \
        hyperfine
elif command -v pacman >/dev/null 2>&1; then
    sudo pacman -Sy --needed --noconfirm \
        base-devel \
        clang \
        valgrind \
        python \
        python-pre-commit \
        go \
        jdk-openjdk \
        hyperfine
elif command -v dnf >/dev/null 2>&1; then
    sudo dnf install -y \
        make \
        gcc \
        clang-tools-extra \
        valgrind \
        python3 \
        pre-commit \
        golang \
        java-latest-openjdk-devel \
        hyperfine
else
    echo "[ERROR]: Gestor de paquetes no reconocido. Instala manualmente: gcc, clang-format, valgrind, go, jdk, python3, pre-commit y hyperfine."
fi


echo "Configurando Rust (rustfmt y clippy)..."
if ! command -v rustup >/dev/null 2>&1; then
    echo "Instalando rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

rustup default stable
rustup component add clippy rustfmt

echo "Verificando herramientas de análisis y benchmarking..."
if ! command -v hyperfine >/dev/null 2>&1; then
    cargo install hyperfine
fi

if ! command -v typos >/dev/null 2>&1; then
    cargo install typos-cli
fi

echo "Configurando Git Hooks (pre-commit)..."
if [ -f ".pre-commit-config.yaml" ]; then
    pre-commit install
fi

echo " Wyrm instalado y listo para usar."