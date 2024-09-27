use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use log::{error, info};
use tcp_server::{pool::ThreadPool, request::Request};

fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let server = std::net::TcpListener::bind("127.0.0.1:8080")?;
    info!("服务器启动");
    let pool = ThreadPool::new(8);

    info!("正在等待请求");
    for stream in server.incoming() {
        let stream = stream?;
        pool.execute(|| handle_connection(stream));
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let lines: Vec<_> = reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let request = Request::from_content(&lines);
}
