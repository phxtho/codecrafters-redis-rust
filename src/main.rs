use async_recursion::async_recursion;
use parser::{parse_resp, RedisType};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

mod parser;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("failed bind to port");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move { handle_connection(stream).await });
            }
            Err(e) => {
                println!("Listener Error: {}", e);
            }
        }
    }
}

#[async_recursion]
async fn process_command(parsed: RedisType, stream: &mut TcpStream) {
    match parsed {
        RedisType::Array(elements) => match elements.first() {
            Some(RedisType::SimpleString(str)) | Some(RedisType::BulkString(str)) => {
                match str.to_uppercase().as_ref() {
                    "PING" => {
                        stream.write(b"+PONG\r\n").await.unwrap();
                    }
                    "ECHO" => match elements.get(1) {
                        Some(RedisType::BulkString(str)) => {
                            stream
                                .write(format!("${}\r\n{}\r\n", str.len(), str).as_bytes())
                                .await
                                .unwrap();
                        }
                        _ => panic!("Echo should contain bulk strings"),
                    },
                    _ => panic!("Unknown command {}", str),
                }
            }
            _ => panic!("Commands should be an array of bulk strings"),
        },
        _ => panic!("Commands should be an array of bulk strings"),
    }
}

async fn handle_connection(mut stream: TcpStream) {
    println!("Accepted new connection");
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(_) => {
                match parser::parse_resp(&buffer) {
                    Ok(parsed_input) => {
                        process_command(parsed_input.redis_type, &mut stream).await;
                    }
                    Err(_) => {
                        stream.write(b"Error\r\n").await.expect("failed to respond");
                        break;
                    }
                }
                buffer = [0; 1024] // reset buffer
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Stream Error: {}", e);
                break;
            }
        }
    }
}
