use crate::prelude::*;

use chrono::Offset;

/// Componente para crear una **lista de selección de zonas horarias** IANA.
///
/// Ofrece zonas horarias agrupadas por región, con el nombre de la región traducido (*"Europa"*,
/// *"América"*, etc.). Cada opción tiene como valor el nombre IANA completo (p. ej.
/// `"America/New_York"`) y como etiqueta ese mismo nombre con espacios en lugar de guiones bajos
/// (p. ej. `"America/New York"`). No admite opciones libres. Se renderiza como cualquier
/// [`form::select::Field`]. Con [`with_utc_offset(true)`] cada etiqueta añade además el desfase
/// actual respecto a UTC (p. ej. `"America/New York (UTC-05:00)"`) y las zonas de cada región se
/// ordenan por ese desfase y, a igual desfase, por nombre.
///
/// Por defecto ofrece las regiones y el orden de la configuración, los mismos que
/// [`Timezone::supported_by_region()`]; [`with_regions()`] y [`with_order()`] permiten cambiarlos
/// para este componente. Cuando las regiones se ordenan por nombre (con
/// [`TimezoneOrder::Alphabetical`], o [`TimezoneOrder::Listed`] sin regiones), se ordenan por su
/// nombre traducido al idioma de la página, sin distinguir mayúsculas ni acentos, y *"Otras"*
/// (`Etc/UTC`) sigue al final.
///
/// La primera opción, con valor vacío, depende de si el campo es obligatorio:
///
/// - Si no lo es (por defecto), siempre se incluye y propone usar la zona horaria del sitio.
///   Quedarse sin zona propia es válido y significa usar la predeterminada de la aplicación. Se
///   selecciona cuando el valor elegido no es ninguna zona horaria válida.
/// - Si lo es ([`with_required(true)`]), sólo se incluye cuando el valor seleccionado no es ninguna
///   zona horaria válida, y pide elegir una; así el navegador no deja enviar el formulario sin
///   elegir una zona horaria.
///
/// Si el valor seleccionado es una zona horaria válida que no se ofrece (p. ej. de una región que
/// se ha dejado de ofrecer después de elegirla), se muestra igualmente, seleccionada, en un grupo
/// *"Zona horaria actual"* antes de las regiones; así volver a guardar el formulario sin tocarla no
/// la descarta.
///
/// El valor recibido debe validarse igualmente en el servidor: con
/// [`Timezone::supported_by_region()`] si se usan las regiones de la configuración, o con
/// [`Timezone::is_supported_in()`] si se han cambiado; y aceptando además, sin cambios, el valor
/// que ya estuviera guardado aunque no se ofrezca.
///
/// Si [`global::SETTINGS.app.timezone_per_user`] es *false*, la zona horaria propia del usuario no
/// se aplica y no conviene mostrar este campo.
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
///
/// [`TimezoneOrder::Alphabetical`]: crate::global::TimezoneOrder::Alphabetical
/// [`TimezoneOrder::Listed`]: crate::global::TimezoneOrder::Listed
/// [`global::SETTINGS.app.timezone_per_user`]: crate::global::App::timezone_per_user
/// [`with_utc_offset(true)`]: Self::with_utc_offset
/// [`with_required(true)`]: Self::with_required
/// [`with_regions()`]: Self::with_regions
/// [`with_order()`]: Self::with_order
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct SelectTimezone {
    /// Devuelve la lista de selección interna con la configuración común (nombre, etiqueta, ayuda,
    /// propiedades...), todavía sin opciones; éstas se añaden al renderizar.
    field: form::select::Field,
    /// Devuelve el nombre IANA de la zona horaria seleccionada.
    selected: String,
    /// Devuelve si las etiquetas muestran el desfase actual respecto a UTC.
    utc_offset: bool,
    /// Devuelve las regiones que se ofrecen, junto a `Etc`, que se ofrece siempre; por defecto, las
    /// de [`global::SETTINGS.app.timezone_regions`](crate::global::App::timezone_regions).
    #[default(_code = "Timezone::configured_regions()")]
    regions: Vec<TzRegion>,
    /// Devuelve el orden de las regiones; por defecto, el de
    /// [`global::SETTINGS.app.timezone_order`](crate::global::App::timezone_order).
    #[default(_code = "global::SETTINGS.app.timezone_order")]
    order: global::TimezoneOrder,
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
        let regions = Timezone::regions_with(*self.order(), self.regions());
        let offered = regions
            .iter()
            .any(|(_, names)| names.contains(&self.selected()));
        // Una zona válida que no se ofrece (p. ej. de una región retirada después) se muestra igual
        // para que guardar el formulario sin tocarla no la descarte.
        let current = (!offered)
            .then(|| self.selected().parse::<Tz>().ok())
            .flatten();
        let known = offered || current.is_some();
        // El desfase se calcula en cada renderizado porque cambia con el horario de verano.
        let now = Utc::now();
        let offset = |name: &str| {
            self.utc_offset()
                .then(|| name.parse::<Tz>().ok())
                .flatten()
                .map(|tz| now.with_timezone(&tz).offset().fix().local_minus_utc())
        };
        if !field.required() {
            let site = Timezone::default_tz().name();
            let label = Lc::l("select_timezone_site_default")
                .with_arg("timezone", zone_label(zone_name(site), offset(site)));
            field.alter_item(form::select::Item::new("", label).with_selected(!known));
        } else if !known {
            let label = Lc::l("select_timezone_placeholder");
            field.alter_item(form::select::Item::new("", label).with_selected(true));
        }
        if let Some(tz) = current {
            let name = tz.name();
            let mut group = form::select::Group::new(Lc::l("select_timezone_current"));
            group.alter_item(
                form::select::Item::new(name, Lc::n(zone_label(zone_name(name), offset(name))))
                    .with_selected(true),
            );
            field.alter_group(group);
        }
        let mut groups: Vec<_> = regions
            .iter()
            .map(|(region, names)| (*region, region.label(), names))
            .collect();
        if Timezone::regions_by_name(*self.order(), self.regions()) {
            sort_by_label(&mut groups, cx);
        }
        for (_, label, names) in groups {
            let mut group = form::select::Group::new(label);
            let mut zones: Vec<_> = names.iter().map(|name| (offset(name), *name)).collect();
            // Ordenación estable: a igual desfase, se mantienen ordenadas por nombre.
            if *self.utc_offset() {
                zones.sort_by_key(|(offset, _)| *offset);
            }
            for (offset, name) in zones {
                let selected = name == self.selected();
                group.alter_item(
                    form::select::Item::new(name, Lc::n(zone_label(zone_name(name), offset)))
                        .with_selected(selected),
                );
            }
            field.alter_group(group);
        }
        Ok(field.render(cx).await)
    }
}

