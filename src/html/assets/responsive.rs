use crate::core::component::Context;
use crate::core::theme::Breakpoint;
use crate::html::{Markup, PreEscaped, html};
use crate::{AutoDefault, CowStr, util};

// Punto de corte (None si no aplica), clase(s) normalizadas y sus declaraciones `propiedad: valor`.
type Entry = (Option<Breakpoint>, CowStr, Vec<(CowStr, CowStr)>);

/// Declaraciones de estilo en línea agrupadas por punto de corte ([`Breakpoint`]).
///
/// También por clase (o clases) que van a renderizarse en bloques `@media` en el `<head>` del
/// documento.
///
/// Por tanto, cada entrada se identifica por la pareja (punto de corte, clases) y acumula sus
/// propias declaraciones `propiedad: valor`, en el orden en que se añaden. Las clases se guardan en
/// bruto, normalizadas igual que hace [`Props`](crate::html::Props) (ASCII, minúsculas y un único
/// espacio entre tokens): por ejemplo `"foo bar"`, no `.foo.bar`.
///
/// El punto de corte es opcional, donde `None` declara una regla siempre activa, sin pasar por el
/// tema ni depender de que resuelva algún [`Breakpoint`]. No ordena ni combina las clases de dos
/// llamadas que las declaren en distinto orden (por ejemplo, `"foo bar"` y `"bar foo"` generan dos
/// entradas distintas). La llamada es a través de [`AssetsOp::AddResponsiveStyle`].
///
/// [`AssetsOp::AddResponsiveStyle`]: crate::core::component::AssetsOp::AddResponsiveStyle
#[derive(AutoDefault, Clone, Debug)]
pub struct ResponsiveStyles(Vec<Entry>);

impl ResponsiveStyles {
    /// Crea un conjunto vacío.
    pub fn new() -> Self {
        Self::default()
    }

    /// Añade una declaración de estilo (`property: value`) para las clases indicadas, dentro del
    /// punto de corte dado.
    ///
    /// Si ya existe una declaración para la misma propiedad, en el mismo punto de corte y con las
    /// mismas clases, la llamada no hace nada: se conserva el valor ya almacenado, no se sustituye.
    /// Pensado para clases utilitarias generadas automáticamente, donde el mismo nombre de clase
    /// implica siempre el mismo valor -- declararla de nuevo es entonces una operación de sólo
    /// lectura, sin normalizar `value`, en vez de una escritura.
    ///
    /// Si `classes` contiene caracteres no ASCII, o si `classes`, `property` o `value` quedan
    /// vacíos tras recortar espacios, la operación se ignora.
    pub fn add_style(
        &mut self,
        breakpoint: impl Into<Option<Breakpoint>>,
        classes: impl AsRef<str>,
        property: impl AsRef<str>,
        value: impl AsRef<str>,
    ) {
        let breakpoint = breakpoint.into();

        let Some(classes) = util::normalize_ascii(classes.as_ref()) else {
            return;
        };
        if classes.is_empty() {
            return;
        }
        let classes: CowStr = classes.into_owned().into();

        let property_norm = property.as_ref().trim().to_ascii_lowercase();
        if property_norm.is_empty() {
            return;
        }
        let property: CowStr = property_norm.into();

        match self
            .0
            .iter_mut()
            .find(|(bp, cls, _)| *bp == breakpoint && *cls == classes)
        {
            Some((_, _, styles)) => {
                // Ya declarada: se descarta sin normalizar `value`, el camino habitual (y que debe
                // ser barato) cuando muchos componentes comparten la misma clase utilitaria.
                if styles.iter().any(|(k, _)| *k == property) {
                    return;
                }
                let Some(value) = util::non_blank(value.as_ref()) else {
                    return;
                };
                styles.push((property, value.to_string().into()));
            }
            None => {
                let Some(value) = util::non_blank(value.as_ref()) else {
                    return;
                };
                self.0.push((
                    breakpoint,
                    classes,
                    vec![(property, value.to_string().into())],
                ));
            }
        }
    }

    // **< ResponsiveStyles GETTERS >***************************************************************

