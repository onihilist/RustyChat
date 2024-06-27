use std::fmt::format;
use lazy_static::lazy_static;
use regex::Regex;

use colored::Colorize;
use crate::utils;
use crate::utils::Logs::UtilsData;

pub const PROTOCOL_DATA_SEP: &str = "::";
pub const PROTOCOL_NOT_EXIST: &str = "PROTOCOL_NOT_EXIST";
pub const INIT_CONNECTION: &str = "INIT_CONNECTION";
pub const REGISTER: &str =  "REGISTER";
pub const LOGIN: &str = "LOGIN";
pub const SEND: &str = "SEND";
pub const RECEIVE: &str = "RECEIVE";

pub struct protocolData {
    pub protocol: String,
    pub sender: String,
    pub receiver: String,
    pub data: String
}

impl protocolData {
    pub(crate) fn to_byte_slices(&self) -> (&[u8], &[u8], &[u8], &[u8]) {
        (
            self.protocol.as_bytes(),
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            self.data.as_bytes(),
        )
    }
}

pub fn getRegex() -> Regex {
    Regex::new(r"^INIT_CONNECTION::(.*?)::(.*?)::(.*?)$
                    |^REGISTER::(.*?)::(.*?)::(.*?)$
                    |^LOGIN::(.*?)::(.*?)::(.*?)$
                    |^SEND::(.*?)::(.*?)::(.*?)$
                    |^RECEIVE::(.*?)::(.*?)::(.*?)$").unwrap()
}

pub fn initProtocolData(protocol: String, sender: String, receiver: String, data: String) -> protocolData {
    let data = protocolData {
        protocol,
        sender,
        receiver,
        data
    };
    return data;
}

pub fn checkProtocol(protocol_data: protocolData) {

    match protocol_data.protocol.as_str() {
        INIT_CONNECTION => {
            if protocol_data.data == "CONNECTION OK".to_string() {
                let logs: UtilsData = utils::Logs::initLog(None, "Successfully connected to the network.".to_string(), None);
                utils::Logs::success(logs);
            } else {
                let logs: UtilsData = utils::Logs::initLog(None, format!("Impossible to connect to the network. ({})", protocol_data.data), None);
                utils::Logs::error(logs);
            }
        }
        REGISTER => {
            if protocol_data.data == "REGISTER OK".to_string() {
                let logs: UtilsData = utils::Logs::initLog(None, "Account successfully created.".to_string(), None);
                utils::Logs::success(logs);
            } else {
                let logs: UtilsData = utils::Logs::initLog(None, "Impossible to create an account.".to_string(), None);
                utils::Logs::error(logs);
            }
        }
        LOGIN => {
            if protocol_data.data == "LOGIN OK".to_string() {
                let logs: UtilsData = utils::Logs::initLog(None, format!("Welcome back {} !", protocol_data.receiver), None);
                utils::Logs::success(logs);
            } else {
                let logs: UtilsData = utils::Logs::initLog(None, "Wrong password or login !".to_string(), None);
                utils::Logs::error(logs);
            }
        }
        SEND => {
            if protocol_data.data == "SEND OK".to_string() {
                let logs: UtilsData = utils::Logs::initLog(None, format!("Message successfully sent to {}.", protocol_data.sender), None);
                utils::Logs::success(logs);
            } else {
                let logs: UtilsData = utils::Logs::initLog(None, "Wrong password or login !".to_string(), None);
                utils::Logs::error(logs);
            }
        }
        RECEIVE => {
            if protocol_data.data == "RECEIVE OK".to_string() {
                let logs: UtilsData = utils::Logs::initLog(None, format!("Message from {} : {}", protocol_data.sender, protocol_data.data), None);
                utils::Logs::success(logs);
            } else {
                let logs: UtilsData = utils::Logs::initLog(None, "You receive a message but there is an error.".to_string(), None);
                utils::Logs::error(logs);
            }
        }
        PROTOCOL_NOT_EXIST => {
            let logs: UtilsData = utils::Logs::initLog(None, format!("Server doesn't know this protocol. ({})", protocol_data.protocol ), None);
            utils::Logs::warning(logs);
        }
        _ => {
            let logs: UtilsData = utils::Logs::initLog(None, format!("You receive an unknown protocol... ({})", protocol_data.protocol), None);
            utils::Logs::error(logs);
        }
    }

}

pub fn createProtocol(request: String) -> protocolData {
    let parts: Vec<&str> = request.split("::").collect();

    let newProtocol: protocolData = protocolData{
        protocol: parts[0].to_string(),
        sender:   parts[1].to_string(),
        receiver: parts[2].to_string(),
        data:     parts[3].to_string(),
    };
    return newProtocol;
}

pub fn dataParser(data: &str) -> Vec<String> {
    data.split("#").map(|s| s.to_string()).collect()
}

pub fn concatenate_slices<'a>(separator: &[u8], slice1: &'a [u8], slice2: &'a [u8], slice3: &'a [u8], slice4: &'a [u8]) -> &'a [u8] {
    let total_length = slice1.len() + separator.len() + slice2.len()+ separator.len() + slice3.len()+ separator.len() + slice4.len();
    let mut result = Vec::with_capacity(total_length);
    result.extend_from_slice(slice1);
    result.extend_from_slice(separator);
    result.extend_from_slice(slice2);
    result.extend_from_slice(separator);
    result.extend_from_slice(slice3);
    result.extend_from_slice(separator);
    result.extend_from_slice(slice4);

    unsafe { std::slice::from_raw_parts(result.as_ptr(), total_length) }

}