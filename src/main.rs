
use std::io;
use std::io::{Read, Write};
use colored::Colorize;
use crate::core::encryption::{encryptData, EncryptionData};
use server::protocols::{protocolData};
use crate::server::protocols::{checkProtocol, createProtocol};

mod utils;
mod core;
mod server;

fn main() {

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

        //println!(" ");
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

                let response: String = format!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                let responseProtocol: protocolData = createProtocol(response);
                checkProtocol(responseProtocol);
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