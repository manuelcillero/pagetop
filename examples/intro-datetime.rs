use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroDatetime;

#[async_trait]
impl Extension for IntroDatetime {
    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(intro_datetime))
    }
}

async fn intro_datetime(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(
            Intro::custom()
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("datetime_slogan", &LOC))
                .with_child(world_block())
                .with_child(formats_block())
                .with_child(relative_block())
                .with_child(since_until_block()),
        )
        .render()
        .await
}

// Zonas IANA de la demostración, con la clave de su etiqueta traducible.
const ZONES: [(&str, &str); 4] = [
    ("datetime_zone_utc", "UTC"),
    ("datetime_zone_madrid", "Europe/Madrid"),
    ("datetime_zone_mexico", "America/Mexico_City"),
    ("datetime_zone_tokyo", "Asia/Tokyo"),
];

// El mismo instante (`Utc::now()`) mostrado en cada zona de `ZONES`: fecha y hora combinadas
// (`DateFormat::Medium` + `TimeFormat::Short`) y su equivalente ISO 8601, con el offset propio de
// cada zona.
fn world_block() -> Block {
    Block::new()
        .with_title(Lc::t("datetime_block_world", &LOC))
        .with_child(Html::with(|cx| {
            let now = Utc::now();
            let rows: Vec<(Lc, String, String)> = ZONES
                .into_iter()
                .map(|(label_key, zone)| {
                    let tz: Tz = zone.parse().expect("valid IANA timezone");
                    let zone_cx = Context::default().with_langid(cx).with_timezone(tz);
                    (
                        Lc::t(label_key, &LOC),
                        zone_cx.format_datetime(now, DateFormat::Medium, TimeFormat::Short),
                        zone_cx.format_iso_datetime(now),
                    )
                })
                .collect();

            html! {
                ul {
                    @for (label, combined, iso) in &rows {
                        li {
                            strong { (label.using(cx)) ": " } (combined)
                            " (ISO " code { (iso) } ")"
                        }
                    }
                }
            }
        }))
}

// Combinaciones de `DateFormat`/`TimeFormat` sobre la zona horaria efectiva del visitante. Para
// añadir un nuevo formato basta con ampliar el array correspondiente.
fn formats_block() -> Block {
    Block::new()
        .with_title(Lc::t("datetime_block_formats", &LOC))
        .with_child(Html::with(|cx| {
            let now = Utc::now();
            let today = now.with_timezone(&cx.timezone()).date_naive();

            let dates: Vec<(&str, String)> = [
                ("Short", DateFormat::Short),
                ("Medium", DateFormat::Medium),
                ("Long", DateFormat::Long),
                ("Iso", DateFormat::Iso),
                ("Custom(\"%d.%m.%Y\")", DateFormat::Custom("%d.%m.%Y")),
            ]
            .into_iter()
            .map(|(label, format)| (label, cx.format_date(today, format)))
            .collect();

            let times: Vec<(&str, String)> = [
                ("Short", TimeFormat::Short),
                ("Long", TimeFormat::Long),
                ("Custom(\"%I:%M %p\")", TimeFormat::Custom("%I:%M %p")),
            ]
            .into_iter()
            .map(|(label, format)| (label, cx.format_time(now, format)))
            .collect();

            let combos: Vec<(&str, String)> = [
                ("Short + Short", DateFormat::Short, TimeFormat::Short),
                ("Medium + Short", DateFormat::Medium, TimeFormat::Short),
                ("Long + Long", DateFormat::Long, TimeFormat::Long),
                ("Long + Short", DateFormat::Long, TimeFormat::Short),
                ("Iso + Long", DateFormat::Iso, TimeFormat::Long),
            ]
            .into_iter()
            .map(|(label, date, time)| (label, cx.format_datetime(now, date, time)))
            .collect();

            html! {
                h3 { (Lc::t("datetime_formats_date", &LOC).using(cx)) }
                ul {
                    @for (label, value) in &dates {
                        li { code { (label) } ": " (value) }
                    }
                }
                h3 { (Lc::t("datetime_formats_time", &LOC).using(cx)) }
                ul {
                    @for (label, value) in &times {
                        li { code { (label) } ": " (value) }
                    }
                }
                h3 { (Lc::t("datetime_formats_combos", &LOC).using(cx)) }
                ul {
                    @for (label, value) in &combos {
                        li { code { (label) } ": " (value) }
                    }
                }
                p {
                    (Lc::t("datetime_formats_iso", &LOC).using(cx))
                    " " code { (cx.format_iso_datetime(now)) }
                }
            }
        }))
}

