use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::io::prelude::*;
use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct clientData {
    uuid: &'static str,
    username: &'static str
}

pub fn connectToServer() {

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 42000);
    let timeout = Duration::new(5, 0);

    if let Ok(stream) = TcpStream::connect_timeout(&socket, timeout) {
        let mut logs: UtilsData = utils::Logs::initLog(None, "Successfully connected to server !", None);
        utils::Logs::success(logs);
    } else {
        let mut logs: UtilsData = utils::Logs::initLog(None, "Impossible to connect to the server...", None);
        utils::Logs::error(logs);
    }
}
