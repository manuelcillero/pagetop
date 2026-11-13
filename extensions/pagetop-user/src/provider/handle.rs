use pagetop::prelude::*;
use pagetop_seaorm::db::query::{Expr, Query};
use pagetop_seaorm::db::*;

use crate::account::UserStatus;
use crate::actions::AfterUserStatusChanged;
use crate::config::SETTINGS;
use crate::entity::{role, session, user, user_identity, user_role, user_token};
use crate::error::AuthError;
use crate::identity::{self, Email, Username};
use crate::password::UNUSABLE_HASH;
use crate::provider::UserRef;
use crate::provider::policy::UserField;
use crate::service::user_admin::{
    ensure_email_available, ensure_username_available, taken_on_conflict,
};

// **< Funciones públicas de consulta >*************************************************************

/// Comprueba que `username` es un nombre de usuario válido.
///
/// Aplica la política de identidad configurada ([`IdentityPolicy`](crate::config::IdentityPolicy))
/// y lo devuelve normalizado, tal como se guardaría. No comprueba si está en uso.
pub fn validate_username(username: &str) -> Result<String, AuthError> {
    identity::username(username, SETTINGS.identity_policy, None).map(|u| u.value().to_owned())
}

/// Comprueba que `email` es una dirección válida y la devuelve tal como se guardaría.
///
/// No comprueba si está en uso.
pub fn validate_email(email: &str) -> Result<String, AuthError> {
    identity::email(email).map(|e| e.value().to_owned())
}

/// Cuenta que se busca con lo que se teclea para iniciar sesión.
///
/// Por email si lleva `@` y por nombre de usuario si no, sin distinguir mayúsculas.
pub async fn find_user_by_login(identifier: &str) -> Result<Option<UserRef>, AuthError> {
    let filter = if identifier.contains('@') {
        user::Column::EmailKey.eq(identity::email_key(identifier))
    } else {
        user::Column::UsernameKey.eq(identity::username_key(identifier))
    };
    Ok(user::Entity::find()
        .filter(filter)
        .one(dbconn())
        .await?
        .map(|user| UserRef::from(&user)))
}

/// Identificador del rol con ese nombre interno (*machine name*), si existe.
pub async fn find_role_by_machine_name(machine_name: &str) -> Result<Option<i32>, AuthError> {
    Ok(role::Entity::find()
        .filter(role::Column::MachineName.eq(machine_name))
        .one(dbconn())
        .await?
        .map(|role| role.id))
}

// **< Datos de las operaciones >*******************************************************************

/// Identidad de un proveedor guardada en una cuenta.
///
/// La devuelve [`ProviderHandle::identity_of()`].
#[derive(Clone, Debug, Getters)]
pub struct ExternalIdentity {
    /// Cuenta a la que pertenece.
    #[getters(copy)]
    user_id: i32,
    /// Identificador de la persona tal como lo da el proveedor (nulo si está pendiente).
    external_id: Option<String>,
    /// Forma normalizada del identificador, la que se busca (nula si está pendiente).
    external_key: Option<String>,
    /// Si es una marca de un administrador que permite vincular la cuenta, aún sin vincular.
    #[getters(copy)]
    pending: bool,
    /// Con `pending`, el nombre de usuario o el email (en minúsculas) con el que se espera a la
    /// persona.
    expected_key: Option<String>,
    /// Datos propios del proveedor.
    data: Option<String>,
    /// Cuándo se vinculó.
    #[getters(copy)]
    linked_at: Option<DateTime<Utc>>,
    /// Último inicio de sesión de la cuenta mientras estaba vinculada.
    #[getters(copy)]
    last_login_at: Option<DateTime<Utc>>,
}

impl From<user_identity::Model> for ExternalIdentity {
    fn from(row: user_identity::Model) -> Self {
        ExternalIdentity {
            user_id: row.user_id,
            external_id: row.external_id,
            external_key: row.external_key,
            pending: row.pending,
            expected_key: row.expected_key,
            data: row.data,
            linked_at: row.linked_at,
            last_login_at: row.last_login_at,
        }
    }
}

/// Datos de una cuenta nueva creada por un proveedor.
///
/// Usado en [`ProviderHandle::create_user()`].
#[derive(Clone, Debug)]
pub struct ExternalUserData {
    username: String,
    email: String,
    email_verified: bool,
    display_name: Option<String>,
    status: UserStatus,
    data: Option<String>,
}

impl ExternalUserData {
    /// Cuenta activa con ese nombre de usuario y email, sin verificar.
    pub fn new(username: impl Into<String>, email: impl Into<String>) -> Self {
        ExternalUserData {
            username: username.into(),
            email: email.into(),
            email_verified: false,
            display_name: None,
            status: UserStatus::Active,
            data: None,
        }
    }

