use crate::core::component::{ComponentArc, Context};
use crate::html::{html, Markup};
use crate::Handle;

pub enum PackOp {
    Add(ComponentArc),
    AddAfterId(&'static str, ComponentArc),
    AddBeforeId(&'static str, ComponentArc),
    AddFirst(ComponentArc),
    RemoveById(&'static str),
    ReplaceById(&'static str, ComponentArc),
    Reset,
}

#[derive(Clone, Default)]
pub struct PackComponents(Vec<ComponentArc>);

impl PackComponents {
    pub fn new() -> Self {
        PackComponents::default()
    }

    pub fn with(arc: ComponentArc) -> Self {
        let mut pack = PackComponents::new();
        pack.alter(PackOp::Add(arc));
        pack
    }

    pub(crate) fn merge(packs: &[Option<&PackComponents>]) -> Self {
        let mut pack = PackComponents::default();
        for p in packs.iter().flatten() {
            pack.0.append(&mut p.0.clone());
        }
        pack
    }

    // PackComponents BUILDER.

    pub fn alter(&mut self, op: PackOp) -> &mut Self {
        match op {
            PackOp::Add(arc) => self.0.push(arc),
            PackOp::AddAfterId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, arc),
                    _ => self.0.push(arc),
                }
            }
            PackOp::AddBeforeId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, arc),
                    _ => self.0.insert(0, arc),
                }
            }
            PackOp::AddFirst(arc) => self.0.insert(0, arc),
            PackOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            PackOp::ReplaceById(id, arc) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = arc;
                        break;
                    }
                }
            }
            PackOp::Reset => self.0.clear(),
        }
        self
    }

    // PackComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ComponentArc> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ComponentArc> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ComponentArc> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // PackComponents PREPARE.

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
