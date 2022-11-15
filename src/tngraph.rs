use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cell::UnsafeCell;
use typed_arena::Arena;
use rand::random;
use std::fmt;

pub type NodeID = usize;

#[derive(Debug)]
pub struct Node<'a> {
    nodeid: NodeID,
    out_edges: UnsafeCell<Vec<&'a Node<'a>>>,
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<'a> Eq for Node<'a> {}

impl<'a> Hash for Node<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = self as *const Node;
        addr.hash(state);
    }
}


/* A node in the tournament graph. IMPORTANT: We use an UnsafeCell to be able to
edit the node while it's immutable. This *must not* be used outside of the
construction of the graph or we will certainly race. To do this, we only allow
mutable access to the cell in private functions. Once graph construction
is complete, we should never use out_edges.get() mutably. */

impl<'a> Node<'a> {
    fn new(node_id: NodeID, arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
        arena.alloc(Self {
            nodeid: node_id,
            out_edges: UnsafeCell::new(Vec::new()),
        })
    }

    pub fn nodeid(&self) -> NodeID {
        self.nodeid
    }

    /// Inserts an edge from this node to the given other node.
    /// Safety: can only be called if there are no live &Node references to this
    ///         node or to this node's out_edges.
    unsafe fn insert_edge_to(&self, other: &'a Node<'a>) {
        /* Since tournament graphs are immutable, this should be safe as long as
           it is unused outside of graph construction. */
        (*self.out_edges.get()).push(other);
    }

    /// Returns a reference to out_edges
    fn get_edges(&self) -> &Vec<&'a Node <'a>> {
        // This is unsafe if used while mutating the vector in the creation phase
        unsafe { &(*self.out_edges.get()) }
    }

    /// Returns a vector of NodeIDs for which there are edges from us to those nodes
    pub fn neighbor_ids(&self) -> Vec<NodeID> {
        unsafe { (*self.out_edges.get()).iter().map(|x| x.nodeid).collect() }
    }

    /// Returns a copy of out_edges
    pub fn neighbors(&self) -> Vec<&'a Node<'a>> {
        unsafe { (*self.out_edges.get()).clone() }
    }

}

pub struct TournamentGraph<'a> {
    nodes: Vec<&'a Node<'a>>,
}

impl<'a> fmt::Display for TournamentGraph<'a>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.nodes.len() {
            let cur_node = self.get_node(x);
            let neighs = cur_node.neighbor_ids();
            for i in 0..self.nodes.len() {
                if neighs.contains(&i) {
                    write!(f, "+1 ")?;
                } else {
                    write!(f, "-1 ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<'a> TournamentGraph<'a> {
    /// Constructs a tournament graph without verifying the tournament property
    pub fn new_unchecked(
        n: usize,
        edges: Vec<(NodeID, NodeID)>,
        arena: &'a Arena<Node<'a>>,
    ) -> Self {
        let mut nodes = Vec::new();
        for i in 0..n {
            let newnode = Node::new(i, arena);
            nodes.push(&*newnode);
        }

        for (src, snk) in edges {
            // This assert necessary to uphold safety of the insert_edge_to call
            assert_ne!(src, snk, "Got request to insert self-edge on node {}", src);
            let sinkref = nodes[snk];
            unsafe{ nodes[src].insert_edge_to(sinkref); }
        }

        Self { nodes }
    }

    pub fn new(
        n: usize,
        edges: Vec<(NodeID, NodeID)>,
        arena: &'a Arena<Node<'a>>,
    ) -> Option<Self> {
        let result = Self::new_unchecked(n, edges, arena);
        if result.is_valid_tournament_graph() {
            Some(result)
        } else {
            None
        }
    }

    pub fn new_random(n: usize, arena: &'a Arena<Node<'a>>) -> Self {
        let edges = Self::random_edges(n);
        Self::new_unchecked(n, edges, arena)
    }

    pub fn get_node(&self, id: NodeID) -> &'a Node<'a> {
        self.nodes[id]
    }

    fn random_edges(n: usize) -> Vec<(NodeID, NodeID)> {
        let mut out = Vec::new();
        for i in 0..n {
            for j in 0..i {
                let x = random::<bool>();
                if x {
                    out.push((i,j));
                } else {
                    out.push((j, i))
                }
            }
        }
        out
    }

    pub fn validate_path(&self, path: &[NodeID]) -> bool {
        if path.len() != self.nodes.len() {
            return false;
        }
        let mut cur_id = path[0];
        for next_id in &path[1..] {
            if self.get_node(cur_id).neighbor_ids().contains(next_id) {
                cur_id = *next_id;
            }
            else {
                return false;
            }
        }
        true
    }

    fn is_valid_tournament_graph(&self) -> bool {
        // Check for no duplicate edges
        for i in 0..self.nodes.len() {
            let mut outs = HashSet::new();
            let oes = self.nodes[i].get_edges();
            for oe in oes {
                outs.insert(*oe);
            }

            if oes.len() > outs.len() {
                return false;
            }
        }

        // Check for tournament property: for every (i,j), there is an edge
        // from i to j or an edge from j to i. Not both, not neither.
        for i in 0..self.nodes.len(){
            for j in i+1..self.nodes.len() {
                // We can check nodeids since we're working in the same graph here (guaranteed)
                let i_to_j = self.nodes[i].get_edges().iter().any(|x| x.nodeid == j);
                let j_to_i = self.nodes[j].get_edges().iter().any(|x| x.nodeid == i);
                if !(j_to_i ^ i_to_j) {
                    return false
                }
            }
        }

        true
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensure that node equivalent is working as expected (address equivalence)
    #[test]
    fn test_tourneynode_eq() {
        let arena = Arena::new();
        let a1 = Node::new(1, &arena);
        let a2 = Node::new(1, &arena);
        assert_eq!(a1, a1);
        assert_ne!(a1, a2);
    }

    #[test]
    fn simple_test_generation(){
        let arena = Arena::new();
        let size = 2;
        let r = vec![(0,1)];
        let n = TournamentGraph::new_unchecked(size, r.clone(), &arena);
        assert!(n.is_valid_tournament_graph(), "Edges {:?} result in an invalid tournament graph", r);
    }

    #[test]
    fn randomized_test_generation(){
        let arena = Arena::new();
        let size = 100;
        for _ in 0..100 {
            let r = TournamentGraph::random_edges(size);
            let n = TournamentGraph::new_unchecked(size, r.clone(), &arena);
            assert!(n.is_valid_tournament_graph(), "Edges {:?} result in an invalid tournament graph", r);
        }
    }
}
