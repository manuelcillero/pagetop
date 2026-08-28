use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

include_locales!(LOC from "examples/locale");

struct FormControls;

#[async_trait]
impl Extension for FormControls {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_aliner::Aliner, &pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(form_controls))
    }
}

async fn form_controls(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(
            Intro::default()
                .with_opening(IntroOpening::Custom)
                .with_title(Lc::t("title", &LOC))
                .with_slogan(Lc::t("slogan", &LOC))
                .with_button(None::<(Lc, Route)>)
                // Bloque 1: casillas, interruptores y botones de opción.
                .with_child(
                    Block::new()
                        .with_title(Lc::t("block_selections", &LOC))
                        .with_child(
                            Form::new()
                                .with_id("form-selections")
                                .with_action("/")
                                .with_method(form::Method::Post)
                                // Casillas e interruptores (form::Checkbox).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(Lc::t("fieldset_checkbox", &LOC))
                                        .with_description(Lc::t("desc_checkbox", &LOC))
                                        .with_child(
                                            form::Checkbox::new()
                                                .with_name("accept_terms")
                                                .with_label(Lc::t("label_terms", &LOC))
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::Checkbox::new()
                                                .with_name("accept_marketing")
                                                .with_label(Lc::t("label_marketing", &LOC))
                                                .with_checked(true)
                                                .with_inline(true),
                                        )
                                        .with_child(
                                            form::Checkbox::new()
                                                .with_name("newsletter")
                                                .with_label(Lc::t("label_newsletter", &LOC))
                                                .with_inline(true),
                                        )
                                        .with_child(
                                            form::Checkbox::switch()
                                                .with_name("notifications")
                                                .with_label(Lc::t("label_notifications", &LOC))
                                                .with_checked(true)
                                                .with_reverse(true),
                                        )
                                        .with_child(
                                            form::Checkbox::switch()
                                                .with_name("dark_mode")
                                                .with_label(Lc::t("label_dark_mode", &LOC))
                                                .with_disabled(true),
                                        ),
                                )
                                // Grupo de casillas de verificación (form::check::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(Lc::t("fieldset_checkgroup", &LOC))
                                        .with_child(
                                            form::check::Field::new()
                                                .with_name("interests")
                                                .with_label(Lc::t("label_interests", &LOC))
                                                .with_help_text(Lc::t("help_interests", &LOC))
                                                .with_item(
                                                    form::check::Item::new(
                                                        "rust",
                                                        Lc::t("check_rust", &LOC),
                                                    )
                                                    .with_checked(true),
                                                )
                                                .with_item(form::check::Item::new(
                                                    "web",
                                                    Lc::t("check_web", &LOC),
                                                ))
                                                .with_item(form::check::Item::new(
                                                    "ai",
                                                    Lc::t("check_ai", &LOC),
                                                ))
                                                .with_item(
                                                    form::check::Item::new(
                                                        "games",
                                                        Lc::t("check_games", &LOC),
                                                    )
                                                    .with_disabled(true),
                                                ),
                                        ),
                                )
                                // Botones de opción (form::radio::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(Lc::t("fieldset_radio", &LOC))
                                        .with_child(
                                            form::radio::Field::new()
                                                .with_name("frequency")
                                                .with_label(Lc::t("label_frequency", &LOC))
                                                .with_item(form::radio::Item::new(
                                                    "daily",
                                                    Lc::t("radio_daily", &LOC),
                                                ))
                                                .with_item(
                                                    form::radio::Item::new(
                                                        "weekly",
                                                        Lc::t("radio_weekly", &LOC),
                                                    )
                                                    .with_checked(true),
                                                )
                                                .with_item(form::radio::Item::new(
                                                    "monthly",
                                                    Lc::t("radio_monthly", &LOC),
                                                ))
                                                .with_item(
                                                    form::radio::Item::new(
                                                        "never",
                                                        Lc::t("radio_never", &LOC),
                                                    )
                                                    .with_disabled(true),
                                                ),
                                        ),
                                )
                                // Campo oculto (form::Hidden).
                                .with_child(form::Hidden::field("origin", "form-selections"))
                                // Botonera de acciones.
                                .with_child(
                                    button::ButtonSet::new()
                                        .with_button(
                                            Button::submit(Lc::t("btn_submit", &LOC))
                                                .with_style(button::Style::Solid(Intent::Primary)),
                                        )
                                        .with_button(
                                            Button::reset(Lc::t("btn_reset", &LOC)).with_style(
                                                button::Style::Outline(Intent::Neutral),
                                            ),
                                        )
                                        .with_button(
                                            Button::plain(Lc::t("btn_cancel", &LOC))
                                                .with_style(button::Style::Link),
                                        ),
                                ),
                        ),
                )
                // Bloque 2: campos de texto, multilínea y rango.
                .with_child(
                    Block::new()
                        .with_title(Lc::t("block_text", &LOC))
                        .with_child(
                            Form::new()
                                .with_id("form-text")
                                .with_action("/")
                                .with_method(form::Method::Post)
                                // Campos de texto (form::input::Field).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(Lc::t("fieldset_text", &LOC))
                                        .with_child(
                                            form::input::Field::text()
                                                .with_name("name")
                                                .with_label(Lc::t("label_name", &LOC))
                                                .with_placeholder(Lc::t("placeholder_name", &LOC))
                                                .with_help_text(Lc::t("help_name", &LOC))
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::input::Field::email()
                                                .with_name("email")
                                                .with_label(Lc::t("label_email", &LOC))
                                                .with_placeholder(Lc::t("placeholder_email", &LOC))
                                                .with_help_text(Lc::t("help_email", &LOC))
                                                .with_autocomplete(
                                                    Some(form::Autocomplete::email()),
                                                )
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::input::Field::password()
                                                .with_name("password")
                                                .with_label(Lc::t("label_password", &LOC))
                                                .with_autocomplete(Some(
                                                    form::Autocomplete::new_password(),
                                                ))
                                                .with_required(true),
                                        )
                                        .with_child(
                                            form::input::Field::telephone()
                                                .with_name("phone")
                                                .with_label(Lc::t("label_phone", &LOC))
                                                .with_placeholder(Lc::t("placeholder_phone", &LOC)),
                                        )
                                        .with_child(
                                            form::input::Field::url()
                                                .with_name("website")
                                                .with_label(Lc::t("label_url", &LOC))
                                                .with_placeholder(Lc::t("placeholder_url", &LOC)),
                                        )
                                        .with_child(
                                            form::input::Field::search()
                                                .with_name("search")
                                                .with_label(Lc::t("label_search", &LOC))
                                                .with_placeholder(Lc::t(
                                                    "placeholder_search",
                                                    &LOC,
                                                )),
                                        ),
                                )
                                // Área de texto (form::Textarea).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(Lc::t("fieldset_textarea", &LOC))
                                        .with_child(
                                            form::Textarea::new()
                                                .with_name("comment")
                                                .with_label(Lc::t("label_comment", &LOC))
                                                .with_placeholder(Lc::t(
                                                    "placeholder_comment",
                                                    &LOC,
                                                ))
                                                .with_rows(Some(4))
                                                .with_help_text(Lc::t("help_comment", &LOC)),
                                        ),
                                )
                                // Control deslizante (form::Range).
                                .with_child(
                                    form::Fieldset::new()
                                        .with_legend(Lc::t("fieldset_range", &LOC))
                                        .with_child(
                                            form::Range::new()
                                                .with_name("rating")
                                                .with_label(Lc::t("label_rating", &LOC))
                                                .with_min(Some(1.0))
                                                .with_max(Some(10.0))
                                                .with_step(Some(1.0))
                                                .with_value(Some(5.0))
                                                .with_help_text(Lc::t("help_rating", &LOC)),
                                        ),
                                )
                                // Campo oculto (form::Hidden).
                                .with_child(form::Hidden::field("origin", "form-text"))
                                // Botonera de acciones.
                                .with_child(
                                    button::ButtonSet::new()
                                        .with_button(
                                            Button::submit(Lc::t("btn_submit", &LOC))
                                                .with_style(button::Style::Solid(Intent::Primary)),
                                        )
                                        .with_button(
                                            Button::reset(Lc::t("btn_reset", &LOC)).with_style(
                                                button::Style::Outline(Intent::Neutral),
                                            ),
                                        )
                                        .with_button(
                                            Button::plain(Lc::t("btn_cancel", &LOC))
                                                .with_style(button::Style::Link),
                                        ),
                                ),
                        ),
                )
                // Bloque 3: listas de selección y etiquetas flotantes.
                .with_child(
                    Block::new()
                        .with_title(
                            if global::SETTINGS.app.theme.eq_ignore_ascii_case("bootsier") {
                                Lc::t("block_lists_floating", &LOC)
                            } else {
                                Lc::t("block_lists", &LOC)
                            },
                        )
                        .with_child(form_lists()),
                )
                // Bloque 4: diálogo modal.
                .with_child(
                    Block::new()
                        .with_title(Lc::t("block_dialog", &LOC))
                        .with_child(
                            Dialog::new()
                                .with_id("delete-confirm")
                                .with_title(Lc::t("dialog_delete_title", &LOC))
                                .with_child(Html::with(|cx| {
                                    html! {
                                        p { (Lc::t("dialog_delete_body", &LOC).using(cx)) }
                                    }
                                }))
                                .with_footer(
                                    Button::plain(Lc::t("btn_ok", &LOC))
                                        .with_prop(PropsOp::set("data-dialog-dismiss", "modal"))
                                        .with_style(button::Style::Solid(Intent::Primary)),
                                )
                                .with_footer(
                                    Button::plain(Lc::t("btn_cancel", &LOC))
                                        .with_prop(PropsOp::set("data-dialog-dismiss", "modal"))
                                        .with_style(button::Style::Outline(Intent::Neutral)),
                                ),
                        )
                        .with_child(
                            Button::plain(Lc::t("btn_delete", &LOC))
                                .with_prop(PropsOp::set("data-dialog-toggle", "modal"))
                                .with_prop(PropsOp::set("data-dialog-target", "#delete-confirm"))
                                .with_style(button::Style::Solid(Intent::Severe)),
                        ),
                ),
        )
        .render()
        .await
}

