mod routes;

use crate::routes::Router;
use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn redirect(req: Request) -> Result<Response> {
    let router = Router::default()?;
    router.redirect(req)
}
