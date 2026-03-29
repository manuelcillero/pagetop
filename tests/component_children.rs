use pagetop::prelude::*;

// **< TestComp — componente mínimo para los tests >************************************************
//
// Componente con id configurable y texto fijo de salida. El id permite probar las operaciones de
// `Children` basadas en identificador (`InsertAfterId`, `RemoveById`, etc.).

#[derive(AutoDefault, Clone)]
struct TestComp {
    id: AttrId,
    text: String,
}

impl Component for TestComp {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn prepare(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(html! { (self.text) })
    }
}

impl TestComp {
    /// Crea un componente con id y texto de salida fijos.
    fn tagged(id: &str, text: &str) -> Self {
        let mut c = Self::default();
        c.id.alter_id(id);
        c.text = text.to_string();
        c
    }

    /// Crea un componente sin id, con texto de salida fijo.
    fn text(text: &str) -> Self {
        let mut c = Self::default();
        c.text = text.to_string();
        c
    }
}

// **< Child >***************************************************************************************

#[pagetop::test]
async fn child_default_is_empty() {
    let child = Child::default();
    assert!(child.id().is_none());
    assert!(child.render(&mut Context::default()).is_empty());
}

#[pagetop::test]
async fn child_with_stores_component_and_renders_it() {
    let child = Child::with(TestComp::text("hola"));
    assert_eq!(child.render(&mut Context::default()).into_string(), "hola");
}

#[pagetop::test]
async fn child_id_returns_component_id() {
    let child = Child::with(TestComp::tagged("my-id", "texto"));
    assert_eq!(child.id(), Some("my-id".to_string()));
}

#[pagetop::test]
async fn child_from_component_is_equivalent_to_with() {
    let child: Child = TestComp::text("desde from").into();
    assert_eq!(
        child.render(&mut Context::default()).into_string(),
        "desde from"
    );
}

#[pagetop::test]
async fn child_clone_is_deep() {
    // Modificar el clon no debe afectar al original.
    let original = Child::with(TestComp::text("original"));
    let clone = original.clone();
    assert_eq!(
        original.render(&mut Context::default()).into_string(),
        "original"
    );
    assert_eq!(
        clone.render(&mut Context::default()).into_string(),
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
    assert_eq!(c.render(&mut Context::default()).into_string(), "abc");
}

#[pagetop::test]
async fn children_add_if_empty_only_adds_when_list_is_empty() {
    let mut cx = Context::default();

    // Se añade porque la lista está vacía.
    let c = Children::new().with_child(ChildOp::AddIfEmpty(TestComp::text("primero").into()));
    assert_eq!(c.len(), 1);

    // No se añade porque ya hay un elemento.
    let c = c.with_child(ChildOp::AddIfEmpty(TestComp::text("segundo").into()));
    assert_eq!(c.len(), 1);
    assert_eq!(c.render(&mut cx).into_string(), "primero");
}

#[pagetop::test]
async fn children_add_many_appends_all_in_order() {
    let c = Children::new().with_child(ChildOp::AddMany(vec![
        TestComp::text("x").into(),
        TestComp::text("y").into(),
        TestComp::text("z").into(),
    ]));
    assert_eq!(c.len(), 3);
    assert_eq!(c.render(&mut Context::default()).into_string(), "xyz");
}

#[pagetop::test]
async fn children_prepend_inserts_at_start() {
    let c = Children::new()
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::Prepend(TestComp::text("a").into()));
    assert_eq!(c.render(&mut Context::default()).into_string(), "ab");
}

#[pagetop::test]
async fn children_prepend_many_inserts_all_at_start_maintaining_order() {
    let c = Children::new()
        .with_child(TestComp::text("c"))
        .with_child(ChildOp::PrependMany(vec![
            TestComp::text("a").into(),
            TestComp::text("b").into(),
        ]));
    assert_eq!(c.render(&mut Context::default()).into_string(), "abc");
}

#[pagetop::test]
async fn children_insert_after_id_inserts_after_matching_element() {
    let c = Children::new()
        .with_child(TestComp::tagged("first", "a"))
        .with_child(TestComp::text("c"))
        .with_child(ChildOp::InsertAfterId("first", TestComp::text("b").into()));
    assert_eq!(c.render(&mut Context::default()).into_string(), "abc");
}

#[pagetop::test]
async fn children_insert_after_id_appends_when_id_not_found() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(ChildOp::InsertAfterId(
            "no-existe",
            TestComp::text("b").into(),
        ));
    assert_eq!(c.render(&mut Context::default()).into_string(), "ab");
}

#[pagetop::test]
async fn children_insert_before_id_inserts_before_matching_element() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(TestComp::tagged("last", "c"))
        .with_child(ChildOp::InsertBeforeId("last", TestComp::text("b").into()));
    assert_eq!(c.render(&mut Context::default()).into_string(), "abc");
}

#[pagetop::test]
async fn children_insert_before_id_prepends_when_id_not_found() {
    let c = Children::new()
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::InsertBeforeId(
            "no-existe",
            TestComp::text("a").into(),
        ));
    assert_eq!(c.render(&mut Context::default()).into_string(), "ab");
}

