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

setup_database_deletion() {
    case "$DB_SYSTEM" in
       mysql) delete_mysql_database ;;
        psql) delete_psql_database ;;
      sqlite) delete_sqlite_database ;;
           *) echo "Invalid system selected."; exit 1 ;;
    esac
}

# Configuración inicial
read -p "Which database system are you using? (mysql/psql/sqlite): " DB_SYSTEM

# Verifica si el sistema de base de datos está instalado
ERROR_DB_NOT_INSTALLED="is not installed or not found in PATH. Please install it."
if [ "$DB_SYSTEM" == "mysql" ]; then
    command -v mysql &>/dev/null || { echo "MySQL $ERROR_DB_NOT_INSTALLED"; exit 1; }
elif [ "$DB_SYSTEM" == "psql" ]; then
    command -v psql &>/dev/null || { echo "PostgreSQL $ERROR_DB_NOT_INSTALLED"; exit 1; }
elif [ "$DB_SYSTEM" == "sqlite" ]; then
    command -v sqlite3 &>/dev/null || { echo "SQLite $ERROR_DB_NOT_INSTALLED"; exit 1; }
else
    echo "Invalid database system. Please choose either 'mysql', 'psql', or 'sqlite'."
    exit 1
fi

# Parámetros básicos
DEFAULT_DB_NAME="drust"
DEFAULT_DB_USER="drust"
DEFAULT_DB_HOST="localhost"
DEFAULT_DB_PORT=""
DEFAULT_DB_ADMIN="root"

echo
echo "You will be prompted to provide details for deleting database."
echo "Press ENTER to accept the default values."
echo

read -p "Enter database name to delete [$DEFAULT_DB_NAME]: " DB_NAME
DB_NAME=${DB_NAME:-$DEFAULT_DB_NAME}

if [[ -z "$DB_NAME" || ! "$DB_NAME" =~ ^[a-zA-Z0-9_\-]+$ ]]; then
    echo "Invalid database name. Use only alphanumeric characters, dashes, or underscores."
    exit 1
fi

if [ "$DB_SYSTEM" == "sqlite" ]; then
    # Verifica si el archivo tiene una extensión válida para SQLite
    [[ "$DB_NAME" != *".sqlite" && "$DB_NAME" != *".db" ]] && DB_NAME="$DB_NAME.sqlite"
else
    read -p "Enter database user to delete [$DEFAULT_DB_USER]: " DB_USER
    DB_USER=${DB_USER:-$DEFAULT_DB_USER}

    read -p "Enter database host [$DEFAULT_DB_HOST]: " DB_HOST
    DB_HOST=${DB_HOST:-$DEFAULT_DB_HOST}

    # Puerto por defecto para MySQL
    [ "$DB_SYSTEM" == "mysql" ] && DEFAULT_DB_PORT="3306"
    # Puerto por defecto para PostgreSQL
    [ "$DB_SYSTEM" == "psql" ] && DEFAULT_DB_PORT="5432"
    read -p "Enter database port [$DEFAULT_DB_PORT]: " DB_PORT
    DB_PORT=${DB_PORT:-$DEFAULT_DB_PORT}

    read -p "Enter $DB_SYSTEM admin user [$DEFAULT_DB_ADMIN]: " DB_ADMIN
    DB_ADMIN=${DB_ADMIN:-$DEFAULT_DB_ADMIN}
    read -sp "Enter $DB_SYSTEM admin password: " DB_ADMIN_PASS
    echo
fi

# Confirmar antes de proceder
echo
echo "You are about to delete the database \"$DB_NAME\" and user \"$DB_USER\"."
read -p "Are you sure you want to proceed? (y/N): " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 1
fi

setup_database_deletion

if [ $? -eq 0 ]; then
    echo "Database and user deleted successfully."
    exit 0
else
    echo "An error occurred during database deletion."
    exit 1
fi
