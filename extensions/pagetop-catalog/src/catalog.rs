use pagetop::prelude::*;

use crate::LOCALES_CATALOG;
use crate::record::Record;

use std::fmt;

/// Función que valida un registro antes de guardarlo. Devuelve el mensaje para el usuario si no es
/// válido. Ver [`Catalog::with_validator()`].
pub type FnValidate = fn(record: &Record) -> Result<(), Lc>;

/// Función que compone el nombre con el que se identifica un registro a partir de su nombre y del
/// nombre del registro al que se refiere su calificador. Ver [`Catalog::with_qualifier()`].
pub type FnQualify = fn(name: &str, qualifier: &str) -> String;

// **< Catalog >************************************************************************************

/// Definición de un catálogo: una tabla con las columnas comunes (ver [`catalog_table()`]) y las
/// propias que describen sus [`Field`].
///
/// A partir de ella, la extensión genera la administración del catálogo en el panel: el listado
/// con búsqueda, filtro y orden, el alta, la edición, la desactivación, el borrado protegido y el
/// historial, y sus dos permisos, uno para consultarlo y otro para gestionarlo (ver
/// [`view_permission()`](Self::view_permission)). Se declara con
/// [`DeclareCatalogs`](crate::DeclareCatalogs).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// use pagetop_catalog::{Catalog, Field};
///
/// let colors = Catalog::new("colors", "shop", Lc::n("Colors"))
///     .with_permission_prefix("shop")
///     .with_field(Field::text("hex", Lc::n("Hex code")).with_max_len(7))
///     .with_field(Field::boolean("is_dark", Lc::n("Dark")).with_in_list(true));
/// ```
///
/// [`catalog_table()`]: crate::schema::catalog_table
#[derive(Getters)]
pub struct Catalog {
    /// Devuelve el nombre del catálogo, único en la aplicación. Se usa en los permisos y, si no se
    /// indica otra, como nombre de la tabla.
    #[getters(copy)]
    name: &'static str,
    /// Devuelve la tabla del catálogo.
    #[getters(copy)]
    table: &'static str,
    /// Devuelve la ruta de su listado, relativa a la raíz del panel.
    path: String,
    /// Devuelve la clave de la sección del panel en la que aparece.
    section: String,
    /// Devuelve el título del catálogo, en plural ("Colores").
    title: Lc,
    /// Devuelve la descripción del catálogo, en la página de su sección.
    description: Lc,
    /// Devuelve el peso del catálogo en su sección (menor = antes).
    #[getters(copy)]
    weight: Weight,
    /// Devuelve las columnas propias del catálogo.
    fields: Vec<Field>,
    /// Devuelve la longitud máxima del código, o 0 si el catálogo no tiene código (ver
    /// [`without_code()`](Self::without_code)).
    #[getters(copy)]
    code_max_len: usize,
    /// Devuelve los conjuntos de columnas cuyos valores no se pueden repetir.
    unique: Vec<&'static [&'static str]>,
    /// Devuelve el icono de su página en el panel, si tiene.
    icon: Option<Icon>,
    /// Devuelve el texto de ayuda al principio del formulario de alta y edición, si tiene.
    form_help: Lc,
    /// Devuelve la ayuda propia del campo del nombre, si tiene (si no, una genérica).
    name_help: Lc,
    /// Devuelve el mensaje propio para un cambio rechazado por una restricción única que el
    /// catálogo no declara, si tiene (si no, uno genérico).
    constraint_message: Lc,
    /// Devuelve si el nombre se traduce a otros idiomas (columna `name_i18n`).
    #[getters(copy)]
    translations: bool,
    #[getters(skip)]
    validator: Option<FnValidate>,
    #[getters(skip)]
    qualifier: Option<(&'static str, FnQualify)>,
    #[getters(skip)]
    permission_prefix: &'static str,
    #[getters(skip)]
    permissions: Option<(PermissionRef, PermissionRef)>,
    #[getters(skip)]
    view: CatalogPermission,
    #[getters(skip)]
    manage: CatalogPermission,
}

impl Catalog {
    /// Crea la definición con el nombre del catálogo, la clave de la sección del panel en la que
    /// aparece y su título.
    ///
    /// Por defecto, la tabla tiene el mismo nombre que el catálogo, la ruta es la de la sección
    /// seguida del nombre (`"shop/colors"`, con `-` en lugar de `_`), los permisos empiezan por
    /// `catalog` y el código admite 16 caracteres.
    pub fn new(name: &'static str, section: impl Into<String>, title: Lc) -> Self {
        debug_assert!(
            !name.is_empty()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
            "catalog name \"{name}\" must be lowercase ASCII letters, digits and underscores"
        );
        let section = section.into();
        let path = util::join!(section.as_str(), "/", name.replace('_', "-"));
        Catalog {
            name,
            table: name,
            path,
            section,
            title,
            description: Lc::none(),
            weight: 0,
            fields: Vec::new(),
            code_max_len: 16,
            unique: Vec::new(),
            icon: None,
            form_help: Lc::none(),
            name_help: Lc::none(),
            constraint_message: Lc::none(),
            translations: false,
            validator: None,
            qualifier: None,
            permission_prefix: "catalog",
            permissions: None,
            view: CatalogPermission::default(),
            manage: CatalogPermission::default(),
        }
    }

