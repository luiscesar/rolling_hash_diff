use std::env;

use rdiff::Rdiff;

pub mod rdiff;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rdiff_main_result = Rdiff::main_rdiff(args);
    match rdiff_main_result {
        Err(e) => eprintln!("{}", e.to_string()),
        _ => {}
    }
}
