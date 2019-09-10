//The file contains the adjacency list representation of a simple undirected graph.
//There are 200 vertices labeled 1 to 200.
//The first column in the file represents the vertex label, and the particular row (other entries except the first column) tells all the vertices that the vertex is adjacent to.
//So for example, the 6th 6th row looks like : "6	155	56	52	120	......".
//This just means that the vertex with label 6 is adjacent to (i.e., shares an edge with) the vertices with labels 155,56,52,120,......,etc
//
//Your task is to code up and run the randomized contraction algorithm for the min cut problem and use it on the above graph to compute the min cut.
//(HINT: Note that you'll have to figure out an implementation of edge contractions.
//Initially, you might want to do this naively, creating a new graph from the old every time there's an edge contraction.
//But you should also think about more efficient implementations.)
//(WARNING: As per the video lectures, please make sure to run the algorithm many times with different random seeds, and remember the smallest cut that you ever find.)
//Write your numeric answer in the space provided. So e.g., if your answer is 5, just type 5 in the space provided.
use std::fmt;
use std::cmp::Ordering;
use crate::file::{read_string_to_ale, read_file_to_string};
use crate::{myprintln, myprintln2};
use std::error::Error;
use rand::Rng;

#[derive(Clone, Eq)]
pub struct ALE {
    pub nodes: Vec<i32>,
    pub edges: Vec<i32>
}

impl fmt::Debug for ALE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ALE {{ node: {:?}, edges: {:?} }}", self.nodes, self.edges)
    }
}

impl PartialEq for ALE {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes &&
            self.edges == other.edges
    }
}

impl Ord for ALE {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nodes.cmp(&other.nodes)
    }
}

impl PartialOrd for ALE {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn naive_contract(node_1: i32, node_2: i32, graph: Vec<ALE>) -> Vec<ALE> {

    let mut vector: Vec<ALE> = Vec::new();


    let mut node_1_ale : ALE = ALE { nodes: vec![], edges: vec![] };
    let mut node_2_ale : ALE = ALE { nodes: vec![], edges: vec![] };


    for i in 0..graph.len(){
        let mut ale = graph[i].clone();

        if !ale.nodes.contains(&node_1) && !ale.nodes.contains(&node_2) {
            for j in 0..ale.edges.len() {
                if ale.edges[j] == node_2 {
                    ale.edges[j] = node_1;
                }
            }
            ale.edges.sort();
            vector.push(ale);
        } else if ale.nodes.contains(&node_1) {
            node_1_ale = ale;
        } else if ale.nodes.contains(&node_2) {
            node_2_ale = ale;
        }
    }

    node_1_ale.nodes.push(node_2);

    let mut edges: Vec<i32> = Vec::new();

    for j in 0..node_2_ale.edges.len() {
        if node_2_ale.edges[j] != node_2 && node_2_ale.edges[j] != node_1 {
            edges.push(node_2_ale.edges[j]);
        }
    }

    for j in 0..node_1_ale.edges.len() {
        if node_1_ale.edges[j] != node_1 && node_1_ale.edges[j] != node_2{
            edges.push(node_1_ale.edges[j]);
        }
    }

    node_1_ale.nodes.sort();
    edges.sort();

    node_1_ale.edges = edges;

    vector.push(node_1_ale);

    vector.sort();

    return vector;
}

fn random_contraction(graph: Vec<ALE>) -> Vec<ALE> {

    let mut g: Vec<ALE> = graph.to_vec();


    while g.len() > 2 {
        let n1: i32 = rng.gen();
        let n2: i32 = rng.gen();
        g = naive_contract(n1, n2, g);
    }

    return g;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_naive_contract0() {

        let initialGraph:Vec<ALE> = vec![ALE { nodes: vec![1], edges: vec![2, 3] }, ALE { nodes: vec![2], edges: vec![1, 3] }, ALE { nodes: vec![3], edges: vec![1, 2] }];

        let resultGraph = naive_contract(1, 2, initialGraph.to_vec());

        let correctResult = vec![ALE { nodes: vec![1, 2], edges: vec![3, 3] }, ALE { nodes: vec![3], edges: vec![1, 1] }];

        assert_eq!(correctResult, resultGraph);
    }

    #[test]
    fn test_naive_contract1() {

        let initialGraph:Vec<ALE> = vec![ALE { nodes: vec![1], edges: vec![2, 3] }, ALE { nodes: vec![2], edges: vec![1, 4] }, ALE { nodes: vec![3], edges: vec![1, 4] }, ALE { nodes: vec![4], edges: vec![2, 3] }];

        let mut resultGraph = naive_contract(1, 2, initialGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1, 2], edges: vec![3, 4] }, ALE { nodes: vec![3], edges: vec![1, 4] }, ALE { nodes: vec![4], edges: vec![1, 3] }];