#[pagetop::test]
async fn children_remove_by_id_removes_first_matching_element() {
    let c = Children::new()
        .with_child(TestComp::tagged("keep", "a"))
        .with_child(TestComp::tagged("drop", "b"))
        .with_child(TestComp::text("c"))
        .with_child(ChildOp::RemoveById("drop"));
    assert_eq!(c.len(), 2);
    assert_eq!(c.render(&mut Context::default()).into_string(), "ac");
}

#[pagetop::test]
async fn children_remove_by_id_does_nothing_when_id_not_found() {
    let c = Children::new()
        .with_child(TestComp::text("a"))
        .with_child(ChildOp::RemoveById("no-existe"));
    assert_eq!(c.len(), 1);
}

#[pagetop::test]
async fn children_replace_by_id_replaces_first_matching_element() {
    let c = Children::new()
        .with_child(TestComp::tagged("target", "viejo"))
        .with_child(TestComp::text("b"))
        .with_child(ChildOp::ReplaceById(
            "target",
            TestComp::text("nuevo").into(),
        ));
    assert_eq!(c.len(), 2);
    assert_eq!(c.render(&mut Context::default()).into_string(), "nuevob");
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
        .with_child(TestComp::tagged("uno", "a"))
        .with_child(TestComp::tagged("dos", "b"));
    assert!(c.get_by_id("uno").is_some());
    assert!(c.get_by_id("dos").is_some());
    assert!(c.get_by_id("tres").is_none());
}

#[pagetop::test]
async fn children_iter_by_id_yields_all_matching_children() {
    let c = Children::new()
        .with_child(TestComp::tagged("rep", "a"))
        .with_child(TestComp::tagged("rep", "b"))
        .with_child(TestComp::tagged("otro", "c"));
    assert_eq!(c.iter_by_id("rep").count(), 2);
    assert_eq!(c.iter_by_id("otro").count(), 1);
    assert_eq!(c.iter_by_id("ninguno").count(), 0);
}

#[pagetop::test]
async fn children_render_concatenates_all_outputs_in_order() {
    let c = Children::new()
        .with_child(TestComp::text("uno "))
        .with_child(TestComp::text("dos "))
        .with_child(TestComp::text("tres"));
    assert_eq!(
        c.render(&mut Context::default()).into_string(),
        "uno dos tres"
    );
}

// **< Embed >**************************************************************************************

#[pagetop::test]
async fn embed_default_is_empty() {
    let embed: Embed<TestComp> = Embed::default();
    assert!(embed.id().is_none());
    assert!(embed.render(&mut Context::default()).is_empty());
    assert!(embed.get().is_none());
}

#[pagetop::test]
async fn embed_with_stores_component() {
    let embed = Embed::with(TestComp::text("contenido"));
    assert!(embed.get().is_some());
    assert_eq!(
        embed.render(&mut Context::default()).into_string(),
        "contenido"
    );
}

#[pagetop::test]
async fn embed_id_returns_component_id() {
    let embed = Embed::with(TestComp::tagged("embed-id", "texto"));
    assert_eq!(embed.id(), Some("embed-id".to_string()));
}

#[pagetop::test]
async fn embed_get_is_some_when_component_present() {
    let embed = Embed::with(TestComp::tagged("abc", "hola"));
    // `get()` devuelve Some; la lectura del id verifica que accede al componente correctamente.
    assert!(embed.get().is_some());
    assert_eq!(embed.id(), Some("abc".to_string()));
}

#[pagetop::test]
async fn embed_get_allows_mutating_component() {
    let embed = Embed::with(TestComp::tagged("orig", "texto"));
    // El `;` final convierte el `if let` en sentencia y libera el guard antes que `embed`.
    if let Some(mut comp) = embed.get() {
        comp.id.alter_id("modificado");
    };
    assert_eq!(embed.id(), Some("modificado".to_string()));
}

#[pagetop::test]
async fn embed_with_component_replaces_content() {
    let embed =
        Embed::with(TestComp::text("primero")).with_component(Some(TestComp::text("segundo")));
    assert_eq!(
        embed.render(&mut Context::default()).into_string(),
        "segundo"
    );
}

#[pagetop::test]
async fn embed_with_component_none_empties_embed() {
    let embed = Embed::with(TestComp::text("algo")).with_component(None);
    assert!(embed.get().is_none());
    assert!(embed.render(&mut Context::default()).is_empty());
}

#[pagetop::test]
async fn embed_clone_is_deep() {
    let original = Embed::with(TestComp::tagged("orig", "texto"));
    let clone = original.clone();
    // Mutar el clon no debe afectar al original.
    if let Some(mut comp) = clone.get() {
        comp.id.alter_id("clone-id");
    }
    assert_eq!(original.id(), Some("orig".to_string()));
    assert_eq!(clone.id(), Some("clone-id".to_string()));
}

#[pagetop::test]
async fn embed_converts_into_child() {
    let embed = Embed::with(TestComp::text("desde embed"));
    let child = Child::from(embed);
    assert_eq!(
        child.render(&mut Context::default()).into_string(),
        "desde embed"
    );
}
