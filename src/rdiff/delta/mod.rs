
use std::{fs::File, io::BufWriter};

use bincode::serialize_into;
use serde::{Serialize, Deserialize};

use super::{error::RdiffError, signature::Signature, hash::{weak::WeakHashPtr, strong::StrongHashPtr}, io::RdiffFile, chunk::iterator::BufferedRdiffChunkIterator};
use super::chunk::iterator::RdiffChunkIterator;

#[derive(Debug,Serialize,Deserialize)]
pub enum ChunkDelta {
    Match(u32),
    D(u8),
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Delta {
    chunk_delta_list:Vec<ChunkDelta>,
}

impl Delta {
    pub fn create_delta_file(file_name:&str,
                            delta_file_name:&str,
                            signature_file_name:&str,
                            weak_hash_ptr:WeakHashPtr,
                            strong_hash_ptr:StrongHashPtr) -> Result<(),RdiffError> {
        // Get signature
        let signature = 
            Signature::get_signature_from_file(signature_file_name)?;
        // Get delta
        let delta = 
            Delta::generate_delta(file_name, signature, weak_hash_ptr, strong_hash_ptr)?;
        // Write serialized delta to file
        let delta_file = File::create(delta_file_name)?;                            
        let mut delta_writer = BufWriter::new(delta_file);
        serialize_into(&mut delta_writer, &delta)?;
        Ok(())
    }

    fn generate_delta(file_name:&str,
                    signature:Signature,
                    weak_hash_ptr:WeakHashPtr,
                    strong_hash_ptr:StrongHashPtr) -> Result<Delta,RdiffError> {
        
        // Get rdiff file from input file given by filename
        let rdiff_file = RdiffFile::new(file_name)?;
        // Get rdiff chunk iterator
        let mut iterator:BufferedRdiffChunkIterator = 
            BufferedRdiffChunkIterator::new_with_chunk_size(signature.chunk_size,rdiff_file)?;
        // Init chunk delta list
        let mut chunk_delta_list:Vec<ChunkDelta> = Vec::new();
        // Init memory buffer
        let mut buffer:Vec<u8> = Vec::new();
        // Set chunk size using the signature chunk size
        let chunk_size = signature.chunk_size;
        // Loop until the input file has been processed
        loop {
            // If buffer is not big enough 
            if buffer.len() < chunk_size {
                // Get next chunk from iterator
                let rdiff_chunk_result = iterator.next_chunk()?;
                if let Some(mut chunk) = rdiff_chunk_result {
                    // If there data more data to process update memory buffer
                    buffer.append(&mut chunk);
                } 
            }
            // If there is more data to process
            if buffer.len() > 0 { 
                // Check the last chunk
                if buffer.len() < chunk_size {
                    // If the byte prefix of buffer has same size of the last chunk in signature's file
                    if signature.last_chunk_size <= buffer.len() {
                        // Get that byte prefix as a last chunk to check
                        let chunk = &buffer[..signature.last_chunk_size];
                        // If that chunk is a match with one in signature
                        if let Some(chunk_delta) = 
                            Delta::get_chunk_delta_match(&signature, &weak_hash_ptr, &strong_hash_ptr, chunk) {
                            // Add it as a match chunk delta
                            chunk_delta_list.push(chunk_delta);
                            // Update buffer by removing that chunk
                            buffer.drain(..signature.last_chunk_size);
                        }
                    }  
                    // Update chunk delta list with the rest of bytes as differences
                    buffer.iter().for_each(|b| chunk_delta_list.push(ChunkDelta::D(*b)));
                    // As there is no more data to process, loop stops
                    break;
                } else {
                    // Otherwise process it as a normal chunk
                    // Get chunk from the memory buffer
                    let chunk = &buffer[..chunk_size];
                    // If the chunk is a match with one in signature
                    if let Some(chunk_delta) = 
                        Delta::get_chunk_delta_match(&signature, &weak_hash_ptr, &strong_hash_ptr, chunk) {
                        // Add it as a match chunk delta
                        chunk_delta_list.push(chunk_delta);
                        // Update buffer by removing that chunk
                        buffer.drain(..chunk_size);
                        // Loop again
                        continue;
                    }
                    // If there is no match
                    // Shift to the right in buffer by removing first byte
                    let diff_byte = buffer.remove(0);
                    // Update chunk delta list with that byte as a difference
                    chunk_delta_list.push(ChunkDelta::D(diff_byte));
                }              
            } else {
                // If there is no more data process, loop stops
                break;
            }
        }
        Ok(Delta{chunk_delta_list})
    }

    fn get_chunk_delta_match(
            signature:&Signature, 
            weak_hash_ptr:&WeakHashPtr, 
            strong_hash_ptr:&StrongHashPtr, 
            chunk:&[u8]) -> Option<ChunkDelta> {
        // Check if chunk has same weak hash and strong hash that one in signature
        // Get chunk checksum
        let checksum = weak_hash_ptr.checksum(chunk);
        // If the checksum exists in chunk table
        if let Some(chunk_digest_list) = 
            signature.rdiff_chunk_table.chunk_table.get(&checksum) {
                // Get chunk digest
                let digest = strong_hash_ptr.digest(chunk);
                // If chunk digest exists in chunk table
                if let Some(chunk_digest) = 
                    chunk_digest_list.iter().find(|c| c.digest == digest) {
                    // Return it as a match chunk delta
                    let chunk_delta = ChunkDelta::Match(chunk_digest.index);
                    return Some(chunk_delta)
                }        
        } 
        // If there is no match, none is returned
        None
    }
}

#[cfg(test)]
mod tests;