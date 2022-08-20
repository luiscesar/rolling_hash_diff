use self::{
    constants::{DELTA, SIGNATURE},
    delta::Delta,
    error::{messages::HELP_USAGE, RollingHashError},
    hash::{strong::rdiff_sha1::RdiffSha1, weak::rdiff_addler::RdiffAddler},
    signature::Signature,
};

pub mod chunk;
pub mod constants;
pub mod delta;
pub mod error;
pub mod hash;
pub mod io;
pub mod signature;
pub mod util;

pub type RdiffMainResult = Result<(), RollingHashError>;

pub struct Rdiff;

impl Rdiff {
    pub fn main_rdiff(args: Vec<String>) -> RdiffMainResult {
        // Check for command
        if args.len() > 1 {
            match args.get(1).unwrap().as_str() {
                SIGNATURE => Rdiff::rdiff_signature(args),
                DELTA => Rdiff::rdiff_delta(args),
                _ => Err(RollingHashError::new(HELP_USAGE)),
            }
        } else {
            Err(RollingHashError::new(HELP_USAGE))
        }
    }

    fn rdiff_signature(args: Vec<String>) -> RdiffMainResult {
        // Process signature command option
        // If there are enough args
        if args.len() >= 4 {
            // Get target file name
            let file_name = args.get(2).unwrap();
            // Get signature file name where the signature will be stored
            let signature_file_name = args.get(3).unwrap();
            // Get strong hash to compute digest
            let strong_hash_ptr = RdiffSha1::new_ptr();
            // Get weak thas to compute checksum
            let weak_hash_ptr = RdiffAddler::new_ptr();
            // Create signature and store it in file
            let result = Signature::create_signature_file(
                file_name,
                signature_file_name,
                weak_hash_ptr,
                strong_hash_ptr,
            )?;
            Ok(result)
        } else {
            Err(RollingHashError::new(HELP_USAGE))
        }
    }

    fn rdiff_delta(args: Vec<String>) -> RdiffMainResult {
        // Process delta comand option
        if args.len() >= 5 {
            // Get signature file name, file where the signature is stored
            let signature_file_name = args.get(2).unwrap();
            // Get file name, new version of original file
            let file_name = args.get(3).unwrap();
            // Get delta file name, file where the differences between the original and
            // new version will be stored
            let delta_file_name = args.get(4).unwrap();
            // Get strong hash to compute digest
            let strong_hash_ptr = RdiffSha1::new_ptr();
            // Get weak hash to compute checksum
            let weak_hash_ptr = RdiffAddler::new_ptr();
            // Create checkum delta and store it in a file
            let result = Delta::create_delta_file(
                file_name,
                delta_file_name,
                signature_file_name,
                weak_hash_ptr,
                strong_hash_ptr,
            )?;
            Ok(result)
        } else {
            Err(RollingHashError::new(HELP_USAGE))
        }
    }
}

#[cfg(test)]
mod tests;
