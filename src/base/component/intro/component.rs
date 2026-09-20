use crate::prelude::*;

/// Componente para divulgar PageTop (como hace [`Welcome`](crate::base::extension::Welcome)), o
/// mostrar presentaciones.
///
/// Usa la imagen de PageTop para mostrar:
///
/// - Una **figura decorativa** (que incluye la *mascota* de PageTop) antecediendo al contenido.
/// - Una vista destacada del **título** de la página con un **eslogan** de presentación.
/// - Un **botón opcional** de llamada a la acción con texto y enlace configurables.
/// - Un **área para la presentación de contenidos**, con *badges* informativos de PageTop (si se
///   opta por [`intro::Kind::PageTop`]) y bloques ([`Block`](crate::base::component::Block)) de
///   contenido libre. Los párrafos que tengan la clase `.intro-text-lead` se mostrarán con una
///   tipografía ampliada, ideal para presentaciones breves e impactantes.
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
/// **Área de contenidos al ancho máximo**
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let intro = Intro::default().with_width(intro::Width::Full);
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
    /// Devuelve el tipo de introducción, fijado al construirla.
    kind: intro::Kind,
    /// Devuelve el ancho máximo del área de contenidos.
    width: intro::Width,
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
            kind: intro::Kind::default(),
            width: intro::Width::default(),
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
        cx.alter_assets(StyleSheet::from("/pagetop/css/intro.css").with_version(PAGETOP_VERSION));
        if *self.kind() == intro::Kind::PageTop {
            cx.alter_assets(JavaScript::on_load_async("intro-js", |cx|
                util::indoc!(r#"
                try {
                    const resp = await fetch("https://crates.io/api/v1/crates/pagetop");
                    const data = await resp.json();
                    const date = new Date(data.versions[0].created_at);
                    const formatted = date.toLocaleDateString("LANGID", { year: "numeric", month: "2-digit", day: "2-digit" });
                    document.getElementById("intro-release").src = `https://img.shields.io/badge/Release%20date-${encodeURIComponent(formatted)}-blue?label=LABEL&style=for-the-badge`;
                } catch (e) {
                    console.error("Failed to fetch release date from crates.io:", e);
                } finally {
                    document.getElementById("intro-badges").style.visibility = "visible";
                }
                "#)
                .replace("LANGID", cx.langid().to_string().as_str())
                .replace("LABEL", Lc::l("intro_release_label").using(cx).as_str())
            ));
        }

        let title = self.title().using(cx);
        let slogan = self.slogan().using(cx);

        Ok(html! {
            div class="intro" {
                div class="intro-header" {
                    @if !title.is_empty() || !slogan.is_empty() {
                        section class="intro-header-body" {
                            h1 class="intro-header-title" {
                                @if !title.is_empty() {
                                    span { (title) }
                                }
                                (slogan)
                            }
                        }
                    }
                    aside class="intro-header-img" aria-hidden="true" {
                        div class="intro-header-mascot" {
                            (PageTopSvg::Color.markup())
                        }
                    }
                }
                div class="intro-content" {
                    section class="intro-content-body" {
                        div class=(self.width().classes()) {
                            @if let Some((txt, lnk)) = self.button() {
                                div class="intro-button" {
                                    a
                                        class="intro-button-link"
                                        href=(lnk.resolve(cx))
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    {
                                        span {} span {} span {}
                                        div class="intro-button-text" {
                                            (txt.using(cx))
                                        }
                                    }
                                }
                            }
                            div class="intro-text-body" {
                                @if *self.kind() == intro::Kind::PageTop {
                                    p class="intro-text-lead" {
                                        (Lc::l("intro_text1").using(cx))
                                    }
                                    div id="intro-badges" {
                                        img
                                            src="https://img.shields.io/crates/v/pagetop.svg?label=PageTop&style=for-the-badge"
                                            alt=[Lc::l("intro_pagetop_label").lookup(cx)] {} (" ")
                                        img
                                            id="intro-release"
                                            alt=[Lc::l("intro_release_label").lookup(cx)] {} (" ")
                                        img
                                            src=(util::join!(
                                                "https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=",
                                                Lc::l("intro_license_label").lookup(cx).unwrap_or_default(),
                                                "&style=for-the-badge"
                                            ))
                                            alt=[Lc::l("intro_license_label").lookup(cx)] {}
                                    }
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
                        div class="intro-footer-logo" {
                            (PageTopSvg::LineLight.markup())
                        }
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

    /// Crea una introducción de tipo [`intro::Kind::Custom`].
    ///
    /// Parte de una introducción sin el contenido de presentación de PageTop (párrafos y *badges*),
    /// sin título, sin eslogan y sin botón de llamada a la acción. Todo ello puede añadirse con
    /// [`with_title()`], [`with_slogan()`] y [`with_button()`].
    ///
    /// El tipo se fija al construirla y no se puede cambiar después; para la introducción estándar
    /// de PageTop (tipo [`intro::Kind::PageTop`]) se usa [`Intro::default()`] o [`Intro::new()`].
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
            kind: intro::Kind::Custom,
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

    /// Configura el botón opcional de llamada a la acción.
    ///
    /// - Usa `Some((texto, ruta))` para mostrarlo, donde [`Route`] resuelve la ruta o URL final al
    ///   pulsar el botón según el contexto de renderizado.
    /// - Usa `None` para ocultarlo.
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
        self
    }

    /// Establece el ancho máximo del área de contenidos.
    ///
    /// Ver [`intro::Width`] para las variantes disponibles.
    ///
    /// # Ejemplo
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let intro = Intro::default().with_width(intro::Width::Wide);
    /// ```
    pub fn with_width(mut self, width: intro::Width) -> Self {
        self.width = width;
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
