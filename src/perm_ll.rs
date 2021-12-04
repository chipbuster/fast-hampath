/*! Rust's std::collections linked list do not let you insert a node once you
 * have partial traversal of the list. Unfortunately that's sort of what we need
 * in order to be able to insert nodes into the path. Fortunately, we can do
 * slightly better: we know that our final data structure will contain a permutation
 * of the numbers 0..N, where N is the size of the graph. We can use this to
 * construct a structure which supports the important linked list operations.
 */

/// A Permutation Linked List
pub struct PLinkedList {
    first: usize,
    last: usize,
    links: Vec<Option<usize>>
}

impl PLinkedList {
    pub fn new(length: usize, first: usize) -> Self {
        Self {
            first,
            last: first,
            links: vec![None; length]
        }
    }
    
    pub fn get_succ(&self, cur: usize) -> Option<usize> {
        self.links[cur]
    }
    
    pub fn insert_after(&mut self, cur: usize, next: usize) {
        let cur_succ = self.links[cur];
        self.links[cur] = Some(next);
        self.links[next] = cur_succ;

        if cur == self.last {
            self.last = next;
        }
    }

    pub fn insert_at_end(&mut self, new: usize) {
        self.links[self.last] = Some(new);
        self.last = new;
    }

    pub fn insert_at_start(&mut self, new: usize) {
        self.links[new] = Some(self.first);
        self.first = new;
    }

    pub fn first(&self) -> usize {
        self.first
    }

    pub fn last(&self) -> usize {
        self.last
    }

    pub fn iter(&self) -> PLinkedListIterator {
        PLinkedListIterator::new(self)
    }
}

pub struct PLinkedListIterator<'a> {
    target: &'a PLinkedList,
    cur: Option<usize>
}

impl<'a> PLinkedListIterator<'a> {
    pub fn new(target: &'a PLinkedList) -> Self {
        Self {
            target,
            cur: Some(target.first)
        }
    }
}

impl<'a> Iterator for PLinkedListIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let rv = self.cur;
        self.cur = self.target.links[rv?];
        rv
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn plinkedlist_insert_create() {
        let mut pll = PLinkedList::new(10, 2);
        pll.insert_after(2, 3);
        pll.insert_after(3, 1);
        pll.insert_after(1, 7);
        pll.insert_after(7, 4);
        pll.insert_after(4, 5);
        pll.insert_after(5, 9);
        pll.insert_after(9, 0);
        pll.insert_after(0, 6);
        pll.insert_after(6, 8);
        let order = pll.iter().collect::<Vec<_>>();
        assert_eq!(order, vec![2,3,1,7,4,5,9,0,6,8]);
    }

    /// Check that the insert_after function correctly retains list info
    #[test]
    pub fn plinkedlist_insert() {
        let mut pll = PLinkedList::new(3, 0);
        pll.insert_after(0, 2);
        pll.insert_after(0, 1);
        let order = pll.iter().collect::<Vec<_>>();
        assert_eq!(order, vec![0, 1, 2]);
    }

    #[test]
    pub fn plinkedlist_prepend() {
        let mut pll = PLinkedList::new(3, 0);
        pll.insert_after(0, 2);
        pll.insert_at_start(1);
        let order = pll.iter().collect::<Vec<_>>();
        assert_eq!(order, vec![1, 0, 2]);
    }

    #[test]
    pub fn plinkedlist_append() {
        let mut pll = PLinkedList::new(3, 0);
        pll.insert_after(0, 2);
        pll.insert_at_end(1);
        let order = pll.iter().collect::<Vec<_>>();
        assert_eq!(order, vec![0, 2, 1]);
    }
    
}