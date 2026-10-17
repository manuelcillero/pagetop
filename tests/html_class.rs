use pagetop::prelude::*;

// Every way an element of `html!` gets its `class` attribute: literal `class="..."`, shorthand
// classes (`.a`, `.a[cond]`, `.(expr)`), both together, and the classes of a `Props` splice.

// **< Literal class attribute >********************************************************************

#[pagetop::test]
async fn literal_class_attribute() {
    assert_eq!(
        html! { div class="a b" {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
}

#[pagetop::test]
async fn literal_class_attribute_from_expression_is_escaped() {
    let value = "a\"b";
    assert_eq!(
        html! { div class=(value) {} }.into_string(),
        r#"<div class="a&quot;b"></div>"#
    );
}

#[pagetop::test]
async fn optional_class_attribute() {
    let some = Some("a");
    let none: Option<&str> = None;
    assert_eq!(
        html! { div class=[some] {} }.into_string(),
        r#"<div class="a"></div>"#
    );
    assert_eq!(html! { div class=[none] {} }.into_string(), "<div></div>");
}

// **< Shorthand classes >**************************************************************************

#[pagetop::test]
async fn shorthand_classes() {
    assert_eq!(
        html! { div.a.b {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
}

#[pagetop::test]
async fn shorthand_classes_without_element_name_are_a_div() {
    assert_eq!(
        html! { .a.b {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
}

#[pagetop::test]
async fn shorthand_toggled_classes() {
    assert_eq!(
        html! { div.a.b[true].c[false] {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
}

#[pagetop::test]
async fn shorthand_toggled_classes_go_after_the_fixed_ones() {
    // Toggled classes are written after all the fixed ones, whatever their order in the source.
    assert_eq!(
        html! { div.a[true].b {} }.into_string(),
        r#"<div class="b a"></div>"#
    );
}

#[pagetop::test]
async fn shorthand_only_toggled_classes() {
    assert_eq!(
        html! { div.a[true].b[true] {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
    // A leading space when the first toggled class is off: harmless, browsers ignore it.
    assert_eq!(
        html! { div.a[false].b[true] {} }.into_string(),
        r#"<div class=" b"></div>"#
    );
    assert_eq!(
        html! { div.a[false] {} }.into_string(),
        r#"<div class=""></div>"#
    );
}

#[pagetop::test]
async fn shorthand_class_from_expression() {
    let name = "dynamic";
    assert_eq!(
        html! { div.a.(name) {} }.into_string(),
        r#"<div class="a dynamic"></div>"#
    );
    assert_eq!(
        html! { div.a.(name)[false] {} }.into_string(),
        r#"<div class="a"></div>"#
    );
}

// **< Literal and shorthand classes together >*****************************************************

#[pagetop::test]
async fn literal_and_shorthand_classes_merge_into_one_attribute() {
    assert_eq!(
        html! { div class="a" .b {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
    assert_eq!(
        html! { div class="a b" .c.d {} }.into_string(),
        r#"<div class="a b c d"></div>"#
    );
}

#[pagetop::test]
async fn literal_class_goes_first_whatever_its_position() {
    assert_eq!(
        html! { div.b class="a" {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
}

#[pagetop::test]
async fn literal_and_toggled_classes_merge() {
    assert_eq!(
        html! { li class="item" .active[true] {} }.into_string(),
        r#"<li class="item active"></li>"#
    );
    assert_eq!(
        html! { li class="item" .active[false] {} }.into_string(),
        r#"<li class="item"></li>"#
    );
    assert_eq!(
        html! { li class="item" .active[true].disabled[false].first {} }.into_string(),
        r#"<li class="item first active"></li>"#
    );
}

#[pagetop::test]
async fn class_from_expression_and_shorthand_classes_merge() {
    let base = "card";
    assert_eq!(
        html! { div class=(base) .shadow {} }.into_string(),
        r#"<div class="card shadow"></div>"#
    );
    let escaped = "a\"b";
    assert_eq!(
        html! { div class=(escaped) .c[true] {} }.into_string(),
        r#"<div class="a&quot;b c"></div>"#
    );
}

#[pagetop::test]
async fn optional_class_and_shorthand_classes_merge() {
    let some = Some("a");
    let none: Option<&str> = None;
    assert_eq!(
        html! { div class=[some] .b {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
    // A leading space when the optional value is missing: harmless, browsers ignore it.
    assert_eq!(
        html! { div class=[none] .b {} }.into_string(),
        r#"<div class=" b"></div>"#
    );
}

#[pagetop::test]
async fn merged_class_keeps_the_order_of_the_other_attributes() {
    // `class` is written first, then `id`, then the rest in source order, as always.
    assert_eq!(
        html! { a href="/" class="link" #home .active[true] title="Home" { "x" } }.into_string(),
        r#"<a class="link active" id="home" href="/" title="Home">x</a>"#
    );
}

#[pagetop::test]
async fn merged_class_works_with_conditional_markup() {
    let items = ["a", "b", "c"];
    let current = 1;
    let markup = html! {
        ul {
            @for (i, item) in items.iter().enumerate() {
                li class="item" .current[i == current] { (item) }
            }
        }
    };
    assert_eq!(
        markup.into_string(),
        r#"<ul><li class="item">a</li><li class="item current">b</li><li class="item">c</li></ul>"#
    );
}

// **< Classes from Props >*************************************************************************

#[pagetop::test]
async fn props_classes() {
    let props = Props::classes("from-props-a from-props-b");
    let mut cx = Context::default();
    assert_eq!(
        html! { div (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class="from-props-a from-props-b"></div>"#
    );
}

#[pagetop::test]
async fn props_classes_with_other_literal_attributes_are_kept() {
    // Only a literal `class` collides with the classes of `Props`.
    let props = Props::classes("from-props");
    let mut cx = Context::default();
    assert_eq!(
        html! { div #fixed style="color: red" (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div id="fixed" style="color: red" class="from-props"></div>"#
    );
}

#[pagetop::test]
async fn literal_class_attribute_prevails_over_props_classes() {
    let props = Props::classes("from-props");
    let mut cx = Context::default();
    assert_eq!(
        html! { div class="literal" (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class="literal"></div>"#
    );
}

#[pagetop::test]
async fn shorthand_classes_prevail_over_props_classes() {
    let props = Props::classes("from-props");
    let mut cx = Context::default();
    assert_eq!(
        html! { div.literal (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class="literal"></div>"#
    );
    // Even if every shorthand class is toggled off.
    assert_eq!(
        html! { div.literal[false] (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class=""></div>"#
    );
}

#[pagetop::test]
async fn merged_literal_classes_prevail_over_props_classes() {
    let props = Props::classes("from-props");
    let mut cx = Context::default();
    assert_eq!(
        html! { div class="a" .b[true] (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class="a b"></div>"#
    );
}

#[pagetop::test]
async fn props_keep_their_id_when_only_the_class_collides() {
    let props = Props::classes("from-props").with_id("from-props-id");
    let mut cx = Context::default();
    assert_eq!(
        html! { div class="literal" (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class="literal" id="from-props-id"></div>"#
    );
}

#[pagetop::test]
async fn props_classes_after_class_operations() {
    let props = Props::classes("b c")
        .with_prop(PropsOp::prepend_classes("a"))
        .with_prop(PropsOp::add_classes("d"))
        .with_prop(PropsOp::remove_classes("c"))
        .with_prop(PropsOp::replace_classes("b", "beta"));
    let mut cx = Context::default();
    assert_eq!(
        html! { div (props.unpack(&mut cx)) {} }.into_string(),
        r#"<div class="a beta d"></div>"#
    );
}

#[pagetop::test]
async fn empty_props_write_no_class_attribute() {
    let props = Props::default();
    let mut cx = Context::default();
    assert_eq!(
        html! { div (props.unpack(&mut cx)) {} }.into_string(),
        "<div></div>"
    );
}

// **< Classes of components >**********************************************************************

#[pagetop::test]
async fn component_merges_its_own_classes_with_the_added_ones() {
    // A component writes its own classes and the ones added with `with_prop()` in one attribute.
    let mut container = Container::new()
        .with_prop(PropsOp::add_classes("extra-a extra-b"))
        .with_child(Lc::n("x"));
    let markup = container
        .render(&mut Context::default())
        .await
        .into_string();
    let class = markup
        .split("class=\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("the container has a class attribute");
    assert!(class.split(' ').any(|c| c == "extra-a"), "{markup}");
    assert!(class.split(' ').any(|c| c == "extra-b"), "{markup}");
    assert_eq!(markup.matches("class=").count(), 1, "{markup}");
}

#[pagetop::test]
async fn component_with_added_classes_never_duplicates_the_class_attribute() {
    // Every component has some content: empty ones are not rendered.
    let mut cx = Context::default();
    let components: Vec<Box<dyn Component>> = vec![
        Box::new(
            Block::new()
                .with_prop(PropsOp::add_classes("extra"))
                .with_child(Lc::n("x")),
        ),
        Box::new(Badge::info(Lc::n("x")).with_prop(PropsOp::add_classes("extra"))),
        Box::new(Button::plain(Lc::n("x")).with_prop(PropsOp::add_classes("extra"))),
        Box::new(
            Nav::new()
                .with_prop(PropsOp::add_classes("extra"))
                .with_item(nav::Item::link(Lc::n("x"), "/")),
        ),
    ];
    for mut component in components {
        let markup = component.render(&mut cx).await.into_string();
        let first_tag = &markup[..markup.find('>').unwrap_or(markup.len())];
        assert!(first_tag.contains("extra"), "{markup}");
        assert_eq!(first_tag.matches("class=").count(), 1, "{markup}");
    }
}
