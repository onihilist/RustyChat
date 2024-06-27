use hardware_id::{get_id, HwIdError};
use reqwest;

pub struct SenderData {
    timestamp: u64,
    context: &'static str,
    message: &'static str
}

fn getHWID() -> String {
    get_id().unwrap()
}

async fn getPublicAddress() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://api.ipify.org/").await?;
    let public_ip = res.text().await?;
    Ok(())
}