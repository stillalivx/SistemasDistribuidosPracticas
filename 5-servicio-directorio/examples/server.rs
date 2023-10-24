use std::{net::TcpListener, thread::spawn, fs};
use std::fmt::format;
use tungstenite::{accept};

fn main() {
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("🚀 Server running on port: 3000");

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut path = String::from("root");

            let commands: [String; 4] = [
                ".create-dir".to_string(),
                ".create-file".to_string(),
                ".upload-file".to_string(),
                ".go".to_string()
            ];

            loop {
                let msg = websocket.read().unwrap();

                println!("⚠️ Transacción iniciada");

                if !msg.is_text() {
                    websocket.send("❌ Comando invalido. Transacción finalizada sin éxito".into()).unwrap();
                }

                let stmt = msg.into_text().expect("El mensaje es invalido");
                let stmt = stmt.split(" ").collect::<Vec<&str>>();

                if !commands.contains(&&stmt[0].to_string()) {
                    websocket.send("❌ Comando no especificado o incorrecto. Transacción finalizada sin éxito".into()).unwrap();
                }
                
                let param = stmt[1].trim();

                if stmt[0].starts_with(".create-dir") {
                    if param.is_empty() {
                        websocket.send("❌ Falta el parámetro para crear un directorio".into()).unwrap();
                    }
                    
                    match fs::read_dir(format!("{}/{}", &path, &param)) {
                        Ok(_) => {
                            websocket.send("❌ El directorio ya existe dentro de la ruta".into()).unwrap();
                        },
                        Err(_) => {
                            fs::create_dir(format!("{}/{}", &path, param)).expect("❌ Error al crear el directorio");
                            websocket.send("✅ Directorio creado".into()).unwrap();
                        }
                    };
                } else if stmt[0].starts_with(".create-file") {
                    if param.is_empty() {
                        websocket.send("❌ Falta el parámetro para crear un directorio".into()).unwrap();
                    }

                    match fs::read(format!("{}/{}", &path, param)) {
                        Ok(_) => {
                            websocket.send("❌ El archivo ya existe dentro de la ruta".into()).unwrap();
                        },
                        Err(_) => {
                            fs::write(format!("{}/{}", &path, &param), "").expect("❌ Error al crear el archivo");
                            websocket.send("✅ Archivo creado".into()).unwrap();
                        }
                    };
                } else if stmt[0].starts_with(".go") {
                    if param.eq("..") {
                        if path.eq("root") {
                            continue;
                        }

                        let mut dirs = path.split("/").collect::<Vec<&str>>();

                        if dirs.len() == 1 {
                            continue;
                        }

                        dirs.pop();

                        path = dirs.join("/");
                    } else {
                        path = format!("{}/{}", path, param).clone();
                    }

                    websocket.send(format!("✅ Directorio cambiado a: {}", &path).into()).unwrap();
                }
            }
        });
    }
}