use super::{Graph,Vertex};
use rand::Rng;

// get the first neighbor of vertex from its adjacency list
pub fn get_first(graph:&Graph,vertex:Vertex) -> Option<Vertex> {
    match graph.lists.get(vertex) {
        None => None,
        Some(list) => match list.get(0) {
            None => None,
            Some (u) => Some(*u),
        }
    }
}

// get a random neighbor of vertex from its adjacency list
pub fn get_random(graph:&Graph,vertex:Vertex) -> Option<Vertex> {
    match graph.lists.get(vertex) {
        None => None,
        Some(list) => {
            if list.len() == 0 {
                return None
            }
            let index = rand::thread_rng().gen_range(0..list.len());
            Some(list[index])
        }
    }
}
