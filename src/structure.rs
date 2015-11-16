//! This module contains all the OpenGEX structures as specified. The module tries to stay as close
//! to the OpenGEX structures as possible.
//!
//! Some of the documentation might be subjected to copyright by Eric Lengyel. For more detailed
//! documentation, please go to http://opengex.org.

use std::collections::HashMap;
use std::default::Default;
use std::sync::Arc;
use vec_map::VecMap;

/// The Material structure contains information about a material. Material structures are
/// referenced by geometry nodes through `Arc<Material>` structures belonging to `GeometryNode`
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

/// The Param structure contains just a float. While the OpenGEX specification defines a Param as a
/// key-value pair, this is better implemented through a HashMap. See ParamMap.
pub type Param = f32;

/// This is a map of different Param structures, for convenience.
pub type ParamMap = HashMap<String, Param>;
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
    Transform(Transform),
    /// A Translation structure.
    Translation(Translation),
    /// A Rotation structure.
    Rotation(Rotation),
    /// A Scale structure.
    Scale(Scale)
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
    X(f32),
    /// The translation occurs along only the Y axis.
    Y(f32),
    /// The translation occurs along only the Z axis.
    Z(f32),
    /// The translation occurs along all three coordinate axes.
    Xyz(f32, f32, f32)
}

/// The Rotation structure represents a rotation along one of several axes.
///
/// There are different variants of this type of tranformation, one for each "kind".
///
/// When contained inside a node structure, a Rotation structure can be the target of a track
/// stored in an Animation structure.
pub enum Rotation {
    /// The rotation occurs about the X axis.
    X(f32),
    /// The rotation occurs about the Y axis.
    Y(f32),
    /// The rotation occurs about the Z axis.
    Z(f32),
    /// The rotation occurs about an arbitrary axis. The first entry of this structure is the angle
    /// of rotation. The remaining three entries are respectively the X, Y and Z components of the
    /// axis of rotation.
    Axis(f32, f32, f32, f32),
    /// The rotation is given by a quaternion. Please refer to the official OpenGEX documentation
    /// for more information.
    Quaternion(f32, f32, f32, f32)
}

/// The Scale structure represents a scale transformation in one of several possible variants.
///
/// There are different variants of this type of tranformation, one for each "kind".
///
/// When contained inside a node structure, a Scale structure can be the target of a strack stored
/// inside an Animation structure.
pub enum Scale {
    /// The scaling occurs along only the X axis.
    X(f32),
    /// The scaling occurs along only the Y axis.
    Y(f32),
    /// The scaling occurs along only the Z axis.
    Z(f32),
    /// The scaling occurs along all three coordinate axes.
    Xyz(f32, f32, f32)
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
    pub begin: Option<f32>,
    /// Specifies when the animation ends. Like with the begin property, if the property is not
    /// specified, the end time for the animation is determined by the latest time values present
    /// in the Track structures belonging to this Animation.
    pub end: Option<f32>,
    /// One or more tracks that each hold animation keys for a single target.
    pub tracks: Vec<Track>,
}

/// The Track structure contains animation key data for a single Transformation or MorphWeight
/// structure.
pub struct Track {
    /// The target transformation or MorphWeight this track applies to.
    pub target: TrackTarget,
    /// The time curve associated with this track.
    pub time: Time,
    /// The value curve associated with this track.
    pub value: Value
}

/// Enum wrapping over all possible animation track targets.
pub enum TrackTarget {
    /// A Transformation structure. See enum to see possibilities.
    Transformation(Arc<Transformation>),
    /// A MorphWeight structure.
    MorphWeight(Arc<MorphWeight>)
}

/// The Time structure contains key time data in an animation track.
///
/// There are two different kinds of this structure; one for every curve kind.
///
/// The variants in this enum contain vectors. One vector item represents on key time.
pub enum Time {
    /// The times are interpolated linearly.
    Linear(Vec<f32>),
    /// The times are interpolated on a one-dimensional Bézier curve.
    ///
    /// The times in each tuple in the vector are the "value", "-control" and "+control" values of
    /// the Bézier curve respectively.
    Bézier(Vec<(f32, f32, f32)>)
}

