//! Independent, fixed-byte regression contracts for untrusted filter chains.
//!
//! These vectors deliberately do not use PdfCraft's encoders as test oracles:
//! they check the behavior of the public decoder with known on-disk bytes.

use pdfcraft_filters::{Filter, FilterError, Params, decode, decode_tolerant};

fn hex_then_runlength() -> [(Filter, Params); 2] {
    [
        (Filter::AsciiHex, Params::default()),
        (Filter::RunLength, Params::default()),
    ]
}

#[test]
fn expanding_second_stage_must_respect_the_exact_output_limit() {
    // ASCIIHex produces [129, 0, 128]. RunLength expands the first two
    // bytes into exactly 128 zero bytes, then consumes the EOD marker.
    let chain = hex_then_runlength();
    let encoded = b"810080>";
    for max in [0, 1, 2, 3, 127] {
        assert!(matches!(
            decode(&chain, encoded, max),
            Err(FilterError::LimitExceeded(n)) if n == max
        ));
        assert!(matches!(
            decode_tolerant(&chain, encoded, max),
            Err(FilterError::LimitExceeded(n)) if n == max
        ));
    }
    assert_eq!(decode(&chain, encoded, 128).unwrap(), [0; 128]);
    assert_eq!(decode_tolerant(&chain, encoded, 128).unwrap(), (vec![0; 128], false));
}

#[test]
fn a_truncated_literal_run_is_an_error_in_strict_mode() {
    // ASCIIHex produces [3, 'A', 'B']; the RunLength header requires
    // four literal bytes, but only two exist.
    let chain = hex_then_runlength();
    let encoded = b"034142>";
    assert!(matches!(
        decode(&chain, encoded, 3),
        Err(FilterError::Corrupt { .. })
    ));
    assert_eq!(decode_tolerant(&chain, encoded, 3).unwrap(), (b"AB".to_vec(), true));
}

#[test]
fn two_corrupt_stages_preserve_only_the_valid_partial_output() {
    // Invalid 'G' terminates ASCIIHex after [1, 'A', 0, 'B'].
    // RunLength decodes the first two literals, then encounters a
    // truncated second run. The partial result must not contain 'B'.
    let chain = hex_then_runlength();
    let encoded = b"01410042GG";
    assert!(matches!(
        decode(&chain, encoded, 100),
        Err(FilterError::Corrupt { .. })
    ));
    assert_eq!(decode_tolerant(&chain, encoded, 100).unwrap(), (vec![b'A', 0], true));
}

#[test]
fn a_limit_violation_is_not_treated_as_recoverable_corruption() {
    let chain = [(Filter::AsciiHex, Params::default())];
    // A corrupt trailing character must not turn a prior limit error into success.
    for encoded in [&b"41424344GG"[..], &b"41424344>"[..]] {
        assert!(matches!(
            decode_tolerant(&chain, encoded, 3),
            Err(FilterError::LimitExceeded(3))
        ));
    }
}

#[test]
fn zero_limit_allows_only_empty_decoded_output() {
    let chain = [(Filter::AsciiHex, Params::default())];
    assert_eq!(decode(&chain, b">", 0).unwrap(), b"");
    assert_eq!(decode_tolerant(&chain, b">", 0).unwrap(), (vec![], false));
    assert!(matches!(
        decode(&chain, b"00>", 0),
        Err(FilterError::LimitExceeded(0))
    ));
    assert!(matches!(
        decode_tolerant(&chain, b"00>", 0),
        Err(FilterError::LimitExceeded(0))
    ));
}
