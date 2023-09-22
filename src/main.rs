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

async fn process(mut stream: TcpStream) {
    println!("Accepted new connection");
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => {
                println!("Read {} bytes", n);
                parser::parse_resp(Vec::from(buffer));
                stream
                    .write_all(b"+PONG\r\n")
                    .await
                    .expect("Failed to write");
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
