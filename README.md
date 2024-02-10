# axum-test-helpers

`axum-test-helpers` exposes [`axum`] original TestClient, which is private to the [`axum`] crate

More information about this crate can be found in the [crate documentation][docs].

## High level features

- Provide an easy to use interface
- Start a server in a different port for each call
- Deal with JSON, text and files response/requests

## Usage example

Add this crate as a dev-dependency:

```
[dev-dependencies]
axum-test-helpers = "0.7.4"
```

Use the TestClient on your own Router:

```rust
use axum::Router;
use axum::http::StatusCode;
use axum_test_helper::TestClient;

// you can replace this Router with your own app
let app = Router::new().route("/", get(|| async {}));

// initiate the TestClient with the previous declared Router
let client = TestClient::new(app);
let res = client.get("/").await;
assert_eq!(res.status(), StatusCode::OK);
```

You can find examples like this in
the [example directory][examples].

See the [crate documentation][docs] for way more examples.

## Disable trace

By default axum-test-helpers print trace like `Listening on 127.0.0.1:36457`. You can disable trace with `axum-test-helpers = { version = "0.*", default-features = false, features = ["withouttrace"] }`.

## Contributing

Before submitting a pull request or after pulling from the main repository, ensure all tests pass:

```shell
# Run axum-test-helpers tests
cargo test

# Test the hello-world example project
(cd examples/hello-world && cargo test)
```

## License

This project is licensed under the [GPL-3.0-only][license].

[`axum`]: https://github.com/tokio-rs/axum/blob/62324aad912f17059c0952bea5989d27f05a96b3/axum/src/test_helpers/test_client.rs
[examples]: https://github.com/joseburgosguntin/axum-test-helpers/tree/main/examples/
[docs]: https://docs.rs/axum-test-helpers
[license]: https://github.com/joseburgosguntin/axum-test-helpers/blob/main/LICENSE
