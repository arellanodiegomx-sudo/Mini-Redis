use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening on port 6379");

    let store: Arc<Mutex<HashMap<String, Entry>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        println!("New connection!");

        let store = store.clone();

        tokio::spawn(async move {
            let mut buffer = [0; 512];
            let bytes_read = stream.read(&mut buffer).await.unwrap();

            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            let request = request.trim();

            println!("Received: {}", request);

            let parts: Vec<&str> = request.split_whitespace().collect();

            let response = match parts.get(0) {
                Some(&"PING") => "+PONG\r\n".to_string(),

                Some(&"SET") => {
                    match (parts.get(1), parts.get(2)) {
                        (Some(key), Some(value)) => {
                            let expires_at = match (parts.get(3), parts.get(4)) {
                                (Some(&"EX"), Some(seconds_str)) => {
                                    match seconds_str.parse::<u64>() {
                                        Ok(seconds) => Some(Instant::now() + Duration::from_secs(seconds)),
                                        Err(_) => None,
                                    }
                                }
                                _ => None,
                            };

                            let entry = Entry {
                                value: value.to_string(),
                                expires_at,
                            };

                            let mut store = store.lock().await;
                            store.insert(key.to_string(), entry);
                            "+OK\r\n".to_string()
                        }
                        _ => "-ERR wrong number of arguments for SET\r\n".to_string(),
                    }
                }

                Some(&"GET") => {
                    match parts.get(1) {
                        Some(key) => {
                            let mut store = store.lock().await;
                            match store.get(*key) {
                                Some(entry) => {
                                    let expired = match entry.expires_at {
                                        Some(expiry) => Instant::now() > expiry,
                                        None => false,
                                    };

                                    if expired {
                                        store.remove(*key);
                                        "$-1\r\n".to_string()
                                    } else {
                                        format!("+{}\r\n", entry.value)
                                    }
                                }
                                None => "$-1\r\n".to_string(),
                            }
                        }
                        None => "-ERR wrong number of arguments for GET\r\n".to_string(),
                    }
                }

                Some(&"DEL") => {
                    match parts.get(1) {
                        Some(key) => {
                            let mut store = store.lock().await;
                            store.remove(*key);
                            "+OK\r\n".to_string()
                        }
                        None => "-ERR wrong number of arguments for DEL\r\n".to_string(),
                    }
                }

                _ => "-ERR unknown command\r\n".to_string(),
            };

            stream.write(response.as_bytes()).await.unwrap();
        });
    }
}