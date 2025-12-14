use crate::global;
use crate::service::HttpRequest;

use super::{LangId, LanguageIdentifier, Locale};

/// Representa el idioma asociado a una petición HTTP.
///
/// Determina qué idioma se usará para renderizar la respuesta asociada a una petición. También
/// indica si es necesario propagar ese idioma en los enlaces usando el parámetro de *query*
/// `?lang=...`. El comportamiento concreto depende de la política global
/// [`LangNegotiation`](crate::global::LangNegotiation) configurada en la aplicación.
///
/// El idioma resultante se expone a través del *trait* [`LangId`], de modo que pueda usarse
/// [`RequestLocale`] como cualquier otra fuente de idioma en PageTop.
pub struct RequestLocale {
    // Idioma elegido por la aplicación para esta petición, combinando la configuración, la cabecera
    // `Accept-Language` y/o el idioma de respaldo.
    base: &'static LanguageIdentifier,
    // Idioma finalmente aplicado a la petición (puede coincidir con `base` o no).
    effective: &'static LanguageIdentifier,
}

impl RequestLocale {
    /// Construye un `RequestLocale` a partir de una petición HTTP.
    ///
    /// El idioma de la petición se decide según la estrategia definida por
    /// [`LangNegotiation`](crate::global::LangNegotiation):
    ///
    /// - [`LangNegotiation::Full`](crate::global::LangNegotiation::Full) determina el idioma en
    ///   este orden:
    ///   1. Parámetro de *query* `?lang=...`, si existe y corresponde a un idioma soportado.
    ///   2. [`Locale::configured_langid()`], si la aplicación tiene un idioma por defecto válido.
    ///   3. Cabecera `Accept-Language`, si puede resolverse con [`Locale::resolve()`].
    ///   4. Idioma de respaldo.
    ///
    /// - [`LangNegotiation::NoQuery`](crate::global::LangNegotiation::NoQuery) descarta el uso del
    ///   parámetro `?lang=...` y determina el idioma en este orden:
    ///   1. [`Locale::configured_langid()`], si la aplicación tiene un idioma por defecto válido.
    ///   2. Cabecera `Accept-Language`, si puede resolverse con [`Locale::resolve()`].
    ///   3. Idioma de respaldo.
    ///
    /// - [`LangNegotiation::ConfigOnly`](crate::global::LangNegotiation::ConfigOnly) sólo usa la
    ///   configuración de la aplicación mediante [`Locale::default_langid()`], sin consultar la
    ///   cabecera `Accept-Language` ni el parámetro `?lang`. Este modo también aplica el idioma de
    ///   respaldo si es necesario.
    ///
    /// En todos los casos, el idioma resultante es siempre un [`LanguageIdentifier`] soportado por
    /// la aplicación y será el que PageTop utilice para renderizar la respuesta de la petición.
    pub fn from_request(request: Option<&HttpRequest>) -> Self {
        let mode = global::SETTINGS.app.lang_negotiation;

        // Idioma elegido por la aplicación para esta petición, antes de considerar ajustes por URL.
        let base: &'static LanguageIdentifier = match mode {
            global::LangNegotiation::ConfigOnly => {
                // Sólo configuración o, en su defecto, idioma de respaldo.
                Locale::default_langid()
            }
            global::LangNegotiation::Full | global::LangNegotiation::NoQuery => {
                if let Some(default) = Locale::configured_langid() {
                    default
                } else {
                    // Sin idioma por defecto, se evalúa la cabecera `Accept-Language`.
                    request
                        .and_then(|req| req.headers().get("Accept-Language"))
                        .and_then(|value| value.to_str().ok())
                        .and_then(|header| {
                            // Puede tener varios idiomas, p. ej. "es-ES,es;q=0.9,en;q=0.8".
                            //
                            // Y cada idioma puede aplicar un factor de calidad. Actualmente se
                            // aplica una estrategia sencilla: usar sólo el primer idioma declarado
                            // antes de la primera coma e ignorar el resto de entradas y sus
                            // factores de calidad (`q=...`).
                            let first = header.split(',').next()?.trim();

                            // En este primer elemento también puede aparecer `;q=...`, así que se
                            // extrae únicamente la etiqueta de idioma: "es-ES;q=0.9" -> "es-ES".
                            let tag = first.split(';').next()?.trim();

                            // TODO: Mejorar el soporte de `Accept-Language` en el futuro:
                            //
                            // - Parsear todos los idiomas con sus factores de calidad (`q`).
                            // - Ordenar por `q` descendente y por aparición en caso de empate.
                            // - Ignorar o tratar explícitamente el comodín `*`.
                            // - Tener en cuenta rangos de idioma (`es`, `en`, etc.) y variantes
                            //   regionales.
                            // - Añadir tests unitarios para distintas combinaciones de cabecera.
                            if tag.is_empty() {
                                None
                            } else if let Locale::Resolved(langid) = Locale::resolve(tag) {
                                Some(langid)
                            } else {
                                None
                            }
                        })
                        // Si no hay cabecera o no puede resolverse, se usa el idioma de respaldo.
                        .unwrap_or(Locale::fallback_langid())
                }
            }
        };

