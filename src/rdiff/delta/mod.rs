use std::{fs::File, io::BufWriter};

use bincode::serialize_into;
use serde::{Deserialize, Serialize};

use super::chunk::iterator::RdiffChunkIterator;
use super::{
    chunk::iterator::BufferedRdiffChunkIterator,
    error::{messages::DELTA_PROCESSED_DATA_SIZE_ERROR, RollingHashError},
    hash::{strong::StrongHashPtr, weak::WeakHashPtr},
    io::RdiffFile,
    signature::Signature,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChunkDelta {
    Match(u32),
    Diff(Vec<u8>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    chunk_delta_list: Vec<ChunkDelta>,
}

impl Delta {
    pub fn new(chunk_delta_list: Vec<ChunkDelta>) -> Delta {
        Delta { chunk_delta_list }
    }

    pub fn create_delta_file(
        file_name: &str,
        delta_file_name: &str,
        signature_file_name: &str,
        weak_hash_ptr: WeakHashPtr,
        strong_hash_ptr: StrongHashPtr,
    ) -> Result<(), RollingHashError> {
        // Get signature
        let signature = Signature::get_signature_from_file(signature_file_name)?;
        // Get delta
        let delta = Delta::generate_delta(file_name, signature, weak_hash_ptr, strong_hash_ptr)?;
        // Write serialized delta to file
        let delta_file =
            File::create(delta_file_name).or_else(|e| Err(RollingHashError::from(Box::new(e))))?;
        let mut delta_writer = BufWriter::new(delta_file);
        serialize_into(&mut delta_writer, &delta)
            .or_else(|e| Err(RollingHashError::from(Box::new(e))))?;

        Ok(())
    }

    fn generate_delta(
        file_name: &str,
        signature: Signature,
        weak_hash_ptr: WeakHashPtr,
        strong_hash_ptr: StrongHashPtr,
    ) -> Result<Delta, RollingHashError> {
        // Get rdiff file from input file given by filename
        let rdiff_file = RdiffFile::new(file_name)?;
        // Get file size
        let file_size = rdiff_file.size();
        // Get rdiff chunk iterator
        let mut iterator: BufferedRdiffChunkIterator =
            BufferedRdiffChunkIterator::new_with_chunk_size(
                signature.get_chunk_size(),
                rdiff_file,
            )?;
        // Init chunk delta list
        let mut chunk_delta_list: Vec<ChunkDelta> = Vec::new();
        // Init memory buffer
        let mut buffer: Vec<u8> = Vec::new();
        // Set chunk size using the signature chunk size
        let chunk_size = signature.get_chunk_size();
        // Init processed data size
        let mut processed_data_size: usize = 0;
        // Loop until the input file has been processed
        'chunk: loop {
            // Get next chunk from iterator
            let rdiff_chunk_result = iterator.next_chunk()?;
            if let Some(mut chunk) = rdiff_chunk_result {
                // If there data more data to process update memory buffer
                buffer.append(&mut chunk);
            }
            // If there is more data to process
            if buffer.len() > 0 {
                // If it is the last chunk
                if buffer.len() < chunk_size {
                    // Check for last chunk match
                    if buffer.len() == signature.get_last_chunk_size() {
                        let last_chunk = &buffer[..];
                        // If last chunk is a match
                        if let Some(chunk_delta) = Delta::get_chunk_delta_match(
                            &signature,
                            &weak_hash_ptr,
                            &strong_hash_ptr,
                            last_chunk,
                        ) {
                            // Add last chunk as a chunk delta match
                            chunk_delta_list.push(chunk_delta);
                            // Update processed data size
                            processed_data_size += last_chunk.len();
                            // No more data to process, the chunk' loop stops
                            break;
                        }
                    }
                    // If there is not chunk delta match,
                    // Update processed data size
                    processed_data_size += buffer.len();
                    // Update chunk delta list with buffer as chunk delta differences
                    chunk_delta_list.push(ChunkDelta::Diff(buffer));
                    // Chunk' loop stops
                    break;
                } else {
                    // Get chunk from the memory buffer
                    let chunk = &buffer[..chunk_size];
                    // If the chunk is a chunk delta match
                    if let Some(chunk_delta) = Delta::get_chunk_delta_match(
                        &signature,
                        &weak_hash_ptr,
                        &strong_hash_ptr,
                        chunk,
                    ) {
                        // Add chunk as a match chunk delta
                        chunk_delta_list.push(chunk_delta);
                        // Update processed data size
                        processed_data_size += chunk.len();
                        // Update buffer by removing that chunk
                        buffer.drain(..chunk_size);
                        // Get the next chunk, continue 'chunk loop
                        continue;
                    }
                    // If there is not a chunk delta match
                    // Search for longest byte sequence until either a chunk delta match
                    // or an eof is found
                    // Init byte difference list
                    let mut differences: Vec<u8> = Vec::new();
                    loop {
                        // Get next byte difference
                        let diff_byte = buffer.remove(0);
                        // Update byte differences list
                        differences.push(diff_byte);
                        // If buffer is not big enough and there is more data
                        // to be buffered
                        if (buffer.len() < chunk_size)
                            && (processed_data_size + differences.len() + buffer.len() < file_size)
                        {
                            // Get next chunk from iterator
                            let rdiff_chunk_result = iterator.next_chunk()?;
                            if let Some(mut chunk) = rdiff_chunk_result {
                                // If there is more data to process.
                                // update memory buffer
                                buffer.append(&mut chunk);
                            }
                        }
                        // Find a chunk delta match
                        if buffer.len() >= chunk_size {
                            // Get chunk from the memory buffer
                            let chunk = &buffer[..chunk_size];
                            // If the chunk is a chunk delta match
                            if let Some(chunk_delta) = Delta::get_chunk_delta_match(
                                &signature,
                                &weak_hash_ptr,
                                &strong_hash_ptr,
                                chunk,
                            ) {
                                // Update processed data size with differences size
                                processed_data_size += differences.len();
                                // Add differences to chunck delta list
                                chunk_delta_list.push(ChunkDelta::Diff(differences));
                                // Update processed data size with chunk size
                                processed_data_size += chunk.len();
                                // Add chunk delta match to chunk delta list
                                chunk_delta_list.push(chunk_delta);
                                // Update buffer by removing that chunk
                                buffer.drain(..chunk_size);
                                // Get next chunk, continue to loop 'chunk
                                continue 'chunk;
                            }
                            // If there is not a chunk delta match,
                            // Get next first byte in buffer, continue to loop
                        } else {
                            // if there are no more chunks
                            // Check for last chunk
                            if buffer.len() == signature.get_last_chunk_size() {
                                let last_chunk = &buffer[..signature.get_last_chunk_size()];
                                // If last chunk is a chunk delta match
                                if let Some(chunk_delta) = Delta::get_chunk_delta_match(
                                    &signature,
                                    &weak_hash_ptr,
                                    &strong_hash_ptr,
                                    last_chunk,
                                ) {
                                    // Update processed data size with differences size
                                    processed_data_size += differences.len();
                                    // Add differences to chunck delta list
                                    chunk_delta_list.push(ChunkDelta::Diff(differences));
                                    // Update processed data size with last chunk size
                                    processed_data_size += last_chunk.len();
                                    // Add chunk delta match to chunk delta list
                                    chunk_delta_list.push(chunk_delta);
                                    // No more data to process, loop 'chunk stops
                                    break 'chunk;
                                }
                            }
                            // If there is no match with last chunk
                            // Update difference list with buffer
                            differences.append(&mut buffer);
                            // Update processed data size with differences size
                            processed_data_size += differences.len();
                            // Add differences to chunck delta list
                            chunk_delta_list.push(ChunkDelta::Diff(differences));
                            // Loop 'chunk stops
                            break 'chunk;
                        }
                    }
                }
            } else {
                // If there is no more data process, loop stops
                break;
            }
        }
        if processed_data_size != file_size {
            return Err(RollingHashError::new(DELTA_PROCESSED_DATA_SIZE_ERROR));
        }
        Ok(Delta { chunk_delta_list })
    }

    fn get_chunk_delta_match(
        signature: &Signature,
        weak_hash_ptr: &WeakHashPtr,
        strong_hash_ptr: &StrongHashPtr,
        chunk: &[u8],
    ) -> Option<ChunkDelta> {
        // Check if chunk has same weak hash and strong hash that one in signature
        // Get chunk checksum
        let checksum = weak_hash_ptr.checksum(chunk);
        // If the checksum exists in chunk table
        if let Some(chunk_digest_list) =
            signature.get_rdiff_chunk_table().chunk_table.get(&checksum)
        {
            // Get chunk digest
            let digest = strong_hash_ptr.digest(chunk);
            // If chunk digest exists in chunk table
            if let Some(chunk_digest) = chunk_digest_list.iter().find(|c| c.digest == digest) {
                // Return it as a match chunk delta
                let chunk_delta = ChunkDelta::Match(chunk_digest.index);
                return Some(chunk_delta);
            }
        }
        // If there is no match, none is returned
        None
    }
}

#[cfg(test)]
mod tests;
