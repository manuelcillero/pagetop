mod figfont;
use figfont::FIGFONT;

use crate::config;

use substring::Substring;


pub fn print_on_startup() {
    if config::SETTINGS.app.startup_banner.to_lowercase() != "off" {
        if let Some((term_width, _)) = term_size::dimensions() {
            if term_width >= 80 {
                let maxlen = (term_width / 10) - 2;
                let mut app = config::SETTINGS.app.name.substring(0, maxlen).to_owned();
                if config::SETTINGS.app.name.len() > maxlen {
                    app = format!("{}...", app);
                }
                println!(
                    "\n{} {}\n\n Powered by PageTop {}\n",
                    FIGFONT.convert(&app).unwrap(),
                    &config::SETTINGS.app.description,
                    env!("CARGO_PKG_VERSION")
                );
                return;
            }
        }
        println!(
            "\n{}\n{}\n\nPowered by PageTop {}\n",
            &config::SETTINGS.app.name,
            &config::SETTINGS.app.description,
            env!("CARGO_PKG_VERSION")
        );
    }
}
