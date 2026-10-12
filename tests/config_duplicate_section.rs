use pagetop::prelude::*;

// Settings that claim `[app]`, a section that already belongs to the global settings.
#[settings(section = "app")]
pub struct ClashingSettings {
    pub name: String,
}

static SETTINGS: LazySettings<ClashingSettings> = LazySettings::new();

// `EXTENSIONS` is a global `OnceLock`, so this test needs its own test binary.
struct ClashingExtension;

#[async_trait]
impl Extension for ClashingExtension {
    fn settings(&self) -> Option<&'static dyn LoadSettings> {
        Some(&SETTINGS)
    }
}

#[pagetop::test]
#[should_panic(expected = "Settings section [app] is claimed by more than one extension")]
async fn a_section_claimed_twice_stops_the_startup() {
    let _ = Application::prepare(&ClashingExtension).await;
}