    // **< Catalog BUILDER >************************************************************************

    /// Establece la tabla, si no se llama como el catálogo.
    pub fn with_table(mut self, table: &'static str) -> Self {
        self.table = table;
        self
    }

    /// Establece la ruta del listado, relativa a la raíz del panel.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into().trim_matches('/').to_owned();
        self
    }

    /// Establece la descripción del catálogo.
    pub fn with_description(mut self, description: Lc) -> Self {
        self.description = description;
        self
    }

    /// Establece el peso del catálogo en su sección.
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    /// Establece el prefijo de las claves de sus permisos, normalmente la extensión o el área que
    /// lo declara (`"shop"` da `shop:view_colors` y `shop:manage_colors`).
    pub fn with_permission_prefix(mut self, prefix: &'static str) -> Self {
        self.permission_prefix = prefix;
        self
    }

    /// Añade una columna propia.
    pub fn with_field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Establece la longitud máxima del código, que debe coincidir con la de la columna.
    pub fn with_code_max_len(mut self, len: usize) -> Self {
        debug_assert!(len > 0, "use without_code() for a catalog without code");
        self.code_max_len = len;
        self
    }

    /// El catálogo no tiene código: sus registros se identifican por el nombre, o por el primer
    /// conjunto de columnas únicas si lo hay ([`with_unique()`](Self::with_unique)), también al
    /// importar un CSV. La tabla se crea sin la columna `code`.
    pub fn without_code(mut self) -> Self {
        self.code_max_len = 0;
        self
    }

    /// Añade un conjunto de columnas (comunes o propias) cuyos valores no se pueden repetir entre
    /// dos registros, como `&["name"]` o `&["site_id", "name"]`. Se comprueba antes de guardar,
    /// con un mensaje que nombra las columnas; la tabla debe tener además su índice único, que lo
    /// garantiza también ante dos peticiones simultáneas. El código ya es único sin declararlo.
    pub fn with_unique(mut self, columns: &'static [&'static str]) -> Self {
        self.unique.push(columns);
        self
    }

    /// Establece un texto de ayuda al principio del formulario de alta y edición, que explica lo que
    /// contiene el catálogo o cómo rellenarlo (la descripción, en cambio, se muestra en la página
    /// de su sección del panel).
    pub fn with_form_help(mut self, help: Lc) -> Self {
        self.form_help = help;
        self
    }

    /// Establece la ayuda del campo del nombre, en lugar de la genérica.
    pub fn with_name_help(mut self, help: Lc) -> Self {
        self.name_help = help;
        self
    }

    /// Establece el mensaje para un cambio que rechaza una restricción única que el catálogo no
    /// declara (normalmente de la tabla de otra extensión que depende de él, como una que exige
    /// nombres distintos en una sede), en lugar del genérico.
    pub fn with_constraint_message(mut self, message: Lc) -> Self {
        self.constraint_message = message;
        self
    }

    /// Establece el icono de su página en el panel.
    pub fn with_icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Califica el nombre con el de otro registro para identificarlo donde el nombre solo no basta
    /// (la confirmación del borrado, el historial, las opciones de otros catálogos): `column` es un
    /// campo de referencia y `qualify` compone los dos nombres, p. ej.
    /// `|name, site| format!("{name} - {site}")` para las ubicaciones de una sede.
    pub fn with_qualifier(mut self, column: &'static str, qualify: FnQualify) -> Self {
        self.qualifier = Some((column, qualify));
        self
    }

    /// Usa permisos propios de la aplicación en lugar de los dos que genera el catálogo: `view`
    /// para consultarlo y `manage` para gestionarlo (pueden ser el mismo). La aplicación los
    /// declara con [`DeclarePermissions`](pagetop::base::action::auth::DeclarePermissions).
    pub fn with_permissions(mut self, view: PermissionRef, manage: PermissionRef) -> Self {
        self.permissions = Some((view, manage));
        self
    }

    /// Indica si el nombre se traduce a otros idiomas. La tabla necesita la columna `name_i18n`
    /// (ver [`catalog_table()`](crate::schema::catalog_table)).
    pub fn with_translations(mut self, translations: bool) -> Self {
        self.translations = translations;
        self
    }

    /// Establece una validación propia, que se aplica después de comprobar los campos uno a uno.
    pub fn with_validator(mut self, validator: FnValidate) -> Self {
        self.validator = Some(validator);
        self
    }

    // **< Catalog GETTERS >************************************************************************

    /// Permiso para consultar el catálogo: ver su listado, sus registros y su historial, y
    /// exportarlo.
    pub fn view_permission(&'static self) -> PermissionRef {
        self.permissions.map_or(&self.view, |(view, _)| view)
    }

    /// Permiso para gestionar el catálogo: dar de alta, modificar, desactivar, borrar e importar.
    pub fn manage_permission(&'static self) -> PermissionRef {
        self.permissions.map_or(&self.manage, |(_, manage)| manage)
    }

    /// Si los permisos son los que genera el catálogo (y no los de la aplicación).
    pub(crate) fn has_own_permissions(&self) -> bool {
        self.permissions.is_none()
    }

    /// Si el catálogo tiene código.
    pub fn has_code(&self) -> bool {
        self.code_max_len > 0
    }

    /// Las columnas que identifican un registro: el código, o el primer conjunto de columnas
    /// únicas, o el nombre.
    pub fn key_columns(&self) -> &[&'static str] {
        if self.has_code() {
            &["code"]
        } else {
            self.unique.first().copied().unwrap_or(&["name"])
        }
    }

    pub(crate) fn qualifier(&self) -> Option<(&'static str, FnQualify)> {
        self.qualifier
    }

    /// La columna propia `column`, si existe.
    pub fn field(&self, column: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.column == column)
    }

    pub(crate) fn validator(&self) -> Option<FnValidate> {
        self.validator
    }

    // Crea los permisos, una vez fijados el nombre, el título y el prefijo.
    pub(crate) fn build_permissions(&mut self) {
        self.view = CatalogPermission {
            key: util::join!(self.permission_prefix, ":view_", self.name),
            label: Lc::t("permission-view", &LOCALES_CATALOG)
                .with_lc("catalog", self.title.clone()),
        };
        self.manage = CatalogPermission {
            key: util::join!(self.permission_prefix, ":manage_", self.name),
            label: Lc::t("permission-manage", &LOCALES_CATALOG)
                .with_lc("catalog", self.title.clone()),
        };
    }
}

