#![deny(missing_docs)]

//! Meta rules for parsing the OpenGEX format.

extern crate piston_meta;
extern crate petgraph;

/// The node type used in the graph description of a scene.
pub enum Node {
    /// An entity is a unique object connected to components.
    /// Components that have no relations to other nodes are stored externally,
    /// while components that have relations are stored as nodes in the graph.
    Entity(Entity),
}

/// The edge type used in the graph description of a scene.
pub enum Edge {
    /// A node is connected to another as a component.
    Component,
}

/// An entity contains external components that are stored outside the graph.
pub struct Entity {
    /// External components.
    pub components: Vec<Component>,
}

/// Components refers to external data with no relations to other nodes.
/// An entity can contain any number of node and object transformations.
/// The node transformations becomes inherited by children in a scene structure.
/// The object transformations are local for the object and does not get inherited.
pub enum Component {
    /// Name id, for display in editor.
    Name(u32),
    /// A 4x4 transformation matrix.
    NodeTransform(u32),
    /// A 4x4 transformation matrix.
    ObjectTransform(u32),
    /// Translation vector.
    NodeTranslation(u32),
    /// Translation vector.
    ObjectTranslation(u32),
    /// Translation along an axis.
    NodeAxisTranslation(u32),
    /// Translation along an axis.
    ObjectAxisTranslation(u32),
    /// Rotation around an arbitrary axis.
    NodeRotation(u32),
    /// Rotation around an arbitrary axis.
    ObjectRotation(u32),
    /// Rotation around a specified axis.
    NodeAxisRotation(u32),
    /// Rotation around a specified axis.
    ObjectAxisRotation(u32),
    /// Rotation using a quaternion.
    NodeQuaternionRotation(u32),
    /// Rotation using a quaternion.
    ObjectQuaternionRotation(u32),
    /// Scale vector.
    NodeScale(u32),
    /// Scale vector.
    ObjectScale(u32),
}

/// Represents different axis.
#[derive(Copy, Clone)]
pub enum Axis {
    /// X axis.
    X,
    /// Y axis.
    Y,
    /// Z axis.
    Z
}

/// Stores the scene data read from the OpenGEX format.
pub struct Scene {
    /// The factor to multiply distances.
    pub distance_metric: f32,
    /// The factor to multiply angles.
    pub angle_metric: f32,
    /// The factor to multiply time.
    pub time_metric: f32,
    /// The axis to use as up direction.
    pub up_metric: Axis,
    /// The graph data.
    pub graph: petgraph::Graph<Node, Edge>,
    /// Names for display in an editor.
    pub names: Vec<String>,
    /// 4x4 transformation matrices.
    pub transformations: Vec<[[f32; 4]; 4]>,
    /// Translation vectors.
    pub translations: Vec<[f32; 3]>,
    /// Translation along an axis.
    pub axis_translations: Vec<(Axis, f32)>,
    /// Rotation around an arbitrary axis.
    pub rotations: Vec<(f32, [f32; 3])>,
    /// Rotation around a specific axis.
    pub axis_rotation: Vec<(Axis, f32)>,
    /// Rotation using a quaternion.
    pub quaternion_rotation: Vec<(f32, [f32; 3])>,
}

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
    let data = match data {
        Ok(data) => data,
        Err((range, err)) => {
            // Report the error to standard error output.
            ParseStdErr::new(&source).error(range, err);
            assert!(false);
            unreachable!();
        }
    };
    json::print(&data);
    // assert!(false);
}
