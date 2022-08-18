use crate::rdiff::error::messages::HELP_USAGE;

use super::{messages::FILE_NOT_FOUND, RollingHashError};

#[test]
fn test_rolling_hash_error_case1() {
    let error = RollingHashError::new(FILE_NOT_FOUND);
    assert_eq!(error.to_string(), FILE_NOT_FOUND);
}

#[test]
fn test_rdiff_error_case1() {
    let rdiff_error = RollingHashError::rdiff_error(FILE_NOT_FOUND);
    assert_eq!(rdiff_error.to_string(), FILE_NOT_FOUND);
}

#[test]
fn test_rdiff_error_case2() {
    let rdiff_error = RollingHashError::rdiff_error(HELP_USAGE);
    assert_eq!(rdiff_error.to_string(), HELP_USAGE);
    eprintln!("{}", HELP_USAGE);
}
