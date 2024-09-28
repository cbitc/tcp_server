use std::{fmt::Write as fmtWrite, io::Write};

#[derive(Debug)]
pub struct Response {
    pub status_code: &'static str,
    pub status_text: &'static str,
    pub content_type: &'static str,
    pub charset: &'static str,
    pub writer: ResponseWriter,
}

impl Response {
    pub fn new(
        status_code: &'static str,
        status_text: &'static str,
        content_type: &'static str,
        charset: &'static str,
    ) -> Self {
        Self {
            status_code,
            status_text,
            content_type,
            charset,
            writer: ResponseWriter::default(),
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: "200",
            status_text: "ok",
            content_type: "text/html",
            charset: "utf-8",
            writer: ResponseWriter::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ResponseWriter {
    buffer: Vec<u8>,
}

impl ResponseWriter {
    pub fn write(&mut self, content: impl AsRef<[u8]>) {
        self.buffer.write_all(content.as_ref()).unwrap();
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}
