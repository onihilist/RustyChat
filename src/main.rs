use std::io;
use std::io::{Read, Write};
use std::ptr::null;
use std::time::Duration;
use crate::core::encryption::{encryptData, EncryptionData};
use crate::utils::Logs::UtilsData;

mod utils;
mod core;
mod server;

fn main() {
    let encrypt: EncryptionData = core::encryption::createInstance();
    let encrypted_data = core::encryption::encryptData(encrypt, "test".to_string());

    let mut stream = match server::client::connectToServer() {
        Ok(s) => s,
        Err(e) => {
            let logs = utils::Logs::initLog(None, format!("Failed to connect to server: {}", e), None);
            utils::Logs::error(logs);
            return;
        }
    };

    let connected = true;

    while connected {
        let mut buffer = [0; 512];

        print!("Protocol to send: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        if let Err(e) = write_to_server(&mut stream, input.trim().as_bytes()) {
            let logs = utils::Logs::initLog(None, format!("Failed to write to server: {}", e), None);
            utils::Logs::error(logs);
        }

        // If you want to handle the response from the server
        if let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read > 0 {
                if String::from_utf8_lossy(&buffer[..bytes_read]) == "Unknown packet"{
                    let logs = utils::Logs::initLog(None, "Unknown packet".to_string(), None);
                    utils::Logs::warning(logs);
                }
            }
        }
    }
}

fn write_to_server(stream: &mut impl Write, data: &[u8]) -> io::Result<()> {
    stream.write_all(data)
}