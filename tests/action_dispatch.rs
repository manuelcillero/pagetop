use pagetop::prelude::*;

use std::sync::Mutex;

// The action registry is built once per process, at startup: all the actions of this test are
// declared in a single extension and the cases are different tests on top of it.

// Test action without referer, with configurable weight.
struct Probe {
    name: &'static str,
    weight: Weight,
}

impl ActionDispatcher for Probe {
    fn weight(&self) -> Weight {
        self.weight
    }
}

// Test referers: `Node` has actions filtered by id; `Plain` only has general ones.
struct Node(&'static str);
struct Plain;

// Test action with referer `R`, with configurable weight and referer id.
struct Tagged {
    name: &'static str,
    weight: Weight,
    referer: ActionReferer,
}

impl Tagged {
    fn new<R: 'static>(name: &'static str, weight: Weight, referer_id: Option<&str>) -> Self {
        let referer = ActionReferer::of::<R>();
        Tagged {
            name,
            weight,
            referer: match referer_id {
                Some(id) => referer.with_id(id),
                None => referer,
            },
        }
    }
}

impl ActionDispatcher for Tagged {
    fn referer(&self) -> Option<&ActionReferer> {
        Some(&self.referer)
    }
    fn weight(&self) -> Weight {
        self.weight
    }
}

// Another action type, with nothing registered.
struct Unregistered;
impl ActionDispatcher for Unregistered {}

// Ids of the components the `BeforeRender` actions were applied to. Tests run concurrently and
// render the same component types, so they are checked by id and not with a shared counter.
static GENERAL_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
static ID_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
// Log of the transformations applied: (component id, which one).
static MARKUP_LOG: Mutex<Vec<(String, &'static str)>> = Mutex::new(Vec::new());

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        let probe = |name, weight| Probe { name, weight };
        actions![
            // Registered out of order: dispatch delivers them by weight and, at equal weight, in
            // registration order.
            probe("c-late", 10),
            probe("a-early", -5),
            probe("b-first-of-zero", 0),
            probe("d-second-of-zero", 0),
            // With referer: the ones for the type and, separately, the ones filtered by id.
            Tagged::new::<Node>("general", 0, None),
            Tagged::new::<Node>("x-2", 3, Some("x")),
            Tagged::new::<Node>("x-1", 1, Some("x")),
            Tagged::new::<Node>("y-1", 0, Some("y")),
            Tagged::new::<Plain>("plain", 0, None),
            action::component::BeforeRender::<Badge>::new(|badge, _cx| {
                GENERAL_LOG
                    .lock()
                    .unwrap()
                    .push(badge.id().unwrap_or_default());
            }),
            action::component::BeforeRender::<Badge>::new(|badge, _cx| {
                ID_LOG.lock().unwrap().push(badge.id().unwrap_or_default());
            })
            .filter_by_referer_id("special-before"),
            action::component::TransformMarkup::<Badge>::new(|badge, _cx, markup| {
                let id = badge.id().unwrap_or_default();
                MARKUP_LOG.lock().unwrap().push((id, "general"));
                html! { div { (markup) } }
            }),
            action::component::TransformMarkup::<Badge>::new(|badge, _cx, markup| {
                let id = badge.id().unwrap_or_default();
                MARKUP_LOG.lock().unwrap().push((id, "by-id"));
                html! { section { (markup) } }
            })
            .filter_by_referer_id("special-markup"),
        ]
    }
}

async fn start() {
    // Starting more than once in the same process (as tests do) does not duplicate the actions.
    let _ = Application::prepare(&Ext).await;
    let _ = Application::prepare(&Ext).await;
}

fn names() -> Vec<&'static str> {
    let mut seen = Vec::new();
    dispatch_actions(|action: &Probe| seen.push(action.name));
    seen
}

// Names of the `Node` actions dispatched for a referer with that id, and how many times the id was
// requested.
fn node_names(id: &'static str) -> (Vec<&'static str>, usize) {
    let mut seen = Vec::new();
    let mut id_calls = 0;
    dispatch_referer(
        &mut Node(id),
        |node| {
            id_calls += 1;
            Some(node.0.to_owned())
        },
        |action: &Tagged, _| seen.push(action.name),
    );
    (seen, id_calls)
}

#[pagetop::test]
async fn actions_are_dispatched_by_weight_then_registration_order() {
    start().await;
    assert_eq!(
        names(),
        ["a-early", "b-first-of-zero", "d-second-of-zero", "c-late"]
    );
}

