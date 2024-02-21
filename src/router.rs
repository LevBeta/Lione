use crate::http::{methods::Method, request::Request, response::Response};

use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Route {
    path: String,
    methods: Vec<Method>,
}

impl From<&Request> for Route {
    fn from(r: &Request) -> Self {
        Self {
            path: r.path.clone(),
            methods: vec![r.method],
        }
    }
}

#[derive(Clone)]
pub struct Router {
    pub routes: HashMap<Route, Arc<dyn Fn(Request) -> Response + Send + Sync>>,
}

impl Router {
    pub(crate) fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub(crate) fn match_route(
        &mut self,
        methods: Vec<Method>,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.routes.insert(
            Route {
                methods,
                path: path.to_string(),
            },
            Arc::new(handler),
        );

        self
    }

    pub fn get(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
    ) -> &mut Self {
        self.match_route(vec![Method::Get], path, handler)
    }

    pub fn post(
        &mut self, 
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static
    ) -> &mut Self {
        self.match_route(vec![Method::Post], path, handler)
    }

    pub fn delete(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static
    ) -> &mut Self {
        self.match_route(vec![Method::Delete], path, handler)
    }

    pub fn put(
        &mut self,
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static 
    ) -> &mut Self {
        self.match_route(vec![Method::Put], path, handler)
    }

    pub(crate) fn handle(&self, request: Request) -> Response {
        let route = Route::from(&request);
        match self.routes.get(&route) {
            Some(handler) => handler.clone()(request),
            None => Response::not_found(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn test_router_get() {
    //   let mut router = Router::new();

    //    router.get("/path", |_| Response::new().text("HI"));
    //    assert_eq!(route.path, "path");
    //}
}
