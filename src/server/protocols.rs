use colored::Colorize;
use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct protocolData {
    protocol: String,
    sender: String,
    receiver: String,
    data: String
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

pub fn checkProtocol(protocol_data: protocolData) -> String {

    let logs: UtilsData = utils::Logs::initLog(None, "This protocol action doesn't exist".to_string(), None);

    match protocol_data.protocol.as_str() {
        "INIT_CONNECTION"=>return "init_connection".to_string(),
        "REGISTER"=>return "register".to_string(),
        "LOGIN"=>return "connect".to_string(),
        "SEND"   =>return "send".to_string(),
        "RECEIVE"=>return "receive".to_string(),
        _=>return format!("{}{}{}", "[", "ERROR".red(), "] -> This protocol action doesn't exist")
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

pub fn protocolParser(packet: protocolData) -> String {

    let splitter: String = "::".to_string();
    let request = format!("{}{}{}{}{}{}{}",
                          packet.protocol,
                          splitter,
                          packet.sender,
                          splitter,
                          packet.receiver,
                          splitter,
                          packet.data
    );
    return request;
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