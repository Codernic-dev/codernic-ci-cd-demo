// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Zero-Copy Parsing Errors
//! Story: bench-zero-copy-parser | Code Slot: zc_parse_errors

#[path = "../backend-rust/src/zero_copy_errors.rs"]
mod zero_copy_errors;

use zero_copy_errors::{zc_parse_errors, ParseError, ZeroCopyError};

fn test_variant_construction_and_offsets() {
    let err_eof = ParseError::UnexpectedEof { expected_bytes: 8 };
    assert_eq!(err_eof.offset(), None);
    assert!(err_eof.is_eof());
    assert_eq!(err_eof.kind_name(), "UnexpectedEof");

    let err_byte = ParseError::UnexpectedByte {
        expected: b',',
        found: b';',
        offset: 14,
    };
    assert_eq!(err_byte.offset(), Some(14));
    assert!(!err_byte.is_eof());
    assert_eq!(err_byte.kind_name(), "UnexpectedByte");

    let err_tok = ParseError::InvalidToken {
        token: "SYNTAX",
        offset: 28,
    };
    assert_eq!(err_tok.offset(), Some(28));
    assert_eq!(err_tok.kind_name(), "InvalidToken");

    let err_utf = ParseError::InvalidUtf8 {
        offset: 40,
        valid_up_to: 38,
    };
    assert_eq!(err_utf.offset(), Some(40));
    assert_eq!(err_utf.kind_name(), "InvalidUtf8");

    let err_num = ParseError::InvalidNumber {
        raw: "NaN",
        offset: 50,
    };
    assert_eq!(err_num.offset(), Some(50));
    assert_eq!(err_num.kind_name(), "InvalidNumber");
}

fn test_advanced_variants() {
    let err_buf = ParseError::BufferTooSmall {
        needed: 64,
        available: 32,
    };
    assert_eq!(err_buf.offset(), None);
    assert_eq!(err_buf.kind_name(), "BufferTooSmall");

    let err_delim = ParseError::DelimiterNotFound {
        delimiter: b'\n',
        offset: 80,
    };
    assert_eq!(err_delim.offset(), Some(80));

    let err_hdr = ParseError::InvalidHeader {
        header: b"MAGIC",
        offset: 0,
    };
    assert_eq!(err_hdr.offset(), Some(0));

    let err_csum = ParseError::ChecksumMismatch {
        expected: 0x1234,
        computed: 0x5678,
        offset: 96,
    };
    assert_eq!(err_csum.offset(), Some(96));

    let err_nest = ParseError::NestingOverflow {
        max_depth: 16,
        offset: 110,
    };
    assert_eq!(err_nest.offset(), Some(110));

    let err_trail = ParseError::TrailingBytes {
        remaining: 5,
        offset: 120,
    };
    assert_eq!(err_trail.offset(), Some(120));
}

fn test_zero_copy_and_traits() {
    let token_data = "BORROWED_TOKEN";
    let err: ZeroCopyError = ParseError::InvalidToken {
        token: token_data,
        offset: 15,
    };

    // Verify Copy semantics (no heap allocation)
    let copied = err;
    assert_eq!(err, copied);

    // Verify std::error::Error trait object compatibility
    let error_dyn: &dyn std::error::Error = &err;
    let formatted = format!("{error_dyn}");
    assert!(formatted.contains("invalid token 'BORROWED_TOKEN'"));

    let err_eof = ParseError::UnexpectedEof { expected_bytes: 4 };
    let eof_fmt = format!("{err_eof}");
    assert!(eof_fmt.contains("unexpected EOF"));
}

// code_slot: zc_parse_errors
#[test]
fn test_zc_parse_errors() {
    let slot = "zc_parse_errors";
    assert_eq!(slot, "zc_parse_errors");

    zc_parse_errors();
    test_variant_construction_and_offsets();
    test_advanced_variants();
    test_zero_copy_and_traits();
}
