use std::collections::HashMap;

use crate::{
    router::{self, Router},
    service::Service,
};

#[derive(Debug)]
pub struct Context {
    pub server_ip: String,
    pub server_port: String,
    router: Router,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub fn route(&self, path: &str) {
        if let Some(service) = self.router.get(path){
            
        }else{

        }
    }
}

#[derive(Debug)]
pub struct ContextBuilder {
    server_ip: String,
    server_port: String,
    router: Router,
}

impl ContextBuilder {
    fn new() -> Self {
        Self {
            server_ip: String::default(),
            server_port: String::default(),
            router: Router::default(),
        }
    }

    pub fn router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    pub fn server_ip(mut self, ip: &str) -> Self {
        self.server_ip = ip.to_string();
        self
    }

    pub fn server_port(mut self, port: &str) -> Self {
        self.server_port = port.to_string();
        self
    }

    pub fn build(self) -> Context {
        Context {
            server_ip: self.server_ip,
            server_port: self.server_port,
            router: self.router,
        }
    }
}

#[derive(Debug)]
pub struct DefaultService {}

impl Service for DefaultService {
    fn get(&self) {
        todo!()
    }

    fn post(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use log::debug;

    use crate::{init_logger, router::Router};

    use super::Context;

    #[test]
    fn test_build() {
        init_logger();

        let router = Router::default();
        let context = Context::builder()
            .router(router)
            .server_ip("127.0.0.1")
            .server_port("8080")
            .build();

        debug!("{:#?}", context);
    }
}
