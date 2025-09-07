pub mod favicon;
pub mod javascript;
pub mod stylesheet;

use crate::html::{html, Markup, Render};
use crate::{AutoDefault, Weight};

/// Representación genérica de un *script* [`JavaScript`](crate::html::JavaScript) o una hoja de
/// estilos [`StyleSheet`](crate::html::StyleSheet).
///
/// Estos recursos se incluyen en los conjuntos de recursos ([`Assets`]) que suelen renderizarse en
/// un documento HTML.
///
/// Cada recurso se identifica por un **nombre único** ([`Asset::name()`]), usado como clave; y un
/// **peso** ([`Asset::weight()`]), que determina su orden relativo de renderizado.
pub trait Asset: Render {
    /// Devuelve el nombre del recurso, utilizado como clave única.
    fn name(&self) -> &str;

    /// Devuelve el peso del recurso, usado para ordenar el renderizado de menor a mayor peso.
    fn weight(&self) -> Weight;
}

/// Gestión común para conjuntos de recursos como [`JavaScript`](crate::html::JavaScript) y
/// [`StyleSheet`](crate::html::StyleSheet).
///
/// Se emplea normalmente para agrupar, administrar y renderizar los recursos de un documento HTML.
/// Cada recurso se identifica por un nombre único ([`Asset::name()`]) y tiene asociado un peso
/// ([`Asset::weight()`]) que determina su orden de renderizado.
///
/// Durante el renderizado, los recursos se procesan en orden ascendente de peso. En caso de
/// igualdad, se respeta el orden de inserción.
#[derive(AutoDefault)]
pub struct Assets<T>(Vec<T>);

impl<T: Asset> Assets<T> {
    /// Crea un nuevo conjunto vacío de recursos.
    ///
    /// Normalmente no se instancia directamente, sino como parte de la gestión de recursos que
    /// hacen páginas o temas.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Inserta un recurso.
    ///
    /// Si no existe otro con el mismo nombre, lo añade. Si ya existe y su peso era mayor, lo
    /// reemplaza. Y si su peso era menor o igual, entonces no realiza ningún cambio.
    ///
    /// Devuelve `true` si el recurso fue insertado o reemplazado.
    pub fn add(&mut self, asset: T) -> bool {
        match self.0.iter().position(|x| x.name() == asset.name()) {
            Some(index) => {
                if self.0[index].weight() > asset.weight() {
                    self.0.remove(index);
                    self.0.push(asset);
                    true
                } else {
                    false
                }
            }
            _ => {
                self.0.push(asset);
                true
            }
        }
    }

    /// Elimina un recurso por nombre.
    ///
    /// Devuelve `true` si el recurso existía y fue eliminado.
    pub fn remove(&mut self, name: impl AsRef<str>) -> bool {
        if let Some(index) = self.0.iter().position(|x| x.name() == name.as_ref()) {
            self.0.remove(index);
            true
        } else {
            false
        }
    }
}

impl<T: Asset> Render for Assets<T> {
    fn render(&self) -> Markup {
        let mut assets = self.0.iter().collect::<Vec<_>>();
        assets.sort_by_key(|a| a.weight());

        html! {
            @for a in assets {
                (a)
            }
        }
    }
}
