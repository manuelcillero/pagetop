use crate::prelude::*;

// Tipo de introducción, fijado al construirla: la presentación estándar de PageTop (con párrafos y
// los datos de la versión) o una completamente personalizada.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    PageTop,
    Custom,
}

// **< Intro >**************************************************************************************

/// Componente para divulgar PageTop (como hace [`Welcome`](crate::base::extension::Welcome)), o
/// mostrar presentaciones.
///
/// Usa la imagen de PageTop, con estilos propios que no dependen del tema activo, para mostrar:
///
/// - Una **cabecera** oscura a todo el ancho, con la ilustración de PageTop difuminada y la
///   *mascota* de PageTop.
/// - Una vista destacada del **título** de la página con un **eslogan** de presentación.
/// - Un **botón opcional** de llamada a la acción con texto y enlace configurables. Si el texto
///   empieza con una parte en `<strong>`, ésta se muestra como línea principal del botón.
/// - Un **área para la presentación de contenidos**, con la presentación de PageTop y los datos de
///   la versión en marcha (salvo con [`Intro::custom()`]) y bloques
///   ([`Block`](crate::base::component::Block)) de contenido libre.
///
/// Los párrafos de los contenidos que tengan la clase `.intro-text-lead` se mostrarán con una
/// tipografía ampliada, ideal para presentaciones breves e impactantes.
///
/// # Ejemplos
///
/// **Intro mínima por defecto**
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let intro = Intro::default();
/// ```
///
/// **Título, eslogan y botón personalizados**
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let intro = Intro::default()
///     .with_title(Lc::l("intro_custom_title"))
///     .with_slogan(Lc::l("intro_custom_slogan"))
///     .with_button(Some((
///         Lc::l("intro_learn_more"),
///         "/learn-more".into()
///     )));
/// ```
///
/// **Modo *Custom* con título y botón propios**
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let intro = Intro::custom()
///     .with_title(Lc::l("intro_custom_title"))
///     .with_button(Some((Lc::l("intro_learn_more"), "/learn-more".into())));
/// ```
///
/// **Añadir contenidos hijo**
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let intro = Intro::default()
///     .with_child(
///         Block::new()
///             .with_title(Lc::l("intro_custom_block_title"))
///             .with_child(Html::with(move |cx| {
///                 html! {
///                     p class="intro-text-lead" {
///                         (Lc::l("intro_custom_paragraph_1").using(cx))
///                     }
///                     p { (Lc::l("intro_custom_paragraph_2").using(cx)) }
///                 }
///             })),
///     );
/// ```
#[derive(Clone, Debug, Getters)]
pub struct Intro {
    /// Devuelve el título de entrada.
    title: Lc,
    /// Devuelve el eslogan de la entrada.
    slogan: Lc,
    /// Devuelve el botón de llamada a la acción, si existe.
    button: Option<(Lc, Route)>,
    /// Devuelve si el botón de llamada a la acción se abre en una nueva ventana o pestaña.
    button_blank: bool,
    #[getters(skip)]
    kind: Kind,
    /// Devuelve la lista de componentes hijo de la intro.
    children: Children,
}

impl Default for Intro {
    fn default() -> Self {
        const BUTTON_LINK: &str = "https://pagetop.cillero.es";

        Intro {
            title: Lc::l("intro_default_title"),
            slogan: Lc::l("intro_default_slogan").with_arg("app", &global::SETTINGS.app.name),
            button: Some((Lc::l("intro_default_button"), BUTTON_LINK.into())),
            button_blank: true,
            kind: Kind::PageTop,
            children: Children::default(),
        }
    }
}

#[async_trait]
impl Component for Intro {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        cx.alter_assets(
            StyleSheet::from("/pagetop/css/intro.min.css").with_version(PAGETOP_VERSION),
        );
        let title = self.title().using(cx);
        let slogan = self.slogan().using(cx);

        let button = self.button().as_ref().map(|(txt, lnk)| {
            html! {
                div class="intro-button" {
                    a
                        class="intro-button-link"
                        href=(lnk.resolve(cx))
                        target=[self.button_blank().then_some("_blank")]
                        rel=[self.button_blank().then_some("noopener noreferrer")]
                    {
                        span class="intro-button-text" { (txt.using(cx)) }
                        span class="intro-button-arrow" aria-hidden="true" {
                            svg viewBox="0 0 24 24" width="24" height="24" fill="none"
                                stroke="currentColor" stroke-width="2.5" stroke-linecap="round"
                                stroke-linejoin="round"
                            {
                                path d="M5 12h14M13 6l6 6-6 6" {}
                            }
                        }
                    }
                }
            }
        });

