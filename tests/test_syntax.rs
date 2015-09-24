extern crate piston_meta;

#[test]
fn test_syntax_opengex() {
    use piston_meta::*;

    let syntax_src = include_str!("assets/opengex-syntax.txt").to_string();
    let rules = stderr_unwrap(&syntax_src, syntax(&syntax_src));

    let ogex_src = include_str!("assets/cube.ogex").to_string();
    let data = stderr_unwrap(&ogex_src, parse(&rules, &ogex_src));
}

#[test]
fn test_syntax_opendll() {
    use piston_meta::*;

    let syntax_src = include_str!("assets/openddl-syntax.txt").to_string();
    let rules = stderr_unwrap(&syntax_src, syntax2(&syntax_src));

    let ogex_src = include_str!("assets/cube.ogex").to_string();
    let data = stderr_unwrap(&ogex_src, parse(&rules, &ogex_src));
}
