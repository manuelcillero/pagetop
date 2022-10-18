mod figfont;
use figfont::FIGFONT;

use crate::config;

use substring::Substring;

pub fn print_on_startup() {
    if config::get("app.startup_banner").to_lowercase() != "off" {
        if let Some((term_width, _)) = term_size::dimensions() {
            if term_width >= 80 {
                let maxlen = (term_width / 10) - 2;
                let mut app = config::get("app.name").substring(0, maxlen).to_owned();
                if config::get("app.name").len() > maxlen {
                    app = format!("{}...", app);
                }
                println!(
                    "\n{} {}\n\n Powered by PageTop {}\n",
                    FIGFONT.convert(&app).unwrap(),
                    config::get("app.description"),
                    env!("CARGO_PKG_VERSION")
                );
                return;
            }
        }
        println!(
            "\n{}\n{}\n\nPowered by PageTop {}\n",
            config::get("app.name"),
            config::get("app.description"),
            env!("CARGO_PKG_VERSION")
        );
    }
}
