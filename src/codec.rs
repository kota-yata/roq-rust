impl RoqCodec {
  fn encode_datagram(&self, packet: RtpPacket, buf: &mut BytesMut) {
      // Write flow ID as variable-length integer
      self.write_varint(packet.flow_id, buf);
      // Write RTP payload
      buf.extend_from_slice(&packet.payload);
  }

  fn encode_stream(&self, packet: RtpPacket, buf: &mut BytesMut) {
      // Write flow ID as variable-length integer
      self.write_varint(packet.flow_id, buf);
      // Write length of RTP payload as variable-length integer
      self.write_varint(packet.payload.len() as u64, buf);
      // Write RTP payload
      buf.extend_from_slice(&packet.payload);
  }

  fn decode_datagram(&self, buf: &mut Cursor<&[u8]>) -> io::Result<RtpPacket> {
      let flow_id = self.read_varint(buf)?;
      let payload = Bytes::copy_from_slice(&buf.remaining_slice());
      Ok(RtpPacket { flow_id, payload })
  }

  fn decode_stream(&self, buf: &mut Cursor<&[u8]>) -> io::Result<RtpPacket> {
      let flow_id = self.read_varint(buf)?;
      let length = self.read_varint(buf)? as usize;
      let payload = Bytes::copy_from_slice(&buf.chunk()[..length]);
      buf.advance(length);
      Ok(RtpPacket { flow_id, payload })
  }

  fn write_varint(&self, n: u64, buf: &mut BytesMut) {
      // Implement variable-length integer encoding
  }

  fn read_varint(&self, buf: &mut Cursor<&[u8]>) -> io::Result<u64> {
      // Implement variable-length integer decoding
  }
}
