use crate::prelude::*;

/// Componente para crear una **lista de selección de zonas horarias** IANA.
///
/// Ofrece las zonas horarias de [`Timezone::supported_by_region()`] agrupadas por región, con el
/// nombre de la región traducido (*"Europa"*, *"América"*...) y el nombre IANA completo como valor
/// y como etiqueta (p. ej. `"Europe/Madrid"`). No admite opciones libres. Se renderiza como
/// cualquier [`form::select::Field`].
///
/// La primera opción, con valor vacío, depende de si el campo es obligatorio:
///
/// - Si no lo es (por defecto), siempre se incluye y propone usar la zona horaria del sitio.
///   Quedarse sin zona propia es válido y significa usar la predeterminada de la aplicación. Se
///   selecciona cuando el valor elegido no corresponde a ninguna zona de la lista.
/// - Si lo es ([`with_required(true)`](Self::with_required)), sólo se incluye cuando el valor
///   seleccionado no corresponde a ninguna zona de la lista, y pide elegir una; así el navegador no
///   deja enviar el formulario sin elegir una zona horaria. El valor recibido debe validarse
///   igualmente en el servidor.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let timezone = form::SelectTimezone::new()
///     .with_name("timezone")
///     .with_label(Lc::n("Time zone"))
///     .with_selected("Europe/Madrid");
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct SelectTimezone {
    /// Devuelve la lista de selección interna con la configuración común (nombre, etiqueta, ayuda,
    /// propiedades...), todavía sin opciones; éstas se añaden al renderizar.
    field: form::select::Field,
    /// Devuelve el nombre IANA de la zona horaria seleccionada.
    selected: String,
}

#[async_trait]
impl Component for SelectTimezone {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.field().id()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut field = self.field().clone();
        let regions = Timezone::supported_by_region();
        let known = regions
            .iter()
            .any(|(_, names)| names.contains(&self.selected()));
        if !field.required() {
            let label = Lc::l("select_timezone_site_default")
                .with_arg("timezone", Timezone::default_tz().name());
            field.alter_item(form::select::Item::new("", label).with_selected(!known));
        } else if !known {
            let label = Lc::l("select_timezone_placeholder");
            field.alter_item(form::select::Item::new("", label).with_selected(true));
        }
        for (region, names) in regions {
            let mut group = form::select::Group::new(match *region {
                "Africa" => Lc::l("timezone_region_africa"),
                "America" => Lc::l("timezone_region_america"),
                "Antarctica" => Lc::l("timezone_region_antarctica"),
                "Arctic" => Lc::l("timezone_region_arctic"),
                "Asia" => Lc::l("timezone_region_asia"),
                "Atlantic" => Lc::l("timezone_region_atlantic"),
                "Australia" => Lc::l("timezone_region_australia"),
                "Etc" => Lc::l("timezone_region_etc"),
                "Europe" => Lc::l("timezone_region_europe"),
                "Indian" => Lc::l("timezone_region_indian"),
                "Pacific" => Lc::l("timezone_region_pacific"),
                _ => Lc::n(*region),
            });
            for name in names {
                let selected = *name == self.selected();
                group.alter_item(
                    form::select::Item::new(*name, Lc::n(*name)).with_selected(selected),
                );
            }
            field.alter_group(group);
        }
        Ok(field.render(cx).await)
    }
}

#[builder_impl]
impl SelectTimezone {
    // **< SelectTimezone BUILDER >*****************************************************************

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
    /// [`SelectTimezone`]).
    pub fn with_required(mut self, required: bool) -> Self {
        self.field.alter_required(required);
        self
    }

    /// Establece si el campo está deshabilitado.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.field.alter_disabled(disabled);
        self
    }

    /// Establece el nombre IANA de la zona horaria seleccionada (p. ej. `"Europe/Madrid"`). Vacía,
    /// o una que no esté en la lista, selecciona la primera opción (ver [`SelectTimezone`]).
    pub fn with_selected(mut self, selected: impl Into<String>) -> Self {
        self.selected = selected.into();
        self
    }
}
