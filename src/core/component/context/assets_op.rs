use crate::CowStr;
use crate::core::theme::Breakpoint;
use crate::html::{Favicon, JavaScript, Preload, StyleSheet};

/// Operaciones para modificar recursos asociados al [`Context`](super::Context) de un documento.
///
/// [`Favicon`], [`Preload`], [`StyleSheet`] y [`JavaScript`] se convierten implícitamente en la
/// operación de añadir correspondiente (ver sus `impl From<...>` más abajo), por lo que no
/// necesitarían ningún constructor. Para el resto de operaciones, el método recomendado es recurrir
/// a los constructores asociados como [`remove_stylesheet()`], [`add_responsive_style()`], etc.
///
/// [`remove_stylesheet()`]: Self::remove_stylesheet
/// [`add_responsive_style()`]: Self::add_responsive_style
pub enum AssetsOp {
    /// Define el *favicon* del documento. Sobrescribe cualquier valor anterior.
    SetFavicon(Option<Favicon>),
    /// Define el *favicon* sólo si no se ha establecido previamente.
    SetFaviconIfNone(Favicon),

    /// Añade un recurso para precarga al documento.
    AddPreload(Preload),
    /// Elimina un recurso para precarga por su ruta.
    RemovePreload(&'static str),

    /// Añade una hoja de estilos CSS al documento.
    AddStyleSheet(StyleSheet),
    /// Elimina una hoja de estilos por su ruta.
    RemoveStyleSheet(&'static str),

    /// Añade un script JavaScript al documento.
    AddJavaScript(JavaScript),
    /// Elimina un script por su ruta o identificador.
    RemoveJavaScript(&'static str),

    /// Añade una declaración de estilo *responsive* (`property: value`) para las clases indicadas,
    /// para un punto de corte dado (`None` para una regla siempre activa). Ver
    /// [`ResponsiveStyles::add_style()`](crate::html::ResponsiveStyles::add_style).
    AddResponsiveStyle(Option<Breakpoint>, CowStr, CowStr, CowStr),
    /// Añade varias declaraciones de estilo *responsive* (`property: value`) para las clases
    /// indicadas, para un punto de corte dado, en una única llamada. Ver
    /// [`ResponsiveStyles::add_styles()`](crate::html::ResponsiveStyles::add_styles).
    AddResponsiveStyles(Option<Breakpoint>, CowStr, Vec<(CowStr, CowStr)>),
}

impl AssetsOp {
    /// Crea la variante [`SetFavicon`](Self::SetFavicon) con el favicon indicado, o `None` para
    /// eliminar cualquier favicon ya establecido.
    pub fn set_favicon(favicon: impl Into<Option<Favicon>>) -> Self {
        Self::SetFavicon(favicon.into())
    }

    /// Crea la variante [`SetFaviconIfNone`](Self::SetFaviconIfNone) con el favicon indicado.
    pub fn set_favicon_if_none(favicon: Favicon) -> Self {
        Self::SetFaviconIfNone(favicon)
    }

    /// Crea la variante [`AddPreload`](Self::AddPreload) con el recurso indicado.
    pub fn add_preload(preload: Preload) -> Self {
        Self::AddPreload(preload)
    }

    /// Crea la variante [`RemovePreload`](Self::RemovePreload) para la ruta indicada.
    pub fn remove_preload(path: &'static str) -> Self {
        Self::RemovePreload(path)
    }

    /// Crea la variante [`AddStyleSheet`](Self::AddStyleSheet) con la hoja de estilos indicada.
    pub fn add_stylesheet(stylesheet: StyleSheet) -> Self {
        Self::AddStyleSheet(stylesheet)
    }

    /// Crea la variante [`RemoveStyleSheet`](Self::RemoveStyleSheet) para la ruta indicada.
    pub fn remove_stylesheet(path: &'static str) -> Self {
        Self::RemoveStyleSheet(path)
    }

    /// Crea la variante [`AddJavaScript`](Self::AddJavaScript) con el script indicado.
    pub fn add_javascript(js: JavaScript) -> Self {
        Self::AddJavaScript(js)
    }

    /// Crea la variante [`RemoveJavaScript`](Self::RemoveJavaScript) para la ruta o identificador
    /// indicado.
    pub fn remove_javascript(path: &'static str) -> Self {
        Self::RemoveJavaScript(path)
    }

    /// Crea la variante [`AddResponsiveStyle`](Self::AddResponsiveStyle) con la declaración de
    /// estilo (`property: value`) indicada, para las clases y el punto de corte dados.
    pub fn add_responsive_style(
        breakpoint: impl Into<Option<Breakpoint>>,
        classes: impl Into<CowStr>,
        property: impl Into<CowStr>,
        value: impl Into<CowStr>,
    ) -> Self {
        Self::AddResponsiveStyle(
            breakpoint.into(),
            classes.into(),
            property.into(),
            value.into(),
        )
    }

    /// Crea la variante [`AddResponsiveStyles`](Self::AddResponsiveStyles) con las declaraciones
    /// de estilo (`property: value`) indicadas, para las clases y el punto de corte dados.
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let op = AssetsOp::add_responsive_styles(
    ///     None,
    ///     "flex-demo-box",
    ///     [("background-color", "#0d6efd"), ("color", "#fff")],
    /// );
    /// ```
    pub fn add_responsive_styles(
        breakpoint: impl Into<Option<Breakpoint>>,
        classes: impl Into<CowStr>,
        styles: impl IntoIterator<Item = (impl Into<CowStr>, impl Into<CowStr>)>,
    ) -> Self {
        Self::AddResponsiveStyles(
            breakpoint.into(),
            classes.into(),
            styles
                .into_iter()
                .map(|(p, v)| (p.into(), v.into()))
                .collect(),
        )
    }
}

impl From<Favicon> for AssetsOp {
    /// Convierte un favicon en [`AssetsOp::SetFavicon`] (lo sobrescribe siempre), permitiendo
    /// pasarlo directamente a métodos como [`Contextual::with_assets`] sin envolverlo
    /// explícitamente. Para establecerlo sólo si no hay uno ya definido, usar
    /// [`AssetsOp::set_favicon_if_none()`] explícitamente.
    ///
    /// [`Contextual::with_assets`]: crate::core::component::Contextual::with_assets
    #[inline]
    fn from(favicon: Favicon) -> Self {
        Self::SetFavicon(Some(favicon))
    }
}

impl From<Preload> for AssetsOp {
    /// Convierte un recurso de precarga en [`AssetsOp::AddPreload`]. Ver la conversión
    /// equivalente para [`Favicon`].
    #[inline]
    fn from(preload: Preload) -> Self {
        Self::AddPreload(preload)
    }
}

impl From<StyleSheet> for AssetsOp {
    /// Convierte una hoja de estilos en [`AssetsOp::AddStyleSheet`]. Ver la conversión
    /// equivalente para [`Favicon`].
    #[inline]
    fn from(stylesheet: StyleSheet) -> Self {
        Self::AddStyleSheet(stylesheet)
    }
}

impl From<JavaScript> for AssetsOp {
    /// Convierte un script en [`AssetsOp::AddJavaScript`]. Ver la conversión equivalente para
    /// [`Favicon`].
    #[inline]
    fn from(js: JavaScript) -> Self {
        Self::AddJavaScript(js)
    }
}
