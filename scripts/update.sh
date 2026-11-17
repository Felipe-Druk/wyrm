#!/usr/bin/env bash
set -e

echo "Actualizando dependencias de Wyrm.."

echo "Actualizando toolchain de Rust..."
rustup update

echo "Actualizando dependencias de Cargo..."
cargo update

echo "Actualizando hooks de pre-commit..."
if command -v pre-commit >/dev/null 2>&1 && [ -f ".pre-commit-config.yaml" ]; then
    pre-commit autoupdate
fi

echo "Wyrm actualizado correctamente"   