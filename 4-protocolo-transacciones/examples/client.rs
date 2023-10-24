use tungstenite::{connect, Message};
use url::Url;
use std::io;

fn main() {
    let (mut socket, _) =
        connect(Url::parse("ws://localhost:3000").unwrap()).expect("Can't connect");

    println!("✅ Conexión establecida\n");

    loop {
        let mut client_msg = String::new();

        io::stdin()
            .read_line(&mut client_msg)
            .expect("Error al leer el id");

        client_msg = client_msg.trim().to_string();

        if client_msg.trim() == ".quit" {
            break;
        }

        socket.send(Message::Text(client_msg.into())).unwrap();

        let server_msg = socket.read().expect("Error al leer la respuesta");
        println!("Servidor > {}\n", server_msg);
    }
}