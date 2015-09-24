/// This structure represents one parsed OpenGEX formatted file.
pub struct Scene;

use std::io::Read;
use std::result::Result;
use std::result::
use piston_meta;

const syntax: 'static str = include_str!("assets/opengex-syntax.txt");

impl Scene {
    /// Creates a new structure from the supplied OpenGEX formatted source.
    pub fn from_src(source: &mut Read) -> Result<()> {
        let mut source_string = String::new();
        try!(source.read_to_string(source_string));
        piston_meta::parse()
    }
}

pub enum Error {
    IoError()
}
