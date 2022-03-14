pub mod admin;
pub mod homepage;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
pub mod user;
