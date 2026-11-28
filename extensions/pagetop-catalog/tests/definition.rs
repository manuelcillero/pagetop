use pagetop::prelude::*;
use pagetop_catalog::{Catalog, Field};

// Wrong uses of the definition API fail in development (`debug_assert!`).

#[pagetop::test]
#[should_panic(expected = "must be lowercase ASCII letters, digits and underscores")]
async fn a_catalog_name_must_be_an_identifier() {
    let _ = Catalog::new("Colors", "shop", Lc::n("Colors"));
}

#[pagetop::test]
#[should_panic(expected = "use without_code()")]
async fn a_zero_code_length_needs_without_code() {
    let _ = Catalog::new("colors", "shop", Lc::n("Colors")).with_code_max_len(0);
}

#[pagetop::test]
#[should_panic(expected = "with_filter() only applies")]
async fn only_some_fields_can_filter() {
    let _ = Field::text("hex", Lc::n("Hex")).with_filter(true);
}

#[pagetop::test]
#[should_panic(expected = "with_range() only applies to integer fields")]
async fn a_range_needs_an_integer() {
    let _ = Field::decimal("inches", Lc::n("Inches")).with_range(1, 10);
}

#[pagetop::test]
#[should_panic(expected = "with_decimal_range() only applies to decimal fields")]
async fn a_decimal_range_needs_a_decimal() {
    let _ = Field::integer("shade", Lc::n("Shade")).with_decimal_range(1.0, 10.0);
}
