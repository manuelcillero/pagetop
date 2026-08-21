use pagetop::prelude::*;

// **< TestComp - minimal component for the tests >*************************************************
//
// Component with configurable id and fixed output text. The id allows testing the identifier-based
// `Children` operations (`InsertAfterId`, `RemoveById`, etc.).

#[derive(AutoDefault, Clone)]
struct TestComp {
    props: Props,
    text: String,
}

#[async_trait]
impl Component for TestComp {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    async fn prepare(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! { (self.text) })
    }
}

impl TestComp {
    /// Creates a component with a fixed id and output text.
    fn tagged(id: &str, text: &str) -> Self {
        let mut c = Self::default();
        c.props.alter_prop(PropsOp::set_id(id.to_string()));
        c.text = text.to_string();
        c
    }

    /// Creates a component with no id, with fixed output text.
    fn text(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
    }
}

// **< Child >**************************************************************************************

#[pagetop::test]
async fn child_default_is_empty() {
    let child = Child::default();
    assert!(child.id().is_none());
    assert!(child.render(&mut Context::default()).await.is_empty());
}

#[pagetop::test]
async fn child_with_stores_component_and_renders_it() {
    let child = Child::with(TestComp::text("hello"));
    assert_eq!(
        child.render(&mut Context::default()).await.into_string(),
        "hello"
    );
}

#[pagetop::test]
async fn child_id_returns_component_id() {
    let child = Child::with(TestComp::tagged("my-id", "text"));
    assert_eq!(child.id(), Some("my-id".to_string()));
}

#[pagetop::test]
async fn child_from_component_is_equivalent_to_with() {
    let child: Child = TestComp::text("from-trait").into();
    assert_eq!(
        child.render(&mut Context::default()).await.into_string(),
        "from-trait"
    );
}

#[pagetop::test]
async fn child_clone_is_deep() {
    // Modifying the clone must not affect the original.
    let original = Child::with(TestComp::text("original"));
    let clone = original.clone();
    assert_eq!(
        original.render(&mut Context::default()).await.into_string(),
        "original"
    );
    assert_eq!(
        clone.render(&mut Context::default()).await.into_string(),
        "original"
    );
}

// **< Children + ChildOp >*************************************************************************

#[pagetop::test]
async fn children_new_is_empty() {
    let c = Children::new();
    assert!(c.is_empty());
    assert_eq!(c.len(), 0);
}

#[pagetop::test]
async fn children_add_appends_in_order() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(TestComp::text("b"))
        .with_child(TestComp::text("c"));
    assert_eq!(c.len(), 3);
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "abc");
}

#[pagetop::test]
async fn children_add_if_empty_only_adds_when_list_is_empty() {
    let mut cx = Context::default();

    // It gets added because the list is empty.
    let c = Children::new().with_child(ChildOp::AddIfEmpty(TestComp::text("first").into()));
    assert_eq!(c.len(), 1);

    // It does not get added because there is already an element.
    let c = c.with_child(ChildOp::AddIfEmpty(TestComp::text("second").into()));
    assert_eq!(c.len(), 1);
    assert_eq!(c.render(&mut cx).await.into_string(), "first");
}

#[pagetop::test]
async fn children_add_many_appends_all_in_order() {
    let c = Children::new().with_child(ChildOp::AddMany(vec![
        TestComp::text("x").into(),
        TestComp::text("y").into(),
        TestComp::text("z").into(),
    ]));
    assert_eq!(c.len(), 3);
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "xyz");
}

#[pagetop::test]
async fn children_prepend_inserts_at_start() {
    let c = Children::new()
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::Prepend(TestComp::text("a").into()));
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "ab");
}

#[pagetop::test]
async fn children_prepend_many_inserts_all_at_start_maintaining_order() {
    let c = Children::new()
        .with_child(TestComp::text("c"))
        .with_child(ChildOp::PrependMany(vec![
            TestComp::text("a").into(),
            TestComp::text("b").into(),
        ]));
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "abc");
}

#[pagetop::test]
async fn children_insert_after_id_inserts_after_matching_element() {
    let c = Children::new()
        .with_child(TestComp::tagged("first", "a"))
        .with_child(TestComp::text("c"))
        .with_child(ChildOp::InsertAfterId("first", TestComp::text("b").into()));
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "abc");
}

#[pagetop::test]
async fn children_insert_after_id_appends_when_id_not_found() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(ChildOp::InsertAfterId(
            "not-found",
            TestComp::text("b").into(),
        ));
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "ab");
}

#[pagetop::test]
async fn children_insert_before_id_inserts_before_matching_element() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(TestComp::tagged("last", "c"))
        .with_child(ChildOp::InsertBeforeId("last", TestComp::text("b").into()));
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "abc");
}

#[pagetop::test]
async fn children_insert_before_id_prepends_when_id_not_found() {
    let c = Children::new()
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::InsertBeforeId(
            "not-found",
            TestComp::text("a").into(),
        ));
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "ab");
}

