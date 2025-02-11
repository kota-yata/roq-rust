mod connection;
use std::{clone, error::Error, net::{IpAddr, Ipv4Addr, SocketAddr}};

use crate::connection::{make_client_endpoint, make_server_endpoint};
mod rtp;
mod roq;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);
  
  let (endpoint, server_cert) = make_server_endpoint(server_addr).unwrap();
  
  let endpoint2 = endpoint.clone();
  tokio::spawn(async move {
    let incoming_conn = endpoint2.accept().await.unwrap();
    let conn = incoming_conn.await.unwrap();
    let dgram_conn = conn.clone();
    tokio::spawn(async move {
      loop {
        match dgram_conn.read_datagram().await {
          Ok(data) => {
            let flow_id = u64::from_be_bytes(data[0..8].try_into().unwrap());
            let rtp_packet = &data[8..];
            println!("[server] Received datagram (flow={}) RTP packet: {} bytes", 
              flow_id, rtp_packet.len());
          }
          Err(e) => {
            eprintln!("Datagram receive error: {:?}", e);
            break;
          }
        }
      }
    });
  });

  let endpoint = make_client_endpoint("0.0.0.0:0".parse().unwrap(), &[&server_cert]).unwrap();
  let connection = endpoint.connect(server_addr, "localhost")?.await?;

  let header = rtp::RtpHeader::new(2, true, 96, 1234, 0, 0x12345678)?;
  let rtp_packet = rtp::RtpPacket::new(header, b"test payload");

  connection.send_datagram(roq::build_datagram_packet(1, &rtp_packet).into())?;
  
  let mut stream = connection.open_uni().await?;
  roq::write_stream_packet(&mut stream, 1, &rtp_packet).await?;
  
  endpoint.wait_idle().await;
  Ok(())
}
