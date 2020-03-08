#[path="../multilayer/mod.rs"]
mod multilayer;

use multilayer::MultiLayerPerceptron as mlp;
use serde_json::{Result, Value};
use std::{self, prelude::*};
use std::fs::File;


pub struct Parser {
    config: Value
}


fn parse_config_file(filename: String) -> Value {
    let mut f = File::open(filename).unwrap();
    serde_json::from_reader(f) 
}


impl Parser {
    pub fn new(config_file: String) -> Parser {
        Parser { config: parse_config_file(config_file) }        
    }

    pub fn train(&mut self) {

    }
}
