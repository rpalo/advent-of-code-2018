/// Day 7: Sum of Its Parts
/// 
/// Unravel the order of instructions with dependencies

use std::collections::{HashMap, HashSet};
use std::cmp;

// Part 1: In what order should the steps be completed?

struct DependencyGraph {
    instructions: HashMap<char, Vec<char>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { instructions: HashMap::new() }
    }

    pub fn from_instructions(text: &str) -> Self {
        let mut deps = DependencyGraph::new();
        for line in text.lines() {
            let parent = line.chars().take(6).last().unwrap();
            let child = line.chars().take(37).last().unwrap();
            deps.add_dependency(parent, child);
        }
        deps
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

    /// Calculate how long it would take if each step has a duration and
    /// you have some elves helping you
    pub fn assisted_assembly_duration(&self, workers: usize, base_delay: usize) -> usize {
        let mut pending: HashSet<char> = self.instructions.keys().cloned().collect();
        let mut active: HashMap<char, usize> = HashMap::new();
        let mut clock: usize = 0;
        
        while pending.len() + active.len() > 0{
            // check for completed steps
            let completed: Vec<char> = active.iter()
                .filter(|(_c, finish)| clock >= **finish)
                .map(|(c, _finish)| c).cloned().collect();
            for letter in completed {
                active.remove(&letter);
            }

            // Get any tasks available to be worked on
            let mut satisfied: Vec<char> = self.instructions.iter()
                .filter(|(c, deps)| {
                    pending.contains(c) &&
                    deps.iter()
                    .all(|dep| !pending.contains(dep) && !active.contains_key(dep))
                })
                .map(|(c, deps)| c.clone())
                .collect();
            satisfied.sort();

            // Give any free workers a task
            let tasks_to_assign = cmp::min(workers - active.len(), satisfied.len());
            for letter in satisfied.iter().take(tasks_to_assign) {
                // This job will get done duration + base_delay seconds from now
                active.insert(*letter, 
                    DependencyGraph::duration_for(*letter) + base_delay + clock);
                pending.remove(letter);
            }

            clock += 1;
        }

        // Un-tick the clock, since the clock ticks at the end.
        clock - 1
    }

    /// Calculates how long a letter will take to process
    /// 
    /// The duration of each letter is increased by one as the letters
    /// increase, starting at A = 1
    /// Assumes (correctly) that all letters are capital
    fn duration_for(letter: char) -> usize {
        (letter as usize) - ('A' as usize) + 1
    }
}

/// Given lines of dependencies, processes those dependencies into a linear
/// ordered string of instructions.
pub fn order_steps(text: &str) -> String {
    let deps = DependencyGraph::from_instructions(text);
    deps.linearize().into_iter().collect()
}

// Part 2: How long will it take to complete all the steps?

/// Find out how long to run a set of tasks with helpers
pub fn assisted_duration(text: &str, workers: usize, base_delay: usize) -> usize {
    let deps = DependencyGraph::from_instructions(text);
    deps.assisted_assembly_duration(workers, base_delay)
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

    #[test]
    fn test_part_two() {
        let instructions = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

        assert_eq!(15, DependencyGraph::from_instructions(instructions)
            .assisted_assembly_duration(2, 0));
    }

    #[test]
    fn test_duration() {
        assert_eq!(1, DependencyGraph::duration_for('A'));
        assert_eq!(26, DependencyGraph::duration_for('Z'));
        assert_eq!(10, DependencyGraph::duration_for('J'));
    }
}