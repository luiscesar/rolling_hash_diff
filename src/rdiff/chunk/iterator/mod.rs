use std::cmp;

use crate::rdiff::{
    constants::BLOCK_SIZE,
    error::{messages::INVALID_CHUNK_SIZE, RdiffError, RollingHashError},
    io::RdiffFile,
};

use super::RdiffChunk;

pub trait RdiffChunkIterator {
    fn next_chunk(&mut self) -> Result<Option<RdiffChunk>, RollingHashError>;
    fn get_chunk_size(&self) -> usize;
}

#[derive(Debug, PartialEq)]
pub struct BufferedRdiffChunkIterator {
    chunk_size: usize,
    buffer: Vec<u8>,
    rdiff_file: RdiffFile,
}

impl BufferedRdiffChunkIterator {
    pub fn new(rdiff_file: RdiffFile) -> BufferedRdiffChunkIterator {
        let chunk_size = BufferedRdiffChunkIterator::compute_chunk_size(&rdiff_file);
        let buffer: Vec<u8> = Vec::new();
        BufferedRdiffChunkIterator {
            chunk_size,
            buffer,
            rdiff_file,
        }
    }

    pub fn new_with_chunk_size(
        chunk_size: usize,
        rdiff_file: RdiffFile,
    ) -> Result<BufferedRdiffChunkIterator, RollingHashError> {
        BufferedRdiffChunkIterator::validate_chunk_size(chunk_size)?;
        let buffer: Vec<u8> = Vec::new();
        Ok(BufferedRdiffChunkIterator {
            chunk_size,
            buffer,
            rdiff_file,
        })
    }

    fn compute_chunk_size(rdiff_file: &RdiffFile) -> usize {
        if rdiff_file.size() > 1 {
            if rdiff_file.size() > BLOCK_SIZE {
                BLOCK_SIZE
            } else {
                (((rdiff_file.size() as f64) / 2.0).round()) as usize
            }
        } else {
            1
        }
    }

    fn validate_chunk_size(chunk_size: usize) -> Result<(), RollingHashError> {
        if chunk_size > BLOCK_SIZE {
            let rdiff_error = RollingHashError::new(INVALID_CHUNK_SIZE);
            return Err(rdiff_error);
        }
        Ok(())
    }
}

impl RdiffChunkIterator for BufferedRdiffChunkIterator {
    fn get_chunk_size(&self) -> usize {
        self.chunk_size
    }

    fn next_chunk(&mut self) -> Result<Option<RdiffChunk>, RollingHashError> {
        // If memory buffer contains at one chunk
        if self.buffer.len() >= self.chunk_size {
            // Get that chunk and remove it from buffer
            let next_chunk_iter = self.buffer.drain(..self.chunk_size);
            let next_chunk = next_chunk_iter.as_slice();
            // Get rdiff chunk
            let rdiff_chunk = Vec::from(next_chunk);
            // Return rdiff chunk
            Ok(Some(rdiff_chunk))
        } else {
            // If memory buffer does not contain a chunk, get one from file,
            // if there is one
            if let Some((size, next_block)) = self
                .rdiff_file
                .read_block()
                .or_else(|e| Err(RollingHashError::from(e)))?
            {
                // Update memory buffer with chunk from file
                let next_block_data = &next_block[..size];
                self.buffer.extend_from_slice(next_block_data);
            }
            // If buffer contains data
            if self.buffer.len() > 0 {
                // Get a normal chunk or the last chunk and update memory buffer
                let next_chunk_size = cmp::min(self.chunk_size, self.buffer.len());
                let next_chunk_iter = self.buffer.drain(..next_chunk_size);
                let next_chunk = next_chunk_iter.as_slice();
                let rdiff_chunk = Vec::from(next_chunk);
                Ok(Some(rdiff_chunk))
            } else {
                // If buffer contains no more data, return none
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests;
