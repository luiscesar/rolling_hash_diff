use rolling_hash_diff::rdiff::Rdiff;

const COMMAND:&str = "rolling_hash_diff";

pub fn integration_test_rdiff_main_signature_case1() {
    // Create signature
    let file_name = "resources/test.txt";
    let signature_file_name = "resources/test.sig";
    let option = "signature";
    let mut args:Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
}

pub fn integration_test_rdiff_main_delta_case1() {
    // Create signature
    let file_name = "resources/test.txt";
    let signature_file_name = "resources/test_delta.sig";
    let option = "signature";
    let mut args:Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
    // Create delta
    let file_name = "resources/test.v2.txt";
    let signature_file_name = "resources/test_delta.sig";
    let delta_file_name = "resources/test.delta";
    let option = "delta";
    let mut args:Vec<String> = Vec::new();
    args.push(COMMAND.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
}