    /// Devuelve el valor de la propiedad indicada para el punto de corte y las clases dados, si
    /// existe.
    pub fn get_style(
        &self,
        breakpoint: impl Into<Option<Breakpoint>>,
        classes: impl AsRef<str>,
        property: impl AsRef<str>,
    ) -> Option<String> {
        let styles = self.entry(breakpoint.into(), classes.as_ref())?;
        let property = property.as_ref().trim().to_ascii_lowercase();
        styles
            .iter()
            .find(|(k, _)| k.as_ref() == property)
            .map(|(_, v)| v.to_string())
    }

    /// Devuelve todas las declaraciones de estilo del punto de corte y las clases dados, como
    /// cadena de texto (separadas por `"; "`), si existen.
    pub fn get_styles(
        &self,
        breakpoint: impl Into<Option<Breakpoint>>,
        classes: impl AsRef<str>,
    ) -> Option<String> {
        let styles = self.entry(breakpoint.into(), classes.as_ref())?;
        if styles.is_empty() {
            return None;
        }
        Some(
            styles
                .iter()
                .map(|(k, v)| util::join!(k.as_ref(), ": ", v.as_ref()))
                .collect::<Vec<_>>()
                .join("; "),
        )
    }

    /// Devuelve `true` si no hay ninguna declaración almacenada.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // **< ResponsiveStyles RENDER >****************************************************************

    /// Renderiza las declaraciones acumuladas como texto CSS.
    ///
    /// Emite primero las declaraciones sin punto de corte (`None`), siempre sin envoltorio. Luego
    /// recorre los puntos de corte reales en orden *mobile-first* (`Xs` a `Xxxl`, omitiendo los que
    /// no tengan ninguna declaración) y, para cada uno, agrupa las reglas de todas sus entradas
    /// (`.clases{propiedad:valor;...}`). Si el ancho mínimo resuelto por el tema activo
    /// ([`Breakpoint::min_width()`]) es una cadena vacía, las reglas se emiten tal cual, sin punto
    /// de corte real; en cualquier otro caso se envuelven en `@media(min-width:...)`.
    ///
    /// El resultado no contiene espacios ni saltos de línea salvo los que pueda llevar el propio
    /// valor de una declaración (p. ej. `font-family: "Segoe UI", sans-serif`).
    pub fn render(&self, cx: &Context) -> Markup {
        let mut css = String::new();

        css.push_str(&self.render_rules(None));

        for breakpoint in Breakpoint::ALL {
            let rules = self.render_rules(Some(breakpoint));
            if rules.is_empty() {
                continue;
            }
            let min_width = breakpoint.min_width(cx);
            if min_width.is_empty() {
                css.push_str(&rules);
            } else {
                css.push_str(&util::join!(
                    "@media(min-width:",
                    min_width,
                    "){",
                    rules,
                    "}"
                ));
            }
        }

        html! { (PreEscaped(css)) }
    }

    // Construye, concatenadas y sin separador, las reglas CSS (`.clases{propiedad:valor;...}`) de
    // todas las entradas del punto de corte indicado.
    fn render_rules(&self, breakpoint: Option<Breakpoint>) -> String {
        let mut rules = String::new();
        for (_, classes, styles) in self
            .0
            .iter()
            .filter(|(bp, _, styles)| *bp == breakpoint && !styles.is_empty())
        {
            let selector = classes.replace(' ', ".");
            let declarations = styles
                .iter()
                .map(|(property, value)| util::join!(property.as_ref(), ":", value.as_ref()))
                .collect::<Vec<_>>()
                .join(";");
            rules.push_str(&util::join!(".", selector, "{", declarations, "}"));
        }
        rules
    }

    // Normaliza `classes` igual que `add_style()` y busca las declaraciones de la entrada
    // correspondiente al punto de corte y las clases dados.
    fn entry(
        &self,
        breakpoint: Option<Breakpoint>,
        classes: &str,
    ) -> Option<&Vec<(CowStr, CowStr)>> {
        let classes = util::normalize_ascii(classes)?;
        if classes.is_empty() {
            return None;
        }
        self.0
            .iter()
            .find(|(bp, cls, _)| *bp == breakpoint && cls.as_ref() == classes.as_ref())
            .map(|(_, _, styles)| styles)
    }
}
