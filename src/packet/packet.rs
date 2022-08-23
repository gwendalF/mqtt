use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::Decoder;

#[derive(Debug)]
pub enum Packet {
    Connect,
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
    Auth,
}

pub enum ProtocolVersion {
    V311,
    V500,
}

pub struct MqttCodec {
    protocol_version: ProtocolVersion,
}

impl Default for MqttCodec {
    fn default() -> Self {
        MqttCodec {
            protocol_version: ProtocolVersion::V500,
        }
    }
}

impl MqttCodec {
    pub fn new() -> MqttCodec {
        MqttCodec::default()
    }
}

impl Decoder for MqttCodec {
    type Item = Packet;
    type Error = std::io::Error;
    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 2 {
            return Ok(None);
        }
        let fixed_header = src[0];
        let packet = match fixed_header << 4 {
            1 => Ok(Packet::Connect),
            2 => Ok(Packet::Connack),
            3 => Ok(Packet::Publish),
            4 => Ok(Packet::Puback),
            5 => Ok(Packet::Pubrec),
            6 => Ok(Packet::Pubrel),
            7 => Ok(Packet::Pubcomp),
            8 => Ok(Packet::Subscribe),
            9 => Ok(Packet::Suback),
            10 => Ok(Packet::Unsubscribe),
            11 => Ok(Packet::Unsuback),
            12 => Ok(Packet::Pingreq),
            13 => Ok(Packet::Pingresp),
            14 => Ok(Packet::Disconnect),
            15 => Ok(Packet::Auth),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid packet type",
            )),
        }?;
        let flags = fixed_header & 0xF;
        let remaining_length = decode_remaining_length(&src[1..])?;
        todo!()
    }
}

fn decode_remaining_length(bytes: &[u8]) -> std::io::Result<(u32, u64)> {
    let mut cursor = std::io::Cursor::new(bytes);
    let mut multiplier = 1u32;
    let mut value = 0;
    loop {
        let encoded_byte = cursor.get_u8();
        value += (encoded_byte & 0x7F) as u32 * multiplier;
        if multiplier > 128 * 128 * 128 {
            break Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "More than 4 bytes for the length",
            ));
        }
        multiplier *= 128;
        if encoded_byte & 0x80 == 0 {
            break Ok((value, cursor.position()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_to_64() {
        let src: [u8; 1] = [64];
        let decoded_value = decode_remaining_length(&src).unwrap();
        assert_eq!(decoded_value, (64, 1));
    }

    #[test]
    fn decode_to_321() {
        let src: [u8; 2] = [193, 2];
        let decoded_value = decode_remaining_length(&src).unwrap();
        assert_eq!(decoded_value, (321, 2));
    }

    #[test]
    fn encode_64() {
        let mut buf = BytesMut::with_capacity(1);
        encode_property_length(64, &mut buf);
        let mut expected = BytesMut::with_capacity(1);
        expected.put_u8(64);
        assert_eq!(expected, buf);
    }

    #[test]
    fn encode_321() {
        let mut buf = BytesMut::with_capacity(2);
        let mut expected = BytesMut::with_capacity(2);
        expected.put_u8(193);
        expected.put_u8(2);
        encode_property_length(321, &mut buf);
        assert_eq!(expected, buf);
    }
}

pub struct FixedHeader {
    packet_type: Packet,
    flags: u8,
    remaining_length: u32,
}

pub struct VariableHeader {
    packet_id: Option<u16>,
}

fn encode_property_length(len: u32, dest: &mut BytesMut) {
    let mut value = len;
    loop {
        let mut encoded_byte = (value % 128) as u8;
        value /= 128;
        if value > 0 {
            encoded_byte = encoded_byte | 0x80;
            dest.put_u8(encoded_byte);
        } else {
            dest.put_u8(encoded_byte);
            break;
        }
    }
}