        assert_eq!(correctResult, resultGraph);

        let mut resultGraph = naive_contract(1, 3, resultGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1, 2, 3], edges: vec![4, 4] }, ALE { nodes: vec![4], edges: vec![1, 1] }];

        assert_eq!(correctResult, resultGraph);
    }

    #[test]
    fn test_naive_contract2() {

        let initialGraph:Vec<ALE> = vec![ALE { nodes: vec![1], edges: vec![2, 3] }, ALE { nodes: vec![2], edges: vec![1, 3, 4] }, ALE { nodes: vec![3], edges: vec![1, 2, 4] }, ALE { nodes: vec![4], edges: vec![2, 3] }];

        let mut resultGraph = naive_contract(1, 2, initialGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1, 2], edges: vec![3, 3, 4] }, ALE { nodes: vec![3], edges: vec![1, 1, 4] }, ALE { nodes: vec![4], edges: vec![1, 3] }];

        assert_eq!(correctResult, resultGraph);

        let mut resultGraph = naive_contract(1, 3, resultGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1, 2, 3], edges: vec![4, 4] }, ALE { nodes: vec![4], edges: vec![1, 1] }];

        assert_eq!(correctResult, resultGraph);
    }

    #[test]
    fn test_naive_contract3() {

        let initialGraph:Vec<ALE> = vec![ALE { nodes: vec![1], edges: vec![2, 3, 4] }, ALE { nodes: vec![2], edges: vec![1, 3, 4] }, ALE { nodes: vec![3], edges: vec![1, 2, 4] }, ALE { nodes: vec![4], edges: vec![1, 2, 3] }];

        let mut resultGraph = naive_contract(2, 3, initialGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1], edges: vec![2, 2, 4] }, ALE { nodes: vec![2, 3], edges: vec![1, 1, 4, 4] }, ALE { nodes: vec![4], edges: vec![1, 2, 2] }];

        assert_eq!(correctResult, resultGraph);

        let mut resultGraph = naive_contract(1, 4, resultGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1, 4], edges: vec![2, 2, 2, 2] }, ALE { nodes: vec![2, 3], edges: vec![1, 1, 1, 1] }];

        assert_eq!(correctResult, resultGraph);
    }

    #[test]
    fn test_naive_contract4() {

        let initialGraph:Vec<ALE> = vec![ALE { nodes: vec![1], edges: vec![2, 3, 4] }, ALE { nodes: vec![2], edges: vec![1, 3, 4] }, ALE { nodes: vec![3], edges: vec![1, 2, 4] }, ALE { nodes: vec![4], edges: vec![1, 2, 3] }];

        let mut resultGraph = naive_contract(2, 3, initialGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1], edges: vec![2, 2, 4] }, ALE { nodes: vec![2, 3], edges: vec![1, 1, 4, 4] }, ALE { nodes: vec![4], edges: vec![1, 2, 2] }];

        assert_eq!(correctResult, resultGraph);

        let mut resultGraph = naive_contract(4, 2, resultGraph.to_vec());

        let mut correctResult = vec![ALE { nodes: vec![1], edges: vec![4, 4, 4] }, ALE { nodes: vec![2, 4], edges: vec![1, 1, 1] }];

        assert_eq!(correctResult, resultGraph);
    }
}

pub fn run() {

    let file_name = "MinCut.txt";

    let mut str = read_file_to_string(file_name).unwrap();

    let mut vec = read_string_to_ale(str);

    myprintln2!("Mincut {:?}", vec);

}