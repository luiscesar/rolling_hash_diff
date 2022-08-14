use std::{fs::File, io::{BufWriter, Write}};

use rolling_hash_diff::rdiff::{Rdiff, constants::BLOCK_SIZE};

use crate::rdiff::{COMMAND, util::now_as_millis};

#[test]
pub fn integration_test_rdiff_main_signature_case1() {
    // Get file names
    let file_name = format!("resources/test_signature_case1.{}.txt",now_as_millis());
    let signature_file_name = format!("{}.sig", file_name);

    // Create test file
    {
        let file = File::create(file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {input_data.push(b'a');}
        for i in 0..BLOCK_SIZE {input_data.push(b'b');}
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x).unwrap();()});
    }  

    // Execute command 
    let option = "signature";
    let mut args:Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();

    // Verify result
    assert_eq!(rdiff_main_result, ());
}

#[test]
pub fn integration_test_rdiff_main_signature_case2() {
    // Get file names
    let file_name = format!("resources/test_signature_case2.{}.txt",now_as_millis());
    let signature_file_name = format!("{}.sig", file_name);

    // Create test file
    {
        let file = File::create(file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {input_data.push(b'a');}
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x).unwrap();()});
    }  

    // Execute command 
    let option = "signature";
    let mut args:Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();

    // Verify result
    assert_eq!(rdiff_main_result, ());
}
