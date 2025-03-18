mod graphs {

    // synonym for the vertex type
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
        pub fn create(n: usize, edges: &Vec<(Vertex, Vertex)>) -> Graph {
            let mut lists = vec![vec![]; n];
            let mut matrix = vec![vec![false; n]; n];
            for (u, v) in edges {
                lists[*u].push(*v);
                lists[*v].push(*u);
                matrix[*u][*v] = true;
                matrix[*v][*u] = true;
            }
            Graph { n, lists, matrix }
        }
    }

    pub mod neighbors {
        use super::{Graph, Vertex};

        // get the first neighbor of vertex from its adjacency list
        pub fn get_first(graph: &Graph, vertex: Vertex) -> Option<Vertex> {
            match graph.lists.get(vertex) {
                None => None,
                Some(list) => match list.get(0) {
                    None => None,
                    Some(u) => Some(*u),
                },
            }
        }
    }
}

mod algos {
    use crate::graphs::Graph;

    // counting triangles via enumerating all triples
    // and checking if they are connected, usin the
    // adjacency matrix
    pub fn triangles_matrix(graph: &Graph) -> usize {
        let n = graph.n;
        let mut count = 0;
        let matrix = &(graph.matrix);
        for u in 0..n {
            for v in u + 1..n {
                for w in v + 1..n {
                    if matrix[u][v] && matrix[u][w] && matrix[v][w] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    // counting triangles by enumerating all pairs of neighbors
    // for every vertex (via adjacency lists) and then checking
    // which ones are connected (via adjacency matrix)
    pub fn triangles_mixed(graph: &Graph) -> usize {
        let n = graph.n;
        let mut count = 0;
        let matrix = &(graph.matrix);
        for u in 0..n {
            for v in &graph.lists[u] {
                for w in &graph.lists[u] {
                    if matrix[*v][*w] {
                        count += 1;
                    }
                }
            }
        }
        count / 6
    }
}

fn main() {
    use algos::{triangles_matrix, triangles_mixed};
    use graphs::neighbors::get_first;
    use graphs::{Graph, Vertex};

    let n: usize = 6;
    let edges: Vec<(Vertex, Vertex)> = vec![(0, 1), (1, 2), (2, 3), (3, 0), (2, 0), (2, 4), (2, 5)];

    let graph = Graph::create(n, &edges);

    // output all vertex degrees
    print!("Vertex degrees:");
    for list in &(graph.lists) {
        print!(" {}", list.len());
    }

    // count triangles in two different ways
    println!();
    println!("triangles: {}", triangles_matrix(&graph));
    println!("triangles: {}", triangles_mixed(&graph));
    println!();

    let v = 2;
    // output v's first neighbor
    println!("vertex {}'s first neighbor: {:?}", v, get_first(&graph, v));
}
