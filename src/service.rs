use std::fmt::Debug;

use anyhow::anyhow;

use crate::{
    common::{HttpMethod, HttpResult},
    request::Request,
    response::Response,
};

pub trait Service: Debug + Send + Sync {
    fn get(&self, _request: &Request, response: &mut Response) -> HttpResult<()> {
        response.status_code = "405";
        response.status_text = "error";
        response.writer.write("Method Not Allowed")?;
        Ok(())
    }
    fn post(&self, _request: &Request, response: &mut Response) -> HttpResult<()> {
        response.status_code = "405";
        response.status_text = "error";
        response.writer.write("Method Not Allowed")?;
        Ok(())
    }
    fn service(
        &self,
        method: HttpMethod,
        request: &Request,
        response: &mut Response,
    ) -> HttpResult<()> {
        match method {
            HttpMethod::GET => self.get(request, response),
            HttpMethod::POST => self.post(request, response),
            other => Err(anyhow!("unknow method {:#?}", other)),
        }
    }
}
