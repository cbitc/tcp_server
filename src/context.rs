use std::{fs, io::Read};

use anyhow::anyhow;

use crate::{
    common::HttpResult, request::Request, response::Response, router::Router, service::Service,
};

#[derive(Debug)]
pub struct Context {
    pub server_ip: String,
    pub server_port: String,
    router: Router,
    default_service: Box<dyn Service>,
    file_service: Box<dyn Service>,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub fn route(&self, path: &str) -> &Box<dyn Service> {
        if path.contains('.') {
            return &self.file_service;
        }
        if let Some(service) = self.router.get(path) {
            service
        } else {
            &self.default_service
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
            default_service: Box::new(DefaultService {}),
            file_service: Box::new(FileService::new()),
        }
    }
}

#[derive(Debug)]
pub struct DefaultService {}

impl Service for DefaultService {}

#[derive(Debug)]
pub struct FileService {}

impl FileService {
    pub fn new() -> Self {
        Self {}
    }

    fn get_real_path(&self, relative_path: &str) -> String {
        format!("./web/{}", relative_path)
    }
}

impl Service for FileService {
    fn get(&self, request: &Request, response: &mut Response) -> HttpResult<()> {
        if let [file_type, _] = request.path.rsplit('.').collect::<Vec<_>>()[..] {
            let mut file = fs::File::open(self.get_real_path(&request.path))?;
            let mut buffer = vec![];
            file.read_to_end(&mut buffer)?;

            match file_type {
                "html" => response.content_type = "text/html",
                "ico" => response.content_type = "image/ico",
                "png" => response.content_type = "image/png",
                other => {
                    return Err(anyhow!("unknow file type {}", other));
                }
            };
            response.writer.write(buffer)?;
            Ok(())
        } else {
            Err(anyhow!("can not parse the file path {}", request.path))
        }
    }
}

#[cfg(test)]
mod test {
    use log::debug;

    use crate::{init_logger, router::Router, service::Service};

    use super::Context;

    #[derive(Debug)]
    struct UserService {}

    impl Service for UserService {}

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

    #[test]
    fn test_route() {
        init_logger();

        let mut router = Router::default();
        router.insert(("/user/name", Box::new(UserService {})));

        let context = Context::builder()
            .router(router)
            .server_ip("127.0.0.1")
            .server_port("8080")
            .build();

        let service = context.route("/user/name");
        debug!("{:#?}", service);

        let service = context.route("/notexist");

        debug!("{:#?}", service);
    }
}
