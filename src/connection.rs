use quinn::{Connection, Endpoint};

async fn send_rtp_packet(connection: &Connection, packet: RtpPacket) -> Result<(), Box<dyn std::error::Error>> {
    let mut codec = RoqCodec;
    let mut buf = BytesMut::new();

    // Encode as datagram
    codec.encode_datagram(packet, &mut buf);
    connection.send_datagram(buf.freeze())?;

    // Or encode as stream
    let mut send_stream = connection.open_uni().await?;
    codec.encode_stream(packet, &mut buf);
    send_stream.write_all(&buf).await?;
    send_stream.finish().await?;

    Ok(())
}
