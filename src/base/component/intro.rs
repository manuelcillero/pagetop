use crate::prelude::*;

/// Tipo de apertura que se mostrará en la introducción del componente [`Intro`].
///
/// Permite elegir entre una apertura con textos predefinidos sobre PageTop (como hace la página de
/// bienvenida [`Welcome`](crate::base::extension::Welcome)) o una introducción completamente
/// personalizada.
#[derive(AutoDefault, Copy, Clone, Debug, Eq, PartialEq)]
pub enum IntroOpening {
    /// Modo por defecto. Muestra una introducción estándar de PageTop e incluye automáticamente
    /// *badges* con información de la última versión liberada, fecha del último lanzamiento y
    /// licencia de uso.
    #[default]
    PageTop,
    /// Modo totalmente personalizado. No añade *badges* ni textos predefinidos. Usa la imagen de
    /// PageTop pero el contenido lo define el propio desarrollador.
    Custom,
}

/// Componente para divulgar PageTop (como hace [`Welcome`](crate::base::extension::Welcome)), o
/// mostrar presentaciones.
///
/// Usa la imagen de PageTop para mostrar:
///
/// - Una **figura decorativa** (que incluye el *monster* de PageTop) antecediendo al contenido.
/// - Una vista destacada del **título** de la página con un **eslogan** de presentación.
/// - Un **botón opcional** de llamada a la acción con texto y enlace configurables.
/// - Un **área para la presentación de contenidos**, con *badges* informativos de PageTop (si se
///   opta por [`IntroOpening::PageTop`]) y bloques ([`Block`](crate::base::component::Block)) de
///   contenido libre para crear párrafos vistosos de texto. Aunque admite todo tipo de componentes.
///
/// # Ejemplos
///
/// **Intro mínima por defecto**
///
/// ```rust
/// # use pagetop::prelude::*;
/// let intro = Intro::default();
/// ```
///
/// **Título, eslogan y botón personalizados**
///
/// ```rust
/// # use pagetop::prelude::*;
/// let intro = Intro::default()
///     .with_title(L10n::l("intro_custom_title"))
///     .with_slogan(L10n::l("intro_custom_slogan"))
///     .with_button(Some((
///         L10n::l("intro_learn_more"),
///         |_| "/learn-more"
///     )));
/// ```
///
/// **Sin botón y en modo *Custom* (sin *badges* predefinidos)**
///
/// ```rust
/// # use pagetop::prelude::*;
/// let intro = Intro::default()
///     .with_button(None::<(L10n, FnPathByContext)>)
///     .with_opening(IntroOpening::Custom);
/// ```
///
/// **Añadir contenidos hijo**
///
/// ```rust
/// # use pagetop::prelude::*;
/// let intro = Intro::default()
///     .add_child(
///         Block::new()
///             .with_title(L10n::l("intro_custom_block_title"))
///             .add_child(Html::with(move |cx| {
///                 html! {
///                     p { (L10n::l("intro_custom_paragraph_1").using(cx)) }
///                     p { (L10n::l("intro_custom_paragraph_2").using(cx)) }
///                 }
///             })),
///     );
/// ```
#[derive(Getters)]
pub struct Intro {
    /// Devuelve el título de entrada.
    title: L10n,
    /// Devuelve el eslogan de la entrada.
    slogan: L10n,
    /// Devuelve el botón de llamada a la acción, si existe.
    button: Option<(L10n, FnPathByContext)>,
    /// Devuelve el modo de apertura configurado.
    opening: IntroOpening,
    /// Devuelve la lista de componentes hijo de la intro.
    children: Children,
}

impl Default for Intro {
    #[rustfmt::skip]
    fn default() -> Self {
        Intro {
            title   : L10n::l("intro_default_title"),
            slogan  : L10n::l("intro_default_slogan").with_arg("app", &global::SETTINGS.app.name),
            button  : Some((L10n::l("intro_default_button"), |_| "https://pagetop.cillero.es")),
            opening : IntroOpening::default(),
            children: Children::default(),
        }
    }
}

impl Component for Intro {
    fn new() -> Self {
        Intro::default()
    }

