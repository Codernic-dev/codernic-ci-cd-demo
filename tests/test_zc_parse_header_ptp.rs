// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Zero-Copy Packet Header Parser
//! Story: bench-zero-copy-parser | Code Slot: zc_parse_header

#[path = "../backend-rust/src/zero_copy_errors.rs"]
pub mod zero_copy_errors;

#[path = "../backend-rust/src/zero_copy_header.rs"]
pub mod zero_copy_header;

use zero_copy_errors::ParseError;
use zero_copy_header::{
    compute_checksum, zc_parse_header, PacketHeader, PacketType, FLAG_COMPRESSED, FLAG_ENCRYPTED,
    FLAG_PRIORITY, HEADER_SIZE, MAGIC, PROTOCOL_VERSION,
};

fn create_test_packet(seq: u32, len: u32, ptype: PacketType, flags: u8) -> [u8; HEADER_SIZE] {
    let mut raw = [0u8; HEADER_SIZE];
    raw[0..4].copy_from_slice(MAGIC);
    raw[4] = PROTOCOL_VERSION;
    raw[5] = ptype.to_u8();
    raw[6] = flags;
    raw[8..12].copy_from_slice(&seq.to_be_bytes());
    raw[12..16].copy_from_slice(&len.to_be_bytes());
    let csum = compute_checksum(&raw[0..16]);
    raw[16..20].copy_from_slice(&csum.to_be_bytes());
    raw
}

fn test_valid_header_and_zero_copy() {
    let raw = create_test_packet(42, 512, PacketType::Data, FLAG_COMPRESSED);
    let (hdr, rem) = PacketHeader::parse_with_remainder(&raw).expect("parse failed");
    assert_eq!(hdr.version, PROTOCOL_VERSION);
    assert_eq!(hdr.packet_type, PacketType::Data);
    assert_eq!(hdr.sequence_id, 42);
    assert_eq!(hdr.payload_len, 512);
    assert_eq!(hdr.magic, MAGIC);
    assert!(hdr.is_flag_set(FLAG_COMPRESSED));
    assert!(!hdr.is_flag_set(FLAG_ENCRYPTED));
    assert_eq!(rem.len(), 0);

    let copy_check = hdr;
    assert_eq!(copy_check, hdr);
}

fn test_buffer_boundary_conditions() {
    let empty_buf: &[u8] = &[];
    assert_eq!(
        PacketHeader::parse(empty_buf),
        Err(ParseError::BufferTooSmall { needed: HEADER_SIZE, available: 0 })
    );

    let partial_buf = [0u8; 15];
    assert_eq!(
        PacketHeader::parse(&partial_buf),
        Err(ParseError::BufferTooSmall { needed: HEADER_SIZE, available: 15 })
    );
}

fn test_header_validation_errors() {
    let mut bad_magic = create_test_packet(1, 10, PacketType::Ack, 0);
    bad_magic[0] = b'Z';
    assert!(matches!(
        PacketHeader::parse(&bad_magic),
        Err(ParseError::InvalidHeader { offset: 0, .. })
    ));

    let mut bad_ver = create_test_packet(1, 10, PacketType::Ack, 0);
    bad_ver[4] = 9;
    assert_eq!(
        PacketHeader::parse(&bad_ver),
        Err(ParseError::UnexpectedByte { expected: PROTOCOL_VERSION, found: 9, offset: 4 })
    );

    let mut bad_type = create_test_packet(1, 10, PacketType::Ack, 0);
    bad_type[5] = 255;
    assert_eq!(
        PacketHeader::parse(&bad_type),
        Err(ParseError::UnexpectedByte { expected: 1, found: 255, offset: 5 })
    );
}

fn test_checksum_handling() {
    let mut corrupted = create_test_packet(10, 100, PacketType::Heartbeat, 0);
    corrupted[16] ^= 0xff;
    assert!(matches!(
        PacketHeader::parse(&corrupted),
        Err(ParseError::ChecksumMismatch { offset: 16, .. })
    ));

    let mut zero_csum = create_test_packet(10, 100, PacketType::Heartbeat, 0);
    zero_csum[16..20].fill(0);
    let parsed_zero = PacketHeader::parse(&zero_csum).expect("zero checksum should be allowed");
    assert_eq!(parsed_zero.checksum, 0);
}

fn test_serialization_and_roundtrip() {
    let original = PacketHeader {
        magic: MAGIC,
        version: PROTOCOL_VERSION,
        packet_type: PacketType::Control,
        flags: FLAG_ENCRYPTED | FLAG_PRIORITY,
        sequence_id: 999,
        payload_len: 1024,
        checksum: 1,
    };
    let mut out_buf = [0u8; HEADER_SIZE];
    let written = original.serialize_into(&mut out_buf).expect("serialize failed");
    assert_eq!(written, HEADER_SIZE);

    let (deserialized, _) = PacketHeader::parse_with_remainder(&out_buf).expect("roundtrip parse failed");
    assert_eq!(deserialized.packet_type, PacketType::Control);
    assert_eq!(deserialized.sequence_id, 999);
    assert_eq!(deserialized.payload_len, 1024);
    assert!(deserialized.is_flag_set(FLAG_ENCRYPTED));
    assert!(deserialized.is_flag_set(FLAG_PRIORITY));
}

// code_slot: zc_parse_header
#[test]
fn test_zc_parse_header() {
    let slot = "zc_parse_header";
    assert_eq!(slot, "zc_parse_header");

    zc_parse_header();
    test_valid_header_and_zero_copy();
    test_buffer_boundary_conditions();
    test_header_validation_errors();
    test_checksum_handling();
    test_serialization_and_roundtrip();
}
