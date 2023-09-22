// Uncomment this block to pass the first stage
// use std::net::TcpListener;

mod parser;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    // let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(_stream) => {
    //             println!("accepted new connection");
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

fn process_command(parsed: RedisType) -> String {
    match parsed {
        RedisType::Array(elements) => match elements.first() {
            Some(RedisType::SimpleString(str)) | Some(RedisType::BulkString(str)) => {
                match str.to_uppercase().as_ref() {
                    "PING" => {
                        return String::from("+PONG\r\n");
                    }
                    "ECHO" => match elements.get(1) {
                        Some(RedisType::BulkString(str)) => {
                            return str.clone();
                        }
                        _ => panic!("Commands should be an array of bulk strings"),
                    },
                    _ => panic!("Unknown cmd"),
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
                        let res = process_command(parsed_input.redis_type);
                        stream
                            .write(res.as_bytes())
                            .await
                            .expect("failed to respond");
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
