mod json_parser;

use std::env::args;
use json_parser::Parser;


fn main() {
    let args: Vec<String> = args().collect();

    let fcnn = Parser::new(args[1]);
    fcnn.train();
}
