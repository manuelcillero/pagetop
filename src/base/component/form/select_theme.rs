use crate::prelude::*;

/// Componente para **elegir un tema** de los temas habilitados en la aplicación.
///
/// Ofrece un elemento por cada tema de [`enabled_themes()`], con su nombre corto como valor (p. ej.
/// `"Bootsier"`) y su nombre traducido como etiqueta, ordenados por ese nombre en el idioma de la
/// página. Se renderiza como cualquier [`form::select::Field`].
///
/// Si sólo hay un tema habilitado, la lista se muestra deshabilitada con ese tema seleccionado (un
/// campo deshabilitado no se envía con el formulario). Si hay varios, la primera opción, con valor
/// vacío, depende de si el campo es obligatorio:
///
/// - Si no lo es (por defecto), siempre se incluye y propone usar el tema del sitio. Quedarse sin
///   tema propio es válido y significa usar el predeterminado de la aplicación. Se selecciona
///   cuando el valor elegido no corresponde a ningún tema de la lista (sin distinguir mayúsculas y
///   minúsculas).
/// - Si lo es ([`with_required(true)`](Self::with_required)), sólo se incluye cuando el valor
///   seleccionado no corresponde a ningún tema de la lista, y pide elegir uno; así el navegador no
///   deja enviar el formulario sin elegir un tema. El valor recibido debe validarse igualmente en
///   el servidor.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let theme = form::SelectTheme::new()
///     .with_name("theme")
///     .with_label(Lc::n("Theme"))
///     .with_selected("Basic");
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct SelectTheme {
    /// Devuelve la lista de selección interna con la configuración común (nombre, etiqueta, ayuda,
    /// propiedades...), todavía sin opciones; éstas se añaden al renderizar.
    field: form::select::Field,
    /// Devuelve el nombre corto del tema seleccionado.
    selected: String,
}

#[async_trait]
impl Component for SelectTheme {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.field().id()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut field = self.field().clone();
        let mut themes = enabled_themes();

        if let [theme] = themes[..] {
            field.alter_disabled(true);
            field.alter_item(
                form::select::Item::new(theme.short_name(), theme.name()).with_selected(true),
            );
            return Ok(field.render(cx).await);
        }

        themes.sort_by_cached_key(|theme| theme.name().collation_key(&*cx));
        let is_selected =
            |theme: ThemeRef| theme.short_name().eq_ignore_ascii_case(self.selected());

        let known = themes.iter().any(|theme| is_selected(*theme));
        if !field.required() {
            let default_theme = default_theme();
            let default_name = default_theme
                .name()
                .lookup(cx)
                .unwrap_or_else(|| default_theme.short_name().to_owned());
            let label = Lc::l("select_theme_site_default").with_arg("theme", default_name);
            field.alter_item(form::select::Item::new("", label).with_selected(!known));
        } else if !known {
            let label = Lc::l("select_theme_placeholder");
            field.alter_item(form::select::Item::new("", label).with_selected(true));
        }

        for theme in themes {
            let item = form::select::Item::new(theme.short_name(), theme.name());
            field.alter_item(item.with_selected(is_selected(theme)));
        }
        Ok(field.render(cx).await)
    }
}

#[builder_impl]
impl SelectTheme {
    // **< SelectTheme BUILDER >********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.field.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.field.alter_prop(op);
        self
    }

    /// Establece el nombre del campo.
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.field.alter_name(name);
        self
    }

    /// Establece la etiqueta del campo.
    pub fn with_label(mut self, label: Lc) -> Self {
        self.field.alter_label(label);
        self
    }

    /// Establece el texto de ayuda del campo.
    pub fn with_help_text(mut self, help_text: Lc) -> Self {
        self.field.alter_help_text(help_text);
        self
    }

    /// Establece si el campo es obligatorio, lo que cambia su primera opción (ver [`SelectTheme`]).
    pub fn with_required(mut self, required: bool) -> Self {
        self.field.alter_required(required);
        self
    }

    /// Establece si el campo está deshabilitado. Con un solo tema habilitado lo está siempre.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.field.alter_disabled(disabled);
        self
    }

    /// Establece el nombre corto del tema seleccionado (p. ej. `"Bootsier"`). Vacío, o uno que no
    /// esté habilitado, selecciona la primera opción (ver [`SelectTheme`]).
    pub fn with_selected(mut self, selected: impl Into<String>) -> Self {
        self.selected = selected.into();
        self
    }
}
