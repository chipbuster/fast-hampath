/*! "Naive" fast Hamiltonian path: do the search breakup, but without intelligent
 * searching for "switching" locations on the interior. */

use crate::tngraph::{TournamentGraph, Node, NodeID};
use crate::perm_ll::PLinkedList;
use typed_arena::Arena;

pub struct HampathBuilder<'a> {
    num_nodes: usize,  // The number of nodes in a completed path
    last_node: usize,  // The last node to appear in the current path
    cur_path: PLinkedList,
    graph: TournamentGraph<'a>,
}

impl<'a> HampathBuilder<'a> {

    pub fn new(n: usize, edges: Vec<(NodeID, NodeID)>, arena: &'a Arena<Node<'a>>) -> Self {
        let graph = TournamentGraph::new(n, edges, arena).expect("Invalid edge array in construction!");
        Self {
            num_nodes: n,
            last_node: 0,
            cur_path: PLinkedList::new(n, 0),
            graph,
        }
    }

    pub fn new_random(n: usize, arena: &'a Arena<Node<'a>>) -> Self {
        let graph = TournamentGraph::new_random(n, arena);
        Self {
            num_nodes: n,
            last_node: 0,
            cur_path: PLinkedList::new(n, 0),
            graph,
        }
    }

    pub fn solution_pair(mut self) -> (Vec<NodeID>, TournamentGraph<'a>) {
        let path = self.solve_path();
        let graph = self.into_graph();
        (path, graph)
    }

    pub fn into_graph(self) -> TournamentGraph<'a> {
        self.graph
    }

    pub fn solve_path(&mut self) -> Vec<NodeID> {
        while self.last_node + 1 < self.num_nodes {
            self.extend(self.last_node + 1);
            self.last_node += 1;
        }
        self.cur_path.iter().collect::<Vec<_>>()
    }

    /// Given the neighbors of the node to be inserted and the path so far,
    /// returns the NodeID that the new node should be inserted after
    fn search_for_insert_point(neighs: Vec<NodeID>, path: Vec<NodeID>) -> NodeID{
        for win in path.windows(2) {
            let (prev_i, next_i) = (win[0], win[1]);
            if neighs.contains(&next_i){
                return prev_i;
            }
        }
        panic!("Did not find insertion point in internal search!");
    }

    /// Extends the path by adding the next unknown node into the cur_path list 
    fn extend(&mut self, new_nid: NodeID) {
        let first_nid = self.cur_path.first();
        let last_nid = self.cur_path.last();
        assert!(new_nid < self.num_nodes, "Tried to extend to node {} in a {} graph", new_nid, self.num_nodes);

        // Easy case: if path from new node to first node, prepend
        if self.graph.get_node(new_nid).neighbor_ids().contains(&first_nid){
            self.cur_path.insert_at_start(new_nid);
            return;
        }

        // Easy case: if path from last node to new node, append
        if self.graph.get_node(last_nid).neighbor_ids().contains(&new_nid){
            self.cur_path.insert_at_end(new_nid);
            return;
        }

        // Tricky case: have to search list for new nodes
        let neighs = self.graph.get_node(new_nid).neighbor_ids();
        let path = self.cur_path.iter().collect::<Vec<_>>();
        let i_id = Self::search_for_insert_point(neighs, path);
        self.cur_path.insert_after(i_id, new_nid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_hampath_solve(){
        let a = Arena::new();
        let b = HampathBuilder::new_random(500, &a);
        let (path, graph) = b.solution_pair();
        assert!(graph.validate_path(&path[..]), "Path {:?} is invalid for graph:\n{}", path, graph);
    }

    #[test]
    fn example_1(){
        let edges = vec![
            (0, 1), (2, 0), (0, 3), (0, 4),
            (2, 1), (3, 1), (4,1),
            (3, 2), (2, 4),
            (3, 4)
        ];
        let a = Arena::new();
        let b = HampathBuilder::new(5, edges, &a);
        let (path, graph) = b.solution_pair();
        assert!(graph.validate_path(&path[..]), "Path {:?} is invalid for graph:\n{}", path, graph);
    }

    #[test]
    fn example_2(){
        let edges = vec![
            (1,0), (0,2), (3,0), (0,4),
            (2,1), (3,1), (1,4),
            (2,3), (2,4),
            (4,3)
        ];
        let a = Arena::new();
        let b = HampathBuilder::new(5, edges, &a);
        let (path, graph) = b.solution_pair();
        assert!(graph.validate_path(&path[..]), "Path {:?} is invalid for graph:\n{}", path, graph);
    }
}