// Casos ilustrativos de `RelativeFormat` sobre la zona horaria efectiva del visitante: hoy, un
// desplazamiento simple de días, uno con el componente de meses en cero, uno con los tres
// componentes y uno en el futuro -- siempre respecto al día actual. Añadir un caso nuevo es sólo
// ampliar `samples`. La primera columna muestra la fecha seleccionada (no una etiqueta fija),
// formateada con `DateFormat::Medium` en el idioma efectivo, para que se vea a qué fecha concreta
// corresponde cada resultado relativo.
fn relative_block() -> Block {
    Block::new()
        .with_title(Lc::t("datetime_block_relative", &LOC))
        .with_child(Html::with(|cx| {
            let today = Utc::now().date_naive();

            // Desplazamientos de meses/años construidos con `checked_sub_months()` (mismo mecanismo
            // que usa `RelativeFormat` por dentro), para que el resultado coincida exactamente con
            // el criterio de cada caso.
            let two_years_and_3_days_ago = today
                .checked_sub_months(Months::new(24))
                .expect("valid date")
                - Duration::days(3);
            let three_years_2_months_10_days_ago = today
                .checked_sub_months(Months::new(38))
                .expect("valid date")
                - Duration::days(10);

            let samples = [
                today,
                today - Duration::days(3),
                two_years_and_3_days_ago,
                three_years_2_months_10_days_ago,
                today + Duration::days(5),
            ];

            let rows: Vec<(String, String, String, String)> = samples
                .into_iter()
                .map(|date| {
                    let dt = at_noon(date);
                    (
                        cx.format_date(date, DateFormat::Medium),
                        cx.format_relative(dt, RelativeFormat::Short),
                        cx.format_relative(dt, RelativeFormat::Medium),
                        cx.format_relative(dt, RelativeFormat::Long),
                    )
                })
                .collect();

            html! {
                table style="width: 100%; border: 1px solid black;" {
                    thead {
                        tr {
                            th { (Lc::t("datetime_relative_col_date", &LOC).using(cx)) }
                            th { "Short" }
                            th { "Medium" }
                            th { "Long" }
                        }
                    }
                    tbody {
                        @for (date, short, medium, long) in &rows {
                            tr {
                                td { (date) }
                                td { (short) }
                                td { (medium) }
                                td { (long) }
                            }
                        }
                    }
                }
            }
        }))
}

// Niveles de `DatePrecision` para `format_since()`/`format_until()`. Añadir un nivel nuevo es sólo
// ampliar este array.
const PRECISIONS: [(&str, DatePrecision); 3] = [
    ("Short", DatePrecision::Short),
    ("Medium", DatePrecision::Medium),
    ("Long", DatePrecision::Long),
];

// `format_since()`/`format_until()` sobre una única fecha de ejemplo: a diferencia de `DateFormat`,
// lo que cambia entre niveles no es el estilo, sino la propia precisión revelada (sólo el mes, mes
// y año, o fecha completa).
fn since_until_block() -> Block {
    Block::new()
        .with_title(Lc::t("datetime_block_since_until", &LOC))
        .with_child(Html::with(|cx| {
            let date = NaiveDate::from_ymd_opt(2026, 6, 3).unwrap();

            let rows: Vec<(&str, String, String)> = PRECISIONS
                .into_iter()
                .map(|(label, precision)| {
                    (
                        label,
                        cx.format_since(date, precision),
                        cx.format_until(date, precision),
                    )
                })
                .collect();

            html! {
                table style="width: 100%; border: 1px solid black;" {
                    thead {
                        tr {
                            th { (Lc::t("datetime_since_until_col_precision", &LOC).using(cx)) }
                            th { (Lc::t("datetime_since_until_col_since", &LOC).using(cx)) }
                            th { (Lc::t("datetime_since_until_col_until", &LOC).using(cx)) }
                        }
                    }
                    tbody {
                        @for (label, since, until) in &rows {
                            tr {
                                td { (*label) }
                                td { (since) }
                                td { (until) }
                            }
                        }
                    }
                }
            }
        }))
}

// Mediodía, para evitar que el redondeo horario de la conversión de zona horaria empuje la fecha
// civil resultante al día anterior o siguiente en zonas con un offset amplio.
fn at_noon(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(12, 0, 0)
        .expect("noon is always a valid time")
        .and_utc()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroDatetime).await.run().await
}
