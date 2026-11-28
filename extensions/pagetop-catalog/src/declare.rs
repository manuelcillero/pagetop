use pagetop::prelude::*;
use pagetop_seaorm::db::DbErr;

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};

use crate::catalog::{Catalog, FieldKind};
use crate::record::ChangeRecorder;
use crate::schema::{CODE, IS_ACTIVE, NAME, WEIGHT};

// **< DeclareCatalogs >****************************************************************************

/// Tipo de función que declara catálogos en un [`CatalogBag`].
pub type FnDeclareCatalogs = fn(bag: &mut CatalogBag);

/// Catálogos declarados por las extensiones. Ver [`DeclareCatalogs`].
#[derive(AutoDefault)]
pub struct CatalogBag {
    catalogs: Vec<Catalog>,
}

impl CatalogBag {
    /// Añade un catálogo.
    pub fn add(&mut self, catalog: Catalog) {
        self.catalogs.push(catalog);
    }
}

/// Acción para declarar catálogos.
///
/// Cada extensión registra esta acción con una función que añade sus catálogos. Se despacha una
/// sola vez, al arrancar, así que los catálogos quedan fijos durante toda la vida de la
/// aplicación. Dos catálogos con el mismo nombre o la misma ruta impiden arrancar.
///
/// La sección del panel en la que aparece cada catálogo la declara la aplicación o la extensión
/// con [`DeclareAdminSections`](pagetop::base::action::admin::DeclareAdminSections).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_catalog::{Catalog, CatalogBag, DeclareCatalogs, Field};
///
/// fn declare_catalogs(bag: &mut CatalogBag) {
///     bag.add(
///         Catalog::new("colors", "shop", Lc::n("Colors"))
///             .with_permission_prefix("shop")
///             .with_field(Field::text("hex", Lc::n("Hex code")).with_max_len(7)),
///     );
/// }
///
/// struct Shop;
///
/// #[async_trait]
/// impl Extension for Shop {
///     fn dependencies(&self) -> Vec<ExtensionRef> {
///         vec![&pagetop_catalog::Catalogs]
///     }
///
///     fn actions(&self) -> Vec<ActionBox> {
///         actions![DeclareCatalogs::new(declare_catalogs)]
///     }
/// }
/// ```
pub struct DeclareCatalogs {
    f: FnDeclareCatalogs,
    weight: Weight,
}

