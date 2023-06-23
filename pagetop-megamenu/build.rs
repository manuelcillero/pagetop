use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./static")
        .with_name("megamenu")
        .build()
}
