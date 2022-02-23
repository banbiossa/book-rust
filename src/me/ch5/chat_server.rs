use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let server_addr = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    // start server
    let server = TcpListener::bind(server_addr).expect("Could not start server");
    server
        .set_nonblocking(true)
        .expect("Could not set nonblocking");
    println!("Server started on {}", server_addr);

    // main loop
    loop {
        // wait for client
        if let Ok((client, addr)) = server.accept() {
            println!("Client connected from {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }
        // wait for message
        if let Ok(msg) = rx.try_recv() {
            println!("send all! {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

// recieve thread for clients
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
// send message
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("Error sending message: {}", e);
            continue;
        }
        collector.push(socket); // keep socket
    }
    collector
}
