use std::env;
use std::fs;
use text_colorizer::*;

fn getname() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!(
            "{} Wrong number of arguments; expected 1, got {}",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    args[0].clone()
}

fn main() {
    let fname = getname();

    let _data = match fs::read_to_string(&fname) {
        Ok(v) => v,
        Err(e) => {
            eprint!(
                "{} Filename '{}' does not exist. {:?}",
                "Error:".red().bold(),
                fname,
                e
            );
            std::process::exit(1);
        }
    };
}