impl fmt::Debug for Catalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Catalog")
            .field("name", &self.name)
            .field("table", &self.table)
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

// **< CatalogPermission >**************************************************************************

// Permiso de un catálogo, con la clave y la etiqueta calculadas a partir de su definición.
#[derive(AutoDefault)]
struct CatalogPermission {
    key: String,
    label: Lc,
}

impl Permission for CatalogPermission {
    fn key(&self) -> CowStr {
        self.key.clone().into()
    }

    fn label(&self) -> Lc {
        self.label.clone()
    }

    fn group(&self) -> &'static str {
        "catalog"
    }

    fn group_label(&self) -> Lc {
        Lc::t("permission-group", &LOCALES_CATALOG)
    }
}

// **< Field >**************************************************************************************

/// Tipo de una columna propia de un catálogo. Ver [`Field`].
#[derive(Clone, Debug)]
pub enum FieldKind {
    /// Texto de una línea (`VARCHAR`).
    Text,
    /// Texto de varias líneas (`TEXT`).
    TextArea,
    /// Sí o no (`BOOLEAN`).
    Boolean,
    /// Número entero (`INTEGER`), opcionalmente entre un mínimo y un máximo.
    Integer {
        /// Valor mínimo, si lo hay.
        min: Option<i32>,
        /// Valor máximo, si lo hay.
        max: Option<i32>,
    },
    /// Número con decimales (`DOUBLE PRECISION`, `double()` en la migración), opcionalmente entre
    /// un mínimo y un máximo. Se admite la coma o el punto como separador decimal.
    Decimal {
        /// Valor mínimo, si lo hay.
        min: Option<f64>,
        /// Valor máximo, si lo hay.
        max: Option<f64>,
    },
    /// Uno de un conjunto fijo de valores, guardado como texto: el valor y su etiqueta.
    Choice(Vec<(&'static str, Lc)>),
    /// Un registro de otro catálogo, por su nombre, guardado como su `id` (`INTEGER`). Sólo se
    /// ofrecen los registros activos.
    Reference(&'static str),
}

/// Columna propia de un catálogo, con lo necesario para pedirla en el formulario, mostrarla en el
/// listado y validarla.
///
/// Un campo opcional vacío se guarda como `NULL`, así que su columna debe admitirlo. Uno de tipo
/// [`FieldKind::Boolean`] nunca está vacío.
#[derive(Clone, Debug, Getters)]
pub struct Field {
    /// Devuelve el nombre de la columna.
    #[getters(copy)]
    column: &'static str,
    /// Devuelve la etiqueta del campo.
    label: Lc,
    /// Devuelve el texto de ayuda: qué anotar, con qué formato y si es opcional.
    help: Lc,
    /// Devuelve el tipo del campo.
    kind: FieldKind,
    /// Devuelve si el campo es obligatorio.
    #[getters(copy)]
    required: bool,
    /// Devuelve si el campo se muestra como columna en el listado.
    #[getters(copy)]
    in_list: bool,
    /// Devuelve la longitud máxima de un texto, si la hay.
    #[getters(copy)]
    max_len: Option<usize>,
    /// Devuelve si el listado se puede filtrar por este campo (referencias, opciones y sí o no).
    #[getters(copy)]
    filter: bool,
}

impl Field {
    fn new(column: &'static str, label: Lc, kind: FieldKind) -> Self {
        Field {
            column,
            label,
            help: Lc::none(),
            kind,
            required: false,
            in_list: false,
            max_len: None,
            filter: false,
        }
    }

    /// Texto de una línea.
    pub fn text(column: &'static str, label: Lc) -> Self {
        Self::new(column, label, FieldKind::Text)
    }

    /// Texto de varias líneas.
    pub fn textarea(column: &'static str, label: Lc) -> Self {
        Self::new(column, label, FieldKind::TextArea)
    }

    /// Sí o no.
    pub fn boolean(column: &'static str, label: Lc) -> Self {
        Self::new(column, label, FieldKind::Boolean)
    }

    /// Número entero.
    pub fn integer(column: &'static str, label: Lc) -> Self {
        Self::new(
            column,
            label,
            FieldKind::Integer {
                min: None,
                max: None,
            },
        )
    }

    /// Número con decimales.
    pub fn decimal(column: &'static str, label: Lc) -> Self {
        Self::new(
            column,
            label,
            FieldKind::Decimal {
                min: None,
                max: None,
            },
        )
    }

    /// Uno de los valores `options`, cada uno con su etiqueta.
    pub fn choice(column: &'static str, label: Lc, options: Vec<(&'static str, Lc)>) -> Self {
        Self::new(column, label, FieldKind::Choice(options))
    }

    /// Un registro del catálogo `catalog`.
    pub fn reference(column: &'static str, label: Lc, catalog: &'static str) -> Self {
        Self::new(column, label, FieldKind::Reference(catalog))
    }

    // **< Field BUILDER >**************************************************************************

    /// Establece el texto de ayuda.
    pub fn with_help(mut self, help: Lc) -> Self {
        self.help = help;
        self
    }

    /// Indica si el campo es obligatorio.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Indica si el campo se muestra como columna en el listado.
    pub fn with_in_list(mut self, in_list: bool) -> Self {
        self.in_list = in_list;
        self
    }

    /// Indica si el listado se puede filtrar por este campo. Sólo en las referencias, las opciones y
    /// los campos de sí o no.
    pub fn with_filter(mut self, filter: bool) -> Self {
        debug_assert!(
            matches!(
                self.kind,
                FieldKind::Reference(_) | FieldKind::Choice(_) | FieldKind::Boolean
            ),
            "with_filter() only applies to references, choices and booleans"
        );
        self.filter = filter;
        self
    }

    /// Establece la longitud máxima de un texto, que debe coincidir con la de la columna.
    pub fn with_max_len(mut self, len: usize) -> Self {
        self.max_len = Some(len);
        self
    }

    /// Establece el mínimo y el máximo de un número entero. No tiene efecto en otros tipos.
    pub fn with_range(mut self, min: impl Into<Option<i32>>, max: impl Into<Option<i32>>) -> Self {
        debug_assert!(
            matches!(self.kind, FieldKind::Integer { .. }),
            "with_range() only applies to integer fields"
        );
        if let FieldKind::Integer { .. } = self.kind {
            self.kind = FieldKind::Integer {
                min: min.into(),
                max: max.into(),
            };
        }
        self
    }

    /// Establece el mínimo y el máximo de un número con decimales. No tiene efecto en otros tipos.
    pub fn with_decimal_range(
        mut self,
        min: impl Into<Option<f64>>,
        max: impl Into<Option<f64>>,
    ) -> Self {
        debug_assert!(
            matches!(self.kind, FieldKind::Decimal { .. }),
            "with_decimal_range() only applies to decimal fields"
        );
        if let FieldKind::Decimal { .. } = self.kind {
            self.kind = FieldKind::Decimal {
                min: min.into(),
                max: max.into(),
            };
        }
        self
    }
}
