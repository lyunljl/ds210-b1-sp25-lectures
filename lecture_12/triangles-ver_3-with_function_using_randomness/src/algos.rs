use crate::graphs::Graph;

// counting triangles via enumerating all triples
// and checking if they are connected, usin the
// adjacency matrix
pub fn triangles_matrix(graph:&Graph) -> usize {
    let n = graph.n;
    let mut count = 0;
    let matrix = &(graph.matrix);
    for u in 0..n {
        for v in u+1..n {
            for w in v+1..n {
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
pub fn triangles_mixed(graph:&Graph) -> usize {
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
    count/6
}
