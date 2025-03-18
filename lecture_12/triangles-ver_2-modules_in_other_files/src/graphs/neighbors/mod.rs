use super::{Graph,Vertex};

// get the first neighbor of vertex from its adjacency list
pub fn get_first(graph: &Graph,vertex:Vertex) -> Option<Vertex> {
    match graph.lists.get(vertex) {
        None => None,
        Some(list) => match list.get(0) {
            None => None,
            Some(u) => Some(*u),
        }
    }
}
