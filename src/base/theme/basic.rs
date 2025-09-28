/// Es el tema básico que incluye PageTop por defecto.
use crate::prelude::*;

/// Tema básico por defecto.
///
/// Ofrece las siguientes composiciones (*layouts*):
///
/// - **Composición predeterminada**
///   - Renderizado genérico con
///     [`ThemePage::render_body()`](crate::core::theme::ThemePage::render_body) usando las regiones
///     predefinidas en [`page_regions()`](crate::core::theme::Theme::page_regions).
///
/// - **`Intro`**
///   - Página de entrada con cabecera visual, título y descripción y un botón opcional de llamada a
///     la acción. Ideal para una página de inicio o bienvenida en el contexto de PageTop.
///   - **Regiones:** `content` (se renderiza dentro de `.intro-content__body`).
///   - **Parámetros:**
///     - `intro_button_txt` (`L10n`) – Texto del botón.
///     - `intro_button_lnk` (`Option<String>`) – URL del botón; si no se indica, el botón no se
///       muestra.
///
/// - **`PageTopIntro`**
///   - Variante de `Intro` con textos predefinidos sobre PageTop al inicio del contenido. Añade una
///     banda de *badges* con la versión de [PageTop en crates.io](https://crates.io/crates/pagetop)
///     más la fecha de la última versión publicada y la licencia de uso.
///   - **Regiones:** `content` (igual que `Intro`).
///   - **Parámetros:** los mismos que `Intro`.
///
/// **Nota:** si no se especifica `layout` o el valor no coincide con ninguno de los anteriores, se
/// aplica la composición predeterminada.
pub struct Basic;

impl Extension for Basic {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

impl Theme for Basic {
    fn render_page_body(&self, page: &mut Page) -> Markup {
        match page.layout() {
            "Intro" => render_intro(page),
            "PageTopIntro" => render_pagetop_intro(page),
            _ => <Self as ThemePage>::render_body(self, page, self.page_regions()),
        }
    }

    fn after_render_page_body(&self, page: &mut Page) {
        let styles = match page.layout() {
            "Intro" => "/css/intro.css",
            "PageTopIntro" => "/css/intro.css",
            _ => "/css/basic.css",
        };
        let pkg_version = env!("CARGO_PKG_VERSION");
        page.alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/css/normalize.css")
                .with_version("8.0.1")
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/css/root.css")
                .with_version(pkg_version)
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/css/components.css")
                .with_version(pkg_version)
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/css/menu.css")
                .with_version(pkg_version)
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from(styles)
                .with_version(pkg_version)
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddJavaScript(
            JavaScript::defer("/js/menu.js")
                .with_version(pkg_version)
                .with_weight(-99),
        ));
    }
}

