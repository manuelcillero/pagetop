use pagetop::util::bundle_resources;

fn main() -> std::io::Result<()> {
    bundle_resources("./static", "mdbook", None)
}
