pub use tracing::{Level, event, span};
pub use tracing::{debug, error, info, trace, warn};

mod trace;
pub use trace::TRACING;
