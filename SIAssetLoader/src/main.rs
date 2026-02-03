use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};

fn handle_connection(mut stream: TcpStream){
    // placeholder, will handle soon
}

fn main()->std::io::Result<()> {
    let listener=TcpListener::bind("localhost:3000")?.expect("Could not bind to port 3000");
    println!("Server listening on port 3000");
    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                handle_connection(stream);
            }
            Err(e)=>{
                eprintln!("Connection failed: {}",e);
            }
        }
    }
    Ok(());
}