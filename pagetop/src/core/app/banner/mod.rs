use crate::Lazy;
use crate::config::SETTINGS;

use figlet_rs::FIGfont;
use substring::Substring;

static FIGFONT: Lazy<FIGfont> = Lazy::new(|| {
    let slant    = include_str!("slant.flf");
    let small    = include_str!("small.flf");
    let speed    = include_str!("speed.flf");
    let starwars = include_str!("starwars.flf");

    FIGfont::from_content(
        match SETTINGS.app.startup_banner.to_lowercase().as_str() {
            "off"      => slant,
            "slant"    => slant,
            "small"    => small,
            "speed"    => speed,
            "starwars" => starwars,
            _ => {
                println!(
                    "\n FIGfont \"{}\" not found for banner. {}. {}.",
                    SETTINGS.app.startup_banner,
                    "Using \"Slant\"",
                    "Check the settings file",
                );
                slant
            }
        }
    ).unwrap()
});

pub fn print_on_startup() {
    if SETTINGS.app.startup_banner.to_lowercase() != "off" {
        if let Some((term_width, _)) = term_size::dimensions() {
            if term_width >= 80 {
                let maxlen = (term_width / 10) - 2;
                let mut app = SETTINGS.app.name.substring(0, maxlen).to_owned();
                if SETTINGS.app.name.len() > maxlen {
                    app = format!("{}...", app);
                }
                println!("\n{} {}\n\n Powered by PageTop {}\n",
                    FIGFONT.convert(&app).unwrap(),
                    &SETTINGS.app.description,
                    env!("CARGO_PKG_VERSION")
                );
                return;
            }
        }
        println!("\n{}\n{}\n\nPowered by PageTop {}\n",
            &SETTINGS.app.name,
            &SETTINGS.app.description,
            env!("CARGO_PKG_VERSION")
        );
    }
}
