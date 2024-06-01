use std::io;
use std::io::Read;
use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;
use crate::server::protocols::protocolData;
use crate::{utils, write_to_server};
use crate::utils::Logs::UtilsData;
use crate::server;

pub struct clientData {
    uuid: &'static str,
    username: &'static str
}

fn handler(mut stream: TcpStream) -> std::io::Result<()> {

    let mut buffer = [0; 512];
    let user: SocketAddr = stream.local_addr().unwrap();
    let msgUser: String = format!("{} has connected successfully", user);
    let logs: UtilsData = utils::Logs::initLog(None, msgUser, None);
    utils::Logs::info(logs);

    loop {

        let bytes_read = stream.read(&mut buffer)?;

        match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(msg) => {

                let protocolReceive: protocolData = server::protocols::createProtocol(msg.to_string());

            }
            Err(e) => {
                let logs = utils::Logs::initLog(None, format!("Invalid UTF-8 sequence: {}", e), None);
                utils::Logs::error(logs);
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
