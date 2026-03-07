use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

type KVStore = Arc<Mutex<HashMap<String, String>>>;

fn handle_client(stream: TcpStream, kv_store: KVStore) -> std::io::Result<()> {
    println!("Accepted connection from: {}", stream.peer_addr()?);

    // Create a second handle to the same connection
    let mut writer = stream.try_clone()?;

    // Pass a reference of the ORIGINAL stream to the reader
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();

    loop {
        line.clear();

        // Read until '\n'
        let bytes_read = reader.read_line(&mut line)?;

        // Check for EOF (Connection closed)
        if bytes_read == 0 {
            println!("Client disconnected.");
            break;
        }

        let input = line.trim();
        let mut parts = input.split_whitespace();

        // Extract parts using .next()
        let command = parts.next();
        let key_arg = parts.next();
        let value_arg = parts.next();

        match (command, key_arg, value_arg) {
            (Some("SET"), Some(key), Some(val)) => {
                let mut map = kv_store.lock().map_err(|_| {
                    io::Error::new(io::ErrorKind::Other, "Internal Server Error: Lock Poisoned")
                })?;

                map.insert(key.to_string(), val.to_string());

                writer.write_all(b"OK\n")?;
            }
            (Some("GET"), Some(key), _) => {
                let map = kv_store
                    .lock()
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Lock poisoned"))?;

                match map.get(key) {
                    Some(value) => {
                        let response = format!("VALUE {}\n", value);
                        writer.write_all(response.as_bytes())?;
                    }
                    None => {
                        writer.write_all(b"NOT_FOUND\n")?;
                    }
                }
            }

            _ => writer.write_all(b"INVALID_COMMAND\n")?,
        }

        println!("MAP: {:?}", kv_store);

        writer.flush()?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let address = "127.0.0.1:8989";
    println!("Connecting to key-value TCP server at {}...", address);

    let listener = TcpListener::bind(address)?;
    println!("Key-Value server listening on {}", address);
    println!("Waiting for connections...");

    let map: HashMap<String, String> = HashMap::new();
    let shared_map = Mutex::new(map); // Prevent data race
    let kv_store = Arc::new(shared_map); // Thread safe ref counter

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                // For each client we use a cloned reference to be shared acrros mutliple threads / clients
                let kv_store_clone = Arc::clone(&kv_store);

                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream, kv_store_clone) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept incoming connection: {}", e);
            }
        }
    }

    Ok(())
}
