use tcp_accul::*;

use log::*;
use simple_logger::SimpleLogger;
use std::env;

fn main() {
    // setup logging
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        error!(
            "Invalid number of arguments. {} takes 2 args, received {}",
            get_program_name("tcp1cli"),
            args.len() - 1
        );
    }
}
