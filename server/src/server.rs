use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{HttpError, HttpRequest, HttpResponse};

pub type HandleHttp = fn(&HttpRequest) -> Result<HttpResponse, HttpError>;

//启动Http服务
pub fn start_http_server(address: &str, port: u16, handle_http: HandleHttp) {
    let listener = TcpListener::bind(format!("{}:{}", address, port));
    match listener {
        Ok(listener) => {
            println!("Http Server Started at {}:{}", address, port);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut tcp_stream) => {
                        let handle_http = handle_http.clone();
                        // 多线程并发处理，可以同时处理多个http请求
                        thread::spawn(move || {
                            handle_stream_connect_default(&mut tcp_stream, handle_http);
                        });
                    }
                    Err(e) => {
                        eprintln!("Accept new connection error:{}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Start Http Server Error:{}", e);
        }
    }
}

fn handle_stream_connect_default(tcp_stream: &mut TcpStream, handle_http: HandleHttp) {
    let result = handle_stream_connect(tcp_stream, handle_http);
    if result.is_err() {
        eprintln!("handle_stream_connect error:{}", result.err().unwrap());
    }
}

// 处理一个http连接请求
fn handle_stream_connect(
    tcp_stream: &mut TcpStream,
    handle_http: HandleHttp,
) -> Result<(), HttpError> {
    println!(
        "Accepted a new connection from {}.",
        tcp_stream.peer_addr()?
    );
    // 读取整个http请求内容放入buf
    let mut buf = [0; 1024];
    tcp_stream.read(&mut buf)?;

    //查找空字节，返回索引值
    let null_index = buf.iter().position(|&c| c == b'\0').unwrap_or(buf.len());
    //将字节转成字符串
    let raw_string: String = String::from_utf8(buf[0..null_index].to_vec())?;
    println!("请求内容: {}", raw_string);
    //将请求内容解析成HttpRequest对象
    let request = HttpRequest::try_from(raw_string)?;
    //不同的uri，返回不同的响应
    let response = handle_http(&request)?;
    //处理返回信息
    HttpResponse::response(response, tcp_stream)?;
    Ok(())
}
