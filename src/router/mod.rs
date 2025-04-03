use std::collections::HashMap;

use crate::{
    models::method::Method,
    server::{MethodHandlerMap, RequestHandler},
};

pub struct Router {
    pub base: String,
    pub routes: HashMap<String, MethodHandlerMap>,
    pub sub_routers: Vec<Router>,
}

impl Router {
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            routes: HashMap::new(),
            sub_routers: Vec::new(),
        }
    }

    pub fn get(mut self, path: &str, handler: RequestHandler) -> Self {
        self.add(Method::Get, path, handler);

        self
    }

    pub fn post(mut self, path: &str, handler: RequestHandler) -> Self {
        self.add(Method::Post, path, handler);

        self
    }

    pub fn put(mut self, path: &str, handler: RequestHandler) -> Self {
        self.add(Method::Put, path, handler);

        self
    }

    pub fn patch(mut self, path: &str, handler: RequestHandler) -> Self {
        self.add(Method::Patch, path, handler);

        self
    }

    pub fn delete(mut self, path: &str, handler: RequestHandler) -> Self {
        self.add(Method::Delete, path, handler);

        self
    }

    pub fn add(&mut self, method: Method, path: &str, handler: RequestHandler) {
        let full_path = format!(
            "{}{}",
            self.base.trim_end_matches("/"),
            path.trim_start_matches("/")
        );

        let entry = self.routes.entry(full_path).or_default();
        entry.insert(method, handler);
    }

    pub fn into_routes(self) -> HashMap<String, MethodHandlerMap> {
        self.routes
    }
}
