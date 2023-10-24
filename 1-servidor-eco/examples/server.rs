use std::{net::TcpListener, thread::spawn};
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("🚀 Eco server running on port: 3000");

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read().unwrap();

                println!("Mensaje recibido: {msg}");

                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}