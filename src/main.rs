use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::Arc,
};

use log::{info, trace};
use tcp_server::{
    context::Context, pool::ThreadPool, request::Request, response::Response, router::Router,
};

fn init_logger() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let router = Router::default();
    let context = Arc::new(
        Context::builder()
            .server_ip("127.0.0.1")
            .server_port("8080")
            .router(router)
            .build(),
    );

    start_server(context)?;

    Ok(())
}

fn start_server(context: Arc<Context>) -> Result<(), Box<dyn std::error::Error>> {
    let host = format!("{}:{}", context.server_ip, context.server_port);
    let server = std::net::TcpListener::bind(&host)?;
    info!("服务器启动");
    let pool = ThreadPool::new(8);

    info!("正在等待请求");

    for stream in server.incoming() {
        let stream = stream?;
        let context = Arc::clone(&context);
        pool.execute(move || handle_connection(stream, context));
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, context: Arc<Context>) {
    let reader = BufReader::new(&mut stream);
    let lines: Vec<_> = reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = Request::from_content(&lines).unwrap();
    trace!("{:#?}", request);
    let service = context.route(&request.path);
    trace!("{:#?}", service);
    let mut response = Response::default();
    service.service(request.method, &request, &mut response);
    trace!("{:#?}", response);

    print_response(&stream, &request, &response);
}

fn print_response(mut stream: &TcpStream, request: &Request, response: &Response) {
    let line = format!(
        "{} {} {}\r\n",
        request.version, response.status_code, response.status_text
    );
    stream.write_all(line.as_bytes()).unwrap();

    let head = format!(
        "Content-Type: {}; charset={}\r\nContent-Length: {}\r\n\r\n",
        response.content_type,
        response.charset,
        response.writer.get_buffer().len()
    );
    stream.write_all(head.as_bytes()).unwrap();

    stream.write_all(response.writer.get_buffer()).unwrap();
}
