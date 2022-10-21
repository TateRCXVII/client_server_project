use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8080");
            stream.write(b"Hello World").unwrap();
            println!("Sent Hello World, awaiting reply...");
            let mut data = [0 as u8; 10];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    println!("Reply: {}", from_utf8(&data).unwrap());
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminating.");
}
