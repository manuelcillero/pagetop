use crate::{config, LazyStatic};

use figlet_rs::FIGfont;

pub static FIGFONT: LazyStatic<FIGfont> = LazyStatic::new(|| {
    let slant = include_str!("slant.flf");
    let small = include_str!("small.flf");
    let speed = include_str!("speed.flf");
    let starwars = include_str!("starwars.flf");

    FIGfont::from_content(
        match config::SETTINGS.app.startup_banner.to_lowercase().as_str() {
            "off" => slant,
            "slant" => slant,
            "small" => small,
            "speed" => speed,
            "starwars" => starwars,
            _ => {
                println!(
                    "\n FIGfont \"{}\" not found for banner. Using \"Slant\". Check settings files.",
                    config::SETTINGS.app.startup_banner,
                );
                slant
            }
        },
    )
    .unwrap()
});
