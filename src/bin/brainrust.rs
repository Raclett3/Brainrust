use brainrust::parser::parse;
use brainrust::processor::execute;
use std::io::{stdin, Read};

fn main() {
    if let Some(filename) = std::env::args().nth(1) {
        let mut buf = String::new();

        let (source, input) = if filename == "-i" {
            if stdin().lock().read_to_string(&mut buf).is_err() {
                eprintln!("An unexpected error occured reading stdin");
            }
            (buf, "")
        } else if let Ok(content) = std::fs::read_to_string(&filename) {
            if stdin().lock().read_to_string(&mut buf).is_err() {
                eprintln!("An unexpected error occured reading stdin");
            }
            (content, buf.as_str())
        } else {
            eprintln!("No such file found: {}", filename);
            std::process::exit(1);
        };

        let tree = match parse(&source) {
            Ok(tree) => tree,
            Err(reason) => {
                eprintln!("{}", reason);
                std::process::exit(1);
            }
        };
        println!("{}", execute(&tree, 16384, input));
    }
}
