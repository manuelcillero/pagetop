#!/bin/bash

# Ask the user which database system to use
read -p "Which database system are you using? (mysql/postgresql/sqlite): " DB_SYSTEM

# Check if selected database system is installed
if [ "$DB_SYSTEM" == "mysql" ]; then
    if ! command -v mysql &> /dev/null; then
        echo "mysql is not installed or not found in PATH. Please install mysql and try again."
        exit 1
    fi
elif [ "$DB_SYSTEM" == "postgresql" ]; then
    if ! command -v psql &> /dev/null; then
        echo "postgresql is not installed or not found in PATH. Please install postgresql and try again."
        exit 1
    fi
elif [ "$DB_SYSTEM" == "sqlite" ]; then
    if ! command -v sqlite3 &> /dev/null; then
        echo "sqlite3 is not installed or not found in PATH. Please install sqlite3 and try again."
        exit 1
    fi
else
    echo "Invalid database system selected. Please choose either 'mysql', 'postgresql', or 'sqlite'."
    exit 1
fi

EXIT_CODE=0

echo
echo "You will be prompted to provide details to delete the Drust database."
echo "Press ENTER to accept the default values."
echo

if [ "$DB_SYSTEM" == "sqlite" ]; then
    DEFAULT_DB_NAME="drust.db"

    # Only prompt for database name
    read -p "Enter database name to delete [$DEFAULT_DB_NAME]: " DB_NAME
    DB_NAME=${DB_NAME:-$DEFAULT_DB_NAME}

    # For SQLite, check if the database file exists and delete it
    if [ -f "$DB_NAME" ]; then
        echo "Deleting SQLite database file: $DB_NAME"
        rm "$DB_NAME"
        EXIT_CODE=$?
    else
        echo "SQLite database file $DB_NAME does not exist."
        exit 1
    fi
else
    DEFAULT_DB_NAME="drust"
    DEFAULT_DB_USER="drust"
    DEFAULT_DB_HOST="localhost"
    DEFAULT_DB_ADMIN="root"

    # Prompt for database details, allow defaults
    read -p "Enter database name to delete [$DEFAULT_DB_NAME]: " DB_NAME
    DB_NAME=${DB_NAME:-$DEFAULT_DB_NAME}

    read -p "Enter database user to delete [$DEFAULT_DB_USER]: " DB_USER
    DB_USER=${DB_USER:-$DEFAULT_DB_USER}

    read -p "Enter database host [$DEFAULT_DB_HOST]: " DB_HOST
    DB_HOST=${DB_HOST:-$DEFAULT_DB_HOST}

    # Prompt for database system root or another privileged user's credentials
    echo
    read -p "Enter $DB_SYSTEM admin user [$DEFAULT_DB_ADMIN]: " DB_ADMIN
    DB_ADMIN=${DB_ADMIN:-$DEFAULT_DB_ADMIN}
    read -sp "Enter $DB_SYSTEM admin password: " DB_ADMIN_PASS
    echo

    # Confirm before proceeding
    echo
    echo "You are about to delete the database \"$DB_NAME\"."
    read -p "Are you sure you want to proceed? (y/N): " confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo "Operation cancelled."
        exit 1
    fi

    # Execute commands for MySQL or PostgreSQL
    if [ "$DB_SYSTEM" == "mysql" ]; then
        MYSQL_PWD="$DB_ADMIN_PASS" mysql -u "$DB_ADMIN" -h "$DB_HOST" <<EOF
            DROP DATABASE $DB_NAME;
            DROP USER '$DB_USER'@'$DB_HOST';
            FLUSH PRIVILEGES;
EOF
    elif [ "$DB_SYSTEM" == "postgresql" ]; then
        PGPASSWORD="$DB_ADMIN_PASS" psql -U "$DB_ADMIN" -h "$DB_HOST" <<EOF
            DROP DATABASE $DB_NAME;
            DROP ROLE IF EXISTS $DB_USER;
EOF
    fi
    EXIT_CODE=$?
fi

if [ $EXIT_CODE -eq 0 ]; then
    echo "Database deletion completed successfully."
else
    echo "An error occurred during database deletion. Exit code: $EXIT_CODE"
fi

# Exit the script with the mysql command's exit code
exit $EXIT_CODE
