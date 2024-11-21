use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::{self, Cursor};

struct RtpPacket {
    flow_id: u64,
    payload: Bytes,
}

enum RoqPacket {
    Datagram(RtpPacket),
    Stream(RtpPacket),
}

struct RoqCodec;