fn render_intro(page: &mut Page) -> Markup {
    let title = page.title().unwrap_or_default();
    let intro = page.description().unwrap_or_default();

    let intro_button_txt: L10n = page.param_or_default("intro_button_txt");
    let intro_button_lnk: Option<&String> = page.param("intro_button_lnk");

    html! {
        body id=[page.body_id().get()] class=[page.body_classes().get()] {
            header class="intro-header" {
                section class="intro-header__body" {
                    h1 class="intro-header__title" {
                        span { (title) }
                        (intro)
                    }
                }
                aside class="intro-header__image" aria-hidden="true" {
                    div class="intro-header__monster" {
                        picture {
                            source
                                type="image/avif"
                                src="/img/monster-pagetop_250.avif"
                                srcset="/img/monster-pagetop_500.avif 1.5x";
                            source
                                type="image/webp"
                                src="/img/monster-pagetop_250.webp"
                                srcset="/img/monster-pagetop_500.webp 1.5x";
                            img
                                src="/img/monster-pagetop_250.png"
                                srcset="/img/monster-pagetop_500.png 1.5x"
                                alt="Monster PageTop";
                        }
                    }
                }
            }
            main class="intro-content" {
                section class="intro-content__body" {
                    @if intro_button_lnk.is_some() {
                        div class="intro-button" {
                            a
                                class="intro-button__link"
                                href=[intro_button_lnk]
                                target="_blank"
                                rel="noreferrer"
                            {
                                span {} span {} span {}
                                div class="intro-button__text" {
                                    (intro_button_txt.using(page))
                                }
                            }
                        }
                    }
                    div class="intro-text" { (page.render_region("content")) }
                }
            }
            footer class="intro-footer" {
                section class="intro-footer__body" {
                    div class="intro-footer__logo" {
                        svg
                            viewBox="0 0 1614 1614"
                            xmlns="http://www.w3.org/2000/svg"
                            role="img"
                            aria-label=[L10n::l("pagetop_logo").lookup(page)]
                            preserveAspectRatio="xMidYMid slice"
                            focusable="false"
                        {
                            path fill="rgb(255,255,255)" d="M 1573,357 L 1415,357 C 1400,357 1388,369 1388,383 L 1388,410 1335,410 1335,357 C 1335,167 1181,13 992,13 L 621,13 C 432,13 278,167 278,357 L 278,410 225,410 225,383 C 225,369 213,357 198,357 L 40,357 C 25,357 13,369 13,383 L 13,648 C 13,662 25,674 40,674 L 198,674 C 213,674 225,662 225,648 L 225,621 278,621 278,1256 C 278,1446 432,1600 621,1600 L 992,1600 C 1181,1600 1335,1446 1335,1256 L 1335,621 1388,621 1388,648 C 1388,662 1400,674 1415,674 L 1573,674 C 1588,674 1600,662 1600,648 L 1600,383 C 1600,369 1588,357 1573,357 L 1573,357 1573,357 Z M 66,410 L 172,410 172,621 66,621 66,410 66,410 Z M 1282,357 L 1282,488 C 1247,485 1213,477 1181,464 L 1196,437 C 1203,425 1199,409 1186,401 1174,394 1158,398 1150,411 L 1133,440 C 1105,423 1079,401 1056,376 L 1075,361 C 1087,352 1089,335 1079,324 1070,313 1054,311 1042,320 L 1023,335 C 1000,301 981,263 967,221 L 1011,196 C 1023,189 1028,172 1021,160 1013,147 997,143 984,150 L 953,168 C 945,136 941,102 940,66 L 992,66 C 1152,66 1282,197 1282,357 L 1282,357 1282,357 Z M 621,66 L 674,66 674,225 648,225 C 633,225 621,237 621,251 621,266 633,278 648,278 L 674,278 674,357 648,357 C 633,357 621,369 621,383 621,398 633,410 648,410 L 674,410 674,489 648,489 C 633,489 621,501 621,516 621,530 633,542 648,542 L 664,542 C 651,582 626,623 600,662 583,653 563,648 542,648 469,648 410,707 410,780 410,787 411,794 412,801 388,805 361,806 331,806 L 331,357 C 331,197 461,66 621,66 L 621,66 621,66 Z M 621,780 C 621,824 586,859 542,859 498,859 463,824 463,780 463,736 498,701 542,701 586,701 621,736 621,780 L 621,780 621,780 Z M 225,463 L 278,463 278,569 225,569 225,463 225,463 Z M 992,1547 L 621,1547 C 461,1547 331,1416 331,1256 L 331,859 C 367,859 400,858 431,851 454,888 495,912 542,912 615,912 674,853 674,780 674,747 662,718 642,695 675,645 706,594 720,542 L 780,542 C 795,542 807,530 807,516 807,501 795,489 780,489 L 727,489 727,410 780,410 C 795,410 807,398 807,383 807,369 795,357 780,357 L 727,357 727,278 780,278 C 795,278 807,266 807,251 807,237 795,225 780,225 L 727,225 727,66 887,66 C 889,111 895,155 905,196 L 869,217 C 856,224 852,240 859,253 864,261 873,266 882,266 887,266 891,265 895,263 L 921,248 C 937,291 958,331 983,367 L 938,403 C 926,412 925,429 934,440 939,447 947,450 954,450 960,450 966,448 971,444 L 1016,408 C 1043,438 1074,465 1108,485 L 1084,527 C 1076,539 1081,555 1093,563 1098,565 1102,566 1107,566 1116,566 1125,561 1129,553 L 1155,509 C 1194,527 1237,538 1282,541 L 1282,1256 C 1282,1416 1152,1547 992,1547 L 992,1547 992,1547 Z M 1335,463 L 1388,463 1388,569 1335,569 1335,463 1335,463 Z M 1441,410 L 1547,410 1547,621 1441,621 1441,410 1441,410 Z" {}
                            path fill="rgb(255,255,255)" d="M 1150,1018 L 463,1018 C 448,1018 436,1030 436,1044 L 436,1177 C 436,1348 545,1468 701,1468 L 912,1468 C 1068,1468 1177,1348 1177,1177 L 1177,1044 C 1177,1030 1165,1018 1150,1018 L 1150,1018 1150,1018 Z M 912,1071 L 1018,1071 1018,1124 912,1124 912,1071 912,1071 Z M 489,1071 L 542,1071 542,1124 489,1124 489,1071 489,1071 Z M 701,1415 L 700,1415 C 701,1385 704,1352 718,1343 731,1335 759,1341 795,1359 802,1363 811,1363 818,1359 854,1341 882,1335 895,1343 909,1352 912,1385 913,1415 L 912,1415 701,1415 701,1415 701,1415 Z M 1124,1177 C 1124,1296 1061,1384 966,1408 964,1365 958,1320 922,1298 894,1281 856,1283 807,1306 757,1283 719,1281 691,1298 655,1320 649,1365 647,1408 552,1384 489,1296 489,1177 L 569,1177 C 583,1177 595,1165 595,1150 L 595,1071 859,1071 859,1150 C 859,1165 871,1177 886,1177 L 1044,1177 C 1059,1177 1071,1165 1071,1150 L 1071,1071 1124,1071 1124,1177 1124,1177 1124,1177 Z" {}
                            path fill="rgb(255,255,255)" d="M 1071,648 C 998,648 939,707 939,780 939,853 998,912 1071,912 1144,912 1203,853 1203,780 1203,707 1144,648 1071,648 L 1071,648 1071,648 Z M 1071,859 C 1027,859 992,824 992,780 992,736 1027,701 1071,701 1115,701 1150,736 1150,780 1150,824 1115,859 1071,859 L 1071,859 1071,859 Z" {}
                        }
                    }
                    div class="intro-footer__links" {
                        a href="https://crates.io/crates/pagetop" target="_blank" rel="noreferrer" { ("Crates.io") }
                        a href="https://docs.rs/pagetop" target="_blank" rel="noreferrer" { ("Docs.rs") }
                        a href="https://git.cillero.es/manuelcillero/pagetop" target="_blank" rel="noreferrer" { (L10n::l("intro_code").using(page)) }
                        em { (L10n::l("intro_have_fun").using(page)) }
                    }
                }
            }
        }
    }
}

