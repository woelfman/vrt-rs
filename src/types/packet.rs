use nom::{
    number::streaming::{be_u32, be_u64},
    Err, IResult, Needed,
};
use nom_derive::Parse;

use super::*;

/// VRT Packet
#[derive(Debug, PartialEq)]
pub struct VrtPacket<'a> {
    /// VRT Packet Header
    pub header: Header,
    /// Optional Stream Id
    pub stream_id: Option<u32>,
    /// Optional Class Id
    pub class_id: Option<ClassId>,
    /// Optional Integer-Seconds Timestamp
    pub tsi: Option<u32>,
    /// Optional Fractional-Seconds Timestamp
    pub tsf: Option<u64>,
    /// Data Payload
    pub data_payload: &'a [u8],
    /// Optional VRT Packet Trailer
    pub trailer: Option<Trailer>,
}

impl VrtPacket<'_> {
    /// Parse the VRT packet
    pub fn parse(i: &[u8]) -> IResult<&[u8], VrtPacket<'_>> {
        let (i, header) = Header::parse(i)?;

        let expected_size = header.packet_size as usize * size_of::<u32>();
        let actual_size = i.len() + size_of::<u32>();
        if actual_size < expected_size {
            return Err(Err::Incomplete(Needed::new(expected_size)));
        }

        // Track the mandatory and optional fields to get the payload length
        let mut payload_len = expected_size;
        payload_len -= size_of::<u32>(); // header word
        if header.t {
            payload_len -= size_of::<u32>(); // trailer word
        }

        let (i, stream_id) = if matches!(
            header.packet_type,
            PktType::IfDataWithStream | PktType::ExtDataWithStream
        ) {
            let (i, stream_id) = be_u32(i)?;
            payload_len -= size_of_val(&stream_id);
            (i, Some(stream_id))
        } else {
            (i, None)
        };

        let (i, class_id) = if header.c {
            let (i, class_id) = ClassId::parse(i)?;
            payload_len -= size_of_val(&class_id);
            (i, Some(class_id))
        } else {
            (i, None)
        };

        let (i, tsi) = if header.tsi == Tsi::None {
            (i, None)
        } else {
            let (i, tsi) = be_u32(i)?;
            payload_len -= size_of_val(&tsi);
            (i, Some(tsi))
        };

        let (i, tsf) = if header.tsf == Tsf::None {
            (i, None)
        } else {
            let (i, tsf) = be_u64(i)?;
            payload_len -= size_of_val(&tsf);
            (i, Some(tsf))
        };

        let (data_payload, i) = i.split_at(payload_len);

        let (i, trailer) = if header.t {
            let (i, trailer) = Trailer::parse(i)?;
            (i, Some(trailer))
        } else {
            (i, None)
        };

        let packet = VrtPacket {
            header,
            stream_id,
            class_id,
            tsi,
            tsf,
            data_payload,
            trailer,
        };

        Ok((i, packet))
    }
}
