//! This module contains all the OpenGEX structures as specified. The module tries to stay as close
//! to the OpenGEX structures as possible.
//!
//! Some of the documentation might be subjected to copyright by Eric Lengyel. For more detailed
//! documentation, please go to http://opengex.org.

use std::collections::HashMap;
use std::rc::Rc;

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

/// A Color structure must contain an RGB or RGBA color value.
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
    pub transformations: Vec<Transformation>,
    /// Any number of animation tracks that are applied to the texture coordinate transformations.
    pub animation: Vec<Animation>
}

/// Helper enum to contain all different kinds of Transformations.
pub enum Transformation {
    /// A Transform structure.
    pub Transform(Transform),
    /// A Translation structure.
    pub Translation(Translation),
    /// A Rotation structure.
    pub Rotation(Rotation),
    /// A Scale structure.
    pub Scale(Scale)
}

/// The Transform structure holds one or more 4 x 4 transformation matrices. In the cases that a
/// Transform structure is contained inside any type of node structure, a Texture structure, or a
/// Skin structure, it must contain a single matrix. In the case that a Transform structure is
/// contained inside a Skeleton structure, is must contain an array of matrices with one entry for
/// each bone referenced by the skeleton.
///
/// When contained inside a node structure, a Transform structure can be the target of a track
/// stored inside an Animation structure.
pub struct Transform([f32; 16]);

/// The Translation structure holds a translation transformation in one of several possible
/// variants.
///
/// There are different variants of this type of tranformation, one for each "kind".
///
/// When contained inside a node structure, a Translation structure can be the target of a track
/// stored inside an Animation structure.
pub enum Translation {
    /// The translation occurs along only the X axis.
    pub X(f32),
    /// The translation occurs along only the Y axis.
    pub Y(f32),
    /// The translation occurs along only the Z axis.
    pub Z(f32),
    /// The translation occurs along all three coordinate axes.
    pub Xyz(f32, f32, f32)
}

/// The Rotation structure represents a rotation along one of several axes.
///
/// There are different variants of this type of tranformation, one for each "kind".
///
/// When contained inside a node structure, a Rotation structure can be the target of a track
/// stored in an Animation structure.
pub enum Rotation {
    /// The rotation occurs about the X axis.
    pub X(f32),
    /// The rotation occurs about the Y axis.
    pub Y(f32),
    /// The rotation occurs about the Z axis.
    pub Z(f32),
    /// The rotation occurs about an arbitrary axis. The first entry of this structure is the angle
    /// of rotation. The remaining three entries are respectively the X, Y and Z components of the
    /// axis of rotation.
    pub Axis(f32, f32, f32, f32),
    /// The rotation is given by a quaternion. Please refer to the official OpenGEX documentation
    /// for more information.
    pub Quaternion(f32, f32, f32, f32)
}

/// The Scale structure represents a scale transformation in one of several possible variants.
///
/// There are different variants of this type of tranformation, one for each "kind".
///
/// When contained inside a node structure, a Scale structure can be the target of a strack stored
/// inside an Animation structure.
pub enum Scale {
    /// The scaling occurs along only the X axis.
    pub X(f32),
    /// The scaling occurs along only the Y axis.
    pub Y(f32),
    /// The scaling occurs along only the Z axis.
    pub Z(f32),
    /// The scaling occurs along all three coordinate axes.
    pub Xyz(f32, f32, f32)
}

/// The Animation structure contains animation data for a single node in a scene. Each animation
/// structure is directly contained inside a node structure or Texture structure. Each animation
/// structure contains the data needed to modify its sibling Transformation structures or sibling
/// MorphWeight structures over time.
///
/// More detailed information can be found in the official OpenGEX specification.
pub struct Animation {
    /// Specifies the animation clip index.
    pub clip: u32,
    /// Specifies when the animation begins. If the property is not specifies, the begin time for
    /// the animation is determined by the earliest time values present in the Track structures
    /// belonging to this Animation.
    pub begin: Opt<f32>,
    /// Specifies when the animation ends. Like with the begin property, if the property is not
    /// specified, the end time for the animation is determined by the latest time values present
    /// in the Track structures belonging to this Animation.
    pub end: Opt<f32>,
    /// One or more tracks that each hold animation keys for a single target.
    pub tracks: Vec<Track>,
}

/// Enum wrapping over all possible animation track targets.
pub enum TrackTarget {
    pub Transformation(Rc<Transformation>),
    pub MorphWeight(Rc<MorphWeight>)
}

/// The Track structure contains animation key data for a single Transformation or MorphWeight
/// structure.
pub struct Track {
    // TODO: Finish this structure
}
