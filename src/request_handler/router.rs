use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

// link header to content
fn concat_bytes(header: Vec<u8>, body: Vec<u8>) -> Vec<u8>{
    [header, body].concat()
}

pub fn handle_route(method: &str,path: &str, mut stream: TcpStream) {
    println!("{} {}", method, path);

    let all_param:Vec<_> = path.split("/").collect();

    println!("{}", all_param[1]);

    //let file = fs::read(all_param[1]).expect("Error");
    
    let full_response = match fs::read(all_param[1]) {
        Ok(file) => {
            let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
            concat_bytes(response.to_vec(), file)
        },
        Err(_err) => {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes();
            concat_bytes(response.to_vec(), b"<h1>404<h1/>".to_vec())
        },
    };
    
    stream.write(&full_response).unwrap();
    stream.flush().unwrap();    
}