#[pagetop::test]
async fn referer_actions_run_for_the_type_then_for_the_matching_id() {
    start().await;
    // First the ones for the type; then the ones filtered by id, each list sorted by weight.
    assert_eq!(node_names("x").0, ["general", "x-1", "x-2"]);
    assert_eq!(node_names("y").0, ["general", "y-1"]);
    // An id without actions only gets the ones for the type.
    assert_eq!(node_names("nobody").0, ["general"]);
}

#[pagetop::test]
async fn the_id_is_only_requested_if_some_action_is_filtered_by_it() {
    start().await;
    assert_eq!(node_names("x").1, 1);

    // `Plain` has no filtered action: there is no need to compute the id.
    let mut seen = Vec::new();
    let mut id_calls = 0;
    dispatch_referer(
        &mut Plain,
        |_| {
            id_calls += 1;
            None
        },
        |action: &Tagged, _| seen.push(action.name),
    );
    assert_eq!(seen, ["plain"]);
    assert_eq!(id_calls, 0);
}

#[pagetop::test]
async fn an_action_with_nothing_registered_dispatches_nothing() {
    start().await;
    let mut calls = 0;
    dispatch_actions(|_: &Unregistered| calls += 1);
    // Not even with a referer type that does not exist.
    let mut id_calls = 0;
    dispatch_referer(
        &mut String::new(),
        |_| {
            id_calls += 1;
            None
        },
        |_: &Tagged, _| calls += 1,
    );
    assert_eq!((calls, id_calls), (0, 0));
}

#[pagetop::test]
async fn try_dispatch_stops_when_asked() {
    start().await;
    let mut seen = Vec::new();
    try_dispatch_actions(|action: &Probe| {
        seen.push(action.name);
        if action.name == "b-first-of-zero" {
            std::ops::ControlFlow::Break(())
        } else {
            std::ops::ControlFlow::Continue(())
        }
    });
    assert_eq!(seen, ["a-early", "b-first-of-zero"]);
}

// Component actions: the ones for the type apply to all of them; the ones filtered by id only to
// the component with that id.
#[pagetop::test]
async fn component_actions_run_for_the_type_and_for_the_matching_id_only() {
    start().await;
    let mut cx = Context::default();
    let count = |log: &Mutex<Vec<String>>, id: &str| {
        log.lock()
            .unwrap()
            .iter()
            .filter(|logged| *logged == id)
            .count()
    };

    let mut other = Badge::labeled(Lc::n("other")).with_id("before-other");
    let _ = other.render(&mut cx).await;
    assert_eq!(count(&GENERAL_LOG, "before-other"), 1);
    assert_eq!(
        count(&ID_LOG, "before-other"),
        0,
        "other id: the filtered one is not applied"
    );

    let mut special = Badge::labeled(Lc::n("special")).with_id("special-before");
    let _ = special.render(&mut cx).await;
    assert_eq!(count(&GENERAL_LOG, "special-before"), 1);
    assert_eq!(
        count(&ID_LOG, "special-before"),
        1,
        "same id: the filtered one is applied"
    );

    // Without an id only the general one applies, and the filtered one is applied to no one else.
    let mut plain = Badge::labeled(Lc::n("plain"));
    let _ = plain.render(&mut cx).await;
    assert!(
        ID_LOG
            .lock()
            .unwrap()
            .iter()
            .all(|logged| logged == "special-before")
    );

    // Another component type gets nothing.
    let mut button = Button::plain(Lc::n("x")).with_id("before-button");
    let _ = button.render(&mut cx).await;
    assert_eq!(count(&GENERAL_LOG, "before-button"), 0);
}

// Each transformation receives the result of the previous one: first the ones for the type, then
// the ones for the id.
#[pagetop::test]
async fn markup_transformations_chain_in_order() {
    start().await;
    let mut cx = Context::default();

    // With another id only the general one applies.
    let mut other = Badge::labeled(Lc::n("chained")).with_id("chained-other");
    let html = other.render(&mut cx).await.into_string();
    assert!(html.starts_with("<div>"), "{html}");
    assert!(!html.contains("<section>"), "{html}");

    // With the id of the filtered action, after the general one.
    let mut special = Badge::labeled(Lc::n("chained")).with_id("special-markup");
    let html = special.render(&mut cx).await.into_string();
    assert!(html.starts_with("<section><div>"), "{html}");
    assert!(html.ends_with("</div></section>"), "{html}");

    // Each one was applied once and in that order (the log is shared with the other tests, which
    // run concurrently: it is filtered by id).
    let log = MARKUP_LOG.lock().unwrap();
    let applied = |id: &str| -> Vec<&'static str> {
        log.iter()
            .filter(|(logged, _)| logged == id)
            .map(|(_, which)| *which)
            .collect()
    };
    assert_eq!(applied("chained-other"), ["general"]);
    assert_eq!(applied("special-markup"), ["general", "by-id"]);
}
