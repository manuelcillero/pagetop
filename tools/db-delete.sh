#!/bin/bash

delete_mysql_database() {
    echo "Deleting MySQL database \"$DB_NAME\" and user \"$DB_USER\"..."

    # Elimina la base de datos si existe
    MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -e \
        "DROP DATABASE IF EXISTS $DB_NAME;"
    [ $? -ne 0 ] && { echo "Error deleting database \"$DB_NAME\"."; return 1; }

    # Elimina el usuario si existe
    MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -e \
        "DROP USER IF EXISTS '$DB_USER'@'$DB_HOST'; FLUSH PRIVILEGES;"
    [ $? -ne 0 ] && { echo "Error deleting user \"$DB_USER\"."; return 1; }

    return 0
}

delete_psql_database() {
    echo "Deleting PostgreSQL database \"$DB_NAME\" and user \"$DB_USER\"..."

    # Elimina la base de datos si existe
    PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -c \
        "DROP DATABASE IF EXISTS $DB_NAME;"
    [ $? -ne 0 ] && { echo "Error deleting database \"$DB_NAME\"."; return 1; }

    # Elimina el usuario si existe
    PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -c \
        "DROP ROLE IF EXISTS $DB_USER;"
    [ $? -ne 0 ] && { echo "Error deleting user \"$DB_USER\"."; return 1; }

    return 0
}

delete_sqlite_database() {
    echo "Deleting SQLite database \"$DB_NAME\"..."

    if [ -f "$DB_NAME" ]; then
        rm "$DB_NAME"
        if [ $? -ne 0 ]; then
            echo "Error deleting SQLite database \"$DB_NAME\"."
            return 1
        fi
    else
        echo "SQLite database \"$DB_NAME\" does not exist."
        return 1
    fi

    return 0
}

# Cambia al directorio donde se localiza el script
cd "$(dirname "$0")" || exit 1

# Captura de errores no esperados
trap 'echo "An unexpected error occurred. Exiting."; exit 1' ERR

# Carga el archivo `db-setup.sh` y ejecuta la configuración inicial
if [ -f ./db-setup.sh ]; then
    source ./db-setup.sh
    initial_setup
else
    echo "Error: Required file 'db-setup.sh' not found. Please make sure it exists in the same directory."
    exit 1
fi

# Confirma antes de proceder
echo "You are about to delete the \"$DB_SYSTEM\" database \"$DB_NAME\" and user \"$DB_USER\" on \"$DB_HOST\"."
read -p "Are you sure you want to proceed? (y/N): " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 1
fi

delete_${DB_SYSTEM}_database || { echo "An error occurred during database deletion."; exit 1; }

echo "Database and user deleted successfully."
