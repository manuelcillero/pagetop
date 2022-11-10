mod figfont;
use figfont::FIGFONT;

use crate::global;

use substring::Substring;

pub fn print_on_startup() {
    if global::SETTINGS.app.startup_banner.to_lowercase() != "off" {
        if let Some((term_width, _)) = term_size::dimensions() {
            if term_width >= 80 {
                let maxlen = (term_width / 10) - 2;
                let mut app = global::SETTINGS.app.name.substring(0, maxlen).to_owned();
                if global::SETTINGS.app.name.len() > maxlen {
                    app = format!("{}...", app);
                }
                println!(
                    "\n{} {}\n\n Powered by PageTop {}\n",
                    FIGFONT.convert(&app).unwrap(),
                    &global::SETTINGS.app.description,
                    env!("CARGO_PKG_VERSION")
                );
                return;
            }
        }
        println!(
            "\n{}\n{}\n\nPowered by PageTop {}\n",
            &global::SETTINGS.app.name,
            &global::SETTINGS.app.description,
            env!("CARGO_PKG_VERSION")
        );
    }
}
