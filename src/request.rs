use crate::common::HttpMethod;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn from_content<T: AsRef<str>>(content: &[T]) -> Result<Request, String> {
        let result = if let [method, path, version] = *(content.first().unwrap())
            .as_ref()
            .split(' ')
            .collect::<Vec<_>>()
            .as_slice()
        {
            let method = match method {
                "GET" => HttpMethod::GET,
                "POST" => HttpMethod::POST,
                "PUT" => HttpMethod::PUT,
                "PATCH" => HttpMethod::PATCH,
                "DELETE" => HttpMethod::DELETE,
                other => return Err(format!("not supported method {other}")),
            };
            let request = Request {
                method,
                path: path.to_string(),
                version: version.to_string(),
            };
            Ok(request)
        } else {
            Err("status line parse error!".to_string())
        };

        result
    }
}

/*
"GET /favicon.ico HTTP/1.1",
    "Host: 127.0.0.1:8080",
    "Connection: keep-alive",
    "sec-ch-ua: \"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Google Chrome\";v=\"128\"",
    "sec-ch-ua-mobile: ?0",
    "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36",
    "sec-ch-ua-platform: \"Windows\"",
    "Accept: image/avif,image/webp,image/apng,image/svg+xml,image/*,*/
*;q=0.8",
    "Sec-Fetch-Site: same-origin",
    "Sec-Fetch-Mode: no-cors",
    "Sec-Fetch-Dest: image",
"Referer: http: //127.0.0.1:8080/",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: zh-CN,zh;q=0.9",
*/


#[cfg(test)]
mod test {
    use log::{debug, info};

    use crate::init_logger;

    use super::Request;

    #[test]
    fn test_request() {
        init_logger();
        let content = vec!["GET /favicon.ico HTTP/1.1"];
        let request = Request::from_content(&content);
        debug!("{:#?}", request);
    }
}
