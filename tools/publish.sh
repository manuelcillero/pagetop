#!/bin/bash

# Este script automatiza la publicación de los 'crates' del proyecto PageTop en crates.io

# Configuración global, tiempo de espera en segundos entre publicaciones
SLEEP_TIME=20

# Comprueba que las herramientas necesarias están disponibles
command -v git > /dev/null || { echo "Error: Git is not installed"; exit 1; }
command -v cargo > /dev/null || { echo "Error: Cargo is not installed"; exit 1; }

# Cambia al directorio raíz del espacio de trabajo
cd "$(dirname "$0")"
cd ..

# Verifica si el repositorio del proyecto tiene cambios locales sin preparar
if [ -n "$(git status --porcelain)" ]; then
    echo "You have local changes!"
    exit 1
fi

# Actualiza la rama 'latest' con los cambios de 'main'
read -p "Do you want to update the 'latest' branch? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "UPDATING 'latest' branch"
    git checkout latest || { echo "Error switching to 'latest'"; exit 1; }
    git merge main || { echo "Error merging 'main' into 'latest'"; exit 1; }
    echo "PUSHING updated 'latest' branch to remote repository"
    git push origin latest || { echo "Error pushing 'latest'"; exit 1; }
    git checkout main || { echo "Error switching back to 'main'"; exit 1; }
else
    read -p "Are you sure you don't want to update the 'latest' branch? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        echo "Exiting without completing the process"
        exit 1
    fi
    echo "Continuing without updating the 'latest' branch"
fi

# Función para restaurar el estado local del repositorio
function clean_up() {
    echo -e "\nCleaning local state"
    git reset HEAD --hard > /dev/null 2>&1
}

# Registra la función 'clean_up' para que se ejecute al finalizar el script, incluso tras errores
trap clean_up EXIT

# Función para publicar un 'crate' en crates.io
function publish_crate() {
    echo -e "\nPUBLISHING $CRATE"
    # Obtiene la última versión publicada en crates.io
    PUBLISHED_VERSION=$(cargo search "$CRATE " | grep "^$CRATE = " | sed -E 's/^.*"([^"]+)".*$/\1/')
    # Lee la versión actual desde Cargo.toml
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | sed -E 's/^version = "([^"]+)".*$/\1/')

    # Compara las versiones y publica si es necesario
    if [ "$PUBLISHED_VERSION" = "$CURRENT_VERSION" ]; then
        echo "Skipping version $CURRENT_VERSION as it already exists on crates.io"
    else
        echo "Publishing version $CURRENT_VERSION..."
        if [ "$CRATE" = "pagetop" ]; then
            cargo publish || { echo "Error publishing $CRATE"; exit 1; }
        else
            cp "../../LICENSE-MIT" .
            cp "../../LICENSE-APACHE" .
            git add LICENSE-MIT LICENSE-APACHE
            cargo publish --allow-dirty || { echo "Error publishing $CRATE"; exit 1; }
        fi
        sleep $SLEEP_TIME
    fi
}

# Si el 'crate' A depende del 'crate' B, entonces B debe aparecer antes que A en estas listas
HELPERS=(
    pagetop-macros
    pagetop-build
)
PACKAGES=(
    pagetop-seaorm
)

# Publica los 'crates' auxiliares
pushd helpers > /dev/null 2>&1
for CRATE in "${HELPERS[@]}"; do
    pushd "$CRATE" > /dev/null 2>&1
    publish_crate
    popd > /dev/null 2>&1
done
popd > /dev/null 2>&1

# Publica la librería principal
CRATE=pagetop; publish_crate

# Publica los paquetes del proyecto
pushd packages > /dev/null 2>&1
for CRATE in "${PACKAGES[@]}"; do
    pushd "$CRATE" > /dev/null 2>&1
    publish_crate
    popd > /dev/null 2>&1
done
popd > /dev/null 2>&1
