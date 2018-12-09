/// Day 8: Memory Maneuver
/// 
/// Build a license tree!

pub struct Node {
    metadata: Vec<usize>,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self { metadata: vec![], children: vec![] }
    }

    pub fn from_text(text: &str) -> Self {
        let data: Vec<usize> = text.split(' ').map(|num|{
            num.parse().unwrap()
        }).collect();
        let (node, _ptr) = Node::build_child(&data, 0);
        node
    }

    fn build_child(data: &Vec<usize>, start: usize) -> (Node, usize) {
        let mut result = Node::new();
        let mut ptr = start;
        let children = data[ptr];
        ptr += 1;
        let metadata = data[ptr];
        ptr += 1;
        for i in 0..children {
            let (node, new_ptr) = Node::build_child(&data,ptr);
            result.children.push(Box::new(node));
            ptr = new_ptr;
        }
        result.metadata.extend(&data[ptr..(ptr+metadata)]);
        ptr += metadata;
        (result, ptr)
    }

    pub fn metadata_total(&self) -> usize {
        let my_metadata: usize = self.metadata.iter().sum();
        let children_total: usize = self.children.iter()
            .map(|child| child.metadata_total()).sum();
        my_metadata + children_total
    }
}

pub fn total_metadata(root: &Node) -> usize {
    root.metadata_total()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let license = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(138, total_metadata(&Node::from_text(license)));
    }
}