    /// Da el email por verificado: el proveedor responde de él (p. ej. lo gestiona el
    /// departamento de sistemas).
    pub fn with_email_verified(mut self, verified: bool) -> Self {
        self.email_verified = verified;
        self
    }

    /// Nombre visible.
    pub fn with_display_name(mut self, display_name: impl Into<Option<String>>) -> Self {
        self.display_name = display_name.into();
        self
    }

    /// Estado inicial: [`UserStatus::Active`] (por defecto) o [`UserStatus::AwaitingApproval`].
    pub fn with_status(mut self, status: UserStatus) -> Self {
        self.status = status;
        self
    }

    /// Datos propios del proveedor para la identidad.
    pub fn with_data(mut self, data: impl Into<Option<String>>) -> Self {
        self.data = data.into();
        self
    }
}

/// Cambios en los datos de una cuenta. Usado en [`ProviderHandle::update_user()`].
///
/// Sólo se tocan los campos indicados. En los opcionales, `None` borra el valor guardado.
#[derive(AutoDefault, Clone, Debug)]
pub struct ExternalUserUpdate {
    email: Option<(String, bool)>,
    display_name: Option<Option<String>>,
    about: Option<Option<String>>,
    language: Option<Option<String>>,
    timezone: Option<Option<String>>,
    theme: Option<Option<String>>,
}

impl ExternalUserUpdate {
    /// Sin cambios.
    pub fn new() -> Self {
        Self::default()
    }

    /// Nuevo email, y si se da por verificado.
    pub fn with_email(mut self, email: impl Into<String>, verified: bool) -> Self {
        self.email = Some((email.into(), verified));
        self
    }

    /// Nuevo nombre visible.
    pub fn with_display_name(mut self, value: Option<String>) -> Self {
        self.display_name = Some(value);
        self
    }

    /// Nuevo texto «Sobre mí».
    pub fn with_about(mut self, value: Option<String>) -> Self {
        self.about = Some(value);
        self
    }

    /// Nuevo idioma (identificador BCP 47 de los que ofrece la aplicación).
    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = Some(value);
        self
    }

    /// Nueva zona horaria (identificador IANA de los que ofrece la aplicación).
    pub fn with_timezone(mut self, value: Option<String>) -> Self {
        self.timezone = Some(value);
        self
    }

    /// Nuevo tema (nombre corto de un tema habilitado).
    pub fn with_theme(mut self, value: Option<String>) -> Self {
        self.theme = Some(value);
        self
    }
}

/// Resultado de [`ProviderHandle::update_user()`].
#[derive(Debug, Default, Getters)]
pub struct UpdateReport {
    /// Si ha cambiado algún dato.
    #[getters(copy)]
    changed: bool,
    /// Campos que no se han aplicado, con el motivo (un valor no válido o un email en uso). El
    /// resto se aplica igualmente.
    rejected: Vec<(UserField, AuthError)>,
}

// **< ProviderHandle >*****************************************************************************

/// API privilegiada de `pagetop-user` para un proveedor de autenticación.
///
/// Crea cuentas, las vincula con su identidad externa, cambia sus datos, su estado y sus roles, y
/// cierra sus sesiones. Sólo la tienen los proveedores activos: `pagetop-user` la entrega a cada
/// uno, una vez, con [`AuthProvider::attach()`](super::AuthProvider::attach).
///
/// # Seguridad
///
/// **Qué protege.** Estas operaciones no son funciones públicas del *crate*, sino métodos de un
/// objeto que sólo `pagetop-user` puede construir, ligado al identificador de un proveedor. Así las
/// reglas que impiden apropiarse de una cuenta viven en un único sitio y ningún proveedor las
/// puede saltar, aunque tenga un error:
///
/// - Cada método que actúa sobre una cuenta comprueba primero que está vinculada a **este**
///   proveedor; si no, falla con [`AuthError::NotOwnedByProvider`]. Un proveedor no puede tocar
///   cuentas locales ni las de otro proveedor.
/// - Una cuenta existente sólo se vincula si un administrador lo ha permitido antes para esa
///   persona (una identidad pendiente de este proveedor), nunca si es de administrador, y al
///   vincularla deja de valer su contraseña local, se cierran sus sesiones y se anulan sus enlaces
///   de un solo uso ([`link_identity()`](Self::link_identity)).
/// - No hay forma de conceder el acceso total (`is_admin`), ni de leer o fijar contraseñas, ni de
///   abrir sesiones; los roles de sistema no se pueden asignar, y una cuenta de administrador no se
///   puede bloquear.
/// - Un proveedor declarado que no figura en
///   [`sources`](crate::config::LoginSettings::sources) no la recibe.
///
/// No tiene constructor público ni implementa `Clone`, `Default` ni `Deserialize`:
///
/// ```rust,compile_fail
/// use pagetop_user::provider::ProviderHandle;
///
/// let handle = ProviderHandle { provider: "mine" };
/// ```
///
/// **Qué no protege.** No es una barrera frente a código malicioso compilado en la misma
/// aplicación, que podría acceder a la base de datos directamente. Protege de errores, no de un
/// atacante con acceso al código.
///
/// Cada operación queda registrada en las trazas con el identificador del proveedor.
pub struct ProviderHandle {
    provider: &'static str,
}

