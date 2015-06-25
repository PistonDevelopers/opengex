#![deny(missing_docs)]

//! Meta rules for parsing the OpenGEX format.

extern crate piston_meta;

#[test]
fn test_syntax() {
    use piston_meta::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    let rules = bootstrap::rules();
    let opengex_syntax: PathBuf = "assets/opengex-syntax.txt".into();
    let mut file_h = File::open(opengex_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let data = parse(&rules, &source);
    let data = match data {
        Ok(data) => data,
        Err((range, err)) => {
            // Report the error to standard error output.
            ParseStdErr::new(&source).error(range, err);
            assert!(false);
            unreachable!();
        }
    };

    let rules = bootstrap::convert(&data, &mut vec![]).unwrap();
    let opengex_syntax: PathBuf = "assets/cube.ogex".into();
    let mut file_h = File::open(opengex_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let data = parse(&rules, &source);
    let _data = match data {
        Ok(data) => data,
        Err((range, err)) => {
            // Report the error to standard error output.
            ParseStdErr::new(&source).error(range, err);
            assert!(false);
            unreachable!();
        }
    };
}
