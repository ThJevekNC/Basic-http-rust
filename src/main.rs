use std::net::TcpListener;
//use std::thread::spawn as new_thread;
mod request_handler;
use std::env;
use std::path::Path;


fn main () -> std::io::Result<()>{
    println!("Creating socket");
    let listen = TcpListener::bind("0.0.0.0:80")
                            .expect("Could not create socket!");
    let root = Path::new("./../www");
    assert!(env::set_current_dir(&root).is_ok());
    println!("Successfully changed working directory to {}!", root.display());

    for stream in listen.incoming(){
        //Spawning the thread to handle incoming connection
        std::thread::spawn(move || {
            let stream = stream.unwrap();
            request_handler::handle_connection(stream);       
        });
        println!("lol");
    }
    Ok(())
}

