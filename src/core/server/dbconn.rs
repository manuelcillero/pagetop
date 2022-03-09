use crate::Lazy;
use crate::database::DatabaseConnection;

use std::sync::RwLock;

pub static DBCONN: Lazy<RwLock<Option<DatabaseConnection>>> = Lazy::new(|| {
    RwLock::new(None)
});
