use sha1::{Digest, Sha1};

use crate::rdiff::chunk::RdiffDigest;

use super::StrongHash;

pub type RdiffSha1Ptr = Box<RdiffSha1>;
pub struct RdiffSha1;

impl RdiffSha1 {
    pub fn new() -> RdiffSha1 {
        RdiffSha1
    }
    pub fn new_ptr() -> RdiffSha1Ptr {
        Box::new(RdiffSha1::new())
    }
}

impl StrongHash for RdiffSha1 {
    fn digest(&self, chunk: &[u8]) -> RdiffDigest {
        let mut hasher = Sha1::new();
        hasher.update(chunk);
        let result = hasher.finalize();
        let result2 = result.as_slice();
        let vec = Vec::from(result2);
        vec
    }
}
