use super::VertexAttr;

pub trait Vertex {
    fn attr() -> Vec<VertexAttr>;
}
