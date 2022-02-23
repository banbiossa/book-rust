use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let server_addr = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(server_addr).unwrap();
    socket
        .set_nonblocking(true)
        .expect("Socket cant bind to address");
    println!("Connected to server: {}", server_addr);
    start_thread(socket.try_clone().unwrap());
    // get input from stdin
    let user = input("Your name please: ");
    println!("Hi {}, enter your message", user);
    loop {
        let msg = input("");
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        // get message from server
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("[recieved] {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

// get input from stdin
fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Stdin read error");
    String::from(buf.trim())
}
