use crate::rdiff::hash::{strong::rdiff_sha1::RdiffSha1, weak::rdiff_addler::RdiffAddler};

use super::Signature;

#[test]
fn test_signature_create_signature_case1() {
    let file_name = "resources/test.txt";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let signature = 
        Signature::create_signature(file_name,weak_hash_ptr,strong_hash_ptr).unwrap();
    for data in signature.rdiff_chunk_table.chunk_table {
        println!("data {:?}",data)
    }
}

#[test]
fn test_signature_create_signature_case2() {
    let file_name = "resources/poem.txt";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let signature = 
        Signature::create_signature(file_name,weak_hash_ptr,strong_hash_ptr).unwrap();
    for data in signature.rdiff_chunk_table.chunk_table {
        println!("data {:?}",data)
    }
}

#[test]
fn test_signature_create_signature_file_case1() {
    let file_name = "resources/poem.txt";
    let signature_file_name = "resources/poem.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());     
    
}

#[test]
fn test_signature_get_signature_from_file_case1() {
    let file_name = "resources/poem.txt";
    let signature_file_name = "resources/poem.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    for data in signature.rdiff_chunk_table.chunk_table {
        println!("data {:?}",data)
    }
}