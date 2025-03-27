// Define some datatype synonyms
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
type Component = usize;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

// reverse direction of edges on a list
fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
}


fn mark_component_dfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    component[vertex] = Some(component_no);
    for w in graph.outedges[vertex].iter() {
        if let None = component[*w] {
            mark_component_dfs(*w,graph,component,component_no);
        }        
    }
}

fn dfs_collect_stack(v:Vertex, graph:&Graph, stack:&mut Vec<Vertex>, visited:&mut Vec<bool>) {
    if !visited[v] {
        visited[v] = true;
        for w in graph.outedges[v].iter() {
            dfs_collect_stack(*w, graph, stack, visited);
        }
        stack.push(v);
        println!("pushed {}", v);
    }
}

fn main() {
    let n: usize = 7;
    let edges: ListOfEdges = vec![(0,1),(1,2),(2,0),(3,4),(4,5),(5,3),(2,3),(6,5)];
    let graph = Graph::create_directed(n, &edges);
    let graph_reverse = Graph::create_directed(n,&reverse_edges(&edges));
    println!("{:?}\n{:?}",graph,graph_reverse);

    // First DFS
    let mut stack: Vec<Vertex> = Vec::new();
    let mut visited = vec![false;graph.n];

    for v in 0..graph.n {
        dfs_collect_stack(v,&graph,&mut stack,&mut visited);
    };

    println!(" {:?}", stack);

    // Second DFS, reversed graph
    let mut component: Vec<Option<Component>> = vec![None;graph.n];
    let mut component_count = 0;

    while let Some(v) = stack.pop() {
        if let None = component[v] {
            component_count += 1;
            mark_component_dfs(v, &graph_reverse, &mut component, component_count);
        }
    };

    print!("{} components:\n",component_count);
    for c in 1..=component_count {
        print!("Component {}: ", c);
        for v in 0..n {
            if component[v].unwrap() == c {
                print!("{} ",v);
            }
        }
        println!();
    }
    println!();

}
