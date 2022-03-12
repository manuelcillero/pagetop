pub use url::Url as DbUri;

pub use sea_orm::{
    DbErr,
    DatabaseConnection as DbConn,
};

pub mod entity {
    pub use sea_orm::entity::prelude::*;
}

pub mod migration {
    pub use sea_schema::migration::prelude::*;
}
