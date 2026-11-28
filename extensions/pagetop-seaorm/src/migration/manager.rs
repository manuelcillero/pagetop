use super::{IntoSchemaManagerConnection, SchemaManagerConnection};
use sea_orm::sea_query::IntoIden;
use sea_orm::sea_query::{
    ForeignKeyCreateStatement, ForeignKeyDropStatement, IndexCreateStatement, IndexDropStatement,
    SelectStatement, TableAlterStatement, TableCreateStatement, TableDropStatement,
    TableRenameStatement, TableTruncateStatement,
    extension::postgres::{TypeAlterStatement, TypeCreateStatement, TypeDropStatement},
};
use sea_orm::{ConnectionTrait, DbBackend, DbErr, Statement, StatementBuilder};
#[allow(unused_imports)]
use sea_schema::probe::SchemaProbe;

/// Helper struct for writing migration scripts in migration file
pub struct SchemaManager<'c> {
    conn: SchemaManagerConnection<'c>,
}

impl<'c> SchemaManager<'c> {
    pub fn new<T>(conn: T) -> Self
    where
        T: IntoSchemaManagerConnection<'c>,
    {
        Self {
            conn: conn.into_schema_manager_connection(),
        }
    }

    pub async fn exec_stmt<S>(&self, stmt: S) -> Result<(), DbErr>
    where
        S: StatementBuilder,
    {
        let builder = self.conn.get_database_backend();
        self.conn.execute(builder.build(&stmt)).await.map(|_| ())
    }

    pub fn get_database_backend(&self) -> DbBackend {
        self.conn.get_database_backend()
    }

    pub fn get_connection(&self) -> &SchemaManagerConnection<'c> {
        &self.conn
    }
}

