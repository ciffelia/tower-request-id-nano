//! A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
//! incoming request.
//!
//! This crate is a fork of [tower-request-id] crate that uses Nano ID instead of ULID.
//!
//! [tower]: https://crates.io/crates/tower
//! [hyper]: https://crates.io/crates/hyper
//! [axum]: https://crates.io/crates/axum
//! [warp]: https://crates.io/crates/warp
//! [tower-request-id]: https://crates.io/crates/tower-request-id

use std::{
    fmt,
    task::{Context, Poll},
};

use http::Request;
use nid::{alphabet::Base62Alphabet, Nanoid};
use tower_layer::Layer;
use tower_service::Service;

/// A newtype around [`nid::Nanoid<21, Base62Alphabet>`]
#[derive(Debug, Clone)]
pub struct RequestId(pub Nanoid<21, Base62Alphabet>);

impl RequestId {
    fn new() -> Self {
        Self(Nanoid::new())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct RequestIdService<S> {
    inner: S,
}

/// Middleware to use [`RequestId`]
impl<S> RequestIdService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<B, S> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let id = RequestId::new();
        req.extensions_mut().insert(id);
        self.inner.call(req)
    }
}

/// Layer to apply [`RequestIdService`] middleware.
#[derive(Clone, Debug)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}
