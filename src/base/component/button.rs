//! Definiciones para crear botones ([`Button`]).

mod props;
pub use props::{Kind, Size, Style};

mod component;
pub use component::Button;

mod save_and_new;
pub use save_and_new::SaveAndNew;