fn render_pagetop_intro(page: &mut Page) -> Markup {
    page.alter_assets(AssetsOp::AddJavaScript(JavaScript::on_load_async("intro-js", |cx|
        util::indoc!(r#"
        try {
            const resp = await fetch("https://crates.io/api/v1/crates/pagetop");
            const data = await resp.json();
            const date = new Date(data.versions[0].created_at);
            const formatted = date.toLocaleDateString("LANGID", { year: "numeric", month: "2-digit", day: "2-digit" });
            document.getElementById("intro-release").src = `https://img.shields.io/badge/Release%20date-${encodeURIComponent(formatted)}-blue?label=LABEL&style=for-the-badge`;
            document.getElementById("intro-badges").style.display = "block";
        } catch (e) {
            console.error("Failed to fetch release date from crates.io:", e);
        }
        "#)
        .replace("LANGID", cx.langid().to_string().as_str())
        .replace("LABEL", L10n::l("intro_release_label").using(cx).as_str())
        .to_string(),
    )))
    .alter_child_in("content", ChildOp::Prepend(Child::with(Html::with(|cx| html! {
        p { (L10n::l("intro_text1").using(cx)) }
        div id="intro-badges" style="display: none; margin-bottom: 1.1rem;" {
            img
                src="https://img.shields.io/crates/v/pagetop.svg?label=PageTop&style=for-the-badge"
                alt=[L10n::l("intro_pagetop_label").lookup(cx)] {} (" ")
            img
                id="intro-release"
                alt=[L10n::l("intro_release_label").lookup(cx)] {} (" ")
            img
                src=(format!(
                    "https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label={}&style=for-the-badge",
                    L10n::l("intro_license_label").lookup(cx).unwrap_or_default()
                ))
                alt=[L10n::l("intro_license_label").lookup(cx)] {}
        }
        p { (L10n::l("intro_text2").using(cx)) }
    }))));

    render_intro(page)
}
