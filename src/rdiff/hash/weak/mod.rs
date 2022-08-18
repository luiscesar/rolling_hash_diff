pub mod rdiff_addler;

use crate::rdiff::chunk::RdiffChecksum;

pub type WeakHashPtr = Box<dyn WeakHash>;

pub trait WeakHash {
    fn checksum(&self, chunk: &[u8]) -> RdiffChecksum;
}
