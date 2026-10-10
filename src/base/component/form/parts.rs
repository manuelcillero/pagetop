//! Piezas de marcado compartidas por los campos de formulario.
//!
//! [`form::parts::Label`](Label) (la etiqueta del campo, más la marca opcional de campo
//! obligatorio) y [`form::parts::Help`](Help) (el texto de ayuda) generan marcado común para los
//! principales campos de formularios, y quedan disponibles para los temas que puedan necesitarlos.
//!
//! **No son componentes**. Se crean dentro de `prepare()` (o del renderizado de un tema) a partir
//! de los textos del campo, y se insertan en [`html!`] con su método `render()`, que los traduce al
//! idioma del contexto.
//!
//! # Accesibilidad
//!
//! Es uno de sus puntos fuertes. Permiten aplicar de forma uniforme las relaciones que las pautas
//! [WCAG 2.2] exigen en los formularios desde el nivel A, con atributos [WAI-ARIA] cuando HTML no
//! basta:
//!
//! - **Cada etiqueta queda asociada a su control** con `for` o, en un grupo de controles, al
//!   contenedor del grupo, que la enlaza con `aria-labelledby`. Así el control o el grupo quedan
//!   con un nombre accesible.
//! - **El texto de ayuda queda asociado al control** (o al contenedor del grupo) con
//!   `aria-describedby`, de modo que las tecnologías de apoyo suelen anunciarlo como descripción.
//! - **No dejan referencias rotas**. Sin texto, o sin identificador al que enlazar, no se genera la
//!   relación.
//!
//! La marca de obligatorio (`*`) es texto de la etiqueta. Que el campo es obligatorio lo determina
//! el atributo `required` del propio control, que pone cada componente, igual que el `role="group"`
//! del contenedor de un grupo. Los atributos `aria-describedby` y `aria-labelledby` también los
//! escribe el componente, con los identificadores que dan [`Help::id()`] y [`Label::id()`].
//!
//! [WCAG 2.2]: https://www.w3.org/TR/WCAG22/
//! [WAI-ARIA]: https://www.w3.org/TR/wai-aria-1.2/
//!
//! # En un tema
//!
//! Un tema que renderice su propio marcado de un campo (con `render_component()`) puede:
//!
//! - **Reutilizarlas**, que es lo recomendable. Basta con añadir `aria-describedby=[help.id()]` al
//!   control (y `aria-labelledby=[label.id()]` al contenedor de un grupo) para conservar los
//!   enlaces de accesibilidad. Las piezas sirven aunque el tema cambie el orden de los elementos
//!   (p. ej. la etiqueta detrás del control, para una etiqueta flotante) o use otras clases
//!   (`with_class()` de cada pieza).
//! - **Reimplementar el marcado por completo**, si necesita otro distinto. En ese caso se
//!   recomienda mantener los enlaces de accesibilidad: la etiqueta asociada al control con `for`
//!   (o, en un grupo de controles, al contenedor con `aria-labelledby`) y el texto de ayuda
//!   asociado con `aria-describedby`.
//!
//! # Ejemplo
//!
//! Un control con su etiqueta, marcado como obligatorio, y su texto de ayuda:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # let cx = Context::default().with_langid(&Locale::resolve("en-US"));
//! let (name, hint) = (Lc::n("Name"), Lc::n("As on your ID card."));
//! let label = form::parts::Label::new(&name, Some("edit-name-input")).with_required(true);
//! let help = form::parts::Help::new(&hint, Some("edit-name-input"));
//! let markup = html! {
//!     (label.render(&cx))
//!     input type="text" id="edit-name-input" aria-describedby=[help.id()];
//!     (help.render(&cx))
//! };
//! # assert_eq!(markup.into_string(), concat!(
//! #     r#"<label for="edit-name-input" class="form-label">Name"#,
//! #     r#"<span class="form-required" title="This field is required">*</span></label>"#,
//! #     r#"<input type="text" id="edit-name-input" aria-describedby="edit-name-input-help">"#,
//! #     r#"<div id="edit-name-input-help" class="form-text">As on your ID card.</div>"#,
//! # ));
//! ```
//!
//! genera:
//!
//! ```html
//! <label for="edit-name-input" class="form-label">
//!     Name<span class="form-required" title="This field is required">*</span>
//! </label>
//! <input type="text" id="edit-name-input" aria-describedby="edit-name-input-help">
//! <div id="edit-name-input-help" class="form-text">As on your ID card.</div>
//! ```
//!
//! Un grupo de controles (casillas o botones de opción), cuyo contenedor enlaza la etiqueta y el
//! texto de ayuda del grupo:
//!
//! ```rust
//! # use pagetop::prelude::*;
//! # let cx = Context::default().with_langid(&Locale::resolve("en-US"));
//! let (name, hint) = (Lc::n("Notifications"), Lc::n("Choose one or more."));
//! let label = form::parts::Label::group(&name, "edit-notify");
//! let help = form::parts::Help::new(&hint, Some("edit-notify"));
//! let markup = html! {
//!     div id="edit-notify" role="group"
//!         aria-labelledby=[label.id()] aria-describedby=[help.id()]
//!     {
//!         (label.render(&cx))
//!         // Aquí, las casillas o los botones de opción del grupo.
//!         (help.render(&cx))
//!     }
//! };
//! # assert_eq!(markup.into_string(), concat!(
//! #     r#"<div id="edit-notify" role="group" aria-labelledby="edit-notify-label" "#,
//! #     r#"aria-describedby="edit-notify-help">"#,
//! #     r#"<label id="edit-notify-label" class="form-label">Notifications</label>"#,
//! #     r#"<div id="edit-notify-help" class="form-text">Choose one or more.</div></div>"#,
//! # ));
//! ```
//!
//! genera:
//!
//! ```html
//! <div id="edit-notify" role="group"
//!      aria-labelledby="edit-notify-label" aria-describedby="edit-notify-help">
//!     <label id="edit-notify-label" class="form-label">Notifications</label>
//!     <!-- Aquí, las casillas o los botones de opción del grupo. -->
//!     <div id="edit-notify-help" class="form-text">Choose one or more.</div>
//! </div>
//! ```
//!
//! [`html!`]: crate::html::html

mod help;
pub use help::Help;

mod label;
pub use label::Label;
