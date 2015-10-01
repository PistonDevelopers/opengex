//! This module contains all the OpenGEX structures as specified. The module tries to stay as close
//! to the OpenGEX structures as possible.
//!
//! Some of the documentation might be subjected to copyright by Eric Lengyel. For more detailed
//! documentation, please go to http://opengex.org.

use std::collections::HashMap;

/// The Material structure contains information about a material. Material structures are
/// referenced by geometry nodes through MaterialRef structures belonging to GeometryNode
/// structures.
pub struct Material {
    /// Whether the material is two-sided.
    pub two_sided: bool,
    /// An optional name.
    pub name: Option<Name>,
    /// Any number of colors.
    pub color: HashMap<String, Color>,
    /// Any number of parameters.
    pub param: ParamMap,
    /// Any number of textures.
    pub texture: HashMap<String, Texture>,
}

/// A Color structure must contain an RGB color, or RGBA color.
pub enum Color {
    /// An RGB color value.
    Rgb(f32, f32, f32),
    /// An RGBA color value.
    Rgba(f32, f32, f32, f32)
}

/// This is a map of different Param structures, for convenience.
pub type ParamMap = HashMap<String, f32>;

/// The Name structure holds the name of a node, morph target, material, or animation clip.
pub type Name = String;

/// The Texture structure holds information about a single texture map, and how it is accessed with
/// texture coordinates.
pub struct Texture {
    /// The index of the texture coordinate set associated with the texture.
    pub texcoord: u32,
    /// A substructure holding the file name of the texture.
    pub file_name: String,
    /// Any number of transformations that are applied to the texture coordinates of a mesh when
    /// they are used to fetch from the texture map.
    pub temp: () // TODO: Finish this structure.
}
