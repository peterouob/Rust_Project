use std::{env,fs,process};
use minigrep::Config;

mod lib;
fn main() {
    // let args : Vec<String> = env::args().collect();
    // test args cargo run -- index index.txt
    // let _query = &args[1]; // index; if &args[2] index.txt
    // let file_path = &args[2]; //index.txt
    // let config = parse_config(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem of parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) =  minigrep::run(config) {
        eprintln!("Application error : {e}");
        process::exit(1);
    }
}