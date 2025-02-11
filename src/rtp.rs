use crate::error::RtpError;

#[derive(Debug)]
pub struct RtpHeader {
  version: u8,
  padding: bool,
  extension: bool,
  marker: bool,
  payload_type: u8,
  sequence_number: u16,
  timestamp: u32,
  ssrc: u32,
}

impl RtpHeader {
  pub fn new(
    version: u8,
    marker: bool,
    payload_type: u8,
    sequence_number: u16,
    timestamp: u32,
    ssrc: u32,
  ) -> Result<Self, RtpError> {
    if version > 3 {
      return Err(RtpError::InvalidHeaderField);
    }
    Ok(Self {
      version,
      padding: false,
      extension: false,
      marker,
      payload_type,
      sequence_number,
      timestamp,
      ssrc,
    })
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(12);
    bytes.push(
      (self.version << 6)
        | (self.padding as u8) << 5
        | (self.extension as u8) << 4
        | 0x0f,
    );
    bytes.push((self.marker as u8) << 7 | self.payload_type);
    bytes.extend(&self.sequence_number.to_be_bytes());
    bytes.extend(&self.timestamp.to_be_bytes());
    bytes.extend(&self.ssrc.to_be_bytes());
    bytes
  }
}

pub struct RtpPacket {
  header: RtpHeader,
  payload: Vec<u8>,
}

impl RtpPacket {
  pub fn new(header: RtpHeader, payload: impl Into<Vec<u8>>) -> Self {
    Self {
      header,
      payload: payload.into(),
    }
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut data = self.header.to_bytes();
    data.extend(&self.payload);
    data
  }
}
