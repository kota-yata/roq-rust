use crate::rtp::{RtpHeader, RtpPacket};

pub fn build_datagram_packet(flow_id: u64, rtp_packet: &RtpPacket) -> Vec<u8> {
  let mut data = flow_id.to_be_bytes().to_vec();
  data.extend(&rtp_packet.serialize());
  data
}

pub async fn write_stream_packet(
  stream: &mut quinn::SendStream,
  flow_id: u64,
  rtp_packet: &RtpPacket,
) -> Result<(), quinn::WriteError> {
  stream.write_all(&build_datagram_packet(flow_id, rtp_packet)).await?;
  Ok(())
}
