#![feature(concat_bytes)]

use std::io;
use std::io::{Read, Write};
use crate::core::encryption::{encryptData, EncryptionData};
use server::protocols::{protocolParser, protocolData};
use crate::server::protocols::createProtocol;

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

        println!(" ");
        print!("Protocol to send: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }
        let packet: protocolData = createProtocol(input);

        if let Err(e) = write_to_server(&mut stream, packet) {
            let logs = utils::Logs::initLog(None, format!("Failed to write to server: {}", e), None);
            utils::Logs::error(logs);
        }

        if let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read > 0 {
                if String::from_utf8_lossy(&buffer[..bytes_read]) == "Unknown packet"{
                    let logs = utils::Logs::initLog(None, "Unknown packet".to_string(), None);
                    utils::Logs::warning(logs);
                } else if String::from_utf8_lossy(&buffer[..bytes_read]).starts_with("INIT_CONNECTION") {
                    let logs = utils::Logs::initLog(None, format!("{}", String::from_utf8_lossy(&buffer[..bytes_read])), None);
                    utils::Logs::debug(logs);
                }
            }
        }
    }
}

fn write_to_server(stream: &mut impl Write, data: protocolData) -> io::Result<()> {
    let (protocol_bytes, sender_bytes, receiver_bytes, data_bytes) = data.to_byte_slices();
    let payload = server::protocols::concatenate_slices(
        b"::",
        protocol_bytes,
        sender_bytes,
        receiver_bytes,
        data_bytes
    );
    stream.write_all(payload)
}