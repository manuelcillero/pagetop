use crate::CowStr;
use crate::auth::Access;
use crate::core::component::Route;
use crate::web::push_percent_encoded;

use std::fmt;

/// Ruta de la aplicación con el acceso que la habilita, para montarla y enlazarla sin repetirlo.
///
/// Se implementa con un `enum` por extensión, igual que [`Permission`]: cada variante es una ruta
/// y, si `access()` es un `match` sin comodín (`_`), añadir una sin decir quién puede acceder a
/// ella no compila. La ruta se monta con [`Routes::endpoint()`] y se convierte en una [`Route`]
/// para usarla como destino de un enlace.
///
/// [`nav::Item`], [`dropdown::Item`] y [`Button::anchor()`] no se renderizan si el usuario actual
/// no tiene acceso a su ruta (y un [`Nav`] o un [`Dropdown`] sin elementos que mostrar, tampoco),
/// así que un enlace nunca nombra el permiso y no puede desalinearse de la ruta. El resto de
/// componentes usan la ruta sin ocultar nada. Sólo es una ayuda para la interfaz: quien escriba la
/// URL recibe la respuesta del acceso de la ruta.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(Clone, Copy, Debug)]
/// # enum BoardPermission { PostNotices }
/// # impl Permission for BoardPermission {
/// #     fn key(&self) -> CowStr { "board:post_notices".into() }
/// # }
/// # async fn notices() -> &'static str { "" }
/// # async fn new_notice() -> &'static str { "" }
/// # async fn edit_notice() -> &'static str { "" }
/// #[derive(Clone, Copy, Debug)]
/// enum BoardRoute {
///     Notices,
///     NewNotice,
///     EditNotice,
/// }
///
/// impl Endpoint for BoardRoute {
///     fn path(&self) -> CowStr {
///         match self {
///             Self::Notices => "/notices".into(),
///             Self::NewNotice => "/notices/new".into(),
///             Self::EditNotice => "/notices/{id}/edit".into(),
///         }
///     }
///
///     fn access(&self) -> Access {
///         match self {
///             Self::Notices => Access::Public,
///             Self::NewNotice | Self::EditNotice => {
///                 Access::Permission(&BoardPermission::PostNotices)
///             }
///         }
///     }
/// }
///
/// fn configure_router(routes: Routes) -> Routes {
///     routes
///         .endpoint(BoardRoute::Notices, web::get(notices))
///         .endpoint(BoardRoute::NewNotice, web::get(new_notice))
///         .endpoint(BoardRoute::EditNotice, web::get(edit_notice))
/// }
///
/// // Quien no pueda publicar no verá ni el segundo enlace ni el botón.
/// fn menu(id: i32) -> (Nav, Button) {
///     let nav = Nav::new()
///         .with_item(nav::Item::link(Lc::n("Notices"), BoardRoute::Notices))
///         .with_item(nav::Item::link(Lc::n("New notice"), BoardRoute::NewNotice));
///     (nav, Button::anchor(Lc::n("Edit"), BoardRoute::EditNotice.to(id)))
/// }
/// ```
///
/// El acceso sólo responde por la ruta, no por lo que compruebe además su *handler*. Si varios
/// métodos de una misma ruta exigen un acceso distinto, cada uno es una variante con la misma ruta,
/// y el enlace usa la del método `GET`.
///
/// [`Permission`]: crate::auth::Permission
/// [`Routes::endpoint()`]: crate::web::Routes::endpoint
/// [`nav::Item`]: crate::base::component::nav::Item
/// [`dropdown::Item`]: crate::base::component::dropdown::Item
/// [`Button::anchor()`]: crate::base::component::Button::anchor
/// [`Nav`]: crate::base::component::Nav
/// [`Dropdown`]: crate::base::component::Dropdown
pub trait Endpoint {
    /// Devuelve la ruta tal como se monta, con la sintaxis de parámetros de Axum si lo tiene
    /// (`"/notices/{id}/edit"`).
    fn path(&self) -> CowStr;

    /// Devuelve quién puede acceder a la ruta.
    fn access(&self) -> Access;

    /// Crea el enlace a una ruta con parámetro, conservando su acceso.
    ///
    /// Sustituye el parámetro `{...}` por el valor indicado: `BoardRoute::EditNotice.to(7)` enlaza
    /// con `/notices/7/edit`. El valor se codifica (`%XX`) salvo letras, dígitos, `-`, `_`, `.` y
    /// `~`, así que no puede añadir tramos, parámetros de consulta ni otro dominio a la URL. Aun
    /// así, un valor vacío deja un tramo vacío, que no coincide con la ruta, y el navegador
    /// resuelve `.` y `..` como otra ruta: los identificadores numéricos o validados no tienen ese
    /// problema.
    ///
    /// En desarrollo y en las pruebas falla si la ruta no tiene exactamente un parámetro.
    fn to(&self, param: impl fmt::Display) -> Route {
        let pattern = self.path();
        let parts = pattern
            .split_once('{')
            .and_then(|(prefix, rest)| Some((prefix, rest.split_once('}')?.1)));
        debug_assert!(
            parts.is_some(),
            "route `{pattern}` has no parameter to fill"
        );
        let Some((prefix, suffix)) = parts else {
            return Route::endpoint(pattern, self.access());
        };
        debug_assert!(
            !suffix.contains('{'),
            "route `{pattern}` has more than one parameter"
        );
        let param = param.to_string();
        let mut path = String::with_capacity(prefix.len() + param.len() + suffix.len());
        path.push_str(prefix);
        push_percent_encoded(&mut path, param);
        path.push_str(suffix);
        Route::endpoint(path, self.access())
    }
}
