#!/bin/bash
set -euo pipefail

# ------------------------------------------------------------------------------
# Script para generar el archivo de cambios del crate indicado.
# Uso:
#   ./tools/changelog.sh <crate> <version> [--stage]
# Ejemplo:
#   ./tools/changelog.sh pagetop-macros 0.1.0     # Sólo genera archivo
#   ./tools/changelog.sh pagetop 0.1.0 --stage    # Prepara archivo para commit
# ------------------------------------------------------------------------------

# Configuración
CRATE="${1:-}"
VERSION="${2:-}"
STAGE="${3:-}"
CLIFF_CONFIG=".cargo/cliff.toml"

# Comprobaciones
if [[ -z "$CRATE" || -z "$VERSION" ]]; then
    echo "Usage: $0 <crate> <version> [--stage]" >&2
    exit 1
fi

# Dependencias
command -v git-cliff >/dev/null || {
    echo "Error: git-cliff is not installed. Use: cargo install git-cliff"
    exit 1
}

# Cambia al directorio del espacio
cd "$(dirname "$0")/.." || exit 1

# ------------------------------------------------------------------------------
# Determina ruta del archivo y ámbito de los archivos afectados para el crate
# ------------------------------------------------------------------------------
case "$CRATE" in
    pagetop-statics)
        CHANGELOG_FILE="helpers/pagetop-statics/CHANGELOG.md"
        PATH_FLAGS=(--include-path "helpers/pagetop-statics/**/*")
        ;;
    pagetop-build)
        CHANGELOG_FILE="helpers/pagetop-build/CHANGELOG.md"
        PATH_FLAGS=(--include-path "helpers/pagetop-build/**/*")
        ;;
    pagetop-macros)
        CHANGELOG_FILE="helpers/pagetop-macros/CHANGELOG.md"
        PATH_FLAGS=(--include-path "helpers/pagetop-macros/**/*")
        ;;
    pagetop)
        CHANGELOG_FILE="CHANGELOG.md"
        PATH_FLAGS=(
            --exclude-path "helpers/pagetop-statics/**/*"
            --exclude-path "helpers/pagetop-build/**/*"
            --exclude-path "helpers/pagetop-macros/**/*"
        )
        ;;
    *)
        echo "Error: unsupported crate '$CRATE'" >&2
        exit 1
        ;;
esac

# ------------------------------------------------------------------------------
# Genera el CHANGELOG para el crate correspondiente
# ------------------------------------------------------------------------------
if [[ -f "$CHANGELOG_FILE" ]]; then
    # Archivo existe: inserta la nueva sección arriba
    OUTPUT_FLAG=(--prepend "$CHANGELOG_FILE")
else
    # Primera vez: crea el fichero desde cero
    OUTPUT_FLAG=(-o "$CHANGELOG_FILE")
fi
COMMON_ARGS=(
    --config "$CLIFF_CONFIG"
    "${PATH_FLAGS[@]}"
    --tag-pattern "^${CRATE}-v"
    --tag "$VERSION"
    "${OUTPUT_FLAG[@]}"
)
LAST_TAG="$(git tag --list "${CRATE}-v*" --sort=-v:refname | head -n 1)"
if [[ -n "$LAST_TAG" ]]; then
    echo "Generating CHANGELOG for '$CRATE' from tag '$LAST_TAG'"
else
    echo "Generating initial CHANGELOG for '$CRATE'"
fi
git-cliff --unreleased "${COMMON_ARGS[@]}"
echo "CHANGELOG generated at '$CHANGELOG_FILE'"

# Pregunta por la revisión del archivo de cambios generado
read -p "Do you want to review the changelog before continuing? (y/n) " -r || exit 1
echo
if [[ "$REPLY" =~ ^[Yy]$ ]]; then
    ${EDITOR:-nano} "$CHANGELOG_FILE"
fi
read -p "Do you want to proceed with the release of $CRATE? (y/n) " -r || exit 1
echo
if [[ ! "$REPLY" =~ ^[Yy]$ ]]; then
    echo "Aborting release process." >&2
    exit 1
fi

# Si hay cambios y procede, añade al stage (cargo-release hará el commit)
if [[ -n $(git status --porcelain -- "$CHANGELOG_FILE") ]]; then
    if [[ "$STAGE" == "--stage" ]]; then
        git add "$CHANGELOG_FILE"
        echo "Staged $CHANGELOG_FILE for commit"
    else
        echo "Changes detected in '$CHANGELOG_FILE', but not staged (no --stage flag)"
    fi
else
    echo "No changes in '$CHANGELOG_FILE', skipping staging"
fi
