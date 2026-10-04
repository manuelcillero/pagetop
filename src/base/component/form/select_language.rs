use crate::prelude::*;

/// Componente para **elegir un idioma** de la lista de idiomas soportados por PageTop.
///
/// Ofrece un elemento por cada idioma de [`Locale::supported_languages()`], con su identificador
/// como valor (p. ej. `"es-ES"`) y su nombre traducido como etiqueta, ordenados por ese nombre en
/// el idioma de la página. Se renderiza como cualquier [`form::select::Field`].
///
/// La primera opción, con valor vacío, depende de si el campo es obligatorio:
///
/// - Si no lo es (por defecto), siempre se incluye y propone usar el idioma del sitio. Quedarse sin
///   idioma propio es válido y significa usar el predeterminado de la aplicación. Se selecciona
///   cuando el valor elegido no corresponde a ningún idioma de la lista.
/// - Si lo es ([`with_required(true)`](Self::with_required)), sólo se incluye cuando el valor
///   seleccionado no corresponde a ningún idioma de la lista, y pide elegir uno; así el navegador
///   no deja enviar el formulario sin elegir un idioma. El valor recibido debe validarse
///   igualmente en el servidor.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let language = form::SelectLanguage::new()
///     .with_name("language")
///     .with_label(Lc::n("Language"))
///     .with_selected("es-ES");
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct SelectLanguage {
    /// Devuelve la lista de selección interna con la configuración común (nombre, etiqueta, ayuda,
    /// propiedades...), todavía sin opciones; éstas se añaden al renderizar.
    field: form::select::Field,
    /// Devuelve el identificador del idioma seleccionado.
    selected: String,
}

#[async_trait]
impl Component for SelectLanguage {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.field().id()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut field = self.field().clone();
        let mut languages = Locale::supported_languages();
        languages.sort_by_cached_key(|(_, name)| name.collation_key(&*cx));

        let selected = Locale::resolve(self.selected()).as_option();
        let known = selected.is_some();
        if !field.required() {
            let default_langid = Locale::default_langid();
            let default_name = languages
                .iter()
                .find(|(langid, _)| *langid == default_langid)
                .and_then(|(_, name)| name.lookup(cx))
                .unwrap_or_else(|| default_langid.to_string());
            let label = Lc::l("select_language_site_default").with_arg("language", default_name);
            field.alter_item(form::select::Item::new("", label).with_selected(!known));
        } else if !known {
            let label = Lc::l("select_language_placeholder");
            field.alter_item(form::select::Item::new("", label).with_selected(true));
        }

        for (langid, name) in languages {
            let item = form::select::Item::new(langid.to_string(), name);
            field.alter_item(item.with_selected(selected == Some(langid)));
        }
        Ok(field.render(cx).await)
    }
}

#[builder_impl]
impl SelectLanguage {
    // **< SelectLanguage BUILDER >*****************************************************************

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

    /// Establece si el campo es obligatorio, lo que cambia su primera opción (ver
    /// [`SelectLanguage`]).
    pub fn with_required(mut self, required: bool) -> Self {
        self.field.alter_required(required);
        self
    }

    /// Establece si el campo está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.field.alter_disabled(disabled);
        self
    }

    /// Establece el identificador del idioma seleccionado (p. ej. `"es-ES"`). Se resuelve con
    /// [`Locale::resolve()`], así que también acepta alias o variantes (`"es"`, `"es-es"`...) del
    /// mismo idioma.
    pub fn with_selected(mut self, selected: impl Into<String>) -> Self {
        self.selected = selected.into();
        self
    }
}
