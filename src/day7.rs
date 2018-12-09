/// Day 7: Sum of Its Parts
/// 
/// Unravel the order of instructions with dependencies

use std::collections::{HashMap, HashSet};

// Part 1: In what order should the steps be completed?

struct DependencyGraph {
    instructions: HashMap<char, Vec<char>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { instructions: HashMap::new() }
    }

    pub fn add_dependency(&mut self, parent: char, child: char) {
        self.instructions.entry(parent).or_insert(vec![]);
        let child_deps = self.instructions.entry(child).or_insert(vec![]);
        child_deps.push(parent);
    }

    pub fn linearize(&self) -> Vec<char> {
        let mut results: Vec<char> = vec![];
        let mut pending: HashSet<char> = self.instructions.keys().cloned().collect();
        while pending.len() > 0 {
            let mut satisfied: Vec<char> = self.instructions.iter()
                .filter(|(c, deps)| {
                    pending.contains(c) &&
                    deps.iter().all(|dep| !pending.contains(dep))
                })
                .map(|(c, deps)| c.clone())
                .collect();
            satisfied.sort();
            results.push(satisfied[0]);
            pending.remove(&satisfied[0]);
        }
        results
    }
}

/// Given lines of dependencies, processes those dependencies into a linear
/// ordered string of instructions.
pub fn order_steps(text: &str) -> String {
    let mut deps = DependencyGraph::new();
    for line in text.lines() {
        let parent = line.chars().take(6).last().unwrap();
        let child = line.chars().take(37).last().unwrap();
        deps.add_dependency(parent, child);
    }
    deps.linearize().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let instructions = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        assert_eq!(String::from("CABDFE"), order_steps(instructions));
    }
}