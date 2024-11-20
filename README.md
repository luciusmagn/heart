# Heart

A Rust-based web stack combining powerful libraries for building modern, efficient web applications.

## Components

- [Warp](https://github.com/seanmonstar/warp): High-performance HTTP server with filter-based routing
- [Maud](https://github.com/lambda-fairy/maud): Compile-time HTML templating
- [HTMX](https://htmx.org): Modern web interactions without complex JavaScript
- [TiKV](https://github.com/tikv/tikv): Distributed, strongly consistent key-value store

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
heart = "0.1.16"
```

## Example

```rust
use tokio;
use heart::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let route = warp::path("hello")
        .map(|| {
            let markup = html! {
                h1 { "Hello, Heart Stack!" }
                p { "This is a tiny example." }
            };

            info!("Serving hello route");

            Response::builder()
                .header("Content-Type", "text/html")
                .body(markup.into_string())
        });

    info!("Starting server on http://localhost:3030");
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
    Ok(())
}
```

## Contributions

Contributions are welcome! Please follow Kant's categorical imperative.

## License

```txt
Fair License (2024)

Copyright (c) 2024 Lukáš Hozda

Usage of the software is PERMITTED, modification and redistribution are PERMITTED, 
however all versions of the derived code MUST be clearly marked as such 
and NOT represented as the original software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
```
