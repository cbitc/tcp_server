#[derive(Debug,Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

pub type HttpResult = anyhow::Result<()>;
