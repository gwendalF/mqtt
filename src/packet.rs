use bytes::Buf;
use tokio_util::codec::Decoder;

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
    use super::decode_remaining_length;

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
}
