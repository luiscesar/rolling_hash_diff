use std::{fs::File, io::{BufWriter, BufReader}};

use bincode::{serialize_into, deserialize_from};
use serde::{Serialize, Deserialize};

use super::{hash::{weak::WeakHashPtr, strong::StrongHashPtr}, error::RdiffError, io::RdiffFile, chunk::{RdiffChunkTable, RdiffChunkDigest, iterator::{BufferedRdiffChunkIterator, RdiffChunkIterator}}};


#[derive(Debug,Serialize,Deserialize)]
pub struct Signature {
    rdiff_chunk_table:RdiffChunkTable,
    chunk_size:usize,
    last_chunk_size:usize,
}

impl Signature {
    
    pub fn new(rdiff_chunk_table:RdiffChunkTable, 
                chunk_size:usize, last_chunk_size:usize) -> Signature {
            Signature { rdiff_chunk_table, chunk_size, last_chunk_size}
    }

    pub fn get_rdiff_chunk_table(&self) -> &RdiffChunkTable {
        &self.rdiff_chunk_table
    }

    pub fn get_chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn get_last_chunk_size(&self) -> usize {
        self.last_chunk_size
    }

    pub fn create_signature_file(file_name:&str,
                                signature_file_name:&str,
                                weak_hash_ptr:WeakHashPtr,
                                strong_hash_ptr:StrongHashPtr) -> Result<(),RdiffError> {
        let signature = 
            Signature::create_signature(file_name, weak_hash_ptr, strong_hash_ptr)?;
        // Write serialized signature to file
        let signature_file = File::create(signature_file_name)?;                            
        let mut sig_writer = BufWriter::new(signature_file);
        serialize_into(&mut sig_writer, &signature)?;
        Ok(())
    }

    pub fn get_signature_from_file(signature_file_name:&str) -> 
        Result<Signature,RdiffError> {
        // Get signture file
        let signature_file = File::open(signature_file_name)?; 
        let sig_reader = BufReader::new(signature_file);
        // Get signature from file
        let signature: Signature = deserialize_from(sig_reader)?;
        Ok(signature)
    }

    fn create_signature(file_name:&str, weak_hash_ptr:WeakHashPtr, strong_hash_ptr:StrongHashPtr) 
        -> Result<Signature, RdiffError> {
        // Get rdiff file    
        let rdiff_file = RdiffFile::new(file_name)?;
        // Build chunk iterator
        let mut iterator = 
            BufferedRdiffChunkIterator::new(rdiff_file);
        // Init chunk table
        let mut rdiff_chunk_table = RdiffChunkTable::new();
        // Init chunk index
        let mut index:u32 = 0;
        // Init last chunk size
        let mut last_chunk_size = 0;
        // Get chunk size
        let chunk_size = iterator.get_chunk_size();
        loop {
            // Get next chunk
            let rdiff_chunk_result = iterator.next_chunk()?;
            if let Some(chunk) = rdiff_chunk_result {
                // If there is another chunk, process it
                // Update last chunk size
                last_chunk_size = chunk.len();
                // Compute checksum using weak hash
                let checksum = weak_hash_ptr.checksum(chunk.as_slice());
                // Compute digest using strong hash
                let digest = strong_hash_ptr.digest(chunk.as_slice());
                // Update chunk table with new chunk data
                // Check for checksum's list, if there is an entry with current checksum,
                // get the list with all previous digests
                // Otherwise create new entry for current checksum with empty list
                let chunk_data = 
                    rdiff_chunk_table.chunk_table.
                    entry(checksum).
                    or_insert(Vec::new());
                // Update chunk index
                index += 1;
                // Get new chunk data from current index and current digest
                let rdiff_chunk_digest = RdiffChunkDigest{index, digest};
                // Update checksum's list with new chunk data
                chunk_data.push(rdiff_chunk_digest);
            } else {
                // If there are more chunks, loop stops
                break;
            }
        }
        // Create signature
        let signature = Signature{rdiff_chunk_table,chunk_size,last_chunk_size};
        Ok(signature)
    }
}

#[cfg(test)]
mod tests;