use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./static/monster")
        .with_name("monster")
        .build()
}
