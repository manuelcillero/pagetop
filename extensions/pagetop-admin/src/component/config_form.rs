use pagetop::prelude::*;

use crate::LOCALES_ADMIN;
use crate::settings::{SettingFieldType, SettingsSchema, get_or};

/// Componente que renderiza un formulario de configuración persistente.
///
/// Genera campos HTML a partir de un [`SettingsSchema`] y carga los valores
/// actuales desde `settings`. El POST es procesado por el handler interno
/// `config_form_post`.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_admin::component::{AdminFrame, ConfigForm};
/// use pagetop_admin::settings::{SettingField, SettingsSchema};
///
/// async fn settings_handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     let schema = SettingsSchema::new("myapp.general")
///         .with_field(SettingField::text("site_name", "Site name").with_required(true));
///
///     let title    = "General settings";
///     let mut form = ConfigForm::with_schema(schema);
///     let mut cx   = Context::new(request.clone());
///     let content  = form.render(&mut cx).await;
///
///     Page::admin(request)
///         .with_title(Lc::n(title))
///         .with_child(
///             AdminFrame::new()
///                 .with_title(Lc::n(title))
///                 .with_child(Html::with(move |_| content.clone())),
///         )
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
    /// Devuelve si el último envío se guardó correctamente.
    saved: bool,
    /// Devuelve si el último envío produjo un error al guardar.
    error: bool,
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

        let save_label = Lc::t("config-save-label", &LOCALES_ADMIN)
            .lookup(cx)
            .unwrap_or_else(|| "Save configuration".into());

        let saved_msg = Lc::t("config-saved-ok", &LOCALES_ADMIN)
            .lookup(cx)
            .unwrap_or_else(|| "Configuration saved.".into());

        let error_msg = Lc::t("config-saved-error", &LOCALES_ADMIN)
            .lookup(cx)
            .unwrap_or_else(|| "Could not save configuration.".into());

        // Pre-computa los valores de BD para cada campo antes del macro html! (que es síncrono).
        struct FieldVals {
            raw_val: String,
            num_val: String,
            checked: bool,
        }
        let mut prepared: Vec<FieldVals> = Vec::with_capacity(schema.fields().len());
        for field in schema.fields() {
            let key = schema.key_for(field.name());
            let raw_val =
                get_or::<String>(&key, field.default_value().cloned().unwrap_or_default()).await;
            let num_val = get_or::<f64>(&key, 0.0).await.to_string();
            let checked = get_or::<bool>(&key, false).await;
            prepared.push(FieldVals {
                raw_val,
                num_val,
                checked,
            });
        }

        Ok(html! {
            @if *self.saved() {
                div.admin-message."admin-message-ok" { (&saved_msg) }
            }
            @if *self.error() {
                div.admin-message."admin-message-error" { (&error_msg) }
            }
            form.admin-config-form method="post" action=(action) {
                @for (i, field) in schema.fields().iter().enumerate() {
                    @let pf = &prepared[i];
                    div.admin-form-field {
                        label.admin-form-label for=(field.name()) { (field.label()) }
                        @match field.field_type() {
                            SettingFieldType::Text { max_length } => {
                                @if let Some(max) = max_length {
                                    input.admin-form-input
                                        type="text"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        maxlength=(max)
                                        required[*field.required()];
                                } @else {
                                    input.admin-form-input
                                        type="text"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.raw_val)
                                        required[*field.required()];
                                }
                            }
                            SettingFieldType::Number { min, max } => {
                                @if let (Some(lo), Some(hi)) = (min, max) {
                                    input.admin-form-input
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.num_val)
                                        min=(lo)
                                        max=(hi)
                                        required[*field.required()];
                                } @else if let Some(lo) = min {
                                    input.admin-form-input
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.num_val)
                                        min=(lo)
                                        required[*field.required()];
                                } @else if let Some(hi) = max {
                                    input.admin-form-input
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.num_val)
                                        max=(hi)
                                        required[*field.required()];
                                } @else {
                                    input.admin-form-input
                                        type="number"
                                        id=(field.name())
                                        name=(field.name())
                                        value=(&pf.num_val)
                                        required[*field.required()];
                                }
                            }
                            SettingFieldType::Boolean => {
                                input.admin-form-checkbox
                                    type="checkbox"
                                    id=(field.name())
                                    name=(field.name())
                                    value="true"
                                    checked[pf.checked];
                            }
                            SettingFieldType::Select { options } => {
                                select.admin-form-select
                                    id=(field.name())
                                    name=(field.name())
                                    required[*field.required()]
                                {
                                    @for (val, label) in options {
                                        @if &pf.raw_val == val {
                                            option value=(val) selected { (label) }
                                        } @else {
                                            option value=(val) { (label) }
                                        }
                                    }
                                }
                            }
                        }
                        @if let Some(help) = field.help_text() {
                            small.admin-form-help { (help) }
                        }
                    }
                }
                div.admin-form-actions {
                    button.admin-btn."admin-btn-primary" type="submit" {
                        (&save_label)
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

    pub fn with_action_path(mut self, v: impl Into<Option<Route>>) -> Self {
        if let Some(v) = v.into() {
            self.action_path = Some(v);
        }
        self
    }

    #[builder_skip]
    pub(crate) fn with_saved(mut self, saved: bool, error: bool) -> Self {
        self.saved = saved;
        self.error = error;
        self
    }
}
