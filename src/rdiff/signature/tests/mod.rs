use std::{time::{SystemTime}, fmt::format, fs::File, io::{BufWriter, Write}};

use crate::rdiff::{hash::{strong::rdiff_sha1::RdiffSha1, weak::rdiff_addler::RdiffAddler}, util::now_as_millis, constants::BLOCK_SIZE};

use super::Signature;

#[test]
fn test_rdiff_signature_create_signature_case1() {
    let file_name = format!("resources/test_signature_case1.{}.txt",now_as_millis());

    let mut file_size = 0;
    {
        // Create test file
        let file = File::create(file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {input_data.push(b'a');}
        for i in 0..BLOCK_SIZE {input_data.push(b'b');}
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        file_size = input_data.len() as usize;
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x);()});
    }
    // Get hash functions
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature
    let signature = 
        Signature::create_signature(file_name.as_str(),weak_hash_ptr,strong_hash_ptr).unwrap();

    // Show signature
    let expected_chunk_size = compute_chunk_size(file_size);
    let expected_number_of_chunks:usize = 
        (((file_size as f64) / (expected_chunk_size as f64)).trunc()) as usize;
    let expected_last_chunk_size = file_size - expected_number_of_chunks  * expected_chunk_size;
    assert_eq!(signature.get_chunk_size(), expected_chunk_size);
    assert_eq!(signature.get_last_chunk_size(), expected_last_chunk_size);
}

#[test]
fn test_rdiff_signature_create_signature_case2() {
   let file_name = format!("resources/test_signature_case2.{}.txt",now_as_millis());
   let mut file_size = 0;
   {
        // Create test file
        let file = File::create(file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        file_size = input_data.len() as usize;
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x);()});
    }
    // Get hash functions
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature
    let signature = 
        Signature::create_signature(file_name.as_str(),weak_hash_ptr,strong_hash_ptr).unwrap();

    // Show signature
    let expected_chunk_size = compute_chunk_size(file_size);
    let expected_number_of_chunks:usize = 
        (((file_size as f64) / (expected_chunk_size as f64)).trunc()) as usize;
    let expected_last_chunk_size = file_size - expected_number_of_chunks  * expected_chunk_size;
    assert_eq!(signature.get_chunk_size(), expected_chunk_size);
    assert_eq!(signature.get_last_chunk_size(), expected_last_chunk_size);
}

#[test]
fn test_rdiff_signature_create_signature_file_case1() {
    let file_name = format!("resources/test_signature_file_case1.{}.txt",now_as_millis());
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
        chunks.for_each(|x| {writer.write(x);()});
    }

    // Get hash functions
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = 
        Signature::create_signature_file(file_name.as_str(), 
                                        signature_file_name.as_str(), 
                                        weak_hash_ptr, 
                                        strong_hash_ptr).unwrap();
    assert_eq!(result,());     
    
}

#[test]
fn test_rdiff_signature_create_signature_file_case2() {
    let file_name = format!("resources/test_signature_file_case2.{}.txt",now_as_millis());
    let signature_file_name = format!("{}.sig", file_name);

    // Create test file
    {
        let file = File::create(file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x);()});
    }

    // Get hash functions
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    
    // Create signature file
    let result = 
        Signature::create_signature_file(file_name.as_str(), 
                                        signature_file_name.as_str(), 
                                        weak_hash_ptr, 
                                        strong_hash_ptr).unwrap();
    assert_eq!(result,());     
}

#[test]
fn test_rdiff_signature_get_signature_from_file_case1() {
    // Get file names
    let file_name = format!("resources/test_get_signature_case1.{}.txt",now_as_millis());
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
        chunks.for_each(|x| {writer.write(x);()});
    }

    // Get hash functions for create signature
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature
    let expected_signature = 
        Signature::create_signature(file_name.as_str(),weak_hash_ptr,strong_hash_ptr).unwrap();

    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = 
        Signature::create_signature_file(file_name.as_str(), 
                                        signature_file_name.as_str(), 
                                        weak_hash_ptr, 
                                        strong_hash_ptr).unwrap();
    assert_eq!(result,());
    
    // Get signature from file
    let computed_signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Compare results
    assert_eq!(computed_signature, expected_signature);
}

#[test]
fn test_rdiff_signature_get_signature_from_file_case2() {
    // Get file names
    let file_name = format!("resources/test_get_signature_case2.{}.txt",now_as_millis());
    let signature_file_name = format!("{}.sig", file_name);

    // Create test file
    {
        let file = File::create(file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x);()});
    }

    // Get hash functions for create signature
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature
    let expected_signature = 
        Signature::create_signature(file_name.as_str(),weak_hash_ptr,strong_hash_ptr).unwrap();

    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = 
        Signature::create_signature_file(file_name.as_str(), 
                                        signature_file_name.as_str(), 
                                        weak_hash_ptr, 
                                        strong_hash_ptr).unwrap();
    assert_eq!(result,());
    
    // Get signature from file
    let computed_signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Compare results
    assert_eq!(computed_signature, expected_signature);
}

fn compute_chunk_size(file_size:usize) -> usize {
    if file_size > 1 {
        if file_size > BLOCK_SIZE {
            BLOCK_SIZE
        } else {
            (((file_size as f64) / 2.0).round() ) as usize
        }
    } else {
        1
    }
}