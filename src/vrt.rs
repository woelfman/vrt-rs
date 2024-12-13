//! Packet data from [VITA 49.0-2015 VITA Radio Transport (VRT) Standard]: Data Packet Classes and Streams.
//!
//! This standard defines the VRT packet types.
//!
//! [VITA 49.0-2015 VITA Radio Transport (VRT) Standard]: https://vitastore.dpdcart.com/product/168632

use nom_derive::NomBE;

/// Minimum VRT Record Size (8 bytes)
pub const VRT_MINIMUM_RECORD_SIZE: i32 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, NomBE)]
/// VRT Packet Type
pub struct VitaPacketType(pub u8);

impl VitaPacketType {
    /// IFDATA is a standard IFDATA packet.
    pub const IFDATA: u8 = 0x00;
    /// IFDATAWITHSTREAM is a standard IFDATA packet with a packet stream.
    pub const IFDATAWITHSTREAM: u8 = 0x01;
    /// EXTDATA is a custom Extension Data packet.
    pub const EXTDATA: u8 = 0x02;
    /// EXTDATAWITHSTREAM is a custom Extension Data packet with a packet stream.
    pub const EXTDATAWITHSTREAM: u8 = 0x03;
    /// IFCONTEXT is a standard IFCONTEXT packet with a context packet stream.
    pub const IFCONTEXT: u8 = 0x04;
    /// EXTCONTEXT is a custom Extension packet with a context packet stream.
    pub const EXTCONTEXT: u8 = 0x05;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, NomBE)]
/// Integer-Seconds Timestamp Type
pub struct Tsi(pub u8);

impl Tsi {
    /// No integer-seconds timestamp field.
    pub const TSI_NONE: u8 = 0;
    /// Integer-seconds timestamp field is a Coordinated Universal Time (UTC).
    pub const TSI_UTC: u8 = 1;
    /// Integer-seconds timestamp field is a GPS time.
    pub const TSI_GPS: u8 = 2;
    /// Integer-seconds timestamp field is in another format.
    pub const TSI_OTHER: u8 = 3;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, NomBE)]
/// Fractional-Seconds Timestamp Type
pub struct Tsf(pub u8);

impl Tsf {
    /// No fractional-seconds timestamp field.
    pub const TSF_NONE: Tsf = Tsf(0);
    /// Fractional-seconds timestamp field is a Sample Count Timestamp.
    pub const TSF_SAMPLE_COUNT: Tsf = Tsf(1);
    /// Fractional-seconds timestamp field is a Real Time (Picoseconds) Timestamp.
    pub const TSF_REAL_TIME: Tsf = Tsf(2);
    /// Fractional-seconds timestamp field is a Free Running Count Timestamp.
    pub const TSF_FREE_RUNNING: Tsf = Tsf(3);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, NomBE)]
/// Class Identifier
pub struct ClassId {
    /// Organizationally Unique Identifier assigned by IEEE, VITA, the VRT Profile author, or a reserved OUI.
    pub oui: u32,
    /// Packet Class Code
    pub packet_class_code: u16,
    /// Information Class Code
    pub information_class_code: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// VRT Packet Header
pub struct Header {
    /// VRT Packet Type
    pub packet_type: VitaPacketType,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// VRT Packet Trailer
pub struct Trailer {
    /// Indicates whether or not the timestamp in teh IF Data packet is calibrated to an external reference.
    pub calibrated_time_enable: bool,
    /// Indicates whether or not the data in the packet is valid.
    pub valid_data_enable: bool,
    /// Indicates whether or not any phase-locked loops affecting the data are locked and stable.
    pub reference_lock_enable: bool,
    /// Indicates the AGC is active when true and indicates the MGC is active when false.
    pub agcmgc_enable: bool,
    /// Indicates whether or not the data contained in the packet containes some detected signal.
    pub detected_signal_enable: bool,
    /// Indicates whether or not the signal conveyed in the data payload has an inverted spectrum referenced to the spectrum of the signal at the system Reference Point.
    pub spectral_inversion_enable: bool,
    /// Indicates whether or not at least 1 data sample in the payload is invalid due to the signal exceeding the range of the data item.
    pub overrange_enable: bool,
    /// Indicates whether or not the packet contains at least one sample discontinuity due to processing errors and/or buffer overflow.
    pub sample_loss_enable: bool,
    /// Indicates whether or not user defined enable option 1 is set.
    pub user_defined_enable_1: bool,
    /// Indicates whether or not user defined enable option 2 is set.
    pub user_defined_enable_2: bool,
    /// Indicates whether or not user defined enable option 3 is set.
    pub user_defined_enable_3: bool,
    /// Indicates whether or not user defined enable option 4 is set.
    pub user_defined_enable_4: bool,
    /// To be documented later
    pub calibrated_time_indicator: bool,
    /// To be documented later
    pub valid_data_indicator: bool,
    /// To be documented later
    pub reference_lock_indicator: bool,
    /// To be documented later
    pub agcmgc_indicator: bool,
    /// To be documented later
    pub detected_signal_indicator: bool,
    /// To be documented later
    pub spectral_inversion_indicator: bool,
    /// To be documented later
    pub overrange_indicator: bool,
    /// To be documented later
    pub sample_loss_indicator: bool,
    /// To be documented later
    pub user_defined_indicator_1: bool,
    /// To be documented later
    pub user_defined_indicator_2: bool,
    /// To be documented later
    pub user_defined_indicator_3: bool,
    /// To be documented later
    pub user_defined_indicator_4: bool,
    /// Indicates whether or not the associated_context_packet_count is defined.
    pub associated_context_packet_count_enable: bool,
    /// A count of all transmitted context packets that are directly or indirectly associated with the IF Data packet or a count of some special subset of these.
    pub associated_context_packet_count: u8,
}

#[derive(Debug, PartialEq)]
/// A VRT Packet
pub struct VrtPacket<'a> {
    /// VRT Packet Header
    pub header: Header,
    /// Optional Stream Id
    pub stream_id: Option<u32>,
    /// Optional Class Id
    pub class_id: Option<u64>,
    /// Optional Integer-Seconds Timestamp
    pub tsi: Option<u32>,
    /// Optional Fractional-Seconds Timestamp
    pub tsf: Option<u64>,
    /// Data Payload
    pub data_payload: &'a [u8],
    /// Optional VRT Packet Trailer
    pub trailer: Option<Trailer>,
}
