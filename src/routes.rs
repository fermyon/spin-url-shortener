use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{not_found, Request, Response};
use std::fs;

const DEFAULT_ROUTER: &str = "router.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub source: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Router {
    #[serde(rename = "route")]
    pub routes: Vec<Route>,
}

impl Router {
    pub fn default() -> Result<Self> {
        Ok(toml::from_slice(&fs::read(DEFAULT_ROUTER)?)?)
    }

    pub fn redirect(self, req: Request) -> Result<Response> {
        let path_info = req
            .headers()
            .get("PATH_INFO")
            .expect("cannot get path info from request headers");
        let route = match self.path(path_info.to_str()?) {
            Some(r) => r,
            None => return not_found(),
        };
        let res = http::Response::builder()
            .status(http::StatusCode::PERMANENT_REDIRECT)
            .header(http::header::LOCATION, route.destination)
            .body(None)?;
        Ok(res)
    }

    fn path(self, path: &str) -> Option<Route> {
        self.routes.into_iter().find(|r| r.source == path)
    }
}
