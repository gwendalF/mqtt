use super::packet::ProtocolVersion;
use bytes::{Bytes, BytesMut};
use std::num::{NonZeroU16, NonZeroU32};

pub struct ConnectPacket {
    // Header
    pub protocol_version: ProtocolVersion,
    pub clean_start: bool,
    pub keep_alive: u16,

    // Properties
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: Option<NonZeroU16>,
    pub max_packet_size: Option<NonZeroU32>,
    pub topic_alias_max: u16,
    pub request_response_info: bool,
    pub request_problem_info: bool,
    pub user_properties: Vec<(Bytes, Bytes)>,
    pub auth_method: Option<Bytes>,
    pub auth_data: Option<Bytes>,

    // Payload
    pub client_id: Bytes,
    pub last_will: Option<LastWill>,
}

pub enum QoS {
    AtLeastOnce,
    AtMostOnce,
    ExactlyOnce,
}

pub struct LastWill {
    pub will_delay_interval: u32,
    pub qos: QoS,
    pub properties: BytesMut,
    pub topic: BytesMut,
    pub payload: BytesMut,
    pub retain: bool,
}
