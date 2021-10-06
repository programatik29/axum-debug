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

#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::mem_forget,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

use bytes::Bytes;
use http::{Request, Response};
use http_body::Body;
use tower_service::Service;

#[doc(hidden)]
pub use axum_debug_macros;

pub use crate::axum_debug_macros::{debug_handler, debug_router};

/// Checks if provided service can be used with [`Router`].
///
/// This function is useful when debugging a [`Service`].
///
/// # Example
/// ```rust,compile_fail
/// use axum::{handler::get, Router};
/// use axum_debug::{debug_handler, debug_router, check_service};
/// use tower::util::BoxService;
///
/// #[tokio::main]
/// async fn main() {
///     let service = BoxService::new(get(handler));
///
///     check_service(&service);
///
///     let app = Router::new().route("/", service);
///
///     debug_router!(app);
///
///     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
///         .serve(app.into_make_service())
///         .await
///         .unwrap();
/// }
///
/// #[debug_handler]
/// async fn handler() -> &'static str {
///     "Hello, world!"
/// }
/// ```
///
/// ```text
/// error[E0277]: the trait bound `BoxService<Request<_>, Response<...>, Infallible>: Clone` is not satisfied
///    --> main.rs:9:19
///     |
/// 9   |     check_service(&service);
///     |                   ^^^^^^^^ the trait `Clone` is not implemented for `BoxService<Request<_>, Response<...>, Infallible>`
/// ```
///
/// [`Router`]: axum::Router
/// [`Service`]: tower_service::Service
pub fn check_service<S, ReqBody, ResBody>(_service: &S)
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + Sync + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
    S::Future: Send,
    ReqBody: Send + 'static,
    ResBody: Body<Data = Bytes> + Send + Sync + 'static,
    ResBody::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
}

/// Checks and returns if provided service can be used with [`Router`].
///
/// This function is useful when debugging a [`Service`].
///
/// # Example
/// ```rust,compile_fail
/// use axum::{handler::get, Router};
/// use axum_debug::{debug_handler, debug_router, debug_service};
/// use tower::util::BoxService;
///
/// #[tokio::main]
/// async fn main() {
///     let service = BoxService::new(get(handler));
///
///     let app = Router::new().route("/", debug_service(service));
///
///     debug_router!(app);
///
///     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
///         .serve(app.into_make_service())
///         .await
///         .unwrap();
/// }
///
/// #[debug_handler]
/// async fn handler() -> &'static str {
///     "Hello, world!"
/// }
/// ```
///
/// ```text
/// error[E0277]: the trait bound `BoxService<Request<_>, Response<...>, Infallible>: Clone` is not satisfied
///    --> main.rs:9:54
///     |
/// 9   |     let app = Router::new().route("/", debug_service(service));
///     |                                                      ^^^^^^^ the trait `Clone` is not implemented for
///                                                                    `BoxService<Request<_>, Response<...>, Infallible>`
/// ```
///
/// [`Router`]: axum::Router
/// [`Service`]: tower_service::Service
pub fn debug_service<S, ReqBody, ResBody>(service: S) -> S
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + Sync + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>> + Send,
    S::Future: Send,
    ReqBody: Send + 'static,
    ResBody: Body<Data = Bytes> + Send + Sync + 'static,
    ResBody::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    check_service(&service);

    service
}

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
