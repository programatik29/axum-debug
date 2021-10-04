//! This is a debugging crate that provides better error messages for [`axum`] framework.
//!
//! [`axum`] is a great framework for developing web applications. But when you make a mistake,
//! error messages can be really complex and long. It can take a long time for you to figure out
//! what is wrong in your code. This crate provides utilities to generate better error messages in
//! case you make a mistake.
//!
//! While using [`axum`], you can get long error messages for simple mistakes. For example:
//!
//! ```rust,compile_fail
//! use axum::{handler::get, Router};
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new().route("/", get(handler));
//!
//!     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//!
//! fn handler() -> &'static str {
//!     "Hello, world"
//! }
//! ```
//!
//! You will get a long error message about function not implementing [`Handler`] trait. But why
//! this function does not implement it? To figure it out [`debug_handler`] macro can be used.
//!
//! ```rust,compile_fail
//! # use axum::{handler::get, Router};
//! # use axum_debug::debug_handler;
//! #
//! # #[tokio::main]
//! # async fn main() {
//! #     let app = Router::new().route("/", get(handler));
//! #
//! #     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//! #         .serve(app.into_make_service())
//! #         .await
//! #         .unwrap();
//! # }
//! #
//! #[debug_handler]
//! fn handler() -> &'static str {
//!     "Hello, world"
//! }
//! ```
//!
//! ```text
//! error: handlers must be async functions
//!   --> main.rs:xx:1
//!    |
//! xx | fn handler() -> &'static str {
//!    | ^^
//! ```
//!
//! As the error message says, `handler` function needs to be async.
//!
//! ## Long Error Messages
//!
//! Sometimes error messages are long even if you are using [`debug_handler`]. To make them
//! shorter without missing anything important, you can use [`debug_router`].
//!
//! ```rust,compile_fail
//! use axum::{handler::get, Router};
//! use axum_debug::{debug_handler, debug_router};
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new().route("/", get(handler));
//!
//!     debug_router!(app);
//!
//!     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//!
//! #[debug_handler]
//! async fn handler() -> bool {
//!     false
//! }
//! ```
//!
//! ## Performance
//!
//! Macros in this crate have no effect when using release profile. (eg. `cargo build --release`)
//!
//! [`axum`]: axum
//! [`Handler`]: axum::handler::Handler
//! [`debug_handler`]: debug_handler
//! [`debug_router`]: debug_router

#[doc(hidden)]
pub use axum_debug_macros;

pub use crate::axum_debug_macros::{debug_handler, debug_router};

#[cfg(test)]
mod tests {
    use axum_debug_macros::debug_handler;

    #[debug_handler]
    async fn _empty() {}

    #[debug_handler]
    async fn _return() -> &'static str {
        ""
    }

    #[debug_handler]
    async fn _extractors(_a: String) {}

    #[debug_handler]
    async fn _extractors_return(_a: String) -> &'static str {
        ""
    }
}
