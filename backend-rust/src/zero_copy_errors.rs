// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Exhaustive Zero-Copy Parsing Error Enumeration
//! Story: bench-zero-copy-parser | Code Slot: zc_parse_errors

/// Exhaustive non-allocating parsing errors for zero-copy deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError<'a> {
    /// Unexpected end of input when more data was required.
    UnexpectedEof { expected_bytes: usize },
    /// Found an unexpected byte at the given offset.
    UnexpectedByte { expected: u8, found: u8, offset: usize },
    /// An unrecognized or invalid token string slice.
    InvalidToken { token: &'a str, offset: usize },
    /// Non UTF-8 byte sequence encountered.
    InvalidUtf8 { offset: usize, valid_up_to: usize },
    /// Numeric literal parsing failed on slice.
    InvalidNumber { raw: &'a str, offset: usize },
    /// Provided buffer is smaller than minimum required size.
    BufferTooSmall { needed: usize, available: usize },
    /// Expected delimiter was not found up to offset.
    DelimiterNotFound { delimiter: u8, offset: usize },
    /// Magic bytes or header prefix mismatch.
    InvalidHeader { header: &'a [u8], offset: usize },
    /// Checksum verification failed.
    ChecksumMismatch { expected: u32, computed: u32, offset: usize },
    /// Recursive structure exceeds maximum nesting depth.
    NestingOverflow { max_depth: usize, offset: usize },
    /// Extraneous trailing bytes remain after complete parse.
    TrailingBytes { remaining: usize, offset: usize },
}

pub type ZeroCopyError<'a> = ParseError<'a>;

impl<'a> ParseError<'a> {
    pub fn offset(&self) -> Option<usize> {
        match self {
            Self::UnexpectedEof { .. } | Self::BufferTooSmall { .. } => None,
            Self::UnexpectedByte { offset, .. }
            | Self::InvalidToken { offset, .. }
            | Self::InvalidUtf8 { offset, .. }
            | Self::InvalidNumber { offset, .. }
            | Self::DelimiterNotFound { offset, .. }
            | Self::InvalidHeader { offset, .. }
            | Self::ChecksumMismatch { offset, .. }
            | Self::NestingOverflow { offset, .. }
            | Self::TrailingBytes { offset, .. } => Some(*offset),
        }
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Self::UnexpectedEof { .. })
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::UnexpectedEof { .. } => "UnexpectedEof",
            Self::UnexpectedByte { .. } => "UnexpectedByte",
            Self::InvalidToken { .. } => "InvalidToken",
            Self::InvalidUtf8 { .. } => "InvalidUtf8",
            Self::InvalidNumber { .. } => "InvalidNumber",
            Self::BufferTooSmall { .. } => "BufferTooSmall",
            Self::DelimiterNotFound { .. } => "DelimiterNotFound",
            Self::InvalidHeader { .. } => "InvalidHeader",
            Self::ChecksumMismatch { .. } => "ChecksumMismatch",
            Self::NestingOverflow { .. } => "NestingOverflow",
            Self::TrailingBytes { .. } => "TrailingBytes",
        }
    }
}

impl<'a> core::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpectedEof { expected_bytes } => {
                write!(f, "unexpected EOF, expected {expected_bytes} bytes")
            }
            Self::UnexpectedByte { expected, found, offset } => {
                write!(f, "unexpected byte {found} (expected {expected}) at offset {offset}")
            }
            Self::InvalidToken { token, offset } => {
                write!(f, "invalid token '{token}' at offset {offset}")
            }
            Self::InvalidUtf8 { offset, valid_up_to } => {
                write!(f, "invalid UTF-8 at offset {offset}, valid up to {valid_up_to}")
            }
            Self::InvalidNumber { raw, offset } => {
                write!(f, "invalid number '{raw}' at offset {offset}")
            }
            Self::BufferTooSmall { needed, available } => {
                write!(f, "buffer too small: need {needed}, available {available}")
            }
            Self::DelimiterNotFound { delimiter, offset } => {
                write!(f, "delimiter byte {delimiter} not found from offset {offset}")
            }
            Self::InvalidHeader { header, offset } => {
                write!(f, "invalid header prefix of len {} at offset {offset}", header.len())
            }
            Self::ChecksumMismatch { expected, computed, offset } => {
                write!(f, "checksum error at offset {offset}: expected {expected}, got {computed}")
            }
            Self::NestingOverflow { max_depth, offset } => {
                write!(f, "nesting depth {max_depth} exceeded at offset {offset}")
            }
            Self::TrailingBytes { remaining, offset } => {
                write!(f, "trailing {remaining} bytes at offset {offset}")
            }
        }
    }
}

impl<'a> std::error::Error for ParseError<'a> {}

// code_slot: zc_parse_errors
pub fn zc_parse_errors() {
    let err_eof = ParseError::UnexpectedEof { expected_bytes: 4 };
    assert!(err_eof.is_eof());
    assert_eq!(err_eof.offset(), None);
    assert_eq!(err_eof.kind_name(), "UnexpectedEof");

    let err_byte = ParseError::UnexpectedByte {
        expected: b':',
        found: b'=',
        offset: 12,
    };
    assert!(!err_byte.is_eof());
    assert_eq!(err_byte.offset(), Some(12));
    assert_eq!(err_byte.kind_name(), "UnexpectedByte");

    let err_tok = ParseError::InvalidToken {
        token: "KEYWORD",
        offset: 35,
    };
    assert_eq!(err_tok.offset(), Some(35));
    assert_eq!(err_tok.kind_name(), "InvalidToken");

    let err_hdr = ParseError::InvalidHeader {
        header: b"HDR",
        offset: 0,
    };
    assert_eq!(err_hdr.offset(), Some(0));

    let copy_check = err_eof;
    assert_eq!(copy_check, err_eof);
}
