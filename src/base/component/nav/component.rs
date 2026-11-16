use crate::prelude::*;

/// Componente para crear un **menú de navegación plano**.
///
/// Presenta una lista de elementos [`nav::Item`](super::Item), donde alguno puede desplegar un
/// [`Dropdown`](super::super::Dropdown) embebido.
///
/// Si no contiene elementos, o si ninguno se muestra (ver [`Endpoint`]), el componente **no se
/// renderiza**.
///
/// Con [`with_renderable()`](Nav::with_renderable) se puede decidir en cada petición si se muestra,
/// y con [`Nav::admin()`] presenta la navegación del panel de administración.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let nav = nav::Nav::new()
///     .with_item(nav::Item::link(Lc::n("Home"), "/"))
///     .with_item(nav::Item::link_blank(Lc::n("External"), "https://docs.rs"))
///     .with_item(nav::Item::dropdown(
///         dropdown::Dropdown::new()
///             .with_title(Lc::n("Options"))
///             .with_item(dropdown::Item::link(Lc::n("Action"), "/action"))
///             .with_item(dropdown::Item::link(Lc::n("Another"), "/another")),
///     ))
///     .with_item(nav::Item::link_disabled(Lc::n("Disabled"), "#"));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Nav {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la distribución y orientación seleccionada.
    nav_layout: nav::Layout,
    /// Devuelve la lista de elementos del menú.
    items: Children,
    /// Devuelve la función que decide en cada petición si se renderiza, si la hay (ver
    /// [`with_renderable()`](Nav::with_renderable)).
    #[getters(copy)]
    renderable: Option<FnIsRenderable>,
    /// Indica si presenta la navegación del panel de administración (ver [`Nav::admin()`]).
    #[getters(copy)]
    is_admin: bool,
}

#[async_trait]
impl Component for Nav {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        self.renderable().is_none_or(|f| f(cx)) && (!self.is_admin() || is_admin_panel(cx))
    }

    fn setup(&mut self, cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes(match self.nav_layout() {
            nav::Layout::Default => "nav",
            nav::Layout::Start => "nav nav-start",
            nav::Layout::Center => "nav nav-center",
            nav::Layout::End => "nav nav-end",
            nav::Layout::Vertical => "nav nav-vertical",
            nav::Layout::Fill => "nav nav-fill",
            nav::Layout::Justified => "nav nav-justified",
        }));
        if self.is_admin() {
            self.alter_prop(PropsOp::add_classes("nav-admin"));
            self.add_admin_items(cx);
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let items = self.items().render(cx).await;
        if items.is_empty() {
            return Ok(html! {});
        }

        Ok(html! {
            ul (self.props().unpack(cx)) {
                (items)
            }
        })
    }
}

impl Nav {
    /// Crea la navegación del panel de administración para el usuario de la petición actual.
    ///
    /// Al renderizarse añade un enlace al inicio del panel (*dashboard*) seguido de un desplegable
    /// por sección, con las páginas de cada una a las que el usuario puede acceder; las secciones
    /// sin ninguna no se muestran. Los construye en cada petición a partir de [`AdminRegistry`],
    /// detrás de los elementos que se hayan añadido con [`with_item()`](Self::with_item).
    ///
    /// Sólo se renderiza en las páginas del panel y si el usuario puede acceder a él
    /// ([`is_admin_panel()`]); en las demás no se despachan sus acciones ni intervienen los temas.
    /// Así puede colocarse en una barra o región presente en todas las páginas. Para que ocupe en
    /// el panel el sitio de otro menú, ese otro menú se oculta en el panel con
    /// [`with_renderable()`](Self::with_renderable) (ver el ejemplo).
    ///
    /// La plantilla [`CoreTemplates::Admin`] no la pinta: la coloca la aplicación donde quiera,
    /// como cualquier otro menú, con su disposición, clases y propiedades. Lleva además la clase
    /// `nav-admin`, para que los temas puedan darle un aspecto propio.
    ///
    /// # Ejemplos
    ///
    /// En una barra aparte bajo la cabecera, centrada:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let admin = Navbar::simple().with_item(navbar::Item::nav(
    ///     Nav::admin()
    ///         .with_layout(nav::Layout::Center)
    ///         .with_prop(FlexItem::new().with_grow(flex::ItemGrow::Is1)),
    /// ));
    ///
    /// InRegion::Global(&CoreRegions::Header).add(admin);
    /// ```
    ///
    /// En la barra principal, junto al menú de la aplicación y centrada en el espacio libre:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let navbar = Navbar::brand_left(Brand::new().with_title(Lc::n("My app")))
    ///     .with_item(navbar::Item::nav(
    ///         Nav::new().with_item(nav::Item::link(Lc::n("Home"), "/")),
    ///     ))
    ///     .with_item(navbar::Item::nav(
    ///         Nav::admin()
    ///             .with_layout(nav::Layout::Center)
    ///             .with_prop(FlexItem::new().with_grow(flex::ItemGrow::Is1)),
    ///     ));
    ///
    /// InRegion::Global(&CoreRegions::Header).add(navbar);
    /// ```
    pub fn admin() -> Self {
        Self {
            is_admin: true,
            ..Self::default()
        }
    }

