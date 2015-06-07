#![deny(missing_docs)]

//! Meta rules for parsing the OpenGEX format.

extern crate piston_meta;

use piston_meta::*;

/// Gets the rules for parsing OpenGEX.
pub fn rules() -> Rule {
    Rule::Whitespace(Whitespace {
        debug_id: 0,
        optional: true
    })
}
