use std::fs::File;
use std::io::{BufReader, Read};

use crate::rdiff::error::RdiffError;
use crate::rdiff::constants::BLOCK_SIZE;

#[derive(Debug)]
pub struct RdiffFile {
    reader:BufReader<File>,
    name:String,
    size:u64,
}

impl RdiffFile {
    pub fn new(filename:&str) -> Result<RdiffFile,RdiffError> {
        let f = File::open(filename)?;
        let size = f.metadata().unwrap().len();
        let reader = BufReader::new(f);
        Ok(RdiffFile{reader:reader,name:String::from(filename),size:size})
    } 

    pub fn size(&self) -> usize {
        let size = self.size as usize;
        size
    }
    
    pub fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0;1];
        let x = self.reader.read(&mut buffer[..]);
        match x {
            Ok(c) if c > 0 => Some(buffer[0]),
            _ => None,
        }
    }

    pub fn read_block(&mut self) -> 
            Result<Option<(usize, [u8;BLOCK_SIZE as usize])>, RdiffError> {
        let mut buffer: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
        let x = self.reader.read(&mut buffer)?;
        match x {
            c if c > 0 => Ok(Some((c,buffer))),
            _ => Ok(None),
        }
    }
}

impl PartialEq for RdiffFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests;