use adler::adler32_slice;

use crate::rdiff::chunk::RdiffChecksum;

use super::WeakHash;

pub type RdiffAddlerPtr = Box<RdiffAddler>;
pub struct RdiffAddler {}

impl RdiffAddler {
    pub fn new() -> RdiffAddler {
        RdiffAddler {}
    }
    pub fn new_ptr() -> RdiffAddlerPtr {
        Box::new(RdiffAddler::new())
    }
}

impl WeakHash for RdiffAddler {
    fn checksum(&self, chunk: &[u8]) -> RdiffChecksum {
        let checksum = adler32_slice(chunk);
        checksum
    }
}

#[cfg(test)]
mod tests;
