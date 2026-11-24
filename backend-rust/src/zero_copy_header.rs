// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Zero-Copy Network Packet Header Parser
//! Story: bench-zero-copy-parser | Code Slot: zc_parse_header

use crate::zero_copy_errors::ParseError;

pub const MAGIC: &[u8; 4] = b"CDNC";
pub const HEADER_SIZE: usize = 20;
pub const PROTOCOL_VERSION: u8 = 1;

pub const FLAG_NONE: u8 = 0;
pub const FLAG_COMPRESSED: u8 = 1;
pub const FLAG_ENCRYPTED: u8 = 2;
pub const FLAG_PRIORITY: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Data,
    Ack,
    Heartbeat,
    Control,
    Reset,
}

impl PacketType {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(Self::Data),
            2 => Some(Self::Ack),
            3 => Some(Self::Heartbeat),
            4 => Some(Self::Control),
            5 => Some(Self::Reset),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            Self::Data => 1,
            Self::Ack => 2,
            Self::Heartbeat => 3,
            Self::Control => 4,
            Self::Reset => 5,
        }
    }
}

pub fn compute_checksum(bytes: &[u8]) -> u32 {
    let mut sum1: u32 = 1;
    let mut sum2: u32 = 0;
    for &b in bytes {
        sum1 = (sum1 + b as u32) % 65521;
        sum2 = (sum2 + sum1) % 65521;
    }
    (sum2 << 16) | sum1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader<'a> {
    pub magic: &'a [u8],
    pub version: u8,
    pub packet_type: PacketType,
    pub flags: u8,
    pub sequence_id: u32,
    pub payload_len: u32,
    pub checksum: u32,
}

fn validate_prefix<'a>(buf: &'a [u8]) -> Result<(), ParseError<'a>> {
    if buf.len() < HEADER_SIZE {
        return Err(ParseError::BufferTooSmall { needed: HEADER_SIZE, available: buf.len() });
    }
    if &buf[0..4] != MAGIC {
        return Err(ParseError::InvalidHeader { header: &buf[0..4], offset: 0 });
    }
    if buf[4] != PROTOCOL_VERSION {
        return Err(ParseError::UnexpectedByte {
            expected: PROTOCOL_VERSION,
            found: buf[4],
            offset: 4,
        });
    }
    Ok(())
}

fn verify_checksum_field<'a>(checksum: u32, buf: &'a [u8]) -> Result<(), ParseError<'a>> {
    if checksum != 0 {
        let computed = compute_checksum(&buf[0..16]);
        if checksum != computed {
            return Err(ParseError::ChecksumMismatch { expected: checksum, computed, offset: 16 });
        }
    }
    Ok(())
}

impl<'a> PacketHeader<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Self, ParseError<'a>> {
        let (header, _) = Self::parse_with_remainder(buf)?;
        Ok(header)
    }

    pub fn parse_with_remainder(buf: &'a [u8]) -> Result<(Self, &'a [u8]), ParseError<'a>> {
        validate_prefix(buf)?;

        let packet_type = PacketType::from_u8(buf[5]).ok_or(ParseError::UnexpectedByte {
            expected: 1,
            found: buf[5],
            offset: 5,
        })?;

        let flags = buf[6];
        let sequence_id = u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]);
        let payload_len = u32::from_be_bytes([buf[12], buf[13], buf[14], buf[15]]);
        let checksum = u32::from_be_bytes([buf[16], buf[17], buf[18], buf[19]]);
        verify_checksum_field(checksum, buf)?;

        let header = Self {
            magic: &buf[0..4],
            version: buf[4],
            packet_type,
            flags,
            sequence_id,
            payload_len,
            checksum,
        };
        Ok((header, &buf[HEADER_SIZE..]))
    }

    pub fn is_flag_set(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }

    pub fn serialize_into(&self, out: &mut [u8]) -> Result<usize, ParseError<'a>> {
        if out.len() < HEADER_SIZE {
            return Err(ParseError::BufferTooSmall { needed: HEADER_SIZE, available: out.len() });
        }
        out[0..4].copy_from_slice(MAGIC);
        out[4] = self.version;
        out[5] = self.packet_type.to_u8();
        out[6] = self.flags;
        out[7] = 0;
        out[8..12].copy_from_slice(&self.sequence_id.to_be_bytes());
        out[12..16].copy_from_slice(&self.payload_len.to_be_bytes());
        let csum = if self.checksum != 0 { compute_checksum(&out[0..16]) } else { 0 };
        out[16..20].copy_from_slice(&csum.to_be_bytes());
        Ok(HEADER_SIZE)
    }
}

// code_slot: zc_parse_header
pub fn zc_parse_header() {
    let mut raw = [0u8; HEADER_SIZE];
    raw[0..4].copy_from_slice(MAGIC);
    raw[4] = PROTOCOL_VERSION;
    raw[5] = PacketType::Data.to_u8();
    raw[6] = FLAG_PRIORITY;
    raw[8..12].copy_from_slice(&100u32.to_be_bytes());
    raw[12..16].copy_from_slice(&64u32.to_be_bytes());
    let csum = compute_checksum(&raw[0..16]);
    raw[16..20].copy_from_slice(&csum.to_be_bytes());

    let (hdr, rem) = PacketHeader::parse_with_remainder(&raw).expect("valid parse");
    assert_eq!(hdr.version, PROTOCOL_VERSION);
    assert_eq!(hdr.packet_type, PacketType::Data);
    assert_eq!(hdr.sequence_id, 100);
    assert_eq!(hdr.payload_len, 64);
    assert_eq!(hdr.checksum, csum);
    assert!(hdr.is_flag_set(FLAG_PRIORITY));
    assert_eq!(rem.len(), 0);

    let too_short = [0u8; 10];
    assert!(matches!(PacketHeader::parse(&too_short), Err(ParseError::BufferTooSmall { .. })));
}