    fn setup_before_prepare(&mut self, cx: &mut Context) {
        cx.alter_assets(ContextOp::AddStyleSheet(
            StyleSheet::from("/css/intro.css").with_version(PAGETOP_VERSION),
        ));
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        if *self.opening() == IntroOpening::PageTop {
            cx.alter_assets(ContextOp::AddJavaScript(JavaScript::on_load_async("intro-js", |cx|
                util::indoc!(r#"
                try {
                    const resp = await fetch("https://crates.io/api/v1/crates/pagetop");
                    const data = await resp.json();
                    const date = new Date(data.versions[0].created_at);
                    const formatted = date.toLocaleDateString("LANGID", { year: "numeric", month: "2-digit", day: "2-digit" });
                    document.getElementById("intro-release").src = `https://img.shields.io/badge/Release%20date-${encodeURIComponent(formatted)}-blue?label=LABEL&style=for-the-badge`;
                    document.getElementById("intro-badges").style.display = "block";
                } catch (e) {
                    console.error("Failed to fetch release date from crates.io:", e);
                }
                "#)
                .replace("LANGID", cx.langid().to_string().as_str())
                .replace("LABEL", L10n::l("intro_release_label").using(cx).as_str())
            )));
        }

        PrepareMarkup::With(html! {
            div class="intro" {
                div class="intro-header" {
                    section class="intro-header__body" {
                        h1 class="intro-header__title" {
                            span { (self.title().using(cx)) }
                            (self.slogan().using(cx))
                        }
                    }
                    aside class="intro-header__image" aria-hidden="true" {
                        div class="intro-header__monster" {
                            (PageTopSvg::Color.render(cx))
                        }
                    }
                }
                div class="intro-content" {
                    section class="intro-content__body" {
                        div class="intro-text" {
                            @if let Some((txt, lnk)) = self.button() {
                                div class="intro-button" {
                                    a
                                        class="intro-button__link"
                                        href=((lnk)(cx))
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    {
                                        span {} span {} span {}
                                        div class="intro-button__text" {
                                            (txt.using(cx))
                                        }
                                    }
                                }
                            }
                            div class="intro-text__children" {
                                @if *self.opening() == IntroOpening::PageTop {
                                    p { (L10n::l("intro_text1").using(cx)) }
                                    div id="intro-badges" {
                                        img
                                            src="https://img.shields.io/crates/v/pagetop.svg?label=PageTop&style=for-the-badge"
                                            alt=[L10n::l("intro_pagetop_label").lookup(cx)] {} (" ")
                                        img
                                            id="intro-release"
                                            alt=[L10n::l("intro_release_label").lookup(cx)] {} (" ")
                                        img
                                            src=(format!(
                                                "https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label={}&style=for-the-badge",
                                                L10n::l("intro_license_label").lookup(cx).unwrap_or_default()
                                            ))
                                            alt=[L10n::l("intro_license_label").lookup(cx)] {}
                                    }
                                    p { (L10n::l("intro_text2").using(cx)) }
                                }
                                (self.children().render(cx))
                            }
                        }
                    }
                }
                div class="intro-footer" {
                    section class="intro-footer__body" {
                        div class="intro-footer__logo" {
                            (PageTopSvg::LineLight.render(cx))
                        }
                        div class="intro-footer__links" {
                            a href="https://crates.io/crates/pagetop" target="_blank" rel="noopener noreferrer" { ("Crates.io") }
                            a href="https://docs.rs/pagetop" target="_blank" rel="noopener noreferrer" { ("Docs.rs") }
                            a href="https://git.cillero.es/manuelcillero/pagetop" target="_blank" rel="noopener noreferrer" { (L10n::l("intro_code").using(cx)) }
                            em { (L10n::l("intro_have_fun").using(cx)) }
                        }
                    }
                }
            }
        })
    }
}

impl Intro {
    // **< Intro BUILDER >**************************************************************************

    /// Establece el título de entrada.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default().with_title(L10n::n("Intro title"));
    /// ```
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Establece el eslogan de entrada (línea secundaria del título).
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default().with_slogan(L10n::n("A short slogan"));
    /// ```
    #[builder_fn]
    pub fn with_slogan(mut self, slogan: L10n) -> Self {
        self.slogan = slogan;
        self
    }

    /// Configura el botón opcional de llamada a la acción.
    ///
    /// - Usa `Some((texto, closure_url))` para mostrarlo, donde [`FnPathByContext`] recibe el
    ///   [`Context`] y devuelve la ruta o URL final al pulsar el botón.
    /// - Usa `None` para ocultarlo.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// // Define un botón con texto y una URL fija.
    /// let intro = Intro::default().with_button(Some((L10n::n("Learn more"), |_| "/start")));
    /// // Descarta el botón de la intro.
    /// let intro_no_button = Intro::default().with_button(None);
    /// ```
    #[builder_fn]
    pub fn with_button(mut self, button: Option<(L10n, FnPathByContext)>) -> Self {
        self.button = button;
        self
    }

    /// Selecciona el tipo de apertura: [`IntroOpening::PageTop`] (por defecto) o
    /// [`IntroOpening::Custom`].
    ///
    /// - `PageTop`: añade *badges* automáticos y una presentación de lo que es PageTop.
    /// - `Custom`: introducción en blanco para añadir cualquier contenido.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default().with_opening(IntroOpening::Custom);
    /// ```
    #[builder_fn]
    pub fn with_opening(mut self, opening: IntroOpening) -> Self {
        self.opening = opening;
        self
    }

    /// Añade un nuevo componente hijo a la intro.
    ///
    /// Si es un bloque ([`Block`]) aplica estilos específicos para destacarlo.
    #[inline]
    pub fn add_child(mut self, component: impl Component) -> Self {
        self.children.add(Child::with(component));
        self
    }

    /// Modifica la lista de componentes (`children`) aplicando una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }
}
