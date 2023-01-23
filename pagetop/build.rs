fn main() -> std::io::Result<()> {
    pagetop_build::bundle_resources("./static/theme", "theme", None)?;
    pagetop_build::bundle_resources("./static/aliner", "aliner", None)?;
    pagetop_build::bundle_resources("./static/bootsier", "bootsier", None)?;
    pagetop_build::bundle_resources("./static/bulmix", "bulmix", None)
}
