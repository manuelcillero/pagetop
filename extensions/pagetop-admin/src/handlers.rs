use std::collections::HashMap;

use pagetop::prelude::*;

use crate::LOCALES_ADMIN;
use crate::action::config_forms;
use crate::component::ConfigForm;
use crate::settings::{self, SettingFieldType, SettingsSchema};

// **< Config form >********************************************************************************

// Página del registro y esquema de la página de ajustes de la petición.
fn config_form_page(
    request: &HttpRequest,
) -> Result<(&'static AdminPage, &'static SettingsSchema), ErrorPage> {
    let not_found = || ErrorPage::NotFound(Some(request.clone()));
    let path = strip_admin_path(request.path()).ok_or_else(not_found)?;
    let page = AdminRegistry::global().page(path).ok_or_else(not_found)?;
    let schema = config_forms().get(path).ok_or_else(not_found)?;
    Ok((page, schema))
}

// **< Config form - GET >**************************************************************************

// GET de una página de ajustes: el formulario con los valores guardados.
pub async fn config_form_get(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let (page, schema) = config_form_page(&request)?;

    Page::admin(request)
        .with_title(page.title().clone())
        .with_child(ConfigForm::with_schema(schema.clone()))
        .render()
        .await
}

// **< Config form - POST >*************************************************************************

// POST de una página de ajustes: guarda los valores enviados y vuelve a mostrar el formulario con
// un mensaje. Si falta algún campo obligatorio no guarda ninguno.
pub async fn config_form_post(
    request: HttpRequest,
    web::Form(data): web::Form<HashMap<String, String>>,
) -> Result<Markup, ErrorPage> {
    let (page, schema) = config_form_page(&request)?;
    let mut admin = Page::admin(request).with_title(page.title().clone());

    let (level, message) = match submitted_values(schema, &data) {
        None => (MessageLevel::Error, "config-required"),
        Some(values) => {
            let user_id = admin.current_user().id();
            let mut saved = true;
            for (key, value) in values {
                if let Err(error) = settings::set(&key, &value, schema.scope(), user_id).await {
                    trace::error!("Failed to save the setting \"{key}\": {error}");
                    saved = false;
                    break;
                }
            }
            if saved {
                (MessageLevel::Info, "config-saved-ok")
            } else {
                (MessageLevel::Error, "config-saved-error")
            }
        }
    };
    admin.push_message(level, Lc::t(message, &LOCALES_ADMIN));

    admin
        .with_child(ConfigForm::with_schema(schema.clone()))
        .render()
        .await
}

// Clave y valor de cada campo del esquema en el envío, o `None` si falta un campo obligatorio o
// llega vacío. Una casilla sin marcar no se envía, así que vale `"false"`.
fn submitted_values(
    schema: &SettingsSchema,
    data: &HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let mut values = Vec::with_capacity(schema.fields().len());
    for field in schema.fields() {
        let value = match (field.field_type(), data.get(field.name())) {
            (SettingFieldType::Boolean, value) => value.map_or("false", String::as_str),
            (_, Some(value)) if !value.trim().is_empty() => value,
            (_, _) if *field.required() => return None,
            (_, value) => value.map_or("", String::as_str),
        };
        values.push((schema.key_for(field.name()), value.to_owned()));
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::SettingField;

    fn schema() -> SettingsSchema {
        SettingsSchema::new("garden")
            .with_field(SettingField::text("name", Lc::n("Name")).with_required(true))
            .with_field(SettingField::text("notice", Lc::n("Notice")))
            .with_field(SettingField::boolean("open", Lc::n("Open")))
    }

    fn data(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[pagetop::test]
    async fn values_are_keyed_by_scope_and_an_unchecked_box_is_false() {
        let values = submitted_values(&schema(), &data(&[("name", "Ours"), ("notice", "")]));
        assert_eq!(
            values.unwrap(),
            [
                ("garden.name".to_owned(), "Ours".to_owned()),
                ("garden.notice".to_owned(), String::new()),
                ("garden.open".to_owned(), "false".to_owned()),
            ]
        );
    }

    #[pagetop::test]
    async fn a_required_field_missing_or_blank_saves_nothing() {
        assert!(submitted_values(&schema(), &data(&[("notice", "Hi")])).is_none());
        assert!(submitted_values(&schema(), &data(&[("name", "  ")])).is_none());
    }
}
