// synonim for the vertex type
pub type Vertex = usize; 

pub struct Graph {
    // number of vertices
    pub n: usize, 
    
    // adjacency lists
    pub lists: Vec<Vec<Vertex>>,

    // adjacency matrix
    pub matrix: Vec<Vec<bool>>,
}

impl Graph {

    // create a graph representations from a vector of edge pairs
    pub fn create(n:usize, edges:&Vec<(Vertex,Vertex)>) -> Graph {
        let mut lists = vec![vec![];n];
        let mut matrix = vec![vec![false;n];n];
        for (u,v) in edges {
            lists[*u].push(*v);
            lists[*v].push(*u);
            matrix[*u][*v] = true;
            matrix[*v][*u] = true;
        }
        Graph{n,lists,matrix}
    }
}

pub mod neighbors;
