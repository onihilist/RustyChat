use crate::server::protocols::{protocolData};
use chrono::{
    DateTime,
    Utc
};

pub struct responseData{
    protocolData: protocolData,
    timeUTC: DateTime<Utc>,
    responseTime: u16
}

impl responseData {
    pub(crate) fn to_byte_slices(&self) -> (&[u8], &[u8], &[u8], &[u8]) {
        (
            self.protocolData.protocol.as_bytes(),
            self.protocolData.sender.as_bytes(),
            self.protocolData.receiver.as_bytes(),
            self.protocolData.data.as_bytes(),
        )
    }
}

pub fn createResponse(protocol_data: protocolData, timeUTC: DateTime<Utc>, responseTime: u16) -> responseData {
    return responseData {
        protocolData: protocol_data,
        timeUTC,
        responseTime
    }
}