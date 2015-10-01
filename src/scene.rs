/// This structure represents one OpenGEX Scene.
pub struct Scene;

/// The Node structure represents a single generic node in the scene with no associated object.
pub struct Node {
    /// The optional OpenGEX name of this Node structure.
    pub name: Option<String>,
    /// The optional OpenDLL name identifier.
    pub iden: Option<String>
}
