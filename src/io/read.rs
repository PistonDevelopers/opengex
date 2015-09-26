extern crate piston_meta;

use scene::Scene;

/// The OpenDDL syntax saved in a &str.
const OPENDDL_SYNTAX: &'static str = include_str!("openddl.syntax");

/// Generates a Scene structure from the supplied OpenGEX-formatted source string.
/// The return type is bound to change.
pub fn scene_from_src_string(ogex_src: String) -> Result<Scene, ()> {
    use self::piston_meta::{ parse, stderr_unwrap, syntax };

    let syntax_src = OPENDDL_SYNTAX.to_string();

    let rules = stderr_unwrap(&syntax_src, syntax(&syntax_src));
    let data = stderr_unwrap(&ogex_src, parse(&rules, &ogex_src));
}
