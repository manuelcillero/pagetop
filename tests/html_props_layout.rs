use pagetop::prelude::*;

// El posicionamiento y el espaciado se guardan aparte para que `Props` (y con él cada componente)
// sea pequeño; esta prueba avisa si algo vuelve a inflarlo.
#[pagetop::test]
async fn props_stays_small() {
    assert!(
        std::mem::size_of::<Props>() <= 192,
        "Props ocupa {} bytes",
        std::mem::size_of::<Props>()
    );
}

#[pagetop::test]
async fn is_empty_tracks_layout_values() {
    let props = Props::default();
    assert!(props.is_empty());

    let with_margin = props
        .clone()
        .with_prop(Margin::new().with_top(UnitValue::Zero));
    assert!(!with_margin.is_empty());
    let with_padding = Props::default().with_prop(Padding::new().with_x(UnitValue::Px(4)));
    assert!(!with_padding.is_empty());

    // Un valor por defecto no cuenta como definido.
    assert!(Props::default().with_prop(Margin::new()).is_empty());
    assert!(Props::default().with_prop(Padding::new()).is_empty());
}

// Un clon comparte el posicionamiento hasta que se modifica; entonces cada uno tiene el suyo.
#[pagetop::test]
async fn a_clone_does_not_share_later_changes() {
    let original = Props::default().with_prop(Margin::new().with_top(UnitValue::RelRem(1.0)));
    let mut copy = original.clone();
    copy.alter_prop(Margin::new().with_top(UnitValue::RelRem(2.0)));
    copy.alter_prop(Padding::new().with_start(UnitValue::Px(3)));

    let mut cx = Context::default();
    let html_original = html! { span (original.unpack(&mut cx)) {} }.into_string();
    let html_copy = html! { span (copy.unpack(&mut cx)) {} }.into_string();

    assert!(
        html_original.contains("_margin-top_1rem_"),
        "{html_original}"
    );
    assert!(!html_original.contains("padding"), "{html_original}");
    assert!(html_copy.contains("_margin-top_2rem_"), "{html_copy}");
    assert!(html_copy.contains("_padding-start_3px_"), "{html_copy}");
}

// Todos los tipos de posicionamiento conviven y se emiten en el mismo orden de siempre.
#[pagetop::test]
async fn layout_kinds_combine_in_the_usual_order() {
    let props = Props::default()
        .with_prop(Padding::new().with_top(UnitValue::Zero))
        .with_prop(Margin::new().with_top(UnitValue::Zero));
    let mut cx = Context::default();
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let margin = html.find("_margin-top_").expect("margin");
    let padding = html.find("_padding-top_").expect("padding");
    assert!(margin < padding, "{html}");
}
