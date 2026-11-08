use pagetop::prelude::*;

#[pagetop::test]
#[should_panic(expected = "type mismatch")]
async fn a_param_of_another_type_is_an_error() {
    let cx = Context::default().with_param("count", 7_i32);
    let _ = cx.param_or("count", 0_u32);
}

#[pagetop::test]
async fn a_missing_param_gets_the_default() {
    let cx = Context::default();
    assert_eq!(cx.param_or("count", 3_u32), 3);
}
