use std::{fs::File, io::{BufWriter, Write, BufReader, Read}};

use crate::rdiff::{hash::{strong::rdiff_sha1::RdiffSha1, weak::rdiff_addler::RdiffAddler}, signature::Signature, delta::{Delta, ChunkDelta}, util::now_as_millis, constants::BLOCK_SIZE};


#[test]
fn test_delta_generate_delta_case1() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case1.{}", now_as_millis()); 
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
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

    // Create new file version
    // Equals to the old one
    {
        let old_file = File::open(file_name).unwrap();
        let mut reader = BufReader::new(old_file);
        let new_file = File::create(new_file_name.clone()).unwrap();
        let mut writer = BufWriter::new(new_file);
        let mut buffer: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
        loop {
            let size = reader.read(&mut buffer).unwrap();
            println!("buffer {:?}", &buffer[0..size]);
            writer.write(&buffer[..size]).unwrap();
            if size == 0 {break;}
        }
    }
    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    
    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(new_file_name.as_str(),signature,weak_hash_ptr,strong_hash_ptr).unwrap();
    println!("delta {:?}", delta);

    // Set expected values
    let mut chunk_delta_list:Vec<ChunkDelta> = Vec::new();
    chunk_delta_list.push(ChunkDelta::Match(1)); 
    chunk_delta_list.push(ChunkDelta::Match(2)); 
    chunk_delta_list.push(ChunkDelta::Match(3)); 
    let expected_delta = Delta{chunk_delta_list};

    // Verify computed values
    assert_eq!(delta, expected_delta);

}

#[test]
fn test_delta_generate_delta_case2() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case2.{}", now_as_millis()); 
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
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

    // Create new file version
    // By removing second chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();                            
        let mut writer = BufWriter::new(file);
        let mut input_data:Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {input_data.push(b'a');}
        for i in 0..BLOCK_SIZE-1 {input_data.push(b'c');}
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {writer.write(x).unwrap();()});
    }
    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    
    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(new_file_name.as_str(),signature,weak_hash_ptr,strong_hash_ptr).unwrap();
    println!("delta {:?}", delta);

    // Set expected values
    let mut chunk_delta_list:Vec<ChunkDelta> = Vec::new();
    chunk_delta_list.push(ChunkDelta::Match(1)); 
    chunk_delta_list.push(ChunkDelta::Match(3)); 
    let expected_delta = Delta{chunk_delta_list};

    // Verify computed values
    assert_eq!(delta, expected_delta);

}

#[test]
fn test_delta_generate_delta_case11() {
    let file_name_new = "resources/test.v2.txt";
    let file_name_old = "resources/test.txt";
    let signature_file_name = "resources/test.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let delta = Delta::generate_delta(file_name_new,signature,weak_hash_ptr,strong_hash_ptr).unwrap();

    println!("delta {:?}", delta);
}

#[test]
fn test_delta_generate_delta_case22() {
    let file_name_new = "resources/poem.v2.txt";
    let file_name_old = "resources/poem.txt";
    let signature_file_name = "resources/poem.v2.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let delta = Delta::generate_delta(file_name_new,signature,weak_hash_ptr,strong_hash_ptr).unwrap();

    println!("delta {:?}", delta);
}

#[test]
fn test_delta_generate_delta_case3() {
    let file_name_new = "resources/poem.v3.txt";
    let file_name_old = "resources/poem.txt";
    let signature_file_name = "resources/poem.v3.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let delta = Delta::generate_delta(file_name_new,signature,weak_hash_ptr,strong_hash_ptr).unwrap();

    println!("delta {:?}", delta);
}

#[test]
fn test_delta_create_delta_file_case1() {
    // Create signature file
    let file_name_old = "resources/test.txt";
    let signature_file_name = "resources/test-delta2.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let signature_result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(signature_result, ());

    // Create delta file
    let file_name_new = "resources/test.v2.txt";
    let delta_file_name = "resources/test2.delta";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Delta::create_delta_file(file_name_new, 
            delta_file_name, 
            signature_file_name, 
            weak_hash_ptr, 
            strong_hash_ptr).unwrap();
    assert_eq!(result,());
}