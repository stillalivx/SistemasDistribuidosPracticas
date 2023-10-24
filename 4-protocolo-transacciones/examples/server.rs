use std::{net::TcpListener, thread::spawn, fs};
use tungstenite::{accept, Message};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Book {
    id: u32,
    titulo: String,
    fecha_publicacion: String,
    autor: String,
    genero: String,
    precio: f32
}

fn main() {
    let file = fs::File::open("./books.json")
        .expect("Error al leer el archivo books.json");
    let books: Vec<Book> = serde_json::from_reader(file).unwrap();

    let server = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("🚀 Server running on port: 3000");

    for stream in server.incoming() {
        let books = books.clone();
    
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {    
                let msg = websocket.read().unwrap();

                println!("⚠️ Transacción iniciada");

                if !msg.is_text() {
                    websocket.send("❌ Comando invalido. Transaccion finalizada sin éxito".into()).unwrap();                    
                    continue;
                }

                let msg = msg.into_text().expect("El mensaje es invalido");

                if !msg.starts_with(".find") {
                    websocket.send("❌ Comando no especificado o incorrecto. Transaccion finalizada sin éxito".into()).unwrap();
                    continue;
                }

                
                let msg = msg.get(msg.find(" ").unwrap()..msg.len()).unwrap();
                
                let id: u32 = match msg.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        websocket.send("❌ ID invalido. Transaccion finalizada sin éxito.".into()).unwrap();                            
                        continue;
                    }
                };
                
                println!("✅ Datos validados");

                for book in &books {
                    if book.id == id {
                       websocket.send(Message::Text(serde_json::to_string(book).unwrap())).unwrap();
                       println!("✅ ID invalido. Transaccion finalizada con éxito.");

                       break;
                    }
                }
            }
        });
    }
}