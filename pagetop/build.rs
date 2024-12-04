use pagetop_build::StaticFilesBundle;

fn main() -> std::io::Result<()> {
    StaticFilesBundle::from_dir("../static", None)
        .with_name("assets")
        .build()
}
