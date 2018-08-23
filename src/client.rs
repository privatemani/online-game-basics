use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::{BufRead, BufReader};

pub fn client(x: i32, y: i32) ->  i32 {
if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
    let number_string = x.to_string();
    writeln!(stream, "{}", number_string).expect("failed to send message");

    let mut stream = BufReader::new(stream);
    let mut buffer: String = "1".to_string();
    stream.read_line(&mut buffer).expect("failed to read line");
    println!("Connected to the server!");
    let my_int = buffer.parse::<i32>().unwrap();
    println!("X value {}", my_int);
    my_int
} else {
    println!("Couldn't connect to server...");
    -1
}
}
