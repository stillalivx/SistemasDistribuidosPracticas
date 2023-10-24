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

                if !msg.is_text() {
                    return
                }

                let msg = msg.into_text().expect("El mensaje es invalido");
                let mut response = String::new();

                if msg == "saludar" {
                    response = String::from("Hola mundo");
                } else if msg == "practica" {
                    response = String::from("Esta es una practica de la unidad 1");
                } else {
                    response = String::from("Error: El comando enviado es invalido");
                }

                websocket.send(response.into()).unwrap();
            }
        });
    }
}