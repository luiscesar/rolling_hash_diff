pub mod rdiff;

#[test]
fn test_rdiff_main_signature_case1() {
    rdiff::integration_test_rdiff_main_signature_case1();
}

#[test]
fn test_rdiff_main_delta_case1() {
    rdiff::integration_test_rdiff_main_delta_case1();
}
