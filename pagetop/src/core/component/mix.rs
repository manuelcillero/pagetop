use crate::core::component::{ComponentArc, Context};
use crate::html::{html, Markup};
use crate::Handle;

pub enum MixOp {
    Add(ComponentArc),
    AddAfterId(&'static str, ComponentArc),
    AddBeforeId(&'static str, ComponentArc),
    AddFirst(ComponentArc),
    RemoveById(&'static str),
    ReplaceById(&'static str, ComponentArc),
    Reset,
}

#[derive(Clone, Default)]
pub struct MixComponents(Vec<ComponentArc>);

impl MixComponents {
    pub fn new() -> Self {
        MixComponents::default()
    }

    pub fn with(arc: ComponentArc) -> Self {
        let mut components = MixComponents::new();
        components.alter(MixOp::Add(arc));
        components
    }

    pub(crate) fn merge(mixes: &[Option<&MixComponents>]) -> Self {
        let mut components = MixComponents::default();
        for m in mixes.iter().flatten() {
            components.0.append(&mut m.0.clone());
        }
        components
    }

    // MixComponents BUILDER.

    pub fn alter(&mut self, op: MixOp) -> &mut Self {
        match op {
            MixOp::Add(arc) => self.0.push(arc),
            MixOp::AddAfterId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, arc),
                    _ => self.0.push(arc),
                }
            }
            MixOp::AddBeforeId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, arc),
                    _ => self.0.insert(0, arc),
                }
            }
            MixOp::AddFirst(arc) => self.0.insert(0, arc),
            MixOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            MixOp::ReplaceById(id, arc) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = arc;
                        break;
                    }
                }
            }
            MixOp::Reset => self.0.clear(),
        }
        self
    }

    // MixComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ComponentArc> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ComponentArc> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ComponentArc> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // MixComponents PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                " " (c.prepare(cx)) " "
            }
        }
    }
}
