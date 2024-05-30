use std::ptr::null;
use crate::core::encryption::{encryptData, EncryptionData};
use crate::utils::Logs::UtilsData;

mod utils;
mod core;
mod server;

fn main() {

    let encrypt: EncryptionData = core::encryption::createInstance();
    let encryptedData = core::encryption::encryptData(encrypt, "test".to_string());
    server::client::connectToServer();


}
