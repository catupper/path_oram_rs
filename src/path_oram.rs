use crate::traits::Encrypted;

#[derive(Debug, Clone)]
pub struct PathORAMTree<T>
where
    T: Encrypted,
{
    nodes: Vec<PathORAMNode<T>>,
    block_size: usize,
    depth: usize,
}

impl<T> PathORAMTree<T>
where
    T: Encrypted,
{
    pub fn new(block_size: usize, size: usize) -> Self {
        let mut depth = 1usize;
        while 1 << depth < size * 2 {
            depth += 1;
        }
        let mut nodes = vec![];
        for _ in 0..1 << depth {
            nodes.push(PathORAMNode::new(block_size));
        }
        Self {
            nodes,
            block_size,
            depth,
        }
    }

    pub fn get_path(&self, pos: usize) -> Vec<PathORAMNode<T>> {
        let base_size = 1 << (self.depth - 1);
        assert!(pos < base_size);
        let mut ret = vec![];
        let mut node_ind = pos + base_size;
        while node_ind > 0 {
            ret.push(self.nodes[node_ind].clone());
            node_ind >>= 1;
        }
        ret
    }

    pub fn set_path(&mut self, pos: usize, values: Vec<PathORAMNode<T>>) {
        assert_eq!(values.len(), self.depth);
        let base_size = 1 << (self.depth - 1);
        assert!(pos < base_size);
        let leaf_ind = pos + base_size;
        for (ind, value) in values.into_iter().enumerate() {
            self.nodes[leaf_ind >> ind] = value;
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathORAMNode<T>
where
    T: Encrypted,
{
    entries: Vec<T>,
    block_size: usize,
}

impl<T> PathORAMNode<T>
where
    T: Encrypted,
{
    pub fn new(block_size: usize) -> Self {
        let mut entries = vec![];
        for _ in 0..block_size {
            entries.push(T::dummy());
        }
        Self {
            entries,
            block_size,
        }
    }
}