/// The Value structure contains key value data in an animation track.
///
/// There are two different kinds of this structure; one for every curve kind.
///
/// The variants in this enum contain vectors. One vector item represents on key value.
pub enum Value {
    /// The values are not interpolated, but remain constant until the next key time.
    Constant(Vec<f32>),
    /// The values are interpolated linearly.
    Linear(Vec<f32>),
    /// The values are interpolated on a one-dimensional Bézier curve.
    ///
    /// The values in each tuple in the vector are the "value", "-control" and "+control" values of
    /// the Bézier curve respectively.
    Bézier(Vec<(f32, f32, f32)>),
    /// The values are interpolated on a tension-continuity-bias (TCB) spline.
    ///
    /// The values in each tuple in the vector are the "value", "tension", "bias" and "continuity"
    /// values of the TCB spline respecitvely. The data contained in these last three values are
    /// always scalar.
    Tcb(Vec<(f32, f32, f32, f32)>)
}

/// A MorphWeight structure holds a single morph weight for a GeometryNode structure, that
/// references a GeometryObject strucure containing vertex data for multiple morph targets.
///
/// A MorphWeight structure can be the target of a track stored inside an Animation structure.
pub struct MorphWeight {
    /// Specifies the morph target index to which this morph weight applies. If the GeometryObject
    /// structure contains no vertex data corresponding to this target index, then this structure
    /// should be ignored. Each MorphWeight structure belonging to any particular GeometryNode
    /// structure must have a unique target index among all morph weights belonging to that
    /// GeometryNode.
    pub target_index: u32,
    /// The weight this MorphWeight structure represents.
    pub weight: f32,
}

/// The Atten structure specifies an attenuation function for a light object.
pub struct Atten {
    /// The kind of attenuation.
    pub kind: AttenuationKind,
    /// The type of curve defining the attenuation.
    pub curve: AttenuationCurve,
    /// Any parameters associated with this Atten structure.
    ///
    /// For the meaning of the parameters, please refer to the official OpenGEX documentation.
    /// There can exist application-defined parameters.
    pub params: ParamMap
}

/// A helper enum representing different kinds of attenuation functions.
pub enum AttenuationKind {
    /// The input to the attenuation function is the radial distance from the LightObject the
    /// parent Atten structure is associated with.
    Distance,
    /// The input to the attenuation function is the angle formed between the negative z-axis and
    /// the direction to the point being illuminated in object space.
    ///
    /// The result of this function should be raised to the power value of a potential "power"
    /// paramter present in the parent Atten structure.
    Angle,
    /// The input ot the attenuation function is the cosine of the angle formed between the
    /// negative z-axis and the direction to the point being illuminated in objet space.
    ///
    /// The result of this function should be raised to the power value of a potential "power"
    /// paramter present in the parent Atten structure.
    CosAngle
}

/// A helper enum representing different kinds of curves for an attenuation function.
///
/// For exact formulas, please refer to the offical OpenGEX documentation.
pub enum AttenuationCurve {
    /// The attenuation is a linear function.
    Linear,
    /// The attenuation is given by a cubic-smooth-step function.
    Cubic,
    /// The attenuation is given by the inverse function.
    Inverse,
    /// The attenuation is given by the inverse square function.
    InverseSquare
}

/// Helper enum to represent all different types of Nodes.
pub enum Nodes {
    /// A `Node`.
    Node(Node),
    /// A `BoneNode`.
    BoneNode(BoneNode),
    /// A `GeometryNode`
    GeometryNode(GeometryNode),
    /// A `CameraNode`
    CameraNode(CameraNode),
    /// A `LightNode`
    LightNode(LightNode)
}

