use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("./static/assets", None)
        .with_name("assets")
        .build()
}