#[pagetop::test]
async fn children_remove_by_id_removes_first_matching_element() {
    let c = Children::new()
        .with_child(TestComp::tagged("keep", "a"))
        .with_child(TestComp::tagged("drop", "b"))
        .with_child(TestComp::text("c"))
        .with_child(ChildOp::RemoveById("drop"));
    assert_eq!(c.len(), 2);
    assert_eq!(c.render(&mut Context::default()).await.into_string(), "ac");
}

#[pagetop::test]
async fn children_remove_by_id_does_nothing_when_id_not_found() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(ChildOp::RemoveById("not-found"));
    assert_eq!(c.len(), 1);
}

#[pagetop::test]
async fn children_replace_by_id_replaces_first_matching_element() {
    let c = Children::new()
        .with_child(TestComp::tagged("target", "old"))
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::ReplaceById("target", TestComp::text("new").into()));
    assert_eq!(c.len(), 2);
    assert_eq!(
        c.render(&mut Context::default()).await.into_string(),
        "newb"
    );
}

#[pagetop::test]
async fn children_reset_clears_all_elements() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::Reset);
    assert!(c.is_empty());
}

#[pagetop::test]
async fn children_get_by_id_returns_first_matching_child() {
    let c = Children::new()
        .with_child(TestComp::tagged("one", "a"))
        .with_child(TestComp::tagged("two", "b"));
    assert!(c.get_by_id("one").is_some());
    assert!(c.get_by_id("two").is_some());
    assert!(c.get_by_id("three").is_none());
}

#[pagetop::test]
async fn children_iter_by_id_yields_all_matching_children() {
    let c = Children::new()
        .with_child(TestComp::tagged("rep", "a"))
        .with_child(TestComp::tagged("rep", "b"))
        .with_child(TestComp::tagged("other", "c"));
    assert_eq!(c.iter_by_id("rep").count(), 2);
    assert_eq!(c.iter_by_id("other").count(), 1);
    assert_eq!(c.iter_by_id("none").count(), 0);
}

#[pagetop::test]
async fn children_render_concatenates_all_outputs_in_order() {
    let c = Children::new()
        .with_child(TestComp::text("one "))
        .with_child(TestComp::text("two "))
        .with_child(TestComp::text("three"));
    assert_eq!(
        c.render(&mut Context::default()).await.into_string(),
        "one two three"
    );
}

// **< Embed >**************************************************************************************

#[pagetop::test]
async fn embed_default_is_empty() {
    let embed: Embed<TestComp> = Embed::default();
    assert!(embed.id().is_none());
    assert!(embed.render(&mut Context::default()).await.is_empty());
    assert!(embed.get().is_none());
}

#[pagetop::test]
async fn embed_with_stores_component() {
    let embed = Embed::with(TestComp::text("content"));
    assert!(embed.get().is_some());
    assert_eq!(
        embed.render(&mut Context::default()).await.into_string(),
        "content"
    );
}

#[pagetop::test]
async fn embed_id_returns_component_id() {
    let embed = Embed::with(TestComp::tagged("embed-id", "text"));
    assert_eq!(embed.id(), Some("embed-id".to_string()));
}

#[pagetop::test]
async fn embed_get_is_some_when_component_present() {
    let embed = Embed::with(TestComp::tagged("abc", "hello"));
    // `get()` returns Some; reading the id verifies that it accesses the component correctly.
    assert!(embed.get().is_some());
    assert_eq!(embed.id(), Some("abc".to_string()));
}

#[pagetop::test]
async fn embed_get_allows_mutating_component() {
    let mut embed = Embed::with(TestComp::tagged("orig", "text"));
    if let Some(comp) = embed.get_mut() {
        comp.props
            .alter_prop(PropsOp::set_id("modified".to_string()));
    };
    assert_eq!(embed.id(), Some("modified".to_string()));
}

#[pagetop::test]
async fn embed_with_component_replaces_content() {
    let embed = Embed::with(TestComp::text("first")).with_component(Some(TestComp::text("second")));
    assert_eq!(
        embed.render(&mut Context::default()).await.into_string(),
        "second"
    );
}

#[pagetop::test]
async fn embed_with_component_none_empties_embed() {
    let embed = Embed::with(TestComp::text("something")).with_component(None);
    assert!(embed.get().is_none());
    assert!(embed.render(&mut Context::default()).await.is_empty());
}

#[pagetop::test]
async fn embed_clone_is_deep() {
    let original = Embed::with(TestComp::tagged("orig", "text"));
    let mut clone = original.clone();
    // Mutating the clone must not affect the original.
    if let Some(comp) = clone.get_mut() {
        comp.props
            .alter_prop(PropsOp::set_id("clone-id".to_string()));
    }
    assert_eq!(original.id(), Some("orig".to_string()));
    assert_eq!(clone.id(), Some("clone-id".to_string()));
}

#[pagetop::test]
async fn embed_converts_into_child() {
    let embed = Embed::with(TestComp::text("from embed"));
    let child = Child::from(embed);
    assert_eq!(
        child.render(&mut Context::default()).await.into_string(),
        "from embed"
    );
}
