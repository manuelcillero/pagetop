#!/bin/bash

# Carga variables de entorno si existe el archivo .env
if [ -f ./.env ]; then
    source ./.env
else
    echo "Warning: .env file not found. Values will be prompted if not set."
fi

# Función para solicitar una variable si no está definida, con un valor por defecto opcional
read_if_not_set() {
    local var_name=$1
    local prompt_message=$2
    local default_value=$3

    if [ -z "${!var_name}" ]; then
        if [ -n "$default_value" ]; then
            read -p "$prompt_message [default: $default_value]: " user_input
            declare -g $var_name="${user_input:-$default_value}"
        else
            read -p "$prompt_message: " user_input
            declare -g $var_name="$user_input"
        fi
    fi
}

# Función para solicitar una contraseña si no está configurada
pass_if_not_set() {
    local var_name=$1
    local prompt_message=$2

    if [ -z "${!var_name}" ]; then
        read -sp "$prompt_message: " user_input
        echo
        declare -g $var_name="$user_input"
    fi
}

# Función para solicitar las variables necesarias según el sistema de base de datos
initial_setup() {
    read_if_not_set "DB_SYSTEM" "Enter database system (mysql/psql/sqlite)" "mysql"

    case "$DB_SYSTEM" in
        mysql)
            read_if_not_set "DB_HOST" "Enter MySQL host" "localhost"
            read_if_not_set "DB_PORT" "Enter MySQL port" "3306"
            read_if_not_set "DB_NAME" "Enter database name" "database"
            read_if_not_set "DB_USER" "Enter database user" "username"
            pass_if_not_set "DB_PASS" "Enter password for database user \"$DB_USER\""
            read_if_not_set "DB_ADMIN" "Enter MySQL admin user" "root"
            pass_if_not_set "DB_ADMIN_PASS" "Enter MySQL admin password"
            # Verifica la contraseña del administrador de MySQL
            MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -e "SELECT 1;" >/dev/null 2>&1
            if [ $? -ne 0 ]; then
                echo "Error: Invalid MySQL admin password. Please check and try again."
                exit 1
            fi
            ;;
        psql)
            read_if_not_set "DB_HOST" "Enter PostgreSQL host" "localhost"
            read_if_not_set "DB_PORT" "Enter PostgreSQL port" "5432"
            read_if_not_set "DB_NAME" "Enter database name" "database"
            read_if_not_set "DB_USER" "Enter database user" "username"
            pass_if_not_set "DB_PASS" "Enter password for database user \"$DB_USER\""
            read_if_not_set "DB_ADMIN" "Enter PostgreSQL admin user" "postgres"
            pass_if_not_set "DB_ADMIN_PASS" "Enter PostgreSQL admin password"
            # Verifica la contraseña del administrador
            PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -c "\q" 2>/dev/null
            if [ $? -ne 0 ]; then
                echo "Error: Invalid PostgreSQL admin password. Please check and try again."
                exit 1
            fi
            ;;
        sqlite)
            read_if_not_set "DB_NAME" "Enter SQLite database name" "database.sqlite"
            ;;
        *)
            echo "Error: Invalid database system. Please choose either 'mysql', 'psql', or 'sqlite'."
            exit 1
            ;;
    esac
}