    // Añade el enlace al *dashboard* y un desplegable por sección con las páginas a las que puede
    // acceder el usuario de la petición.
    fn add_admin_items(&mut self, cx: &Context) {
        let registry = AdminRegistry::global();

        self.alter_item(nav::Item::link(Lc::l("admin_dashboard"), admin_path("")));
        for section in registry.sections() {
            let mut pages = registry
                .pages_for_section(section.key())
                .iter()
                .filter(|p| p.is_accessible(cx))
                .peekable();
            if pages.peek().is_none() {
                continue;
            }
            let mut menu = Dropdown::new().with_title(section.title().clone());
            for page in pages {
                menu = menu.with_item(dropdown::Item::link(
                    page.title().clone(),
                    admin_path(page.path()),
                ));
            }
            self.alter_item(nav::Item::dropdown(menu));
        }
    }
}

#[builder_impl]
impl Nav {
    // **< Nav BUILDER >****************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Selecciona la distribución y orientación del menú.
    pub fn with_layout(mut self, layout: nav::Layout) -> Self {
        self.nav_layout = layout;
        self
    }

    /// Asigna una función que decide en cada petición si el menú se renderiza.
    ///
    /// Se evalúa antes que cualquier otra fase de su ciclo: si devuelve `false`, el menú no se
    /// renderiza, no se despachan sus acciones ni intervienen los temas. Así un menú creado una
    /// sola vez (por ejemplo, en una barra registrada con [`InRegion`]) puede mostrarse sólo en
    /// las páginas que corresponda. Un cierre sin capturas sirve como función.
    ///
    /// # Ejemplo
    ///
    /// Menú principal que, en las páginas del panel de administración, deja su sitio a la
    /// navegación del panel ([`Nav::admin()`]). Como ésta se muestra justo cuando se cumple
    /// [`is_admin_panel()`], el menú principal usa la condición contraria: en cada página se ve uno
    /// de los dos, también en una página del panel que responde a quien no tiene acceso (un error
    /// 403), donde se ve el menú principal:
    ///
    /// ```rust,no_run
    /// # use pagetop::prelude::*;
    /// let main = Nav::new()
    ///     .with_item(nav::Item::link(Lc::n("Home"), "/"))
    ///     .with_renderable(|cx| !is_admin_panel(cx));
    ///
    /// let navbar = Navbar::brand_left(Brand::new().with_title(Lc::n("My app")))
    ///     .with_item(navbar::Item::nav(main))
    ///     .with_item(navbar::Item::nav(Nav::admin()));
    ///
    /// InRegion::Global(&CoreRegions::Header).add(navbar);
    /// ```
    pub fn with_renderable(mut self, f: FnIsRenderable) -> Self {
        self.renderable = Some(f);
        self
    }

    /// Añade un nuevo elemento al menú o modifica la lista de elementos del menú con una
    /// operación [`TypedOp`].
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// nav.with_item(nav::Item::link("Inicio", "/"));
    /// nav.with_item(TypedOp::AddMany(vec![
    ///     nav::Item::link(...),
    ///     nav::Item::link_disabled(...),
    /// ]));
    /// ```
    pub fn with_item(mut self, op: impl Into<TypedOp<nav::Item>>) -> Self {
        self.items.alter_child(op.into());
        self
    }
}
