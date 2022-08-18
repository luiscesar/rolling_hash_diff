use crate::rdiff::chunk::RdiffDigest;

pub mod rdiff_sha1;

pub type StrongHashPtr = Box<dyn StrongHash>;

pub trait StrongHash {
    fn digest(&self, chunk: &[u8]) -> RdiffDigest;
}
