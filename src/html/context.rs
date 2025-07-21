use crate::core::theme::all::{theme_by_short_name, DEFAULT_THEME};
use crate::core::theme::ThemeRef;
use crate::core::TypeInfo;
use crate::html::{html, Markup, Render};
use crate::html::{Assets, Favicon, JavaScript, StyleSheet};
use crate::join;
use crate::locale::{LanguageIdentifier, DEFAULT_LANGID};
use crate::service::HttpRequest;

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use std::fmt;

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

/// Representa el contexto asociado a un documento HTML.
///
/// Esta estructura se crea internamente para recoger información relativa al documento asociado,
/// como la solicitud HTTP de origen, el idioma, el tema para renderizar ([`ThemeRef`]), y los
/// recursos *favicon* ([`Favicon`]), las hojas de estilo ([`StyleSheet`]) y los *scripts*
/// ([`JavaScript`]). También admite parámetros de contexto definidos en tiempo de ejecución.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
/// fn configure_context(mut ctx: Context) {
///     // Establece el idioma del documento a español.
///     ctx.set_langid(LangMatch::langid_or_default("es-ES"));
///
///     // Selecciona un tema (por su nombre corto).
///     ctx.set_theme("aliner");
///
///     // Asigna un favicon.
///     ctx.set_favicon(Some(Favicon::new().with_icon("/icons/favicon.ico")));
///
///     // Añade una hoja de estilo externa.
///     ctx.add_stylesheet(StyleSheet::from("/css/style.css"));
///
///     // Añade un script JavaScript.
///     ctx.add_javascript(JavaScript::defer("/js/main.js"));
///
///     // Añade un parámetro dinámico al contexto.
///     ctx.set_param("usuario_id", 42);
///
///     // Recupera el parámetro y lo convierte a su tipo original.
///     let id: i32 = ctx.get_param("usuario_id").unwrap();
///     assert_eq!(id, 42);
///
///     // Recupera el tema seleccionado.
///     let active_theme = ctx.theme();
///     assert_eq!(active_theme.short_name(), "aliner");
///
///     // Genera un identificador único para un componente de tipo `Menu`.
///     struct Menu;
///     let unique_id = ctx.required_id::<Menu>(None);
///     assert_eq!(unique_id, "menu-1"); // Si es el primero generado.
/// }
/// ```
#[rustfmt::skip]
pub struct Context {
    request    : HttpRequest,                 // Solicitud HTTP de origen.
    langid     : &'static LanguageIdentifier, // Identificador del idioma.
    theme      : ThemeRef,                    // Referencia al tema para renderizar.
    favicon    : Option<Favicon>,             // Favicon, si se ha definido.
    stylesheets: Assets<StyleSheet>,          // Hojas de estilo CSS.
    javascripts: Assets<JavaScript>,          // Scripts JavaScript.
    params     : HashMap<String, String>,     // Parámetros definidos en tiempo de ejecución.
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
            favicon    : None,
            stylesheets: Assets::<StyleSheet>::new(),
            javascripts: Assets::<JavaScript>::new(),
            params     : HashMap::<String, String>::new(),
            id_counter : 0,
        }
    }

    /// Modifica el identificador de idioma del documento.
    pub fn set_langid(&mut self, langid: &'static LanguageIdentifier) -> &mut Self {
        self.langid = langid;
        self
    }

    /// Establece el tema que se usará para renderizar el documento.
    ///
    /// Localiza el tema por su [`short_name`](crate::core::AnyInfo::short_name), y si no aplica
    /// ninguno entonces usará el tema por defecto.
    pub fn set_theme(&mut self, short_name: impl AsRef<str>) -> &mut Self {
        self.theme = theme_by_short_name(short_name).unwrap_or(*DEFAULT_THEME);
        self
    }

    /// Define el *favicon* del documento. Sobrescribe cualquier valor anterior.
    pub fn set_favicon(&mut self, favicon: Option<Favicon>) -> &mut Self {
        self.favicon = favicon;
        self
    }

    /// Define el *favicon* solo si no se ha establecido previamente.
    pub fn set_favicon_if_none(&mut self, favicon: Favicon) -> &mut Self {
        if self.favicon.is_none() {
            self.favicon = Some(favicon);
        }
        self
    }

    /// Añade una hoja de estilos CSS al documento.
    pub fn add_stylesheet(&mut self, css: StyleSheet) -> &mut Self {
        self.stylesheets.add(css);
        self
    }

    /// Elimina una hoja de estilos por su ruta o identificador.
    pub fn remove_stylesheet(&mut self, name: impl AsRef<str>) -> &mut Self {
        self.stylesheets.remove(name);
        self
    }

    /// Añade un *script* JavaScript al documento.
    pub fn add_javascript(&mut self, js: JavaScript) -> &mut Self {
        self.javascripts.add(js);
        self
    }

    /// Elimina un *script* por su ruta o identificador.
    pub fn remove_javascript(&mut self, name: impl AsRef<str>) -> &mut Self {
        self.javascripts.remove(name);
        self
    }

    /// Añade o modifica un parámetro del contexto almacenando el valor como [`String`].
    pub fn set_param<T: ToString>(&mut self, key: impl AsRef<str>, value: T) -> &mut Self {
        self.params
            .insert(key.as_ref().to_string(), value.to_string());
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

    /// Recupera un parámetro del contexto convertido al tipo especificado.
    ///
    /// Devuelve un error si el parámetro no existe ([`ErrorParam::NotFound`]) o la conversión falla
    /// ([`ErrorParam::ParseError`]).
    pub fn get_param<T: FromStr>(&self, key: impl AsRef<str>) -> Result<T, ErrorParam> {
        self.params
            .get(key.as_ref())
            .ok_or(ErrorParam::NotFound)
            .and_then(|v| T::from_str(v).map_err(|_| ErrorParam::ParseError(v.clone())))
    }

    // Context EXTRAS ******************************************************************************

    /// Elimina un parámetro del contexto. Devuelve `true` si existía y se eliminó.
    pub fn remove_param(&mut self, key: impl AsRef<str>) -> bool {
        self.params.remove(key.as_ref()).is_some()
    }

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

impl Render for Context {
    fn render(&self) -> Markup {
        html! {
            @if let Some(favicon) = &self.favicon {
                (favicon)
            }
            (self.stylesheets)
            (self.javascripts)
        }
    }
}