/// Schema Creation
impl SchemaManager<'_> {
    pub async fn create_table(&self, stmt: TableCreateStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn create_index(&self, stmt: IndexCreateStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn create_foreign_key(&self, stmt: ForeignKeyCreateStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn create_type(&self, stmt: TypeCreateStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }
}

/// Schema Mutation
impl SchemaManager<'_> {
    pub async fn alter_table(&self, stmt: TableAlterStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn drop_table(&self, stmt: TableDropStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn rename_table(&self, stmt: TableRenameStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn truncate_table(&self, stmt: TableTruncateStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn drop_index(&self, stmt: IndexDropStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn drop_foreign_key(&self, stmt: ForeignKeyDropStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn alter_type(&self, stmt: TypeAlterStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }

    pub async fn drop_type(&self, stmt: TypeDropStatement) -> Result<(), DbErr> {
        self.exec_stmt(stmt).await
    }
}

/// Permissions.
impl SchemaManager<'_> {
    /// Deja la tabla en sólo inserción para el usuario de la aplicación: podrá leer y añadir filas,
    /// pero no modificarlas ni borrarlas. Es lo que exige, por ejemplo, una pista de auditoría.
    ///
    /// Se llama en la migración que crea la tabla, después de crearla. La protección la da la base
    /// de datos, así que vale también frente a un error del código o una consulta inyectada.
    ///
    /// Sólo tiene efecto en **PostgreSQL** y con un usuario de migración propio
    /// ([`migration_user`](crate::config::DatabaseSettings::migration_user)): quita a `db_user`
    /// los permisos `UPDATE`, `DELETE`, `TRUNCATE` y `TRIGGER` sobre la tabla (con un disparador
    /// podría alterar las filas que se van a insertar). Los permisos se retiran a `db_user`, así
    /// que deben habérsele concedido a él, no a un rol del que sea miembro ni a `PUBLIC`; si
    /// después de retirarlos aún conserva alguno, avisa en las trazas. Si la aplicación se conecta
    /// con el mismo usuario que creó las tablas, éste puede volver a concederse cualquier permiso,
    /// así que no hace nada y también avisa. En MySQL tampoco: allí un permiso concedido para toda
    /// la base de datos no se puede retirar tabla a tabla.
    ///
    /// ```rust,no_run
    /// use pagetop_seaorm::migration::*;
    ///
    /// # async fn up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    /// manager
    ///     .create_table(
    ///         Table::create()
    ///             .table(AuditLog::Table)
    ///             .col(pk_uuid(AuditLog::Id))
    ///             .col(text(AuditLog::Detail))
    ///             .to_owned(),
    ///     )
    ///     .await?;
    /// manager.append_only(AuditLog::Table).await
    /// # }
    /// #[derive(DeriveIden)]
    /// enum AuditLog {
    ///     Table,
    ///     Id,
    ///     Detail,
    /// }
    /// ```
    pub async fn append_only(&self, table: impl IntoIden) -> Result<(), DbErr> {
        let table = table.into_iden().to_string();
        let settings = &*crate::config::SETTINGS;
        if self.get_database_backend() != DbBackend::Postgres || !settings.has_migration_user() {
            pagetop::trace::warn!(
                "Table \"{table}\" is not protected against updates and deletes: it needs \
                 PostgreSQL and a migration_user different from db_user"
            );
            return Ok(());
        }
        let quoted = quote_pg(&table);
        let sql = format!(
            "REVOKE {PROTECTED} ON {quoted} FROM {}",
            quote_pg(&settings.db_user)
        );
        self.conn.execute_unprepared(&sql).await?;

        // Un permiso que le llega por un rol o por `PUBLIC` no se retira con el `REVOKE` anterior.
        let still = self
            .conn
            .query_one(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "SELECT has_table_privilege($1, $2, $3) AS still",
                [
                    settings.db_user.as_str().into(),
                    quoted.into(),
                    PROTECTED.into(),
                ],
            ))
            .await?
            .map(|row| row.try_get::<bool>("", "still"))
            .transpose()?
            .unwrap_or(false);
        if still {
            pagetop::trace::warn!(
                "Table \"{table}\" is not protected against updates and deletes: \"{}\" keeps \
                 some of {PROTECTED} through a role or PUBLIC",
                settings.db_user
            );
        }
        Ok(())
    }
}

// Permisos que `append_only()` retira al usuario de la aplicación.
const PROTECTED: &str = "UPDATE, DELETE, TRUNCATE, TRIGGER";

// Identificador entre comillas dobles para PostgreSQL, con las comillas internas duplicadas.
fn quote_pg(ident: &str) -> String {
    let mut quoted = String::with_capacity(ident.len() + 2);
    quoted.push('"');
    quoted.push_str(&ident.replace('"', "\"\""));
    quoted.push('"');
    quoted
}

/// Schema Inspection.
impl SchemaManager<'_> {
    pub async fn has_table<T>(&self, table: T) -> Result<bool, DbErr>
    where
        T: AsRef<str>,
    {
        has_table(&self.conn, table).await
    }

    pub async fn has_column<T, C>(&self, _table: T, _column: C) -> Result<bool, DbErr>
    where
        T: AsRef<str>,
        C: AsRef<str>,
    {
        let _stmt: SelectStatement = match self.conn.get_database_backend() {
            #[cfg(feature = "mysql")]
            DbBackend::MySql => sea_schema::mysql::MySql.has_column(_table, _column),
            #[cfg(feature = "postgres")]
            DbBackend::Postgres => sea_schema::postgres::Postgres.has_column(_table, _column),
            #[cfg(feature = "sqlite")]
            DbBackend::Sqlite => sea_schema::sqlite::Sqlite.has_column(_table, _column),
            #[allow(unreachable_patterns)]
            other => panic!("{other:?} feature is off"),
        };

        #[allow(unreachable_code)]
        let builder = self.conn.get_database_backend();
        let res = self
            .conn
            .query_one(builder.build(&_stmt))
            .await?
            .ok_or_else(|| DbErr::Custom("Failed to check column exists".to_owned()))?;

        res.try_get("", "has_column")
    }

    pub async fn has_index<T, I>(&self, _table: T, _index: I) -> Result<bool, DbErr>
    where
        T: AsRef<str>,
        I: AsRef<str>,
    {
        let _stmt: SelectStatement = match self.conn.get_database_backend() {
            #[cfg(feature = "mysql")]
            DbBackend::MySql => sea_schema::mysql::MySql.has_index(_table, _index),
            #[cfg(feature = "postgres")]
            DbBackend::Postgres => sea_schema::postgres::Postgres.has_index(_table, _index),
            #[cfg(feature = "sqlite")]
            DbBackend::Sqlite => sea_schema::sqlite::Sqlite.has_index(_table, _index),
            #[allow(unreachable_patterns)]
            other => panic!("{other:?} feature is off"),
        };

        #[allow(unreachable_code)]
        let builder = self.conn.get_database_backend();
        let res = self
            .conn
            .query_one(builder.build(&_stmt))
            .await?
            .ok_or_else(|| DbErr::Custom("Failed to check index exists".to_owned()))?;

        res.try_get("", "has_index")
    }
}

pub(crate) async fn has_table<C, T>(conn: &C, _table: T) -> Result<bool, DbErr>
where
    C: ConnectionTrait,
    T: AsRef<str>,
{
    let _stmt: SelectStatement = match conn.get_database_backend() {
        #[cfg(feature = "mysql")]
        DbBackend::MySql => sea_schema::mysql::MySql.has_table(_table),
        #[cfg(feature = "postgres")]
        DbBackend::Postgres => sea_schema::postgres::Postgres.has_table(_table),
        #[cfg(feature = "sqlite")]
        DbBackend::Sqlite => sea_schema::sqlite::Sqlite.has_table(_table),
        #[allow(unreachable_patterns)]
        other => panic!("{other:?} feature is off"),
    };

    #[allow(unreachable_code)]
    let builder = conn.get_database_backend();
    let res = conn
        .query_one(builder.build(&_stmt))
        .await?
        .ok_or_else(|| DbErr::Custom("Failed to check table exists".to_owned()))?;

    res.try_get("", "has_table")
}

#[cfg(test)]
mod tests {
    use super::quote_pg;

    #[test]
    fn quote_pg_doubles_inner_quotes() {
        assert_eq!(quote_pg("audit_log"), "\"audit_log\"");
        assert_eq!(quote_pg("we\"ird"), "\"we\"\"ird\"");
    }
}
