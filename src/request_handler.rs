use std::net::TcpStream;
use std::io::prelude::*;
use httparse;
mod router;

pub fn handle_connection(mut stream: TcpStream) {
    
    let mut header_buffer = [0; 1024];
    stream.read(&mut header_buffer).unwrap();
    
    let mut headers = [httparse::EMPTY_HEADER; 1024];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&header_buffer);

    match res {
        Err(_) => println!("fuck"),
        Ok(_) => println!("succesfully read"),
    }

    let _pheader = String::from_utf8_lossy(&header_buffer[..]);
    //let parsed_header = http_tools::parse(&header_buffer);
    //println!("{:?}", req);

    //Match the method and redirect to router 
    match req.method{
        Some("GET") =>  router::handle_route("GET", req.path.unwrap(), stream),
        Some("POST") => println!("{:?}", req),
        Some(_) => println!("unhandle"),
        None => println!("malformed request"),
    }


}