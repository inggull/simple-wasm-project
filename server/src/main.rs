mod http_message;

use http_message::{RequestMessage, ResponseMessage};
use std::{
    fs,
    io::{self, Read, Write},
    net,
};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();

    for (id, stream) in listener.incoming().take(15).enumerate() {
        println!("[INFO] Connected to client {}", id);
        let stream = stream.unwrap();
        request_handle(stream, id);
    }
}

fn request_handle(mut stream: net::TcpStream, id: usize) -> Result<(), io::Error> {
    let mut buf = [0u8; 8192];
    let mut read_len;
    loop {
        read_len = stream.read(&mut buf)?;
        if read_len == 0 {
            println!("[INFO] Disconnected from client {}", id);
            return Ok(());
        }
        let request_message = RequestMessage::new(buf[0..read_len].to_vec());
        let mut response_message = ResponseMessage::new();

        if request_message.is_none() {
            println!("[ERROR] Failed to read request line");
            response_message.set_version("HTTP/1.1");
            response_message.set_status_code("400");
            response_message.set_phrase("Bad Request");
            response_message.insert_header_line("Content-Length", &BAD_REQUEST.len().to_string());
            response_message.set_entity_body(BAD_REQUEST.to_owned().into_bytes());
        } else {
            let request_message = request_message.unwrap();
            println!("[Request Message]\n{}", request_message.to_string());
            match fs::read(format!(
                ".{}",
                request_message
                    .get_url()
                    .unwrap_or("/index.html".to_owned())
            )) {
                Ok(text) => {
                    response_message.set_version("HTTP/1.1");
                    response_message.set_status_code("200");
                    response_message.set_phrase("OK");
                    response_message.insert_header_line("Content-Length", &text.len().to_string());
                    response_message.set_entity_body(text);
                }
                Err(_) => {
                    response_message.set_version("HTTP/1.1");
                    response_message.set_status_code("404");
                    response_message.set_phrase("Not Found");
                    response_message
                        .insert_header_line("Content-Length", &NOT_FOUND.len().to_string());
                    response_message.set_entity_body(NOT_FOUND.to_owned().into_bytes());
                }
            };
        }
        println!("[Response Message]\n{}", response_message.to_string());

        stream
            .write_all(&response_message.to_vec())
            .expect("[ERROR] Failed to write response message");
    }
}

const BAD_REQUEST: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>400 Bad Request</title>
</head>
<body>
<h1>Oops!</h1>
<p>Sorry, I don't know what you're asking for.</p>
</body>
</html>
"#;

const NOT_FOUND: &'static str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>404 Not Found</title>
</head>
<body>
<h1>Oops!</h1>
<p>Sorry, I don't know what you're asking for.</p>
</body>
</html>
"#;
