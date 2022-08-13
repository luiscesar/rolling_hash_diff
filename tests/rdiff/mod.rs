use rolling_hash_diff::rdiff::Rdiff;

#[test]
fn test_rdiff_main_signature_case1() {
    let file_name = "resources/test.txt";
    let signature_file_name = "resources/test.sig";
    let option = "signature";
    let command = "rdiff-rolling-hash";
    let mut args:Vec<String> = Vec::new();
    args.push(command.to_string());
    args.push(option.to_string());
    args.push(file_name.to_string());
    args.push(signature_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
}

#[test]
fn test_rdiff_main_delta_case1() {
    let file_name = "resources/test.v2.txt";
    let signature_file_name = "resources/test.sig";
    let delta_file_name = "resources/test.sig";
    let option = "delta";
    let command = "rdiff-rolling-hash";
    let mut args:Vec<String> = Vec::new();
    args.push(command.to_string());
    args.push(option.to_string());
    args.push(signature_file_name.to_string());
    args.push(file_name.to_string());
    args.push(delta_file_name.to_string());
    let rdiff_main_result = Rdiff::main_rdiff(args).unwrap();
}
