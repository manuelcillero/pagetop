#!/bin/bash

create_mysql_database() {
    echo "Setting up MySQL database \"$DB_NAME\"..."

    # Verifica si la base de datos ya existe
    DB_EXISTS=$(MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -sse \
        "SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = '$DB_NAME';")
    if [ "$DB_EXISTS" == "$DB_NAME" ]; then
        echo "Database \"$DB_NAME\" already exists."
    else
        MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -e \
            "CREATE DATABASE $DB_NAME;"
        [ $? -ne 0 ] && { echo "Error creating database \"$DB_NAME\"."; return 1; }
    fi

    # Verifica si el usuario ya existe
    USER_EXISTS=$(MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -sse \
        "SELECT User FROM mysql.user WHERE User = '$DB_USER';")
    if [ "$USER_EXISTS" == "$DB_USER" ]; then
        echo "User \"$DB_USER\" already exists."
    else
        MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -e \
            "CREATE USER '$DB_USER'@'$DB_HOST' IDENTIFIED BY '$DB_PASS';"
        [ $? -ne 0 ] && { echo "Error creating user \"$DB_USER\"."; return 1; }
    fi

    # Asigna privilegios al usuario para la base de datos
    MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" -P "$DB_PORT" -e \
        "GRANT ALL PRIVILEGES ON $DB_NAME.* TO '$DB_USER'@'$DB_HOST'; FLUSH PRIVILEGES;"
    [ $? -ne 0 ] && { echo "Error granting privileges to user \"$DB_USER\" for database \"$DB_NAME\"."; return 1; }

    return 0
}

create_psql_database() {
    echo "Setting up PostgreSQL database \"$DB_NAME\"..."

    # Verifica si la base de datos ya existe
    DB_EXISTS=$(PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -tAc \
        "SELECT 1 FROM pg_database WHERE datname='$DB_NAME'")
    if [ "$DB_EXISTS" == "1" ]; then
        echo "Database \"$DB_NAME\" already exists."
    else
        PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -c "CREATE DATABASE $DB_NAME;"
        [ $? -ne 0 ] && { echo "Error creating database \"$DB_NAME\"."; return 1; }
    fi

    # Verifica si el usuario ya existe
    USER_EXISTS=$(PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -tAc \
        "SELECT 1 FROM pg_catalog.pg_user WHERE usename='$DB_USER'")
    if [ "$USER_EXISTS" == "1" ]; then
        echo "User \"$DB_USER\" already exists."
    else
        PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" -c "CREATE USER $DB_USER WITH ENCRYPTED PASSWORD '$DB_PASS';"
        [ $? -ne 0 ] && { echo "Error creating user \"$DB_USER\"."; return 1; }
    fi

    # Asigna privilegios al usuario para la base de datos
    PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" -p "$DB_PORT" <<EOF
        GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;
        GRANT pg_read_server_files TO $DB_USER;
EOF
    [ $? -ne 0 ] && { echo "Error granting privileges to user \"$DB_USER\" for database \"$DB_NAME\"."; return 1; }

    return 0
}

create_sqlite_database() {
    echo "Setting up SQLite database \"$DB_NAME\"..."

    if [ ! -f "$DB_NAME" ]; then
        sqlite3 "$DB_NAME" ".quit"
        return $?
    else
        echo "SQLite database file $DB_NAME already exists."
        return 1
    fi
}

setup_database() {
    case "$DB_SYSTEM" in
       mysql) create_mysql_database ;;
        psql) create_psql_database ;;
      sqlite) create_sqlite_database ;;
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
DEFAULT_DB_PASS="demo"
DEFAULT_DB_HOST="localhost"
DEFAULT_DB_PORT=""
DEFAULT_DB_ADMIN="root"

echo
echo "You will be prompted to provide details for creating database."
echo "Press ENTER to accept the default values."
echo

read -p "Enter database name [$DEFAULT_DB_NAME]: " DB_NAME
DB_NAME=${DB_NAME:-$DEFAULT_DB_NAME}

if [[ -z "$DB_NAME" || ! "$DB_NAME" =~ ^[a-zA-Z0-9_\-]+$ ]]; then
    echo "Invalid database name. Use only alphanumeric characters, dashes, or underscores."
    exit 1
fi

if [ "$DB_SYSTEM" == "sqlite" ]; then
    # Verifica si el archivo tiene una extensión válida para SQLite
    [[ "$DB_NAME" != *".sqlite" && "$DB_NAME" != *".db" ]] && DB_NAME="$DB_NAME.sqlite"
else
    read -p "Enter database user [$DEFAULT_DB_USER]: " DB_USER
    DB_USER=${DB_USER:-$DEFAULT_DB_USER}

    read -p "Enter database password [$DEFAULT_DB_PASS]: " DB_PASS
    DB_PASS=${DB_PASS:-$DEFAULT_DB_PASS}

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
echo "You are about to create the database \"$DB_NAME\"."
read -p "Are you sure you want to proceed? (y/N): " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 1
fi

setup_database

if [ $? -eq 0 ]; then
    echo "Database setup completed successfully."
    exit 0
else
    echo "An error occurred during database setup."
    exit 1
fi