/// Macro to do away with the redundancy of the different kinds of Node structures. The common Node
/// properties are placed at the start of the generated structure.
///
/// The common properties are:
/// * `name`: An optional name of the node.
/// * `transformations`: A vector of local transformations to be applied to the node.
/// * `animations`: A vector of animations that can be applied to the node.
/// * `children`: A vector of child nodes part of this node.
macro_rules! node {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $($(#[$doc:meta])* pub $prop:ident : $type_:ty),*
        }
    ) => (
        $(#[$attr])*
        pub struct $name {
            /// The optional name for this node. This property is generic to all types of nodes.
            pub name: Option<Name>,
            /// Any local transformations to be applied to this node. This property is generic to
            /// all types of nodes.
            pub transformations: Vec<Transformation>,
            /// Any animations for this node. This property is generic to all types of nodes.
            pub animations: Vec<Animation>,
            /// Any sub-nodes of this node. This property is generic to all types of nodes.
            pub children: Vec<Nodes>,
            $($(#[$doc])* pub $prop : $type_),*
        }
    )
}

node! {
    /// A Node structure represents a single generic node in a scene, with no associated object.
    pub struct Node {}
}

node! {
    /// A BoneNode structure represents a single bone in a scene. The collection of bone nodes
    /// forming a complete skeleton for a skinned mesh is referenced by a `BoneRefArray` structure
    /// contained inside a `Skeleton` structrue.
    pub struct BoneNode {}
}

node! {
    /// A GeometryNode structure represents a single geometry node in a scene. "]
    pub struct GeometryNode {
        /// Whether this geometry is visible. Overrides the visibility of the referenced
        /// `GeometryObject` structure.
        pub visibile: Option<bool>,
        /// Whether this geometry casts shadows. If unset, this `GeometryNode` inherits the
        /// visibility of the referenced `GeometryObject` structure.
        pub casts_shadows: Option<bool>,
        /// Whether the geomery is rendered with motion blur. If unset, this `GeometryNode`
        /// inherits the visibility of the referenced `GeometryObject` structure.
        pub motion_blur: Option<bool>,
        /// A reference to a `GeometryObject` structure containing all of the required mesh data
        /// and optional skinning data.
        pub geometry: Arc<GeometryObject>,
        /// A `HashMap` with references to materials associated with this geometry. Each material's
        /// index in the HashMap specifies to which part of a mesh the material is applied, by
        /// matching it with the `material` property of each `IndexArray` structure in the mesh.
        pub materials: VecMap<Arc<Material>>,
        /// If the `GeometryObject` referenced by this node contains vertex data for multiple morph
        /// targets, then the node may contain one or more `MorphWeight` structures that specify
        /// the blending weight for each target. Each `MorphWeight` structure may be the target of
        /// a Track structure in the animation belonging to the node.
        pub morph_weights: Vec<MorphWeight>
    }
}

node! {
    /// A `CameraNode` structure represents a single camera node in a scene.
    pub struct CameraNode {
        /// A reference to a `CameraObject` that contains the information neccesary to construct
        /// the properly configured camera.
        pub camera: Arc<CameraObject>
    }
}

node! {
    /// A `LightNode` structure represents a single camera node in a scene. "]
    pub struct LightNode {
        /// Whether this light is visible. Overrides the visibility of the referenced `LightObject`
        /// structure.
        pub visibile: Option<bool>,
        /// A reference to a `LightObject` that contains the information neccesary to construct the
        /// proper type of light.
        pub light: Arc<LightObject>
    }
}

/// The `GeometryObject` structure contains data for a geometry object. Multiple `GeometryNode`
/// structures may reference a single `GeometryObject`. This allows a scene to contain multiple
/// instances of the same geometry with different transforms and materials.
///
/// The `colors` and `textures` properties are for application-specfic use.
pub struct GeometryObject {
    /// Whether this geometry is visible. Can be overriden by any `GeometryNode` structure
    /// referencing this geometry.
    pub visible: bool,
    /// Whether this geometry casts shadows. Can be overriden by any `GeometryNode` structure
    /// referencing this geometry.
    pub casts_shadows: bool,
    /// Whether this geometry is rendered with motion blur. Can be overriden by any `GeometryNode`
    /// structure referencing this geometry.
    pub motion_blur: bool,
    /// A mesh for every level of detail. The map is indexed by the level of detail.
    pub meshes: VecMap<Mesh>,
    /// May contain a `Morph` structure for each morph target for which vertex data exists inside
    /// the `Mesh` structures in `meshes`. The key of the `HashMap` is their target index.
    pub morphs: VecMap<Morph>
}

/// A `CameraObject` structure contains data for a camera object.
pub struct CameraObject {
    /// A map of parameters associated with this camera.
    ///
    /// The OpenGEX specification defined three parameters: "fov", "near" and "far". Any other
    /// parameters are application-specific.
    pub params: ParamMap,
    /// A map of colors associated with this camera. The OpenGEX specification does not define any
    /// kinds of colors.
    pub colors: HashMap<String, Color>,
    /// A map of textures associated with this camera. The OpenGEX specification does not define
    /// any kinds of textures.
    pub textures: HashMap<String, Texture>
}

/// The LightObject struture contains data for a light object. Multiple LightNode structures may
/// reference a single LightObject. This allows a scene to contain multiple instances of the same
/// light, with different transformations.
pub struct LightObject {
    /// The type of light emitted by this LightObject.
    pub light_type: LightType,
    /// Whether this LightObject casts shadows. This can be overiden by the LightNode referencing
    /// this LightObject.
    pub casts_shadows: bool,
    /// The colors associated with this LightObject.
    ///
    /// The OpenGEX specification only references one type of color: "light". This is the main
    /// color of light emitted by the light souArce. This defaults to an RGB value of
    /// (1.0, 1.0, 1.0).
    ///
    /// There can be any other number of colors in this HashMap, for application-specific kinds.
    pub colors: HashMap<String, Color>,
    /// The parameters associated with this LightObject.
    ///
    /// The OpenGEX specification only references one type of parameter: "intensity". This is the
    /// intensity of the light emitted by the light souArce. This defaults to 0.
    ///
    /// There can be any other number of parameters in this HashMap, for application-specific
    /// kinds.
    pub params: ParamMap,
    /// The textures associated with this LightObject.
    ///
    /// The OpenGEX specification only references one type of texture: "projection". This is an
    /// optional texture projection of this LightObject. The texture map should be oriented so that
    /// the right direction is aligned to the object-space positive x-axis, and the up direction is
    /// aligned to the object-space positive y-axis.
    ///
    /// There can be any other number of textures in this HashMap, for application-specific kinds.
    pub textures: HashMap<String, Texture>,
    /// Any number of attenuation functions to be applied to the LightObject. The values produced
    /// by all of them are multiplied together to determine the intensity of the light reaching
    /// any particular point in space.
    pub attenuations: Atten
}

/// This is an helper-enum representing all different types of lights that a LightObject can emit.
pub enum LightType {
    /// The light souArce is to be treated as if it were infinitely far away so its rays are
    /// parallel. In object space, the rays point in the direction of the negative z-axis.
    Infinite,
    /// The lght souArce is a point light that radiates in all directions.
    Point,
    /// The light source is a spot light that radiates from a single points byt in a limited range
    /// of directions In object space, the primary direction is the negative z-axis.
    Spot
}

/// The `Morph` structure holds information about a morph target belonging to a `GeometryObject`
/// structure.
pub struct Morph {
    /// The base morph target index for a relative morph target.
    pub base_target_index: Option<u32>,
    /// An optional name for this structure.
    pub name: Option<Name>
}

/// A `Mesh` structure cotains data for a single geometric mesh. Each mesh typically contains
/// several arrays of per-vertex data, and one or more index arrays.
///
/// A mesh may contain vertex data for multiple morph targets. The morph target to which the vertex
/// array belongs is determined by the value op its `morph` property.
pub struct Mesh {
    /// Specifies the type of geometric primitive used by the mesh. This must be the same for each
    /// level of detail. See the helper-enum `GeometricPrimitive` for more details about the
    /// different kinds of primitives.
    pub primitive: GeometricPrimitive
    // TODO: Finish this
}

/// Helper enum for the `Mesh` structure, representing different geometric primitives supported by
/// OpenGEX.
///
/// In the documentation, `n` refers to the number of indices if an `IndexArray` structure is
/// present, and otherwise, the number of vertices in every `VertexArray` structure. Primitives are
/// indexed by the letter `i`, starting at zero.
pub enum GeometricPrimitive {
    /// The mesh is composed of a set of independent points. The number of points is `n`, and point
    ///  `i` is given by vertex `i`.
    Points,
    /// The mesh is composed of a set of independent lines. The number of lines equals `n/2`, and
    /// line `i` is composed of vertices `2i` and
    /// `2i+1`.
    Lines,
    /// The mesh is composed of one or more line strips. The number of lines equals `n-1`, and line
    ///  `i` is composed of vertices `i` and
    /// `i+1`.
    LineStrip,
    /// The mesh is composed of a set of independent triangles. The number of triangles equals
    /// `n/3`, and triangle `i` is composed of vertices `3i`, `3i+1` and `3i+1`.
    Triangles,
    /// The mesh is composed of one or more triangle strips.
    ///
    /// When `i` is even, the triangle is composed out of vertices `i`, `i+1` and `i+2`. When `i`
    /// is odd, the triangle is composed out of vertices `i`, `i+2` and `i+1`.
    TriangleStrip,
    /// The mesh is composed of a set of individual quads. The number of quads equals `n/4`, and
    /// quad `i` is composed of vertices `4i`, `4i+1`, `4i+2` and `4i+3`.
    Quads
}

impl Default for GeometricPrimitive {
    fn default() -> GeometricPrimitive {
        GeometricPrimitive::Triangles
    }
}
