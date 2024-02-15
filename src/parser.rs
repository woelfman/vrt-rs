use crate::vrt::*;
use nom::number::streaming::be_u8;
use nom::{Err, IResult, Needed};

fn u8_to_bool(v: u8) -> bool {
    match v {
        0 => false,
        1 => true,
        _ => panic!("Invalid bool in u8 {}", v),
    }
}

/// Parses the VRT packet's header
pub fn parse_vrt_header(i: &[u8]) -> IResult<&[u8], Header> {
    if i.len() < 4 {
        return Err(Err::Incomplete(Needed::new(4)));
    }

    let (i, first_byte) = be_u8(i)?;
    let packet_type: u8 = first_byte >> 4;
    let c: u8 = (first_byte >> 3) & 0x01;
    let t: u8 = (first_byte >> 2) & 0x01;

    let (i, second_byte) = be_u8(i)?;
    let tsi: u8 = (second_byte >> 6) & 0x03;
    let tsf: u8 = (second_byte >> 4) & 0x03;
    let packet_count: u8 = (second_byte) & 0xf;

    let (i, third_byte) = be_u8(i)?;
    let (i, fourth_byte) = be_u8(i)?;
    let packet_size: u16 = (third_byte + fourth_byte) as u16;

    let hdr = Header {
        packet_type: VitaPacketType(packet_type),
        c: u8_to_bool(c),
        t: u8_to_bool(t),
        tsi: Tsi(tsi),
        tsf: Tsf(tsf),
        packet_count,
        packet_size,
    };
    Ok((i, hdr))
}

/// Parses the VRT packet's trailer
pub fn parse_vrt_trailer(i: &[u8]) -> IResult<&[u8], Trailer> {
    if i.len() < 4 {
        return Err(Err::Incomplete(Needed::new(4)));
    }

    let (i, first_byte) = be_u8(i)?;
    let (i, second_byte) = be_u8(i)?;
    let (i, third_byte) = be_u8(i)?;
    let (i, fourth_byte) = be_u8(i)?;

    // first byte
    let calibrated_time_enable: u8 = (first_byte >> 7) & 0x01;
    let valid_data_enable: u8 = (first_byte >> 6) & 0x01;
    let reference_lock_enable: u8 = (first_byte >> 5) & 0x01;
    let agcmgc_enable: u8 = (first_byte >> 4) & 0x01;
    let detected_signal_enable: u8 = (first_byte >> 3) & 0x01;
    let spectral_inversion_enable: u8 = (first_byte >> 2) & 0x01;
    let overrange_enable: u8 = (first_byte >> 1) & 0x01;
    let sample_loss_enable: u8 = (first_byte) & 0x01;

    // second byte
    let user_defined_enable_1: u8 = (second_byte >> 7) & 0x01;
    let user_defined_enable_2: u8 = (second_byte >> 6) & 0x01;
    let user_defined_enable_3: u8 = (second_byte >> 5) & 0x01;
    let user_defined_enable_4: u8 = (second_byte >> 4) & 0x01;
    let calibrated_time_indicator: u8 = (second_byte >> 3) & 0x01;
    let valid_data_indicator: u8 = (second_byte >> 2) & 0x01;
    let reference_lock_indicator: u8 = (second_byte >> 1) & 0x01;
    let agcmgc_indicator: u8 = (second_byte) & 0x01;

    // third byte
    let detected_signal_indicator: u8 = (third_byte >> 7) & 0x01;
    let spectral_inversion_indicator: u8 = (third_byte >> 6) & 0x01;
    let overrange_indicator: u8 = (third_byte >> 5) & 0x01;
    let sample_loss_indicator: u8 = (third_byte >> 4) & 0x01;
    let user_defined_indicator_1: u8 = (third_byte >> 3) & 0x01;
    let user_defined_indicator_2: u8 = (third_byte >> 2) & 0x01;
    let user_defined_indicator_3: u8 = (third_byte >> 1) & 0x01;
    let user_defined_indicator_4: u8 = (third_byte) & 0x01;

    // fourth byte
    let associated_context_packet_count_enable: u8 = (fourth_byte >> 7) & 0x01;
    let associated_context_packet_count_value: u8 = (fourth_byte) & 0x7f;

    let hdr: Trailer = Trailer {
        calibrated_time_enable: u8_to_bool(calibrated_time_enable),
        valid_data_enable: u8_to_bool(valid_data_enable),
        reference_lock_enable: u8_to_bool(reference_lock_enable),
        agcmgc_enable: u8_to_bool(agcmgc_enable),
        detected_signal_enable: u8_to_bool(detected_signal_enable),
        spectral_inversion_enable: u8_to_bool(spectral_inversion_enable),
        overrange_enable: u8_to_bool(overrange_enable),
        sample_loss_enable: u8_to_bool(sample_loss_enable),
        user_defined_enable_1: u8_to_bool(user_defined_enable_1),
        user_defined_enable_2: u8_to_bool(user_defined_enable_2),
        user_defined_enable_3: u8_to_bool(user_defined_enable_3),
        user_defined_enable_4: u8_to_bool(user_defined_enable_4),
        calibrated_time_indicator: u8_to_bool(calibrated_time_indicator),
        valid_data_indicator: u8_to_bool(valid_data_indicator),
        reference_lock_indicator: u8_to_bool(reference_lock_indicator),
        agcmgc_indicator: u8_to_bool(agcmgc_indicator),
        detected_signal_indicator: u8_to_bool(detected_signal_indicator),
        spectral_inversion_indicator: u8_to_bool(spectral_inversion_indicator),
        overrange_indicator: u8_to_bool(overrange_indicator),
        sample_loss_indicator: u8_to_bool(sample_loss_indicator),
        user_defined_indicator_1: u8_to_bool(user_defined_indicator_1),
        user_defined_indicator_2: u8_to_bool(user_defined_indicator_2),
        user_defined_indicator_3: u8_to_bool(user_defined_indicator_3),
        user_defined_indicator_4: u8_to_bool(user_defined_indicator_4),
        associated_context_packet_count_enable: u8_to_bool(associated_context_packet_count_enable),
        associated_context_packet_count: associated_context_packet_count_value,
    };
    Ok((i, hdr))
}
