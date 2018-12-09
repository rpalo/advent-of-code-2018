/// Day 8: Memory Maneuver
/// 
/// Build a license tree!

/// A node in a GPS Licensing tree structure
pub struct Node {
    metadata: Vec<usize>,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self { metadata: vec![], children: vec![] }
    }

    /// Generates a node from a string of space-separated integers
    pub fn from_text(text: &str) -> Self {
        let data: Vec<usize> = text.split(' ').map(|num|{
            num.parse().unwrap()
        }).collect();
        let (node, _ptr) = Node::build_child(&data, 0);
        node
    }

    /// Builds a child based on a strand of data and a pointer to start at.
    /// 
    /// These nodes are recursive in their layout.  So, for example, 
    /// the root node has a header at the start of the string, and its
    /// metadata comes after all of the rest of the nodes in the tree
    fn build_child(data: &Vec<usize>, start: usize) -> (Node, usize) {
        let mut result = Node::new();
        let mut ptr = start;
        let children = data[ptr];
        ptr += 1;
        let metadata = data[ptr];
        ptr += 1;

        // Generate and add children
        for _i in 0..children {
            let (node, new_ptr) = Node::build_child(&data,ptr);
            result.children.push(Box::new(node));
            ptr = new_ptr;
        }
        
        result.metadata.extend(&data[ptr..(ptr+metadata)]);
        ptr += metadata;

        (result, ptr)
    }

    /// Calculate the recurive total of all the metadata here and below
    pub fn metadata_total(&self) -> usize {
        let my_metadata: usize = self.metadata.iter().sum();
        let children_total: usize = self.children.iter()
            .map(|child| child.metadata_total()).sum();
        my_metadata + children_total
    }

    /// Calculates a node's value.
    /// 
    /// Value is defined like this:
    ///  - if a node has no children, it's the sum of the metadata
    ///  - if a node *does* have children, value is defined recursively,
    ///    and each metadata is a pointer at a particular child.
    ///    This node's value is the sum of *those* nodes' values.
    ///    If a pointer is invalid, skip it.
    pub fn value(&self) -> usize {
        if self.children.is_empty() { return self.metadata.iter().sum(); }

        let mut total: usize = 0;
        for pointer in self.metadata.iter() {
            if *pointer < 1 || *pointer > self.children.len() { continue; }

            total += self.children.get(*pointer - 1)
                .expect("Couldn't get child value")
                .value();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let license = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(138, Node::from_text(license).metadata_total());
    }

    #[test]
    fn test_part_two() {
        let license = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(66, Node::from_text(license).value());
    }
}