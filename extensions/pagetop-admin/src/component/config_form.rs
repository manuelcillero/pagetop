use pagetop::prelude::*;

use crate::LOCALES_ADMIN;
use crate::settings::{SettingFieldType, SettingsSchema, get_or};

/// Formulario de un [`SettingsSchema`], con los valores guardados en la tabla `settings`.
///
/// En las páginas declaradas con [`DeclareConfigForms`](crate::action::DeclareConfigForms),
/// `pagetop-admin` lo muestra y guarda lo que se envía. En una página propia sólo lo muestra: el
/// envío llega a la ruta de la página, o a la de [`with_action_path()`](Self::with_action_path), y
/// lo guarda su *handler* (ver [`settings::set()`](crate::settings::set)).
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_admin::component::ConfigForm;
/// use pagetop_admin::settings::{SettingField, SettingsSchema};
///
/// async fn settings_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     let schema = SettingsSchema::new("myapp.general")
///         .with_field(SettingField::text("site_name", Lc::n("Site name")).with_required(true));
///
///     Page::admin(request)
///         .with_title(Lc::n("General settings"))
///         .with_child(ConfigForm::with_schema(schema))
///         .render()
///         .await
/// }
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct ConfigForm {
    /// Devuelve el esquema del formulario, si se ha establecido uno.
    schema: Option<SettingsSchema>,
    /// Devuelve la ruta de destino del formulario, si se ha personalizado.
    action_path: Option<Route>,
}

#[async_trait]
impl Component for ConfigForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let Some(schema) = self.schema() else {
            return Ok(html! {});
        };

        let action = match self.action_path() {
            Some(route) => route.resolve(cx),
            None => cx.route(cx.request().map(|r| r.path()).unwrap_or("/")),
        };

        // Pre-computa los valores de BD para cada campo antes del macro html! (que es síncrono). El
        // formulario guarda todos los valores como texto (`"42"`, `"true"`), así que se leen como
        // texto también los de los campos numéricos y las casillas.
        struct FieldVals {
            raw_val: String,
            checked: bool,
        }
        let mut prepared: Vec<FieldVals> = Vec::with_capacity(schema.fields().len());
        for field in schema.fields() {
            let key = schema.key_for(field.name());
            let raw_val =
                get_or::<String>(&key, field.default_value().cloned().unwrap_or_default()).await;
            let checked = raw_val == "true";
            prepared.push(FieldVals { raw_val, checked });
        }

        Ok(html! {
            form class="admin-config-form" method="post" action=(action) {
                @for (i, field) in schema.fields().iter().enumerate() {
                    @let pf = &prepared[i];
                    div class="admin-form-field" {
                        label class="admin-form-label" for=(field.name()) {
                            (field.label().using(cx))
                        }
                        @match field.field_type() {
                            SettingFieldType::Text { max_length } => {
                                @if let Some(max) = max_length {
                                    input class="admin-form-input"
                                        type="text"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        maxlength=(max)
                                        required[*field.required()];
                                } @else {
                                    input class="admin-form-input"
                                        type="text"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        required[*field.required()];
                                }
                            }
                            SettingFieldType::Number { min, max } => {
                                @if let (Some(lo), Some(hi)) = (min, max) {
                                    input class="admin-form-input"
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        min=(lo)
                                        max=(hi)
                                        required[*field.required()];
                                } @else if let Some(lo) = min {
                                    input class="admin-form-input"
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        min=(lo)
                                        required[*field.required()];
                                } @else if let Some(hi) = max {
                                    input class="admin-form-input"
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        max=(hi)
                                        required[*field.required()];
                                } @else {
                                    input class="admin-form-input"
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        required[*field.required()];
                                }
                            }
                            SettingFieldType::Boolean => {
                                input class="admin-form-checkbox"
                                    type="checkbox"
                                    id=(field.name())
                                    name=(field.name())
                                    value="true"
                                    checked[pf.checked];
                            }
                            SettingFieldType::Select { options } => {
                                select class="admin-form-select"
                                    id=(field.name())
                                    name=(field.name())
                                    required[*field.required()]
                                {
                                    @for (val, label) in options {
                                        @if &pf.raw_val == val {
                                            option value=(val) selected { (label.using(cx)) }
                                        } @else {
                                            option value=(val) { (label.using(cx)) }
                                        }
                                    }
                                }
                            }
                        }
                        @if !field.help_text().is_none() {
                            small class="admin-form-help" { (field.help_text().using(cx)) }
                        }
                    }
                }
                div class="admin-form-actions" {
                    button class="admin-btn admin-btn-primary" type="submit" {
                        (Lc::t("config-save-label", &LOCALES_ADMIN).using(cx))
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl ConfigForm {
    /// Crea el componente con el [`SettingsSchema`] dado.
    #[builder_skip]
    pub fn with_schema(schema: SettingsSchema) -> Self {
        ConfigForm {
            schema: Some(schema),
            ..Self::default()
        }
    }

    /// Establece la ruta a la que se envía el formulario, o `None` para enviarlo a la de la
    /// página.
    pub fn with_action_path(mut self, path: impl Into<Option<Route>>) -> Self {
        self.action_path = path.into();
        self
    }
}
