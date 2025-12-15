use crate::global;

use std::sync::LazyLock;

use figlet_rs::FIGfont;

pub static FIGFONT: LazyLock<FIGfont> = LazyLock::new(|| {
    let slant = include_str!("slant.flf");
    let small = include_str!("small.flf");
    let speed = include_str!("speed.flf");
    let starwars = include_str!("starwars.flf");

    FIGfont::from_content(match global::SETTINGS.app.startup_banner {
        global::StartupBanner::Off | global::StartupBanner::Slant => slant,
        global::StartupBanner::Small => small,
        global::StartupBanner::Speed => speed,
        global::StartupBanner::Starwars => starwars,
    })
    .unwrap()
});
