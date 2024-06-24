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