        Ok(html! {
            div class="intro" {
                div class="intro-header" {
                    @if !title.is_empty() || !slogan.is_empty() || button.is_some() {
                        section class="intro-header-body" {
                            @if !title.is_empty() || !slogan.is_empty() {
                                h1 class="intro-header-title" {
                                    @if !title.is_empty() {
                                        span class="intro-header-name" { (title) }
                                    }
                                    @if !slogan.is_empty() {
                                        span class="intro-header-slogan" { (slogan) }
                                    }
                                }
                            }
                            @if let Some(button) = button {
                                (button)
                            }
                        }
                    }
                    aside class="intro-header-img" aria-hidden="true" {
                        div class="intro-header-mascot" {
                            (PageTopSvg::Color.markup())
                            // Párpados sobre los ojos de la mascota, que sólo se ven al parpadear.
                            svg class="intro-header-blink" viewBox="0 0 1614 1614" {
                                circle cx="545" cy="780" r="92" {}
                                circle cx="1071" cy="780" r="92" {}
                                path d="M 475,790 Q 545,835 615,790" {}
                                path d="M 1001,790 Q 1071,835 1141,790" {}
                            }
                        }
                    }
                }
                div class="intro-content" {
                    section class="intro-content-body" {
                        div class="intro-text" {
                            div class="intro-text-body" {
                                @if self.kind == Kind::PageTop {
                                    p class="intro-text-lead" {
                                        (Lc::l("intro_text1").using(cx))
                                    }
                                    (Self::version_pills(cx))
                                    p class="intro-text-lead" {
                                        (Lc::l("intro_text2").using(cx))
                                    }
                                }
                                (self.children().render(cx).await)
                            }
                        }
                    }
                }
                div class="intro-footer" {
                    section class="intro-footer-body" {
                        div class="intro-footer-links" {
                            a href="https://crates.io/crates/pagetop" target="_blank" rel="noopener noreferrer" { ("Crates.io") }
                            a href="https://docs.rs/pagetop" target="_blank" rel="noopener noreferrer" { ("Docs.rs") }
                            a href="https://git.cillero.es/manuelcillero/pagetop" target="_blank" rel="noopener noreferrer" { (Lc::l("intro_code").using(cx)) }
                            em { (Lc::l("intro_have_fun").using(cx)) }
                        }
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl Intro {
    // **< Intro BUILDER >**************************************************************************

    /// Crea una introducción personalizada.
    ///
    /// Parte de una introducción sin el contenido de presentación de PageTop (párrafos y versión),
    /// sin título, sin eslogan y sin botón de llamada a la acción. Todo ello puede añadirse con
    /// [`with_title()`], [`with_slogan()`] y [`with_button()`].
    ///
    /// El tipo se fija al construirla y no se puede cambiar después; para la introducción estándar
    /// de PageTop se usa [`Intro::default()`] o [`Intro::new()`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let intro = Intro::custom().with_title(Lc::n("Intro title"));
    /// ```
    ///
    /// [`with_title()`]: Self::with_title
    /// [`with_slogan()`]: Self::with_slogan
    /// [`with_button()`]: Self::with_button
    pub fn custom() -> Self {
        Self {
            kind: Kind::Custom,
            title: Lc::none(),
            slogan: Lc::none(),
            button: None,
            ..Default::default()
        }
    }

    /// Establece el título de entrada.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default().with_title(Lc::n("Intro title"));
    /// ```
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece el eslogan de entrada (línea secundaria del título).
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default().with_slogan(Lc::n("A short slogan"));
    /// ```
    pub fn with_slogan(mut self, slogan: Lc) -> Self {
        self.slogan = slogan;
        self
    }

    /// Configura el botón opcional de llamada a la acción, que se abre en la misma ventana.
    ///
    /// - Usa `Some((texto, ruta))` para mostrarlo, donde [`Route`] resuelve la ruta o URL final al
    ///   pulsar el botón según el contexto de renderizado.
    /// - Usa `None` para ocultarlo.
    ///
    /// Para abrirlo en una nueva ventana o pestaña se usa
    /// [`with_button_blank()`](Self::with_button_blank).
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// // Define un botón con texto y una ruta interna (preserva `lang` si corresponde).
    /// let intro = Intro::default().with_button(Some((Lc::n("Start"), "/start".into())));
    /// // Descarta el botón de la intro.
    /// let intro_no_button = Intro::default().with_button(None);
    /// ```
    pub fn with_button(mut self, button: impl Into<Option<(Lc, Route)>>) -> Self {
        self.button = button.into();
        self.button_blank = false;
        self
    }

    /// Configura el botón opcional de llamada a la acción, que se abre en una nueva ventana o
    /// pestaña. Es el de la introducción por defecto, que lleva a la web de PageTop. Ver
    /// [`with_button()`](Self::with_button).
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default()
    ///     .with_button_blank(Some((Lc::n("Docs"), "https://docs.rs/pagetop".into())));
    /// ```
    pub fn with_button_blank(mut self, button: impl Into<Option<(Lc, Route)>>) -> Self {
        self.button = button.into();
        self.button_blank = true;
        self
    }

    /// Añade un nuevo componente a la intro o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    ///
    /// Si se añade un bloque ([`Block`]) se aplicarán estilos específicos para destacarlo.
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

impl Intro {
    /// Devuelve las píldoras con los datos de la versión de PageTop en uso: número
    /// ([`PAGETOP_VERSION`]), fecha de publicación (según su entrada en el `CHANGELOG.md`, si ya se
    /// ha publicado) y licencia.
    ///
    /// La introducción estándar de PageTop las muestra tras su primer párrafo. También pueden
    /// mostrarse en los contenidos de una introducción personalizada, donde toman sus estilos.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let intro = Intro::custom()
    ///     .with_title(Lc::n("PageTop"))
    ///     .with_child(Html::with(|cx| html! { (Intro::version_pills(cx)) }));
    /// ```
    pub fn version_pills(cx: &Context) -> Markup {
        let released = NaiveDate::parse_from_str(env!("PAGETOP_RELEASE_DATE"), "%Y-%m-%d").ok();
        let label = |key: &'static str| Lc::l(key).lookup(cx).unwrap_or_default();
        html! {
            ul class="intro-pills" {
                (pill("PageTop", PAGETOP_VERSION))
                @if let Some(date) = released {
                    (pill(
                        &label("intro_release_label"),
                        &cx.format_date(date, DateFormat::Medium),
                    ))
                }
                (pill(&label("intro_license_label"), "MIT/Apache-2.0"))
            }
        }
    }
}

// Una píldora de datos de PageTop: nombre y valor.
fn pill(key: &str, value: &str) -> Markup {
    html! {
        li class="intro-pill" {
            span class="intro-pill-key" { (key) }
            span class="intro-pill-val" { (value) }
        }
    }
}
