#![deny(missing_docs)]

//! Meta rules for parsing the OpenGEX format.

extern crate piston_meta;

use piston_meta::*;
use std::rc::Rc;

pub const DEBUG_ID_SIZE: usize = 1000;

/// Gets the rules for parsing OpenGEX.
pub fn rules(debug_id_offset: usize) -> Rule {
    fn name(text: &str) -> Rc<String> { Rc::new(text.into()) }

    /*
    let animation_name = name("Animation");
    let atten_name = name("Atten");
    let bone_count_array_name = name("BoneCountArray");
    let bone_index_array_name = name("BoneIndexArray");
    let bone_node_name = name("BoneNode");
    let bone_ref_array_name = name("BoneRefArray");
    let bone_weight_array_name = name("BoneWeightArray");
    let camera_node_name = name("CameraNode");
    let camera_object_name = name("CameraObject");
    let clip_name = name("Clip");
    let color_name = name("Color");
    let extension_name = name("Extension");
    let geometry_node_name = name("GeometryNode");
    let geometry_object_name = name("GeometryObject");
    let index_array_name = name("IndexArray");
    let key_name = name("Key");
    let light_node_name = name("LightNode");
    let light_object_name = name("LightObject");
    let material_name = name("Material");
    let material_ref_name = name("MaterialRef");
    let mesh_name = name("Mesh");
    let metric_name = name("Metric");
    let morph_name = name("Morph");
    let morph_weight_name = name("MorphWeight");
    let name_name = name("Name");
    let node_name = name("Node");
    let object_ref_name = name("ObjectRef");
    let param_name = name("Param");
    let rotation_name = name("Rotation");
    let scale_name = name("Scale");
    let skeleton_name = name("Skeleton");
    let skin_name = name("Skin");
    let texture_name = name("Texture");
    let time_name = name("Time");
    let track_name = name("Track");
    let transform_name = name("Transform");
    let translation_name = name("Translation");
    let value_name = name("Value");
    let vertex_array_name = name("VertexArray");
    */

    Rule::Whitespace(Whitespace {
        debug_id: 0,
        optional: true
    })
}
