[![License](https://img.shields.io/crates/l/axum-debug)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/axum-debug)](https://crates.io/crates/axum-debug)
[![Docs - Master](https://img.shields.io/badge/docs-master-blue)](https://programatik29.github.io/axum-debug/docs/axum_debug/)
[![Docs - Stable](https://img.shields.io/crates/v/axum-debug?color=blue&label=docs)](https://docs.rs/axum-debug/)

# axum-debug

This is a debugging crate that provides better error messages for [`axum`] framework.

[`axum`] is a great framework for developing web applications. But when you make a mistake,
error messages can be really complex and long. It can take a long time for you to figure out
what is wrong in your code. This crate provides utilities to generate better error messages in
case you make a mistake.

## Usage example

Will fail with a better error message:

```rust
use axum::{handler::get, Router};
use axum_debug::{debug_handler, debug_router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    debug_router!(app);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn handler() -> bool {
    false
}
```

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Performance

Macros in this crate have no effect when using release profile. (eg. `cargo build --release`)

## License

This project is licensed under the [MIT license](LICENSE).

[`axum`]: https://crates.io/crates/axum
