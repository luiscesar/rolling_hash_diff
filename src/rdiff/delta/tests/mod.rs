use crate::rdiff::{hash::{strong::rdiff_sha1::RdiffSha1, weak::rdiff_addler::RdiffAddler}, signature::Signature, delta::Delta};


#[test]
fn test_delta_generate_delta_case1() {
    let file_name_new = "resources/test.v2.txt";
    let file_name_old = "resources/test.txt";
    let signature_file_name = "resources/test.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let delta = Delta::generate_delta(file_name_new,signature,weak_hash_ptr,strong_hash_ptr).unwrap();

    println!("delta {:?}", delta);
}

#[test]
fn test_delta_generate_delta_case2() {
    let file_name_new = "resources/poem.v2.txt";
    let file_name_old = "resources/poem.txt";
    let signature_file_name = "resources/poem.v2.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let delta = Delta::generate_delta(file_name_new,signature,weak_hash_ptr,strong_hash_ptr).unwrap();

    println!("delta {:?}", delta);
}

#[test]
fn test_delta_generate_delta_case3() {
    let file_name_new = "resources/poem.v3.txt";
    let file_name_old = "resources/poem.txt";
    let signature_file_name = "resources/poem.v3.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(result,());
    let signature = 
        Signature::get_signature_from_file(signature_file_name).unwrap();
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let delta = Delta::generate_delta(file_name_new,signature,weak_hash_ptr,strong_hash_ptr).unwrap();

    println!("delta {:?}", delta);
}

#[test]
fn test_delta_create_delta_file_case1() {
    // Create signature file
    let file_name_old = "resources/test.txt";
    let signature_file_name = "resources/test-delta2.sig";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let signature_result = 
        Signature::create_signature_file(file_name_old, signature_file_name, weak_hash_ptr, strong_hash_ptr).unwrap();
    assert_eq!(signature_result, ());

    // Create delta file
    let file_name_new = "resources/test.v2.txt";
    let delta_file_name = "resources/test2.delta";
    let strong_hash_ptr = RdiffSha1::new_ptr();
    let weak_hash_ptr = RdiffAddler::new_ptr();
    let result = 
        Delta::create_delta_file(file_name_new, 
            delta_file_name, 
            signature_file_name, 
            weak_hash_ptr, 
            strong_hash_ptr).unwrap();
    assert_eq!(result,());
}