// Ordena las regiones por su nombre traducido al idioma dado, sin distinguir mayúsculas ni acentos
// (`Lc::collation_key()`); `Etc` sigue al final, como en `supported_by_region()`.
fn sort_by_label<T>(groups: &mut [(TzRegion, Lc, T)], language: &impl LangId) {
    groups.sort_by_cached_key(|(region, label, _)| {
        (*region == TzRegion::Etc, label.collation_key(language))
    });
}

// Nombre legible de una zona horaria usando espacios en lugar de guiones bajos (p. ej.
// `"America/New York"` para `"America/New_York"`).
fn zone_name(name: &'static str) -> CowStr {
    if name.contains('_') {
        name.replace('_', " ").into()
    } else {
        name.into()
    }
}

// Etiqueta de una zona horaria: su nombre legible y, si se indica, su desfase en segundos respecto
// a UTC (p. ej. `"Europe/Madrid (UTC+02:00)"`).
fn zone_label(name: CowStr, offset: Option<i32>) -> CowStr {
    let Some(offset) = offset else {
        return name;
    };
    let sign = if offset < 0 { '-' } else { '+' };
    let minutes = offset.abs() / 60;
    format!("{name} (UTC{sign}{:02}:{:02})", minutes / 60, minutes % 60).into()
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

    /// Establece si las etiquetas muestran el desfase actual respecto a UTC, incluida la de la
    /// opción de usar la zona horaria del sitio (p. ej. `"Europe/Madrid (UTC+02:00)"`), y ordenan
    /// las zonas de cada región por ese desfase y, a igual desfase, por nombre. Por defecto sólo se
    /// muestra el nombre de la zona y las zonas se ordenan por nombre.
    pub fn with_utc_offset(mut self, utc_offset: bool) -> Self {
        self.utc_offset = utc_offset;
        self
    }

    /// Establece las regiones que se ofrecen (p. ej. `[TzRegion::Europe, TzRegion::America]`).
    /// [`TzRegion::Etc`] (`Etc/UTC`) se ofrece siempre, se indique o no; una lista vacía ofrece
    /// todas. Valida después el valor recibido con [`Timezone::is_supported_in()`] y la misma
    /// lista.
    pub fn with_regions(mut self, regions: impl IntoIterator<Item = TzRegion>) -> Self {
        self.regions = regions.into_iter().collect();
        self
    }

    /// Establece el orden de las regiones (ver [`TimezoneOrder`](crate::global::TimezoneOrder)).
    pub fn with_order(mut self, order: global::TimezoneOrder) -> Self {
        self.order = order;
        self
    }

    /// Establece el nombre IANA de la zona horaria seleccionada (p. ej. `"Europe/Madrid"`). Vacía,
    /// o un nombre que no sea una zona horaria válida, selecciona la primera opción (ver
    /// [`SelectTimezone`]).
    pub fn with_selected(mut self, selected: impl Into<String>) -> Self {
        self.selected = selected.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::sort_by_label;
    use crate::datetime::TzRegion::{self, *};
    use crate::locale::{Lc, Locale};

    fn sorted(groups: &[(TzRegion, Lc)], language: &str) -> Vec<TzRegion> {
        let mut groups: Vec<_> = groups
            .iter()
            .map(|(region, label)| (*region, label.clone(), ()))
            .collect();
        sort_by_label(&mut groups, &Locale::resolve(language));
        groups.into_iter().map(|(region, _, _)| region).collect()
    }

    #[test]
    fn regions_are_sorted_ignoring_accents_with_etc_last() {
        let groups: Vec<_> = [Etc, Pacific, Asia, Arctic, Antarctica, Africa]
            .into_iter()
            .map(|region| (region, region.label()))
            .collect();
        assert_eq!(
            sorted(&groups, "es-ES"),
            [Africa, Antarctica, Arctic, Asia, Pacific, Etc]
        );
    }

    // With the current translations the translated order matches the IANA names; these labels
    // change it to check that regions are sorted by their translation.
    #[test]
    fn regions_are_sorted_by_translated_label_not_by_iana_name() {
        let groups = [
            (Atlantic, Lc::n("Océano Atlántico")),
            (Etc, Lc::n("Alguna otra")),
            (Europe, Lc::n("Europa")),
            (Pacific, Lc::n("Pacífico")),
        ];
        assert_eq!(sorted(&groups, "es-ES"), [Europe, Atlantic, Pacific, Etc]);
    }
}
