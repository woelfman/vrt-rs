/// Class Identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq, nom_derive::NomBE)]
pub struct ClassId {
    /// Organizationally Unique Identifier assigned by IEEE, VITA, the VRT Profile author, or a reserved OUI.
    pub oui: u32,
    /// Packet Class Code
    pub packet_class_code: u16,
    /// Information Class Code
    pub information_class_code: u16,
}