fn form_lists() -> Form {
    let mut form = Form::new()
        .with_id("form-lists")
        .with_action("/")
        .with_method(form::Method::Post)
        // Listas de selección (form::select::Field).
        .with_child(
            form::Fieldset::new()
                .with_legend(Lc::t("fieldset_select", &LOC))
                .with_child(
                    form::select::Field::new()
                        .with_name("language")
                        .with_label(Lc::t("label_language", &LOC))
                        .with_item(
                            form::select::Item::new("", Lc::t("select_choose", &LOC))
                                .with_selected(true),
                        )
                        .with_group(
                            form::select::Group::new(Lc::t("select_group_europe", &LOC))
                                .with_item(form::select::Item::new(
                                    "es",
                                    Lc::t("select_spanish", &LOC),
                                ))
                                .with_item(form::select::Item::new(
                                    "fr",
                                    Lc::t("select_french", &LOC),
                                )),
                        )
                        .with_group(
                            form::select::Group::new(Lc::t("select_group_americas", &LOC))
                                .with_item(form::select::Item::new(
                                    "en",
                                    Lc::t("select_english", &LOC),
                                ))
                                .with_item(form::select::Item::new(
                                    "pt",
                                    Lc::t("select_portuguese", &LOC),
                                )),
                        )
                        .with_item(
                            form::select::Item::new("xx", Lc::t("select_disabled", &LOC))
                                .with_disabled(true),
                        )
                        .with_required(true),
                )
                .with_child(
                    form::select::Field::new()
                        .with_name("technologies")
                        .with_label(Lc::t("label_technologies", &LOC))
                        .with_item(
                            form::select::Item::new("rust", Lc::n("Rust")).with_selected(true),
                        )
                        .with_item(
                            form::select::Item::new("python", Lc::n("Python")).with_selected(true),
                        )
                        .with_item(form::select::Item::new("javascript", Lc::n("JavaScript")))
                        .with_item(form::select::Item::new("go", Lc::n("Go")))
                        .with_item(form::select::Item::new("typescript", Lc::n("TypeScript")))
                        .with_multiple(true)
                        .with_rows(Some(4))
                        .with_help_text(Lc::t("help_technologies", &LOC)),
                ),
        );

    // Etiquetas flotantes: solo disponibles con el tema Bootsier.
    if global::SETTINGS.app.theme.eq_ignore_ascii_case("bootsier") {
        form = form.with_child(
            form::Fieldset::new()
                .with_legend(Lc::t("fieldset_floating", &LOC))
                .with_child(
                    form::input::Field::text()
                        .with_name("fl_name")
                        .with_label(Lc::t("label_name", &LOC))
                        .with_placeholder(Lc::t("placeholder_name", &LOC))
                        .with_floating_label(true)
                        .with_required(true),
                )
                .with_child(
                    form::Textarea::new()
                        .with_name("fl_comment")
                        .with_label(Lc::t("label_comment", &LOC))
                        .with_placeholder(Lc::t("placeholder_comment", &LOC))
                        .with_floating_label(true),
                )
                .with_child(
                    form::select::Field::new()
                        .with_name("fl_country")
                        .with_label(Lc::t("label_country", &LOC))
                        .with_item(
                            form::select::Item::new("", Lc::t("select_choose", &LOC))
                                .with_selected(true),
                        )
                        .with_item(form::select::Item::new("de", Lc::t("select_germany", &LOC)))
                        .with_item(form::select::Item::new("es", Lc::t("select_spain", &LOC)))
                        .with_item(form::select::Item::new("fr", Lc::t("select_france", &LOC)))
                        .with_item(form::select::Item::new(
                            "pt",
                            Lc::t("select_portugal", &LOC),
                        ))
                        .with_floating_label(true)
                        .with_required(true),
                ),
        );
    }

    form
        // Campo oculto (form::Hidden).
        .with_child(form::Hidden::field("origin", "form-lists"))
        // Botonera de acciones.
        .with_child(
            button::ButtonSet::new()
                .with_button(
                    Button::submit(Lc::t("btn_submit", &LOC))
                        .with_style(button::Style::Solid(Intent::Primary)),
                )
                .with_button(
                    Button::reset(Lc::t("btn_reset", &LOC))
                        .with_style(button::Style::Outline(Intent::Neutral)),
                )
                .with_button(
                    Button::plain(Lc::t("btn_cancel", &LOC)).with_style(button::Style::Link),
                ),
        )
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&FormControls).await.run().await
}
