use nom::{number::streaming::be_u8, Err, IResult, Needed};

use super::*;

/// VRT Packet Header
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
    /// VRT Packet Type
    pub packet_type: PktType,
    /// Is the Class Identifier (Class ID) field included in the packet?
    pub c: bool,
    /// Is the VRT Packet Trailer included in the packet?
    pub t: bool,
    /// Type of Integer-seconds Timestamp included in the packet.
    pub tsi: Tsi,
    /// Type of Fractional-seconds Timestamp included in the packet.
    pub tsf: Tsf,
    /// Incremental count of data packets in the same stream. Rolls over from "1111" to "0000" as needed.
    pub packet_count: u8,
    /// The total number of 32-bit words present in the data packet including the header, payload, and any optional fields.
    pub packet_size: u16,
}

impl Header {
    /// Parse the VRT packet header
    pub fn parse(i: &[u8]) -> IResult<&[u8], Header> {
        if i.len() < 4 {
            return Err(Err::Incomplete(Needed::new(4)));
        }

        let (i, first_byte) = be_u8(i)?;
        let packet_type = PktType::try_from(first_byte >> 4 & 0b1111)
            .map_err(|_| Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Verify)))?;
        let c = ((first_byte >> 3) & 0x01) != 0;
        let t = ((first_byte >> 2) & 0x01) != 0;

        let (i, second_byte) = be_u8(i)?;
        let tsi = Tsi::try_from((second_byte >> 6) & 0b11)
            .map_err(|_| Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Verify)))?;
        let tsf = Tsf::try_from((second_byte >> 4) & 0b11)
            .map_err(|_| Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Verify)))?;
        let packet_count: u8 = (second_byte) & 0xf;

        let (i, third_byte) = be_u8(i)?;
        let (i, fourth_byte) = be_u8(i)?;
        let packet_size: u16 = (third_byte + fourth_byte) as u16;

        let hdr = Header {
            packet_type,
            c,
            t,
            tsi,
            tsf,
            packet_count,
            packet_size,
        };
        Ok((i, hdr))
    }
}
