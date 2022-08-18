use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use bincode::deserialize_from;
use rolling_hash_diff::rdiff::{
    constants::BLOCK_SIZE,
    delta::{ChunkDelta, Delta},
    error::messages::{FILE_NOT_FOUND, HELP_USAGE},
    Rdiff,
};

use crate::rdiff::{util::now_as_millis, COMMAND};

#[test]
pub fn integration_test_rdiff_main_delta_equals_files_case1() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_file_case1.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x);
            ()
        });
    }

    // Create signature
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

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

    // Create delta
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta::new(chunk_delta_list);

    // Verify computed values
    assert_eq!(delta, expected_delta);
}

#[test]
pub fn integration_test_rdiff_main_delta_chunk_removed_case2() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_file_case2.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x);
            ()
        });
    }

    // Create signature
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Create new file version
    // By removing second chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x).unwrap();
            ()
        });
    }

    // Create delta
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta::new(chunk_delta_list);

    // Verify computed values
    assert_eq!(delta, expected_delta);
}

#[test]
pub fn integration_test_rdiff_main_delta_chunk_changed_case3() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_file_case3.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x);
            ()
        });
    }

    // Create signature
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Create new file version
    // By changing first chunk
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'd');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x).unwrap();
            ()
        });
    }

    // Create delta
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    let mut differences: Vec<u8> = Vec::new();
    for i in 0..BLOCK_SIZE {
        differences.push(b'd');
    }
    chunk_delta_list.push(ChunkDelta::Diff(differences));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta::new(chunk_delta_list);

    // Verify computed values
    assert_eq!(delta, expected_delta);
}

#[test]
pub fn integration_test_rdiff_main_delta_chunk_shifted_case4() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_file_case4.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x);
            ()
        });
    }

    // Create signature
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Create new file version
    // By shifted first chunk
    let shifted_size = 20;
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..shifted_size {
            input_data.push(b'd');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x).unwrap();
            ()
        });
    }

    // Create delta
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    let mut differences: Vec<u8> = Vec::new();
    for i in 0..shifted_size {
        differences.push(b'd');
    }
    chunk_delta_list.push(ChunkDelta::Diff(differences));
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta::new(chunk_delta_list);

    // Verify computed values
    assert_eq!(delta, expected_delta);
}

#[test]
pub fn integration_test_rdiff_main_delta_addition_between_chunks_case5() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_file_case5.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Create old file
    {
        let file = File::create(file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x);
            ()
        });
    }

    // Create signature
    let option = "signature";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Create new file version
    // By addition between chunks
    let addition_size = 30;
    {
        let file = File::create(new_file_name.as_str()).unwrap();
        let mut writer = BufWriter::new(file);
        let mut input_data: Vec<u8> = Vec::new();
        for i in 0..BLOCK_SIZE {
            input_data.push(b'a');
        }
        for i in 0..addition_size {
            input_data.push(b'd');
        }
        for i in 0..BLOCK_SIZE {
            input_data.push(b'b');
        }
        for i in 0..BLOCK_SIZE - 1 {
            input_data.push(b'c');
        }
        let chunks = input_data.chunks(BLOCK_SIZE);
        chunks.for_each(|x| {
            writer.write(x).unwrap();
            ()
        });
    }

    // Create delta
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    assert_eq!(rdiff_main_result, ());

    // Get computed Delta from file
    let delta_file = File::open(delta_file_name.as_str()).unwrap();
    let reader = BufReader::new(delta_file);
    let delta: Delta = deserialize_from(reader).unwrap();

    // Set expected values
    let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
    let mut differences: Vec<u8> = Vec::new();
    for i in 0..addition_size {
        differences.push(b'd');
    }
    chunk_delta_list.push(ChunkDelta::Match(1));
    chunk_delta_list.push(ChunkDelta::Diff(differences));
    chunk_delta_list.push(ChunkDelta::Match(2));
    chunk_delta_list.push(ChunkDelta::Match(3));
    let expected_delta = Delta::new(chunk_delta_list);

    // Verify computed values
    assert_eq!(delta, expected_delta);
}

#[test]
fn integration_test_rdiff_main_delta_error_no_option_case1() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_error_case1.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Execute command
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    //args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap_err();

    // Verify result
    assert_eq!(rdiff_main_result.to_string(), HELP_USAGE);
}

#[test]
fn integration_test_rdiff_main_delta_error_file_name_missing_case2() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_error_case2.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Execute command
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    //args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap_err();

    // Verify result
    assert_eq!(rdiff_main_result.to_string(), HELP_USAGE);
}

#[test]
fn integration_test_rdiff_main_delta_error_file_missing_case3() {
    // Get file names
    let prefix_file_name = format!("resources/test_main_delta_error_case3.{}", now_as_millis());
    let file_name = format!("{}.txt", prefix_file_name);
    let signature_file_name = format!("{}.sig", file_name);
    let new_file_name = format!("{}.v1.txt", prefix_file_name);
    let delta_file_name = format!("{}.v1.txt.delta", prefix_file_name);

    // Execute command
    let option = "delta";
    let mut args: Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(new_file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap_err();

    // Verify result
    assert!(rdiff_main_result.to_string().contains(FILE_NOT_FOUND));
}
