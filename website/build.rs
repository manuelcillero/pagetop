use pagetop::util::bundle_resources;
use pagetop_mdbook::util::except_mdbook_common_resources;

fn main() -> std::io::Result<()> {
    bundle_resources("./doc/en", "guides_en", Some(except_mdbook_common_resources))?;
    bundle_resources("./doc/es", "guias_es",  Some(except_mdbook_common_resources))
}
