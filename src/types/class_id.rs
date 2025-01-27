use nom::{
    number::streaming::{be_u16, be_u32},
    IResult,
};

/// Class Identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassId {
    /// Organizationally Unique Identifier assigned by IEEE, VITA, the VRT Profile author, or a reserved OUI.
    pub oui: u32,
    /// Packet Class Code
    pub packet_class_code: u16,
    /// Information Class Code
    pub information_class_code: u16,
}

impl ClassId {
    /// Parse the Class ID
    pub fn parse(i: &[u8]) -> IResult<&[u8], ClassId> {
        let (i, oui) = be_u32(i)?;
        let (i, packet_class_code) = be_u16(i)?;
        let (i, information_class_code) = be_u16(i)?;

        Ok((
            i,
            ClassId {
                oui,
                packet_class_code,
                information_class_code,
            },
        ))
    }
}
