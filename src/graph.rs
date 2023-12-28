use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;
use itertools::Itertools;
use num::{One, Zero};
use pathfinding::prelude::{bfs, dijkstra};

pub fn all_expand<N, FN, IN>(start: N, mut successors: FN) -> Vec<HashSet<N>>
where
    N: Eq + Hash + Clone,
    FN: FnMut(N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut final_expansions = vec![];
    let mut paths = vec![(start.clone(), HashSet::from([start]))];
    while let Some((cur, visited)) = paths.pop() {
        let mut any = false;
        for suc in successors(cur) {
            if visited.contains(&suc) { continue; }
            any = true;
            let mut set = visited.clone();
            set.insert(suc.clone());
            paths.push((suc, set));            
        }
        if !any {
            final_expansions.push(visited);
        }
    }
    
    final_expansions
}

pub fn floyd_warshall<C>(adjacency_matrix: &[Vec<C>]) -> Vec<Vec<C>>
    where
        C: Zero + Ord + Copy + One,
{
    #[derive(Eq, PartialEq, PartialOrd, Copy, Clone)]
    enum Grid<C: Ord + Add<Output = C>> { Some(C), Inf }
    impl<C: Ord + Add<Output = C>> Add for Grid<C> {
        type Output = Grid<C>;

        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Grid::Some(a), Grid::Some(b)) => Grid::Some(a + b),
                (_, _) => Grid::Inf,
            }
        }
    }
    impl<C: Ord + Add<Output = C>> Grid<C> {
        pub fn unwrap_or_inf(self, def: C) -> C {
            match self {
                Grid::Some(c) => c,
                Grid::Inf => def
            }
        }
    }
    impl<C: Ord + Add<Output = C>> Ord for Grid<C> {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Grid::Some(a), Grid::Some(b)) => a.cmp(b),
                (Grid::Some(_), Grid::Inf) => Ordering::Less,
                (Grid::Inf, Grid::Some(_)) => Ordering::Greater,
                (_, _) => Ordering::Equal,
            }
        }
    }

    let num_nodes = adjacency_matrix.len();
    let mut dist_matrix = adjacency_matrix.into_iter().enumerate().map(|(i, s)| {
        s.into_iter().enumerate().map(|(j, x)| {
            if i == j || !x.is_zero() { Grid::Some(*x) } else { Grid::Inf }
        }).collect_vec()
    }).collect_vec();

    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                dist_matrix[i][j] = std::cmp::min(dist_matrix[i][j], dist_matrix[i][k] + dist_matrix[k][j]);
            }
        }
    }
    
    dist_matrix.into_iter().map(|s| {
        s.into_iter().map(|x| x.unwrap_or_inf(C::zero())).collect_vec()
    }).collect_vec()
}

pub fn create_adjacency_matrix<N, C, FN, IN>(
    nodes: &[N],
    successors: FN,
) -> (Vec<Vec<C>>, HashMap<N, usize>)
    where
        N: Eq + Clone + Hash,
        C: Zero + Ord + Copy + One,
        FN: Fn(&N) -> IN,
        IN: IntoIterator<Item = (N, C)>,
{
    let mut adjacency_matrix = vec![vec![C::zero(); nodes.len()]; nodes.len()];
    let mut node_index_map: HashMap<N, usize> = HashMap::new();

    // Create a mapping from nodes to their indices
    for (index, node) in nodes.iter().enumerate() {
        node_index_map.insert(node.clone(), index);
    }

    // Populate the adjacency matrix
    for (source_index, source_node) in nodes.iter().enumerate() {
        let successors_iter = successors(source_node);

        for (target_node, weight) in successors_iter {
            if let Some(&target_index) = node_index_map.get(&target_node) {
                adjacency_matrix[source_index][target_index] = weight;
            } else {
                // Handle the case where the target node is not found in the provided nodes slice
                panic!("Target node not found in the provided nodes slice");
            }
        }
    }

    (adjacency_matrix, node_index_map)
}

pub fn test_adjacency_matrix() {
    // Example usage
    let nodes = vec!["A".to_owned(), "B".to_owned(), "C".to_owned()];
    let successors = |node: &String| match node.as_str() {
        "A" => vec![("B".to_owned(), 1), ("C".to_owned(), 2)],
        "B" => vec![("A".to_owned(), 3)],
        "C" => vec![("A".to_owned(), 4)],
        _ => vec![],
    };

    let (mut adjacency_matrix, node_index_map) = create_adjacency_matrix(&nodes, successors);

    // Print the adjacency matrix
    for row in &adjacency_matrix {
        println!("{:?}", row);
    }

    // Print the node index map
    for (node, index) in &node_index_map {
        println!("Node: {}, Index: {}", node, index);
    }
    
    adjacency_matrix = floyd_warshall(&adjacency_matrix);

    // Print the adjacency matrix
    for row in &adjacency_matrix {
        println!("{:?}", row);
    }

    // Print the node index map
    for (node, index) in &node_index_map {
        println!("Node: {}, Index: {}", node, index);
    }
}
