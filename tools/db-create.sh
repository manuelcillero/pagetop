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
echo "You are about to create the \"$DB_SYSTEM\" database \"$DB_NAME\" with access privileges to user \"$DB_USER\" on \"$DB_HOST\"."
read -p "Are you sure you want to proceed? (y/N): " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    echo "Operation cancelled."
    exit 1
fi

create_${DB_SYSTEM}_database || { echo "An error occurred during database setup."; exit 1; }

echo "Database setup completed successfully."