impl ProviderHandle {
    // Sólo `pagetop-user` la construye, en `provider::initialize()`. NO hacerla pública ni añadir
    // `Clone`, `Default` o `Deserialize` al tipo: cualquier copia fuera de su proveedor permitiría
    // saltarse las comprobaciones de ámbito.
    pub(crate) fn issue(provider: &'static str) -> Self {
        ProviderHandle { provider }
    }

    /// Identificador del proveedor al que pertenece.
    pub fn provider(&self) -> &'static str {
        self.provider
    }

    // **< Consultas >******************************************************************************

    /// Cuenta vinculada a este proveedor con esa clave externa (ver
    /// [`ExternalIdentity::external_key()`]).
    pub async fn find_user(&self, external_key: &str) -> Result<Option<UserRef>, AuthError> {
        let found = user_identity::Entity::find()
            .filter(user_identity::Column::Provider.eq(self.provider))
            .filter(user_identity::Column::ExternalKey.eq(external_key))
            .filter(user_identity::Column::Pending.eq(false))
            .find_also_related(user::Entity)
            .one(dbconn())
            .await?;
        Ok(found
            .and_then(|(_, user)| user)
            .map(|user| UserRef::from(&user)))
    }

    /// Cuentas con una identidad pendiente de este proveedor que esperan a la persona con ese
    /// nombre de usuario o email (se compara sin distinguir mayúsculas). Normalmente una o
    /// ninguna; más de una es un conflicto que debe resolver el proveedor.
    pub async fn find_pending(&self, expected: &str) -> Result<Vec<UserRef>, AuthError> {
        let rows = user_identity::Entity::find()
            .filter(user_identity::Column::Provider.eq(self.provider))
            .filter(user_identity::Column::Pending.eq(true))
            .filter(user_identity::Column::ExpectedKey.eq(expected_key(expected)))
            .find_also_related(user::Entity)
            .all(dbconn())
            .await?;
        Ok(rows
            .into_iter()
            .filter_map(|(_, user)| user)
            .map(|user| UserRef::from(&user))
            .collect())
    }

    /// Cuenta vinculada a este proveedor.
    pub async fn user(&self, user_id: i32) -> Result<UserRef, AuthError> {
        self.owned(user_id).await?;
        Ok(UserRef::from(&find_user(user_id).await?))
    }

    /// Identidad de este proveedor en la cuenta, vinculada o pendiente.
    pub async fn identity_of(&self, user_id: i32) -> Result<Option<ExternalIdentity>, AuthError> {
        Ok(user_identity::Entity::find()
            .filter(user_identity::Column::Provider.eq(self.provider))
            .filter(user_identity::Column::UserId.eq(user_id))
            .one(dbconn())
            .await?
            .map(ExternalIdentity::from))
    }

    /// Cuentas vinculadas a este proveedor con alguna sesión abierta.
    pub async fn users_with_live_sessions(&self) -> Result<Vec<i32>, AuthError> {
        Ok(session::Entity::find()
            .select_only()
            .column(session::Column::UserId)
            .distinct()
            .filter(session::Column::ExpiresAt.gt(Utc::now()))
            .filter(
                session::Column::UserId.in_subquery(
                    Query::select()
                        .column(user_identity::Column::UserId)
                        .from(user_identity::Entity)
                        .and_where(user_identity::Column::Provider.eq(self.provider))
                        .and_where(user_identity::Column::Pending.eq(false))
                        .to_owned(),
                ),
            )
            .into_tuple::<i32>()
            .all(dbconn())
            .await?)
    }

    // **< Cuentas >********************************************************************************

    /// Crea una cuenta vinculada a este proveedor y devuelve su identificador.
    ///
    /// La cuenta no tiene contraseña local (no se puede entrar con ninguna). El nombre de usuario
    /// debe cumplir la política de identidad, y ni él (ni uno que se confunda con él) ni el email
    /// pueden estar en uso. El estado sólo puede ser [`UserStatus::Active`] o
    /// [`UserStatus::AwaitingApproval`]. La cuenta y su identidad se crean en una sola transacción.
    ///
    /// Falla con [`AuthError::IdentityTaken`] si otra cuenta ya está vinculada a esa identidad
    /// (p. ej. dos inicios de sesión simultáneos de la misma persona): basta con buscarla de nuevo.
    pub async fn create_user(
        &self,
        external_id: &str,
        external_key: &str,
        data: ExternalUserData,
    ) -> Result<i32, AuthError> {
        if !matches!(
            data.status,
            UserStatus::Active | UserStatus::AwaitingApproval
        ) {
            return Err(AuthError::InvalidStatus);
        }
        let username = identity::username(&data.username, SETTINGS.identity_policy, None)?;
        let email = identity::email(&data.email)?;
        ensure_username_available(&username, None).await?;
        ensure_email_available(&email, None).await?;
        if self.find_user(external_key).await?.is_some() {
            return Err(AuthError::IdentityTaken);
        }

        let now = Utc::now();
        let mut new_user = user::ActiveModel {
            id: ActiveValue::NotSet,
            email_verified_at: Set(data.email_verified.then_some(now)),
            password_hash: Set(UNUSABLE_HASH.to_owned()),
            status: Set(data.status.as_i16()),
            display_name: Set(data.display_name),
            failed_login_count: Set(0),
            is_admin: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        username.set_on(&mut new_user);
        email.set_on(&mut new_user);
        let identity = user_identity::ActiveModel {
            id: ActiveValue::NotSet,
            user_id: ActiveValue::NotSet,
            provider: Set(self.provider.to_owned()),
            external_id: Set(Some(external_id.to_owned())),
            external_key: Set(Some(external_key.to_owned())),
            pending: Set(false),
            expected_key: Set(None),
            data: Set(data.data),
            created_at: Set(now),
            linked_at: Set(Some(now)),
            last_login_at: Set(None),
        };

        let result = dbconn()
            .transaction::<_, _, AuthError>(|txn| {
                Box::pin(async move {
                    let user_id = user::Entity::insert(new_user)
                        .exec(txn)
                        .await?
                        .last_insert_id;
                    let mut identity = identity;
                    identity.user_id = Set(user_id);
                    user_identity::Entity::insert(identity).exec(txn).await?;
                    Ok(user_id)
                })
            })
            .await
            .map_err(flatten_txn_err);
        let user_id = match result {
            Ok(user_id) => user_id,
            Err(err) => {
                if self.find_user(external_key).await?.is_some() {
                    return Err(AuthError::IdentityTaken);
                }
                return Err(taken_on_conflict(err, Some(&username), &email, None).await);
            }
        };
        trace::info!(
            "Provider \"{}\" created user {user_id} ({})",
            self.provider,
            if data.status == UserStatus::Active {
                "active"
            } else {
                "awaiting approval"
            }
        );
        Ok(user_id)
    }

    /// Vincula una cuenta existente con una identidad de este proveedor.
    ///
    /// Sólo si un administrador lo ha permitido (la cuenta tiene una identidad **pendiente** de
    /// este proveedor) para esta persona, y nunca en una cuenta de administrador. `person` son los
    /// nombres de usuario o emails con los que el proveedor conoce a la persona (los mismos con los
    /// que la buscó en [`find_pending()`](Self::find_pending)); alguno debe ser el que espera la
    /// identidad pendiente, sin distinguir mayúsculas. Al vincularla, en una sola
    /// transacción, su contraseña local deja de valer, se cierran todas sus sesiones, se anulan sus
    /// enlaces de recuperación, verificación y cambio de email sin usar, y se descarta el cambio de
    /// email pendiente: quien la usara hasta ahora no debe conservar el acceso.
    ///
    /// Falla con [`AuthError::NotPendingIdentity`] si la cuenta no tiene esa identidad pendiente
    /// o espera a otra persona, con [`AuthError::ProtectedAccount`] si es de administrador y con
    /// [`AuthError::IdentityTaken`] si otra cuenta ya está vinculada a esa identidad.
    pub async fn link_identity(
        &self,
        user_id: i32,
        person: &[&str],
        external_id: &str,
        external_key: &str,
    ) -> Result<(), AuthError> {
        let Some(pending) = user_identity::Entity::find()
            .filter(user_identity::Column::Provider.eq(self.provider))
            .filter(user_identity::Column::UserId.eq(user_id))
            .filter(user_identity::Column::Pending.eq(true))
            .one(dbconn())
            .await?
        else {
            return Err(AuthError::NotPendingIdentity);
        };
        // La marca del administrador es para una persona concreta: un proveedor con un error no
        // debe poder dar la cuenta a otra.
        let expected = pending.expected_key.as_deref();
        if !person
            .iter()
            .any(|value| expected == Some(expected_key(value).as_str()))
        {
            return Err(AuthError::NotPendingIdentity);
        }
        let account = find_user(user_id).await?;
        if account.is_admin {
            return Err(AuthError::ProtectedAccount);
        }
        if self.find_user(external_key).await?.is_some() {
            return Err(AuthError::IdentityTaken);
        }

        let (external_id, external_key) = (external_id.to_owned(), external_key.to_owned());
        dbconn()
            .transaction::<_, _, AuthError>(|txn| {
                Box::pin(async move {
                    let now = Utc::now();
                    user_identity::ActiveModel {
                        id: Set(pending.id),
                        external_id: Set(Some(external_id)),
                        external_key: Set(Some(external_key)),
                        pending: Set(false),
                        expected_key: Set(None),
                        linked_at: Set(Some(now)),
                        ..Default::default()
                    }
                    .update(txn)
                    .await?;
                    take_over(txn, user_id, now).await
                })
            })
            .await
            .map_err(flatten_txn_err)?;
        trace::info!("Provider \"{}\" linked user {user_id}", self.provider);
        Ok(())
    }

    /// Anula la contraseña local de una cuenta vinculada a este proveedor, como al vincularla:
    /// cierra sus sesiones, descarta su cambio de email pendiente y anula todos sus enlaces sin
    /// usar (recuperación, verificación y cambio de email). Para cuando el proveedor vuelve a
    /// gestionar una cuenta que había liberado
    /// ([`AuthProvider::releases()`](super::AuthProvider::releases)) y que mientras tanto recibió
    /// una contraseña local.
    pub async fn disable_password(&self, user_id: i32) -> Result<(), AuthError> {
        self.owned(user_id).await?;
        dbconn()
            .transaction::<_, _, AuthError>(|txn| {
                Box::pin(async move { take_over(txn, user_id, Utc::now()).await })
            })
            .await
            .map_err(flatten_txn_err)?;
        trace::info!(
            "Provider \"{}\" disabled the local password of user {user_id}",
            self.provider
        );
        Ok(())
    }

    /// Cambia el identificador externo de una cuenta vinculada a este proveedor, y sus datos
    /// propios si se indican (p. ej. cuando la persona cambia de nombre en el proveedor).
    pub async fn update_identity(
        &self,
        user_id: i32,
        external_id: &str,
        external_key: &str,
        data: Option<String>,
    ) -> Result<(), AuthError> {
        let identity = self.owned(user_id).await?;
        let mut model = user_identity::ActiveModel {
            id: Set(identity.id),
            external_id: Set(Some(external_id.to_owned())),
            external_key: Set(Some(external_key.to_owned())),
            ..Default::default()
        };
        if data.is_some() {
            model.data = Set(data);
        }
        if let Err(err) = model.update(dbconn()).await {
            if matches!(err.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) {
                return Err(AuthError::IdentityTaken);
            }
            return Err(err.into());
        }
        trace::info!(
            "Provider \"{}\" updated the identity of user {user_id}",
            self.provider
        );
        Ok(())
    }

    /// Desvincula una cuenta de este proveedor y cierra sus sesiones, en una sola transacción. La
    /// cuenta sigue sin contraseña local: para que pueda volver a entrar, un administrador debe
    /// restablecérsela.
    pub async fn unlink_identity(&self, user_id: i32) -> Result<(), AuthError> {
        let identity = self.owned(user_id).await?;
        dbconn()
            .transaction::<_, _, AuthError>(|txn| {
                Box::pin(async move {
                    user_identity::Entity::delete_by_id(identity.id)
                        .exec(txn)
                        .await?;
                    session::Entity::delete_many()
                        .filter(session::Column::UserId.eq(user_id))
                        .exec(txn)
                        .await?;
                    Ok(())
                })
            })
            .await
            .map_err(flatten_txn_err)?;
        trace::info!(
            "Provider \"{}\" unlinked user {user_id} and closed its sessions",
            self.provider
        );
        Ok(())
    }

    /// Actualiza los datos indicados de una cuenta vinculada a este proveedor, con las mismas
    /// validaciones que el formulario del perfil. Un valor no válido (o un email que usa otra
    /// cuenta) no se aplica y se anota en el [`UpdateReport`]; el resto se aplica igualmente.
    ///
    /// Un email nuevo descarta el cambio de email que el usuario tuviera pendiente y anula los
    /// enlaces de recuperación y verificación enviados a la dirección anterior.
    pub async fn update_user(
        &self,
        user_id: i32,
        update: ExternalUserUpdate,
    ) -> Result<UpdateReport, AuthError> {
        self.owned(user_id).await?;
        let current = find_user(user_id).await?;
        let mut report = UpdateReport::default();
        let mut model = user::ActiveModel {
            id: Set(user_id),
            ..Default::default()
        };

        let mut new_email = None;
        if let Some((email, verified)) = update.email {
            match identity::email(&email) {
                Ok(email)
                    if *email.key() == current.email_key && email.value() == current.email => {}
                Ok(email) => match ensure_email_available(&email, Some(user_id)).await {
                    Ok(()) => {
                        email.set_on(&mut model);
                        if *email.key() != current.email_key {
                            model.email_verified_at = Set(verified.then(Utc::now));
                        }
                        new_email = Some(email);
                    }
                    Err(err) => report.rejected.push((UserField::Email, err)),
                },
                Err(err) => report.rejected.push((UserField::Email, err)),
            }
        }

        let mut set = |field: UserField,
                       value: Option<Option<String>>,
                       current: &Option<String>,
                       column: &mut ActiveValue<Option<String>>| {
            let Some(value) = value else { return };
            let value = value.and_then(|v| util::non_blank(&v).map(str::to_owned));
            if &value == current {
                return;
            }
            match normalized_field(field, value) {
                Ok(value) => *column = Set(value),
                Err(err) => report.rejected.push((field, err)),
            }
        };
        set(
            UserField::DisplayName,
            update.display_name,
            &current.display_name,
            &mut model.display_name,
        );
        set(
            UserField::About,
            update.about,
            &current.about,
            &mut model.about,
        );
        set(
            UserField::Language,
            update.language,
            &current.language,
            &mut model.language,
        );
        set(
            UserField::Timezone,
            update.timezone,
            &current.timezone,
            &mut model.timezone,
        );
        set(
            UserField::Theme,
            update.theme,
            &current.theme,
            &mut model.theme,
        );

        if !model.is_changed() {
            return Ok(report);
        }
        model.updated_at = Set(Utc::now());
        if let Err(err) = model.update(dbconn()).await {
            let err = match &new_email {
                Some(email) => taken_on_conflict(err.into(), None, email, Some(user_id)).await,
                None => err.into(),
            };
            return Err(err);
        }
        report.changed = true;
        trace::info!("Provider \"{}\" updated user {user_id}", self.provider);
        if let Some(email) = new_email
            && *email.key() != current.email_key
        {
            crate::auth::cancel_email_change(user_id).await?;
            crate::auth::revoke_address_tokens(user_id).await?;
            trace::info!(
                "Provider \"{}\" changed the email of user {user_id}",
                self.provider
            );
        }
        for (field, err) in &report.rejected {
            trace::warn!(
                "Provider \"{}\" could not set {} of user {user_id}: {err}",
                self.provider,
                field.as_str()
            );
        }
        Ok(report)
    }

    /// Cambia el nombre de usuario de una cuenta vinculada a este proveedor. El nuevo nombre debe
    /// cumplir la política de identidad y no estar en uso (ni uno que se confunda con él).
    pub async fn rename_user(&self, user_id: i32, new_username: &str) -> Result<(), AuthError> {
        self.owned(user_id).await?;
        let current = find_user(user_id).await?;
        let username: Username = identity::username(
            new_username,
            SETTINGS.identity_policy,
            Some(&current.username_key),
        )?;
        ensure_username_available(&username, Some(user_id)).await?;
        let mut model = user::ActiveModel {
            id: Set(user_id),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        username.set_on(&mut model);
        if let Err(err) = model.update(dbconn()).await {
            let email: Email = identity::email(&current.email)?;
            return Err(
                taken_on_conflict(err.into(), Some(&username), &email, Some(user_id)).await,
            );
        }
        trace::info!(
            "Provider \"{}\" renamed user {user_id} to \"{}\"",
            self.provider,
            username.value()
        );
        Ok(())
    }

    /// Cambia el estado de una cuenta vinculada a este proveedor. Si deja de estar activa, se
    /// cierran sus sesiones. Nunca bloquea una cuenta de administrador
    /// ([`AuthError::ProtectedAccount`]).
    pub async fn set_status(&self, user_id: i32, status: UserStatus) -> Result<(), AuthError> {
        self.owned(user_id).await?;
        let current = find_user(user_id).await?;
        if current.is_admin && status != UserStatus::Active {
            return Err(AuthError::ProtectedAccount);
        }
        let before = UserStatus::from_i16(current.status);
        if before == status {
            return Ok(());
        }
        user::ActiveModel {
            id: Set(user_id),
            status: Set(status.as_i16()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        }
        .update(dbconn())
        .await?;
        if status != UserStatus::Active {
            self.destroy_sessions(user_id).await?;
        }
        trace::info!(
            "Provider \"{}\" changed the status of user {user_id}: {before:?} -> {status:?}",
            self.provider
        );
        AfterUserStatusChanged::dispatch(user_id, before, status);
        Ok(())
    }

    /// Cierra todas las sesiones de una cuenta vinculada a este proveedor.
    pub async fn destroy_sessions(&self, user_id: i32) -> Result<(), AuthError> {
        self.owned(user_id).await?;
        crate::session::destroy_user_sessions(user_id).await?;
        trace::info!(
            "Provider \"{}\" closed the sessions of user {user_id}",
            self.provider
        );
        Ok(())
    }

    // **< Roles >**********************************************************************************

    /// Roles asignados a una cuenta vinculada a este proveedor (nunca los de sistema).
    pub async fn role_ids(&self, user_id: i32) -> Result<Vec<i32>, AuthError> {
        self.owned(user_id).await?;
        role_ids(user_id).await
    }

    /// Asigna a una cuenta vinculada a este proveedor los roles indicados que aún no tenga, y
    /// devuelve los que ha añadido. Ignora los roles de sistema y los que no existen.
    pub async fn grant_roles(&self, user_id: i32, role_ids: &[i32]) -> Result<Vec<i32>, AuthError> {
        self.owned(user_id).await?;
        let current = self::role_ids(user_id).await?;
        let mut wanted: Vec<i32> = role_ids
            .iter()
            .copied()
            .filter(|id| !crate::is_system_role(*id) && !current.contains(id))
            .collect();
        wanted.sort_unstable();
        wanted.dedup();
        if wanted.is_empty() {
            return Ok(wanted);
        }
        let existing: Vec<i32> = role::Entity::find()
            .select_only()
            .column(role::Column::Id)
            .filter(role::Column::Id.is_in(wanted))
            .into_tuple()
            .all(dbconn())
            .await?;
        if existing.is_empty() {
            return Ok(existing);
        }
        user_role::Entity::insert_many(existing.iter().map(|&role_id| user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
        }))
        .exec_without_returning(dbconn())
        .await?;
        trace::info!(
            "Provider \"{}\" granted roles {existing:?} to user {user_id}",
            self.provider
        );
        Ok(existing)
    }

    /// Retira de una cuenta vinculada a este proveedor los roles indicados que tenga, y devuelve
    /// los que ha quitado. Ignora los roles de sistema.
    pub async fn revoke_roles(
        &self,
        user_id: i32,
        role_ids: &[i32],
    ) -> Result<Vec<i32>, AuthError> {
        self.owned(user_id).await?;
        let current = self::role_ids(user_id).await?;
        let removed: Vec<i32> = current
            .into_iter()
            .filter(|id| role_ids.contains(id))
            .collect();
        if removed.is_empty() {
            return Ok(removed);
        }
        user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(user_role::Column::RoleId.is_in(removed.clone()))
            .exec(dbconn())
            .await?;
        trace::info!(
            "Provider \"{}\" revoked roles {removed:?} from user {user_id}",
            self.provider
        );
        Ok(removed)
    }

    // Identidad vinculada (no pendiente) de este proveedor en la cuenta; si no la hay, la cuenta no
    // es suya: `NotOwnedByProvider`.
    async fn owned(&self, user_id: i32) -> Result<user_identity::Model, AuthError> {
        user_identity::Entity::find()
            .filter(user_identity::Column::Provider.eq(self.provider))
            .filter(user_identity::Column::UserId.eq(user_id))
            .filter(user_identity::Column::Pending.eq(false))
            .one(dbconn())
            .await?
            .ok_or(AuthError::NotOwnedByProvider)
    }
}

// Deja la cuenta sólo en manos del proveedor: sin contraseña local, sin sesiones y sin cambio de
// email ni enlaces pendientes, que valían para la cuenta local.
async fn take_over(
    txn: &DatabaseTransaction,
    user_id: i32,
    now: DateTime<Utc>,
) -> Result<(), AuthError> {
    user::Entity::update_many()
        .col_expr(user::Column::PasswordHash, Expr::value(UNUSABLE_HASH))
        .col_expr(
            user::Column::PendingEmail,
            Expr::value(Option::<String>::None),
        )
        .col_expr(user::Column::UpdatedAt, Expr::value(now))
        .filter(user::Column::Id.eq(user_id))
        .exec(txn)
        .await?;
    session::Entity::delete_many()
        .filter(session::Column::UserId.eq(user_id))
        .exec(txn)
        .await?;
    user_token::Entity::update_many()
        .col_expr(user_token::Column::ConsumedAt, Expr::value(now))
        .col_expr(user_token::Column::UpdatedAt, Expr::value(now))
        .filter(user_token::Column::UserId.eq(user_id))
        .filter(user_token::Column::ConsumedAt.is_null())
        .exec(txn)
        .await?;
    Ok(())
}

// **< Identidades pendientes >*********************************************************************

// Clave con la que se espera a una persona en una identidad pendiente: el email o el nombre de
// usuario, normalizados como para buscar la cuenta al iniciar sesión.
pub(crate) fn expected_key(value: &str) -> String {
    identity::login_key(value)
}

// Marca la cuenta para que el proveedor pueda vincularla cuando entre la persona esperada. Sólo lo
// hace un administrador (alta de una cuenta de proveedor, o "Permitir vincular" en su ficha).
pub(crate) async fn allow_linking(
    user_id: i32,
    provider: &str,
    expected: &str,
) -> Result<(), AuthError> {
    let account = find_user(user_id).await?;
    if account.is_admin {
        return Err(AuthError::ProtectedAccount);
    }
    let existing = user_identity::Entity::find()
        .filter(user_identity::Column::Provider.eq(provider))
        .filter(user_identity::Column::UserId.eq(user_id))
        .one(dbconn())
        .await?;
    match existing {
        Some(row) if !row.pending => return Err(AuthError::IdentityTaken),
        Some(row) => {
            user_identity::ActiveModel {
                id: Set(row.id),
                expected_key: Set(Some(expected_key(expected))),
                ..Default::default()
            }
            .update(dbconn())
            .await?;
        }
        None => {
            user_identity::Entity::insert(pending_identity(user_id, provider, expected))
                .exec(dbconn())
                .await?;
        }
    }
    trace::info!("User {user_id} may be linked by provider \"{provider}\"");
    Ok(())
}

// Fila de la marca de `allow_linking()`, para insertarla también dentro de otra transacción (el
// alta de una cuenta de proveedor, ver `user_admin::create_user()`).
pub(crate) fn pending_identity(
    user_id: i32,
    provider: &str,
    expected: &str,
) -> user_identity::ActiveModel {
    user_identity::ActiveModel {
        id: ActiveValue::NotSet,
        user_id: Set(user_id),
        provider: Set(provider.to_owned()),
        external_id: Set(None),
        external_key: Set(None),
        pending: Set(true),
        expected_key: Set(Some(expected_key(expected))),
        data: Set(None),
        created_at: Set(Utc::now()),
        linked_at: Set(None),
        last_login_at: Set(None),
    }
}

// Retira la marca de `allow_linking()`, si la cuenta aún no se ha vinculado.
pub(crate) async fn cancel_linking(user_id: i32, provider: &str) -> Result<(), AuthError> {
    user_identity::Entity::delete_many()
        .filter(user_identity::Column::Provider.eq(provider))
        .filter(user_identity::Column::UserId.eq(user_id))
        .filter(user_identity::Column::Pending.eq(true))
        .exec(dbconn())
        .await?;
    Ok(())
}

// Todas las identidades (vinculadas o pendientes) de una cuenta, para la administración.
pub(crate) async fn identities_of(user_id: i32) -> Result<Vec<user_identity::Model>, AuthError> {
    Ok(user_identity::Entity::find()
        .filter(user_identity::Column::UserId.eq(user_id))
        .all(dbconn())
        .await?)
}

// **< Auxiliares >*********************************************************************************

async fn find_user(user_id: i32) -> Result<user::Model, AuthError> {
    user::Entity::find_by_id(user_id)
        .one(dbconn())
        .await?
        .ok_or(AuthError::UserNotFound)
}

async fn role_ids(user_id: i32) -> Result<Vec<i32>, AuthError> {
    Ok(user_role::Entity::find()
        .select_only()
        .column(user_role::Column::RoleId)
        .filter(user_role::Column::UserId.eq(user_id))
        .into_tuple::<i32>()
        .all(dbconn())
        .await?
        .into_iter()
        .filter(|id| !crate::is_system_role(*id))
        .collect())
}

// Valida un dato del perfil con las reglas del formulario y lo devuelve tal como se guarda.
fn normalized_field(field: UserField, value: Option<String>) -> Result<Option<String>, AuthError> {
    use crate::service::user_admin as admin;
    let to_owned = |value: Option<&str>| value.map(str::to_owned);
    match field {
        UserField::DisplayName => admin::validate_display_name(value.as_deref()).map(|()| value),
        UserField::About => {
            let about = value.map(|about| about.replace("\r\n", "\n"));
            admin::validate_about(about.as_deref()).map(|()| about)
        }
        UserField::Language => admin::validate_language(value.as_deref()).map(to_owned),
        UserField::Timezone => admin::validate_timezone(value.as_deref()).map(to_owned),
        UserField::Theme => admin::validate_theme(value.as_deref()).map(to_owned),
        UserField::Email => Ok(value),
    }
}
