use pagetop::prelude::*;

use pagetop_bootsier::theme::*;

include_locales!(LOC from "examples/locale");

struct FormControls;

impl Extension for FormControls {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_aliner::Aliner, &pagetop_bootsier::Bootsier]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(form_controls));
    }
}

async fn form_controls(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_child(
            Intro::default()
                .with_opening(IntroOpening::Custom)
                .with_title(L10n::t("title", &LOC))
                .with_slogan(L10n::t("slogan", &LOC))
                .with_button(None::<(L10n, FnPathByContext)>)
                // Bloque 1: casillas, interruptores y botones de opción.
                .with_child(
                    Block::new()
                        .with_title(L10n::t("block_selections", &LOC))
                        .with_child(
                            Form::new()
                                .with_id("form-selections")
                                .with_action("/")
                                .with_method(form::Method::Post)
                                // Casillas e interruptores (form::Checkbox).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_checkbox", &LOC))
                                        .with_description(L10n::t("desc_checkbox", &LOC))
                                        .with_child(
                                            form::Checkbox::new()
                                                .with_name("accept_terms")
                                                .with_label(L10n::t("label_terms", &LOC))
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::Checkbox::new()
                                                .with_name("accept_marketing")
                                                .with_label(L10n::t("label_marketing", &LOC))
                                                .with_checked(true)
                                                .with_inline(true),
                                        )
                                        .with_child(
                                            form::Checkbox::new()
                                                .with_name("newsletter")
                                                .with_label(L10n::t("label_newsletter", &LOC))
                                                .with_inline(true),
                                        )
                                        .with_child(
                                            form::Checkbox::switch()
                                                .with_name("notifications")
                                                .with_label(L10n::t("label_notifications", &LOC))
                                                .with_checked(true)
                                                .with_reverse(true),
                                        )
                                        .with_child(
                                            form::Checkbox::switch()
                                                .with_name("dark_mode")
                                                .with_label(L10n::t("label_dark_mode", &LOC))
                                                .with_disabled(true),
                                        ),
                                )
                                // Grupo de casillas de verificación (form::check::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_checkgroup", &LOC))
                                        .with_child(
                                            form::check::Field::new()
                                                .with_name("interests")
                                                .with_label(L10n::t("label_interests", &LOC))
                                                .with_help_text(L10n::t("help_interests", &LOC))
                                                .with_item(
                                                    form::check::Item::new(
                                                        "rust",
                                                        L10n::t("check_rust", &LOC),
                                                    )
                                                    .with_checked(true),
                                                )
                                                .with_item(form::check::Item::new(
                                                    "web",
                                                    L10n::t("check_web", &LOC),
                                                ))
                                                .with_item(form::check::Item::new(
                                                    "ai",
                                                    L10n::t("check_ai", &LOC),
                                                ))
                                                .with_item(
                                                    form::check::Item::new(
                                                        "games",
                                                        L10n::t("check_games", &LOC),
                                                    )
                                                    .with_disabled(true),
                                                ),
                                        ),
                                )
                                // Botones de opción (form::radio::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_radio", &LOC))
                                        .with_child(
                                            form::radio::Field::new()
                                                .with_name("frequency")
                                                .with_label(L10n::t("label_frequency", &LOC))
                                                .with_item(form::radio::Item::new(
                                                    "daily",
                                                    L10n::t("radio_daily", &LOC),
                                                ))
                                                .with_item(
                                                    form::radio::Item::new(
                                                        "weekly",
                                                        L10n::t("radio_weekly", &LOC),
                                                    )
                                                    .with_checked(true),
                                                )
                                                .with_item(form::radio::Item::new(
                                                    "monthly",
                                                    L10n::t("radio_monthly", &LOC),
                                                ))
                                                .with_item(
                                                    form::radio::Item::new(
                                                        "never",
                                                        L10n::t("radio_never", &LOC),
                                                    )
                                                    .with_disabled(true),
                                                ),
                                        ),
                                )
                                // Campo oculto (form::Hidden).
                                .with_child(
                                    form::Hidden::new()
                                        .with_name("origin")
                                        .with_value("form-selections"),
                                )
                                // Botones de acción.
                                .with_child(
                                    Button::submit(L10n::t("btn_submit", &LOC))
                                        .with_color(ButtonColor::Background(Color::Primary)),
                                )
                                .with_child(
                                    Button::reset(L10n::t("btn_reset", &LOC))
                                        .with_color(ButtonColor::Outline(Color::Secondary)),
                                )
                                .with_child(
                                    Button::plain(L10n::t("btn_cancel", &LOC))
                                        .with_color(ButtonColor::Link),
                                ),
                        ),
                )
                // Bloque 2: campos de texto, multilínea y rango.
                .with_child(
                    Block::new()
                        .with_title(L10n::t("block_text", &LOC))
                        .with_child(
                            Form::new()
                                .with_id("form-text")
                                .with_action("/")
                                .with_method(form::Method::Post)
                                // Campos de texto (form::input::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_text", &LOC))
                                        .with_child(
                                            form::input::Field::text()
                                                .with_name("name")
                                                .with_label(L10n::t("label_name", &LOC))
                                                .with_placeholder(L10n::t("placeholder_name", &LOC))
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::input::Field::email()
                                                .with_name("email")
                                                .with_label(L10n::t("label_email", &LOC))
                                                .with_placeholder(L10n::t(
                                                    "placeholder_email",
                                                    &LOC,
                                                ))
                                                .with_autocomplete(
                                                    Some(form::Autocomplete::email()),
                                                )
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::input::Field::password()
                                                .with_name("password")
                                                .with_label(L10n::t("label_password", &LOC))
                                                .with_autocomplete(Some(
                                                    form::Autocomplete::new_password(),
                                                ))
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::input::Field::telephone()
                                                .with_name("phone")
                                                .with_label(L10n::t("label_phone", &LOC))
                                                .with_placeholder(L10n::t(
                                                    "placeholder_phone",
                                                    &LOC,
                                                )),
                                        )
                                        .with_child(
                                            form::input::Field::url()
                                                .with_name("website")
                                                .with_label(L10n::t("label_url", &LOC))
                                                .with_placeholder(L10n::t("placeholder_url", &LOC)),
                                        )
                                        .with_child(
                                            form::input::Field::search()
                                                .with_name("search")
                                                .with_label(L10n::t("label_search", &LOC))
                                                .with_placeholder(L10n::t(
                                                    "placeholder_search",
                                                    &LOC,
                                                )),
                                        ),
                                )
                                // Área de texto (form::Textarea).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_textarea", &LOC))
                                        .with_child(
                                            form::Textarea::new()
                                                .with_name("comment")
                                                .with_label(L10n::t("label_comment", &LOC))
                                                .with_placeholder(L10n::t(
                                                    "placeholder_comment",
                                                    &LOC,
                                                ))
                                                .with_rows(Some(4))
                                                .with_help_text(L10n::t("help_comment", &LOC)),
                                        ),
                                )
                                // Control deslizante (form::Range).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_range", &LOC))
                                        .with_child(
                                            form::Range::new()
                                                .with_name("rating")
                                                .with_label(L10n::t("label_rating", &LOC))
                                                .with_min(Some(1.0))
                                                .with_max(Some(10.0))
                                                .with_step(Some(1.0))
                                                .with_value(Some(5.0))
                                                .with_help_text(L10n::t("help_rating", &LOC)),
                                        ),
                                )
                                // Campo oculto (form::Hidden).
                                .with_child(
                                    form::Hidden::new()
                                        .with_name("origin")
                                        .with_value("form-text"),
                                )
                                // Botones de acción.
                                .with_child(
                                    Button::submit(L10n::t("btn_submit", &LOC))
                                        .with_color(ButtonColor::Background(Color::Primary)),
                                )
                                .with_child(
                                    Button::reset(L10n::t("btn_reset", &LOC))
                                        .with_color(ButtonColor::Outline(Color::Secondary)),
                                )
                                .with_child(
                                    Button::plain(L10n::t("btn_cancel", &LOC))
                                        .with_color(ButtonColor::Link),
                                ),
                        ),
                )
                // Bloque 3: listas de selección y etiquetas flotantes.
                .with_child(
                    Block::new()
                        .with_title(L10n::t("block_lists", &LOC))
                        .with_child(
                            Form::new()
                                .with_id("form-lists")
                                .with_action("/")
                                .with_method(form::Method::Post)
                                // Listas de selección (form::select::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_select", &LOC))
                                        .with_child(
                                            form::select::Field::new()
                                                .with_name("language")
                                                .with_label(L10n::t("label_language", &LOC))
                                                .with_item(
                                                    form::select::Item::new(
                                                        "",
                                                        L10n::t("select_choose", &LOC),
                                                    )
                                                    .with_selected(true),
                                                )
                                                .with_group(
                                                    form::select::Group::new(L10n::t(
                                                        "select_group_europe",
                                                        &LOC,
                                                    ))
                                                    .with_item(form::select::Item::new(
                                                        "es",
                                                        L10n::t("select_spanish", &LOC),
                                                    ))
                                                    .with_item(form::select::Item::new(
                                                        "fr",
                                                        L10n::t("select_french", &LOC),
                                                    )),
                                                )
                                                .with_group(
                                                    form::select::Group::new(L10n::t(
                                                        "select_group_americas",
                                                        &LOC,
                                                    ))
                                                    .with_item(form::select::Item::new(
                                                        "en",
                                                        L10n::t("select_english", &LOC),
                                                    ))
                                                    .with_item(form::select::Item::new(
                                                        "pt",
                                                        L10n::t("select_portuguese", &LOC),
                                                    )),
                                                )
                                                .with_item(
                                                    form::select::Item::new(
                                                        "xx",
                                                        L10n::t("select_disabled", &LOC),
                                                    )
                                                    .with_disabled(true),
                                                )
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::select::Field::new()
                                                .with_name("technologies")
                                                .with_label(L10n::t("label_technologies", &LOC))
                                                .with_item(
                                                    form::select::Item::new(
                                                        "rust",
                                                        L10n::n("Rust"),
                                                    )
                                                    .with_selected(true),
                                                )
                                                .with_item(
                                                    form::select::Item::new(
                                                        "python",
                                                        L10n::n("Python"),
                                                    )
                                                    .with_selected(true),
                                                )
                                                .with_item(form::select::Item::new(
                                                    "javascript",
                                                    L10n::n("JavaScript"),
                                                ))
                                                .with_item(form::select::Item::new(
                                                    "go",
                                                    L10n::n("Go"),
                                                ))
                                                .with_item(form::select::Item::new(
                                                    "typescript",
                                                    L10n::n("TypeScript"),
                                                ))
                                                .with_multiple(true)
                                                .with_rows(Some(4))
                                                .with_help_text(L10n::t("help_technologies", &LOC)),
                                        ),
                                )
                                // Etiquetas flotantes.
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(L10n::t("fieldset_floating", &LOC))
                                        .with_child(
                                            form::input::Field::text()
                                                .with_name("fl_name")
                                                .with_label(L10n::t("label_name", &LOC))
                                                .with_placeholder(L10n::t("placeholder_name", &LOC))
                                                .with_floating_label(true)
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::Textarea::new()
                                                .with_name("fl_comment")
                                                .with_label(L10n::t("label_comment", &LOC))
                                                .with_placeholder(L10n::t(
                                                    "placeholder_comment",
                                                    &LOC,
                                                ))
                                                .with_floating_label(true),
                                        )
                                        .with_child(
                                            form::select::Field::new()
                                                .with_name("fl_country")
                                                .with_label(L10n::t("label_country", &LOC))
                                                .with_item(
                                                    form::select::Item::new(
                                                        "",
                                                        L10n::t("select_choose", &LOC),
                                                    )
                                                    .with_selected(true),
                                                )
                                                .with_item(form::select::Item::new(
                                                    "de",
                                                    L10n::t("select_germany", &LOC),
                                                ))
                                                .with_item(form::select::Item::new(
                                                    "es",
                                                    L10n::t("select_spain", &LOC),
                                                ))
                                                .with_item(form::select::Item::new(
                                                    "fr",
                                                    L10n::t("select_france", &LOC),
                                                ))
                                                .with_item(form::select::Item::new(
                                                    "pt",
                                                    L10n::t("select_portugal", &LOC),
                                                ))
                                                .with_floating_label(true)
                                                .with_required(true),
                                        ),
                                )
                                // Campo oculto (form::Hidden).
                                .with_child(
                                    form::Hidden::new()
                                        .with_name("origin")
                                        .with_value("form-lists"),
                                )
                                // Botones de acción.
                                .with_child(
                                    Button::submit(L10n::t("btn_submit", &LOC))
                                        .with_color(ButtonColor::Background(Color::Primary)),
                                )
                                .with_child(
                                    Button::reset(L10n::t("btn_reset", &LOC))
                                        .with_color(ButtonColor::Outline(Color::Secondary)),
                                )
                                .with_child(
                                    Button::plain(L10n::t("btn_cancel", &LOC))
                                        .with_color(ButtonColor::Link),
                                ),
                        ),
                ),
        )
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&FormControls).run()?.await
}
