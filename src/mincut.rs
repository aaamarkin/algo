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

use crate::file::{read_file_to_vec, read_file_to_ale_list};
use crate::{myprintln, myprintln2};

pub struct ALE {
    pub nodes: Vec<i32>,
    pub edges: Vec<i32>
}

impl fmt::Debug for ALE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ALE {{ node: {:?}, edges: {:?} }}", self.nodes, self.edges)
    }
}

fn contract(node_1: usize, node_2: usize, graph: Vec<ALE>) -> Vec<ALE>{
    return graph;
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_contract0() {
        let initialGraph:Vec<ALE> = Vec::new();
        let mut nodes:Vec<i32> = Vec::new();
        let mut edges:Vec<i32> = Vec::new();
        nodes.push(1);
        edges.push(2);
        edges.push(3);


//        contract(t, 0);
        assert_eq!(t, &mut [1,2,3]);
    }


}

pub fn run() {

    let file_name = "MinCut.txt";

    let mut vec = read_file_to_ale_list(file_name).unwrap();

    myprintln2!("Mincut {:?}", vec);

}