impl ActionDispatcher for DeclareCatalogs {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareCatalogs {
    /// Registra una función que declara catálogos.
    pub fn new(f: FnDeclareCatalogs) -> Self {
        DeclareCatalogs { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< DeclareDeactivationGuards >******************************************************************

/// Resultado de una salvaguarda: `Some(motivo)` si el registro no se puede desactivar ni borrar.
pub type GuardResult = Result<Option<Lc>, DbErr>;

type GuardFn = Arc<dyn Fn(i32) -> Pin<Box<dyn Future<Output = GuardResult> + Send>> + Send + Sync>;

/// Salvaguarda que impide desactivar o borrar un registro de un catálogo mientras se usa. Ver
/// [`DeclareDeactivationGuards`].
pub struct Guard {
    catalog: &'static str,
    check: GuardFn,
}

impl Guard {
    /// Crea la salvaguarda del catálogo `catalog`. `check` recibe el `id` del registro y devuelve
    /// el motivo por el que no se puede desactivar ni borrar, o `None` si no hay ninguno.
    pub fn new<F, Fut>(catalog: &'static str, check: F) -> Self
    where
        F: Fn(i32) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = GuardResult> + Send + 'static,
    {
        Guard {
            catalog,
            check: Arc::new(move |id| Box::pin(check(id))),
        }
    }
}

/// Tipo de función que declara salvaguardas en un [`GuardBag`].
pub type FnDeclareGuards = fn(bag: &mut GuardBag);

/// Salvaguardas declaradas por las extensiones. Ver [`DeclareDeactivationGuards`].
#[derive(AutoDefault)]
pub struct GuardBag {
    guards: Vec<Guard>,
}

impl GuardBag {
    /// Añade una salvaguarda.
    pub fn add(&mut self, guard: Guard) {
        self.guards.push(guard);
    }
}

/// Acción para declarar salvaguardas de desactivación: reglas como "un color no se puede desactivar
/// mientras lo use algún producto activo".
///
/// Las claves foráneas ya impiden **borrar** un registro que otros usan, pero no **desactivarlo**,
/// y muchas reglas dependen de datos de extensiones posteriores al catálogo, que el catálogo no
/// conoce. Con esta acción es la extensión que usa el catálogo la que declara la regla, sin que el
/// catálogo dependa de ella. Antes de desactivar o borrar un registro se consultan todas las
/// salvaguardas de su catálogo, y si alguna lo impide, se muestran sus motivos.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_catalog::{DeclareDeactivationGuards, Guard, GuardBag};
///
/// async fn color_in_use(id: i32) -> pagetop_catalog::GuardResult {
///     // Consulta los productos activos con el color `id`...
///     # let in_use = id == 1;
///     Ok(in_use.then(|| Lc::n("Some active products use this color.")))
/// }
///
/// fn declare_guards(bag: &mut GuardBag) {
///     bag.add(Guard::new("colors", color_in_use));
/// }
///
/// // En Extension::actions():
/// // actions![DeclareDeactivationGuards::new(declare_guards)]
/// ```
pub struct DeclareDeactivationGuards {
    f: FnDeclareGuards,
    weight: Weight,
}

impl ActionDispatcher for DeclareDeactivationGuards {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareDeactivationGuards {
    /// Registra una función que declara salvaguardas.
    pub fn new(f: FnDeclareGuards) -> Self {
        DeclareDeactivationGuards { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< DeclareChangeRecorders >*********************************************************************

/// Tipo de función que declara registradores de cambios en un [`RecorderBag`].
pub type FnDeclareRecorders = fn(bag: &mut RecorderBag);

/// Registradores de cambios declarados por las extensiones. Ver [`DeclareChangeRecorders`].
#[derive(AutoDefault)]
pub struct RecorderBag {
    recorders: Vec<&'static dyn ChangeRecorder>,
}

impl RecorderBag {
    /// Añade un registrador.
    pub fn add(&mut self, recorder: &'static dyn ChangeRecorder) {
        self.recorders.push(recorder);
    }
}

/// Acción para declarar los registradores de los cambios de los catálogos, normalmente la pista de
/// auditoría de la aplicación. Ver [`ChangeRecorder`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_catalog::{ChangeRecorder, ChangeSet, DeclareChangeRecorders, RecorderBag};
/// use pagetop_seaorm::db::{DatabaseTransaction, DbErr};
///
/// struct AuditLog;
///
/// #[async_trait]
/// impl ChangeRecorder for AuditLog {
///     async fn record(&self, txn: &DatabaseTransaction, set: &ChangeSet<'_>) -> Result<(), DbErr> {
///         // Inserta una fila por cambio en la tabla de auditoría, con `txn`...
///         Ok(())
///     }
/// }
///
/// fn declare_recorders(bag: &mut RecorderBag) {
///     bag.add(&AuditLog);
/// }
///
/// // En Extension::actions():
/// // actions![DeclareChangeRecorders::new(declare_recorders)]
/// ```
pub struct DeclareChangeRecorders {
    f: FnDeclareRecorders,
    weight: Weight,
}

impl ActionDispatcher for DeclareChangeRecorders {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareChangeRecorders {
    /// Registra una función que declara registradores de cambios.
    pub fn new(f: FnDeclareRecorders) -> Self {
        DeclareChangeRecorders { f, weight: 0 }
    }

    /// Opcional. Las acciones de menor peso se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }
}

// **< Registry >***********************************************************************************

// Lo declarado por todas las extensiones, recogido la primera vez que se consulta. Las acciones ya
// están publicadas cuando PageTop construye el catálogo de permisos y el registro del panel, que
// son las primeras consultas.
pub(crate) struct Registry {
    pub(crate) catalogs: Vec<Catalog>,
    pub(crate) guards: Vec<Guard>,
    pub(crate) recorders: Vec<&'static dyn ChangeRecorder>,
    // Errores de declaración, que impiden arrancar (ver `Catalogs::initialize()`).
    pub(crate) errors: Vec<String>,
}

pub(crate) static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    let mut catalogs = CatalogBag::default();
    dispatch_actions(|action: &DeclareCatalogs| (action.f)(&mut catalogs));
    let mut guards = GuardBag::default();
    dispatch_actions(|action: &DeclareDeactivationGuards| (action.f)(&mut guards));
    let mut recorders = RecorderBag::default();
    dispatch_actions(|action: &DeclareChangeRecorders| (action.f)(&mut recorders));

    let mut catalogs = catalogs.catalogs;
    let errors = check(&catalogs, &guards.guards);
    for catalog in &mut catalogs {
        catalog.build_permissions();
    }
    Registry {
        catalogs,
        guards: guards.guards,
        recorders: recorders.recorders,
        errors,
    }
});

// Nombres y rutas repetidos, columnas únicas o calificadores que no existen, y referencias o
// salvaguardas a catálogos que no existen.
fn check(catalogs: &[Catalog], guards: &[Guard]) -> Vec<String> {
    let mut errors = Vec::new();
    for (i, catalog) in catalogs.iter().enumerate() {
        let earlier = &catalogs[..i];
        if earlier.iter().any(|c| c.name() == catalog.name()) {
            errors.push(format!("catalog \"{}\" is declared twice", catalog.name()));
        }
        if earlier.iter().any(|c| c.path() == catalog.path()) {
            errors.push(format!(
                "catalogs \"{}\" and another one use the same path \"{}\"",
                catalog.name(),
                catalog.path()
            ));
        }
        // Una columna única que no existe se compararía con `NULL` y nunca daría un duplicado.
        for column in catalog.unique().iter().flat_map(|columns| columns.iter()) {
            let common = [NAME, WEIGHT, IS_ACTIVE].contains(column)
                || (catalog.has_code() && *column == CODE);
            if !common && catalog.field(column).is_none() {
                errors.push(format!(
                    "unique column \"{column}\" of catalog \"{}\" does not exist",
                    catalog.name()
                ));
            }
        }
        if let Some((column, _)) = catalog.qualifier()
            && !matches!(
                catalog.field(column).map(|f| f.kind()),
                Some(FieldKind::Reference(_))
            )
        {
            errors.push(format!(
                "qualifier \"{column}\" of catalog \"{}\" is not a reference field",
                catalog.name()
            ));
        }
        for field in catalog.fields() {
            if let FieldKind::Reference(target) = field.kind()
                && !catalogs.iter().any(|c| c.name() == *target)
            {
                errors.push(format!(
                    "field \"{}\" of catalog \"{}\" refers to unknown catalog \"{target}\"",
                    field.column(),
                    catalog.name()
                ));
            }
        }
    }
    for guard in guards {
        if !catalogs.iter().any(|c| c.name() == guard.catalog) {
            errors.push(format!(
                "a deactivation guard refers to unknown catalog \"{}\"",
                guard.catalog
            ));
        }
    }
    errors
}

/// Todos los catálogos declarados.
pub fn catalogs() -> &'static [Catalog] {
    &REGISTRY.catalogs
}

/// El catálogo declarado con el nombre `name`, si existe.
pub fn catalog(name: &str) -> Option<&'static Catalog> {
    REGISTRY.catalogs.iter().find(|c| c.name() == name)
}

// Motivos por los que las salvaguardas del catálogo impiden desactivar o borrar el registro `id`.
pub(crate) async fn guard_reasons(catalog: &Catalog, id: i32) -> Result<Vec<Lc>, DbErr> {
    let mut reasons = Vec::new();
    for guard in REGISTRY
        .guards
        .iter()
        .filter(|g| g.catalog == catalog.name())
    {
        if let Some(reason) = (guard.check)(id).await? {
            reasons.push(reason);
        }
    }
    Ok(reasons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Field;

    fn errors(catalog: Catalog) -> Vec<String> {
        check(&[catalog], &[])
    }

    #[pagetop::test]
    async fn unique_columns_must_exist() {
        let catalog = || Catalog::new("sizes", "tests", Lc::n("Sizes")).without_code();
        assert!(errors(catalog().with_unique(&["name", "weight"])).is_empty());
        let typo = errors(catalog().with_unique(&["nmae"]));
        assert_eq!(typo.len(), 1, "{typo:?}");
        assert!(typo[0].contains("\"nmae\""), "{typo:?}");
        // Without code, `code` is not a column either.
        assert_eq!(errors(catalog().with_unique(&["code"])).len(), 1);
    }

    #[pagetop::test]
    async fn the_qualifier_must_be_a_reference() {
        let catalog = || {
            Catalog::new("rooms", "tests", Lc::n("Rooms"))
                .without_code()
                .with_field(Field::reference("site_id", Lc::n("Site"), "rooms"))
                .with_field(Field::text("floor", Lc::n("Floor")))
        };
        let qualify = |name: &str, _: &str| name.to_owned();
        assert!(errors(catalog().with_qualifier("site_id", qualify)).is_empty());
        assert_eq!(errors(catalog().with_qualifier("floor", qualify)).len(), 1);
        assert_eq!(
            errors(catalog().with_qualifier("missing", qualify)).len(),
            1
        );
    }
}
