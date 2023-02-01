fn main() -> std::io::Result<()> {
    pagetop_build::bundle_resources("./static/theme", "theme", None)
}
