use std::cmp::max;

/// Interval Tree for time-window queries.
/// Stores attractions by their [open_time, close_time] interval.
/// Query: "which attractions are open at time T?" in O(log n + k)
///
/// Implementation: augmented BST where each node stores the
/// maximum close_time in its subtree for efficient pruning.

#[derive(Debug, Clone)]
pub struct Interval {
    pub start: u32,
    pub end: u32,
    pub data: u32,  // attraction id
}

#[derive(Debug)]
struct Node {
    interval: Interval,
    max_end: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct IntervalTree {
    root: Option<Box<Node>>,
}

impl IntervalTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, interval: Interval) {
        self.root = Self::insert_node(self.root.take(), interval);
    }

    fn insert_node(node: Option<Box<Node>>, interval: Interval) -> Option<Box<Node>> {
        let mut node = match node {
            None => {
                return Some(Box::new(Node {
                    max_end: interval.end,
                    interval,
                    left: None,
                    right: None,
                }));
            }
            Some(n) => n,
        };

        if interval.start < node.interval.start {
            node.left = Self::insert_node(node.left.take(), interval);
        } else {
            node.right = Self::insert_node(node.right.take(), interval);
        }

        node.max_end = max(
            node.interval.end,
            max(
                node.left.as_ref().map_or(0, |n| n.max_end),
                node.right.as_ref().map_or(0, |n| n.max_end),
            ),
        );

        Some(node)
    }

    pub fn query_open_at(&self, time: u32) -> Vec<u32> {
        let mut result = Vec::new();
        Self::query_recursive(&self.root, time, &mut result);
        result
    }

    fn query_recursive(node: &Option<Box<Node>>, time: u32, result: &mut Vec<u32>) {
        if let Some(n) = node {
            // If max_end in this subtree is before query time, prune
            if n.max_end < time {
                return;
            }

            // Check left subtree
            Self::query_recursive(&n.left, time, result);

            // Check current interval
            if n.interval.start <= time && time < n.interval.end {
                result.push(n.interval.data);
            }

            // Check right subtree
            Self::query_recursive(&n.right, time, result);
        }
    }
}

impl Default for IntervalTree {
    fn default() -> Self {
        Self::new()
    }
}