#![allow(dead_code)]
#![doc(html_no_source)]
#![allow(clippy::needless_doctest_main)]

mod resource {
    include!("src/resource.rs");
}
use resource::generate_resources_mapping;
mod resource_dir {
    include!("src/resource_dir.rs");
}
use resource_dir::resource_dir;
mod sets {
    include!("src/sets.rs");
}
use sets::{generate_resources_sets, SplitByCount};

use std::{env, path::Path};

fn main() -> std::io::Result<()> {
    resource_dir("./tests").build_test()?;

    let out_dir = env::var("OUT_DIR").unwrap();

    generate_resources_mapping(
        "./tests",
        None,
        Path::new(&out_dir).join("generated_mapping.rs"),
        "pagetop_statics",
    )?;

    generate_resources_sets(
        "./tests",
        None,
        Path::new(&out_dir).join("generated_sets.rs"),
        "sets",
        "generate",
        &mut SplitByCount::new(2),
        "pagetop_statics",
    )?;

    Ok(())
}
