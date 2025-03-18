mod graphs;
mod algos;

fn main() {
    use graphs::{Vertex,Graph};
    use graphs::neighbors::{get_first,get_random};

    let n : usize = 6;
    let edges: Vec<(Vertex,Vertex)> = vec![(0,1), (1,2), (2,3), (3,0), (2,0), (2,4), (2,5)];
                       
    let graph = Graph::create(n,&edges);

    // output all vertex degrees
    print!("Vertex degrees:");
    for list in &(graph.lists) {
        print!(" {}",list.len());
    }

    // count triangles in two different ways
    println!();
    println!("triangles: {}", algos::triangles_matrix(&graph));
    println!("triangles: {}", algos::triangles_mixed(&graph));
    println!();

    let v = 2;
    // output v's first neighbor
    println!("vertex {}'s first neighbor: {:?}", v, get_first(&graph,v));
    // output v's random neighbor
    println!("vertex {}'s random neighbor: {:?}", v, get_random(&graph,v));
}
