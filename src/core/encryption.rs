use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct EncryptionData{
    pub timestamp: u32,
    pub strToEncrypt: &'static str,
    pub timeToEncrypt: f32
}

pub fn createInstance() -> EncryptionData {
    let instance:EncryptionData = EncryptionData {
        timestamp: 0,
        strToEncrypt: "null",
        timeToEncrypt: 0.0
    };

    return instance;
}

pub fn encryptData(obj: EncryptionData, data: String) -> Vec<u8> {
    let datatrim = data.trim_end().to_string();
    let mut logs: UtilsData = utils::Logs::initLog(None, "Message encrypté avec succès !", None);
    utils::Logs::success(logs);
    return datatrim.into_bytes();
}