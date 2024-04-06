# tower-request-id-nano

A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
incoming request.

This crate is a fork of [tower-request-id] crate that uses Nano ID instead of ULID.

[tower]: https://crates.io/crates/tower
[hyper]: https://crates.io/crates/hyper
[axum]: https://crates.io/crates/axum
[warp]: https://crates.io/crates/warp
[tower-request-id]: https://crates.io/crates/tower-request-id

## License

This project is licensed under the [MIT license](LICENSE).
