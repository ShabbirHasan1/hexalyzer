use intelhexlib::{IntelHex, IntelHexError, IntelHexErrorKind};
use std::fs;

#[test]
fn test_loading_and_writing_hex_1() {
    // Define in/out paths
    let input_path = "tests/fixtures/ih_valid_2.hex";
    let output_path = "build/t1/ih.hex";

    // Load hex and check the result
    let res = IntelHex::from_hex(input_path);
    assert!(res.is_ok());

    // If loaded Ok -> write it back to the disk
    if let Ok(mut ih) = res {
        let res = ih.write_hex(output_path);
        assert!(res.is_ok());

        // Load them in memory (small files -> OK)
        let f1 = fs::read(input_path);
        let f2 = fs::read(output_path);

        // Verify both are Ok and their contents match
        assert!(f1.is_ok());
        assert!(f2.is_ok());
        assert!(f1.is_ok_and(|content1| f2.is_ok_and(|content2| content1 == content2)));
    }
}

#[test]
fn test_loading_and_writing_hex_2() {
    // Define in/out paths
    let input_path = "tests/fixtures/ih_valid_2.hex";
    let output_path = "build/t2/ih.hex";

    // Load hex and check the resut
    let mut ih = IntelHex::new();
    let res = ih.load_hex(input_path);
    assert!(res.is_ok());

    let res = ih.write_hex(output_path);
    assert!(res.is_ok());

    // Load them in memory (small files -> OK)
    let f1 = fs::read(input_path);
    let f2 = fs::read(output_path);

    // Verify both are Ok and their contents match
    assert!(f1.is_ok());
    assert!(f2.is_ok());
    assert!(f1.is_ok_and(|content1| f2.is_ok_and(|content2| content1 == content2)));
}

#[test]
#[allow(clippy::panic)]
fn test_hex_parsing_returns_error() {
    // Define in/out paths
    let input_path = "tests/fixtures/ih_bad_checksum.hex";

    // Parse hex file
    let res = IntelHex::from_hex(input_path);

    // Check the error
    match res {
        Err(e) => {
            if let Some(ih_err) = e.downcast_ref::<IntelHexError>() {
                assert_eq!(
                    ih_err,
                    &IntelHexError::ParseRecordError(
                        IntelHexErrorKind::RecordChecksumMismatch(0x55, 0xFF),
                        1
                    )
                );
            } else {
                panic!("Error was not an IntelHexError");
            }
        }
        Ok(_) => panic!("Expected an error, but got Ok"),
    }
}
