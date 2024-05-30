use colored::Colorize;
use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct protocolData {
    protocol: String,
}

pub fn findProtocol(protocol_data: protocolData) -> String {

    let logs: UtilsData = utils::Logs::initLog(None, "This protocol action doesn't exist", None)

    match protocol_data.protocol.as_str() {
        "CONNECT"=>return "connect".to_string(),
        "SEND"   =>return "send".to_string(),
        "RECEIVE"=>return "receive".to_string(),
        _=>return format!("{}{}{}", "[", "ERROR".red(), "] -> This protocol action doesn't exist"),
    }

}