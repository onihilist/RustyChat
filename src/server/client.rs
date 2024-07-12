use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream,
               SocketAddr,
               IpAddr,
               Ipv4Addr
};
use std::time::Duration;
use crate::{server, utils, write_to_server};
use crate::server::protocols::{checkProtocol, createProtocol, protocolData};

pub struct clientData {
    uuid: &'static str,
    username: &'static str
}

pub fn handler() {
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

pub fn connectToServer() -> io::Result<TcpStream> {

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 42000);
    let timeout = Duration::new(5, 0);

    let stream_result = TcpStream::connect_timeout(&socket, timeout);

    match stream_result {
        Ok(stream) => {
            let logs = utils::Logs::initLog(None, "Connected to the server".to_string(), None);
            utils::Logs::success(logs);
            Ok(stream)
        },
        Err(e) => {
            println!("Erreur de connexion : {}", e);
            Err(e)
        },
    }

}
