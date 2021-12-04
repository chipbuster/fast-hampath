/*! "Naive" fast Hamiltonian path: do the search breakup, but without intelligent
 * searching for "switching" locations on the interior. */

use crate::tngraph::{TournamentGraph, Node, NodeID};
use crate::perm_ll::PLinkedList;
use typed_arena::Arena;

/* General strategy: 

*/

pub struct HampathBuilder<'a> {
    num_nodes: usize,  // The number of nodes in a completed path
    last_node: usize,  // The last node to appear in the current path
    cur_path: PLinkedList,
    graph: TournamentGraph<'a>,
    arena: &'a Arena<Node<'a>>,
}

impl<'a> HampathBuilder<'a> {
    pub fn new_random(n: usize, arena: &'a Arena<Node<'a>>) -> Self {
        let graph = TournamentGraph::new_random(n, &arena);
        Self {
            num_nodes: n,
            last_node: 0,
            cur_path: PLinkedList::new(n, 0),
            graph,
            arena: &arena
        }
    }

    pub fn into_graph(self) -> TournamentGraph<'a> {
        self.graph
    }

    pub fn solve_path(&mut self) -> Vec<NodeID> {
        while self.last_node < self.num_nodes-1 {
            println!("{}", self.last_node);
            self.extend()
        }
        self.cur_path.iter().collect::<Vec<_>>()
    }

    /// Extends the path by adding the next unknown node into the cur_path list 
    fn extend(&mut self) {
        let new_nid = self.last_node + 1;
        let first_nid = self.cur_path.first();
        let last_nid = self.cur_path.last();
        assert!(new_nid < self.num_nodes, "Tried to extend to node {} in a {} graph", new_nid, self.num_nodes);

        // Increment last_node here to avoid having to duplicate it below, but
        // note that last_node is *incorrect* until this function finishes
        self.last_node += 1;


        // Easy case: if path from new node to first node, prepend
        if self.graph.get_node(new_nid).neighbor_ids().contains(&first_nid){
            self.cur_path.insert_at_start(new_nid);
            return;
        }

        // Easy case: if path from last node to new node, append
        if self.graph.get_node(last_nid).neighbor_ids().contains(&new_nid){
            self.cur_path.insert_at_end(new_nid);
        }

        // Tricky case: have to search list for new nodes
        let neighs = self.graph.get_node(new_nid).neighbor_ids();
        let path = self.cur_path.iter().skip(1).collect::<Vec<_>>();
        for i in path.into_iter() {
            if neighs.contains(&i){
                self.cur_path.insert_after(i, new_nid);
            }
        }
    }
}