        // Idioma aplicado a la petición tras considerar la *query* `?lang=...`.
        let effective: &'static LanguageIdentifier = match mode {
            global::LangNegotiation::ConfigOnly | global::LangNegotiation::NoQuery => {
                // En estos modos no se permite que la URL modifique el idioma.
                base
            }
            global::LangNegotiation::Full => {
                request
                    // Se obtiene el valor de `lang` de la petición, si existe.
                    .and_then(|req| {
                        req.query_string().split('&').find_map(|pair| {
                            let mut param = pair.splitn(2, '=');
                            match (param.next(), param.next()) {
                                (Some("lang"), Some(value)) if !value.is_empty() => Some(value),
                                _ => None,
                            }
                        })
                    })
                    // Se comprueba si es un idioma soportado.
                    .and_then(|language| {
                        if let Locale::Resolved(langid) = Locale::resolve(language) {
                            Some(langid)
                        } else {
                            None
                        }
                    })
                    // Si no hay `lang` o no es válido, se usa `base`.
                    .unwrap_or(base)
            }
        };

        RequestLocale { base, effective }
    }

    /// Fuerza el idioma que se utilizará para las traducciones de esta petición.
    ///
    /// Este método permite sustituir el idioma calculado (por configuración, cabecera, `?lang`,
    /// etc.) por otro idioma. Normalmente se usa cuando quieres que toda la respuesta se genere en
    /// un idioma concreto, independientemente de cómo se haya llegado a él.
    #[inline]
    pub fn with_langid(&mut self, language: &impl LangId) -> &mut Self {
        self.effective = language.langid();
        self
    }

    /// Indica si conviene propagar `lang=...` en los enlaces generados.
    ///
    /// El comportamiento depende de la estrategia configurada en
    /// [`LangNegotiation`](crate::global::LangNegotiation):
    ///
    /// - En modo [`LangNegotiation::Full`](crate::global::LangNegotiation::Full) devuelve `true`
    ///   cuando la respuesta se está generando en un idioma distinto del que la aplicación habría
    ///   elegido automáticamente a partir de la configuración, el navegador y el idioma de
    ///   respaldo. En la práctica suele significar que el usuario ha pedido expresamente otro
    ///   idioma (por ejemplo, con `?lang=...`) o que se ha forzado con
    ///   [`with_langid()`](Self::with_langid), y por tanto es recomendable propagar `lang=...` en
    ///   los enlaces para mantener esa preferencia mientras se navega.
    ///
    /// - En modos [`LangNegotiation::NoQuery`](crate::global::LangNegotiation::NoQuery) y
    ///   [`LangNegotiation::ConfigOnly`](crate::global::LangNegotiation::ConfigOnly) siempre
    ///   devuelve `false`, ya que en estas estrategias la aplicación no utiliza el parámetro
    ///   `?lang=...` para seleccionar ni para propagar el idioma.
    #[inline]
    pub(crate) fn needs_lang_query(&self) -> bool {
        match global::SETTINGS.app.lang_negotiation {
            global::LangNegotiation::Full => self.base != self.effective,
            global::LangNegotiation::NoQuery | global::LangNegotiation::ConfigOnly => false,
        }
    }
}

/// Permite a [`RequestLocale`] actuar como proveedor de idioma.
impl LangId for RequestLocale {
    #[inline]
    fn langid(&self) -> &'static LanguageIdentifier {
        self.effective
    }
}
