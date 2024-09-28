use std::fmt::Debug;

use crate::{common::HttpMethod, request::Request, response::Response};

pub trait Service: Debug + Send + Sync {
    fn get(&self, request: &Request, response: &mut Response) {
        response.status_code = "405";
        response.status_text = "error";
        response.writer.write("Method Not Allowed");
    }
    fn post(&self, request: &Request, response: &mut Response) {
        response.status_code = "405";
        response.status_text = "error";
        response.writer.write("Method Not Allowed");
    }
    fn service(&self, method: HttpMethod, request: &Request, response: &mut Response) {
        match method {
            HttpMethod::GET => self.get(request, response),
            HttpMethod::POST => self.post(request, response),
            _ => panic!(),
        }
    }
}
