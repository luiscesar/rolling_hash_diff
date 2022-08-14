use std::{collections::HashMap};
use serde::{Serialize, Deserialize};

pub mod iterator;

pub type RdiffChecksum = u32;
pub type RdiffDigest = Vec<u8>;


pub type RdiffChunk = Vec<u8>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RdiffChunkDigest {
    pub index:u32,
    pub digest:RdiffDigest,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RdiffChunkTable {
    pub(crate) chunk_table:HashMap<RdiffChecksum,Vec<RdiffChunkDigest>>,
}

impl RdiffChunkTable {
    pub fn new() -> RdiffChunkTable {
        let chunk_table:HashMap<RdiffChecksum,Vec<RdiffChunkDigest>> = 
            HashMap::new();
        RdiffChunkTable{chunk_table}
    }    
}

#[cfg(test)]
mod tests;