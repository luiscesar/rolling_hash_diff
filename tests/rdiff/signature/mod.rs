use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
};

use rolling_hash_diff::rdiff::{
    constants::BLOCK_SIZE,
    error::{messages::HELP_USAGE, RollingHashError},
    Rdiff,
};

use crate::rdiff::{util::now_as_millis, COMMAND};

#[test]
pub fn integration_test_rdiff_main_signature_case1() {
    // Get file names
    let file_name = format!("resources/test_signature_case1.{}.txt", now_as_millis());
    let signature_file_name = format!("{}.sig", file_name);

    // Create test file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for _ in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x).unwrap();
            ()
        });
    }

    // Execute command
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();

    // Verify computed value
    assert_eq!(rdiff_main_result, ());

    // Clean up verification
    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
}

#[test]
pub fn integration_test_rdiff_main_signature_case2() {
    // Get file names
    let file_name = format!("resources/test_signature_case2.{}.txt", now_as_millis());
    let signature_file_name = format!("{}.sig", file_name);

    // Create test file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for _ in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x).unwrap();
            ()
        });
    }

    // Execute command
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();

    // Verify computed value
    assert_eq!(rdiff_main_result, ());

    // Clean up verification
    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
}

#[test]
fn integration_test_rdiff_main_signature_error_no_option_case1() {
    // Get file names
    let file_name = format!(
        "resources/test_signature_error_case1.{}.txt",
        now_as_millis()
    );
    let signature_file_name = format!("{}.sig", file_name);

    // Execute command
    //let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    //args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let error = Rdiff::main_rdiff(args).unwrap_err();

    // Set expected value
    let expected_error = RollingHashError::new(HELP_USAGE);
    // Verify computed value
    assert_eq!(error, expected_error);
}

#[test]
fn integration_test_rdiff_main_signature_error_file_name_missing_case2() {
    // Get file names
    let file_name = format!(
        "resources/test_signature_error_case2.{}.txt",
        now_as_millis()
    );
    let signature_file_name = format!("{}.sig", file_name);

    // Execute command
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    //args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let error = Rdiff::main_rdiff(args).unwrap_err();
    // Set expected value
    let expected_error = RollingHashError::new(HELP_USAGE);
    // Verify computed value
    assert_eq!(error, expected_error);
}

#[test]
fn integration_test_rdiff_main_signature_error_file_missing_case3() {
    // Get file names
    let file_name = format!(
        "resources/test_signature_error_case3.{}.txt",
        now_as_millis()
    );
    let signature_file_name = format!("{}.sig", file_name);

    // Execute command
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let error = Rdiff::main_rdiff(args).unwrap_err();
    // Set expected value
    let expected_error = RollingHashError::from(Box::new(io::Error::from_raw_os_error(2)));
    // Verify computed value
    assert_eq!(error, expected_error);
}
