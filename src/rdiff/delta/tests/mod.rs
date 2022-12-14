use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
};

use bincode::deserialize_from;

use crate::rdiff::{
    constants::BLOCK_SIZE,
    delta::{ChunkDelta, Delta},
    hash::{strong::rdiff_sha1::RdiffSha1, weak::rdiff_addler::RdiffAddler},
    signature::Signature,
    util::now_as_millis,
};

#[test]
fn test_delta_generate_delta_equals_files_case1() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case1.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    // Create old file
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
            writer.write(x);
            ()
        });
    }
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

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
            writer.write(&buffer[..size]).unwrap();
            if size == 0 {
                break;
            }
        }
    }
    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta { chunk_delta_list };

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
}

#[test]
fn test_delta_generate_delta_chunk_removed_case2() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case2.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By removing second chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();
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
    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta { chunk_delta_list };

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
}

#[test]
fn test_delta_generate_delta_chunk_changed_case3() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case3.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By changing first chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'd');
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
    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    let mut differences: Vec<u8> = Vec::new();
    for _ in 0..BLOCK_SIZE {
        differences.push(b'd');
    }
    chunk_delta_list.push(ChunkDelta::Diff(differences));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta { chunk_delta_list };

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verifation
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
}

#[test]
fn test_delta_generate_delta_chunk_shifted_case4() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case4.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By shifted first chunk
    let shifted_size = 20;
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..shifted_size {
            input_data.push(b'd');
        }
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
    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    let mut differences: Vec<u8> = Vec::new();
    for _ in 0..shifted_size {
        differences.push(b'd');
    }
    chunk_delta_list.push(ChunkDelta::Diff(differences));
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta { chunk_delta_list };

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
}

#[test]
fn test_delta_generate_delta_addition_between_chunks_case5() {
    // Get file names
    let prefix_file_name = format!("resources/test_generate_delta_case5.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By addition between chunks
    let addition_size = 30;
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for _ in 0..addition_size {
            input_data.push(b'd');
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

    // Compute Delta
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    let mut differences: Vec<u8> = Vec::new();
    for _ in 0..addition_size {
        differences.push(b'd');
    }
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Diff(differences));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta { chunk_delta_list };

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
}

#[test]
fn test_delta_create_delta_file_equals_files_case1() {
    // Get file names
    let prefix_file_name = format!("resources/test_create_delta_file_case1.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

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
            writer.write(&buffer[..size]).unwrap();
            if size == 0 {
                break;
            }
        }
    }
    // Create delta file
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    let result = Delta::create_delta_file(
        new_file_name.as_str(),
        delta_file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);

    // Get delta from file
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let expected_delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
    fs::remove_file(delta_file_name).unwrap();
}

#[test]
fn test_delta_create_delta_file_chunk_removed_case2() {
    // Get file names
    let prefix_file_name = format!("resources/test_create_delta_file_case2.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By removing second chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();
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

    // Create delta file
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    let result = Delta::create_delta_file(
        new_file_name.as_str(),
        delta_file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    // Get Delta from file
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let expected_delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
    fs::remove_file(delta_file_name).unwrap();
}

#[test]
fn test_delta_create_delta_file_chunk_changed_case3() {
    // Get file names
    let prefix_file_name = format!("resources/test_create_delta_file_case3.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By changing first chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'd');
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

    // Create delta file
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    let result = Delta::create_delta_file(
        new_file_name.as_str(),
        delta_file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    // Get Delta from file
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let expected_delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
    fs::remove_file(delta_file_name).unwrap();
}

#[test]
fn test_delta_create_delta_file_chunk_shifted_case4() {
    // Get file names
    let prefix_file_name = format!("resources/test_create_delta_file_case4.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By shifted first chunk
    let shifted_size = 20;
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..shifted_size {
            input_data.push(b'd');
        }
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

    // Create delta file
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    let result = Delta::create_delta_file(
        new_file_name.as_str(),
        delta_file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    // Get Delta from file
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let expected_delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
    fs::remove_file(delta_file_name).unwrap();
}

#[test]
fn test_delta_create_delta_file_addition_between_chunks_case5() {
    // Get file names
    let prefix_file_name = format!("resources/test_create_delta_file_case5.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
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
    // Create signature file
    // Get hash functions for create signature file
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Create signature file
    let result = Signature::create_signature_file(
        file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Create new file version
    // By addition between chunks
    let addition_size = 30;
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for _ in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for _ in 0..addition_size {
            input_data.push(b'd');
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

    // Create delta file
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    let result = Delta::create_delta_file(
        new_file_name.as_str(),
        delta_file_name.as_str(),
        signature_file_name.as_str(),
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();
    assert_eq!(result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    // Get Delta from file
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    // Get hash functions for Delta
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();

    // Get signature from file
    let signature = Signature::get_signature_from_file(signature_file_name.as_str()).unwrap();

    // Get Delta
    let expected_delta = Delta::generate_delta(
        new_file_name.as_str(),
        signature,
        weak_hash_ptr,
        strong_hash_ptr,
    )
    .unwrap();

    // Verify computed values
    assert_eq!(delta, expected_delta);

    // Clean up verification
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    fs::remove_file(file_name).unwrap();
    fs::remove_file(signature_file_name).unwrap();
    fs::remove_file(new_file_name).unwrap();
    fs::remove_file(delta_file_name).unwrap();
}
