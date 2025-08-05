#!/bin/bash
set -euo pipefail

# ------------------------------------------------------------------------------
# Script para publicar un crate individual del workspace con cargo-release.
# Uso:
#   ./tools/release.sh <crate> [patch|minor|major] [--execute]
# Ejemplos:
#   ./tools/release.sh pagetop-macros patch       # Dry run (por defecto)
#   ./tools/release.sh pagetop minor --execute    # Publicación real
# ------------------------------------------------------------------------------

# Configuración
CRATE="${1:-}"
LEVEL="${2:-patch}"
EXECUTE="${3:-}"
CONFIG=".cargo/release.toml"

# Comprobaciones
if [[ -z "$CRATE" ]]; then
    echo "Usage: $0 <crate> [patch|minor|major] [--execute]"
    exit 1
fi
if [[ ! "$LEVEL" =~ ^(patch|minor|major)$ ]]; then
    echo "Error: invalid level '$LEVEL'. Use: patch, minor, or major"
    exit 1
fi

# Dependencias
command -v cargo-release >/dev/null || {
    echo "Error: cargo-release is not installed. Use: cargo install cargo-release"
    exit 1
}

# Cambia al directorio del espacio
cd "$(dirname "$0")/.." || exit 1

# ------------------------------------------------------------------------------
# DRY-RUN (por defecto) o ejecución real con --execute
# ------------------------------------------------------------------------------
if [[ "$EXECUTE" != "--execute" ]]; then
    echo "Running dry-run (default mode). Add --execute to publish"
    cargo release --config "$CONFIG" --package "$CRATE" "$LEVEL"
else
    echo "Releasing $CRATE ($LEVEL)…"
    cargo release --config "$CONFIG" --package "$CRATE" "$LEVEL" --execute
    echo "Release completed."
fi
