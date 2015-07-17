#![deny(missing_docs)]

//! Meta rules for parsing the OpenGEX format.

extern crate piston_meta;

#[test]
fn test_syntax() {
    use piston_meta::*;
    use std::fs::File;
    use std::io::Read;

    let mut file_h = File::open("assets/opengex-syntax.txt").unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let rules = stderr_unwrap(&source, syntax(&source));
    
    let mut file_h = File::open("assets/cube.ogex").unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    
    let data = stderr_unwrap(&source, parse(&rules, &source));
    json::print(&data);
    // assert!(false);
}
