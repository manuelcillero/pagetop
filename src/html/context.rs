use crate::core::theme::all::{theme_by_short_name, DEFAULT_THEME};
use crate::core::theme::ThemeRef;
use crate::core::TypeInfo;
use crate::html::{html, Markup};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::locale::{LanguageIdentifier, DEFAULT_LANGID};
use crate::service::HttpRequest;
use crate::{builder_fn, join};

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use std::fmt;

/// Operaciones para modificar el contexto ([`Context`]) del documento.
pub enum AssetsOp {
    // Favicon.
    /// Define el *favicon* del documento. Sobrescribe cualquier valor anterior.
    SetFavicon(Option<Favicon>),
    /// Define el *favicon* solo si no se ha establecido previamente.
    SetFaviconIfNone(Favicon),

    // Stylesheets.
    /// Añade una hoja de estilos CSS al documento.
    AddStyleSheet(StyleSheet),
    /// Elimina una hoja de estilos por su ruta o identificador.
    RemoveStyleSheet(&'static str),

    // JavaScripts.
    /// Añade un *script* JavaScript al documento.
    AddJavaScript(JavaScript),
    /// Elimina un *script* por su ruta o identificador.
    RemoveJavaScript(&'static str),
}

/// Errores de lectura o conversión de parámetros almacenados en el contexto.
#[derive(Debug)]
pub enum ErrorParam {
    /// El parámetro solicitado no existe.
    NotFound,
    /// El valor del parámetro no pudo convertirse al tipo requerido.
    ParseError(String),
}

impl fmt::Display for ErrorParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorParam::NotFound => write!(f, "Parameter not found"),
            ErrorParam::ParseError(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl Error for ErrorParam {}

/// Representa el contexto de un documento HTML.
///
/// Se crea internamente para manejar información relevante del documento, como la solicitud HTTP de
/// origen, el idioma, tema y composición para el renderizado, los recursos *favicon* ([`Favicon`]),
/// hojas de estilo ([`StyleSheet`]) y *scripts* ([`JavaScript`]), así como parámetros de contexto
/// definidos en tiempo de ejecución.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// fn configure_context(cx: &mut Context) {
///     // Establece el idioma del documento a español.
///     cx.alter_langid(LangMatch::langid_or_default("es-ES"))
///     // Selecciona un tema (por su nombre corto).
///     .alter_theme("aliner")
///     // Añade un parámetro dinámico al contexto.
///     .alter_param("usuario_id", 42)
///     // Asigna un favicon.
///     .alter_assets(AssetsOp::SetFavicon(Some(
///         Favicon::new().with_icon("/icons/favicon.ico")
///     )))
///     // Añade una hoja de estilo externa.
///     .alter_assets(AssetsOp::AddStyleSheet(
///         StyleSheet::from("/css/style.css")
///     ))
///     // Añade un script JavaScript.
///     .alter_assets(AssetsOp::AddJavaScript(
///         JavaScript::defer("/js/main.js")
///     ));
///
///     // Recupera el tema seleccionado.
///     let active_theme = cx.theme();
///     assert_eq!(active_theme.short_name(), "aliner");
///
///     // Recupera el parámetro a su tipo original.
///     let id: i32 = cx.param("usuario_id").unwrap();
///     assert_eq!(id, 42);
///
///     // Genera un identificador para un componente de tipo `Menu`.
///     struct Menu;
///     let unique_id = cx.required_id::<Menu>(None);
///     assert_eq!(unique_id, "menu-1"); // Si es el primero generado.
/// }
/// ```
#[rustfmt::skip]
pub struct Context {
    request    : HttpRequest,                 // Solicitud HTTP de origen.
    langid     : &'static LanguageIdentifier, // Identificador del idioma.
    theme      : ThemeRef,                    // Referencia al tema para renderizar.
    layout     : &'static str,                // Composición del documento para renderizar.
    params     : HashMap<String, String>,     // Parámetros definidos en tiempo de ejecución.
    favicon    : Option<Favicon>,             // Favicon, si se ha definido.
    stylesheets: Assets<StyleSheet>,          // Hojas de estilo CSS.
    javascripts: Assets<JavaScript>,          // Scripts JavaScript.
    id_counter : usize,                       // Contador para generar identificadores únicos.
}

impl Context {
    // Crea un nuevo contexto asociado a una solicitud HTTP.
    //
    // El contexto inicializa el idioma por defecto, sin favicon ni recursos cargados.
    #[rustfmt::skip]
    pub(crate) fn new(request: HttpRequest) -> Self {
        Context {
            request,
            langid     : &DEFAULT_LANGID,
            theme      : *DEFAULT_THEME,
            layout     : "default",
            params     : HashMap::<String, String>::new(),
            favicon    : None,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            id_counter : 0,
        }
    }

    // Context BUILDER *****************************************************************************

    /// Modifica el identificador de idioma del documento.
    #[builder_fn]
    pub fn with_langid(&mut self, langid: &'static LanguageIdentifier) -> &mut Self {
        self.langid = langid;
        self
    }

    /// Establece el tema que se usará para renderizar el documento.
    ///
    /// Localiza el tema por su [`short_name`](crate::core::AnyInfo::short_name), y si no aplica
    /// ninguno entonces usará el tema por defecto.
    #[builder_fn]
    pub fn with_theme(&mut self, theme_name: impl AsRef<str>) -> &mut Self {
        self.theme = theme_by_short_name(theme_name).unwrap_or(*DEFAULT_THEME);
        self
    }

    /// Define el tipo de composición usado para renderizar el documento.
    #[builder_fn]
    pub fn with_layout(&mut self, layout_name: &'static str) -> &mut Self {
        self.layout = layout_name;
        self
    }

    /// Añade o modifica un parámetro del contexto almacenando el valor como [`String`].
    #[builder_fn]
    pub fn with_param<T: ToString>(&mut self, key: impl AsRef<str>, value: T) -> &mut Self {
        self.params
            .insert(key.as_ref().to_string(), value.to_string());
        self
    }

    /// Elimina un parámetro del contexto. Devuelve `true` si existía y se eliminó.
    pub fn remove_param(&mut self, key: impl AsRef<str>) -> bool {
        self.params.remove(key.as_ref()).is_some()
    }

    /// Modifica información o recursos del contexto usando [`AssetsOp`].
    #[builder_fn]
    pub fn with_assets(&mut self, op: AssetsOp) -> &mut Self {
        match op {
            // Favicon.
            AssetsOp::SetFavicon(favicon) => {
                self.favicon = favicon;
            }
            AssetsOp::SetFaviconIfNone(icon) => {
                if self.favicon.is_none() {
                    self.favicon = Some(icon);
                }
            }
            // Stylesheets.
            AssetsOp::AddStyleSheet(css) => {
                self.stylesheets.add(css);
            }
            AssetsOp::RemoveStyleSheet(path) => {
                self.stylesheets.remove(path);
            }
            // JavaScripts.
            AssetsOp::AddJavaScript(js) => {
                self.javascripts.add(js);
            }
            AssetsOp::RemoveJavaScript(path) => {
                self.javascripts.remove(path);
            }
        }
        self
    }

    // Context GETTERS *****************************************************************************

    /// Devuelve la solicitud HTTP asociada al documento.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    /// Devuelve el identificador del idioma asociado al documento.
    pub fn langid(&self) -> &LanguageIdentifier {
        self.langid
    }

    /// Devuelve el tema que se usará para renderizar el documento.
    pub fn theme(&self) -> ThemeRef {
        self.theme
    }

    /// Devuelve el tipo de composición usado para renderizar el documento. El valor predeterminado
    /// es `"default"`.
    pub fn layout(&self) -> &str {
        self.layout
    }

    /// Recupera un parámetro del contexto convertido al tipo especificado.
    ///
    /// Devuelve un error si el parámetro no existe ([`ErrorParam::NotFound`]) o la conversión falla
    /// ([`ErrorParam::ParseError`]).
    pub fn param<T: FromStr>(&self, key: impl AsRef<str>) -> Result<T, ErrorParam> {
        self.params
            .get(key.as_ref())
            .ok_or(ErrorParam::NotFound)
            .and_then(|v| T::from_str(v).map_err(|_| ErrorParam::ParseError(v.clone())))
    }

    // Context RENDER ******************************************************************************

    /// Renderiza los recursos del contexto.
    pub fn render_assets(&self) -> Markup {
        html! {
            @if let Some(favicon) = &self.favicon {
                (favicon)
            }
            (self.stylesheets)
            (self.javascripts)
        }
    }

    // Context EXTRAS ******************************************************************************

    /// Genera un identificador único si no se proporciona uno explícito.
    ///
    /// Si no se proporciona un `id`, se genera un identificador único en la forma `<tipo>-<número>`
    /// donde `<tipo>` es el nombre corto del tipo en minúsculas (sin espacios) y `<número>` es un
    /// contador interno incremental.
    pub fn required_id<T>(&mut self, id: Option<String>) -> String {
        if let Some(id) = id {
            id
        } else {
            let prefix = TypeInfo::ShortName
                .of::<T>()
                .trim()
                .replace(' ', "_")
                .to_lowercase();
            let prefix = if prefix.is_empty() {
                "prefix".to_owned()
            } else {
                prefix
            };
            self.id_counter += 1;
            join!(prefix, "-", self.id_counter.to_string())
        }
    }
}
