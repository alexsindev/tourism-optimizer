use std::cmp::Ordering;

#[derive(Debug)]
struct HeapNode<T> {
    priority: u64,  // f64 bits for correct ordering
    value: T,
}

impl<T> PartialEq for HeapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for HeapNode<T> {}

impl<T> PartialOrd for HeapNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for HeapNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

pub struct PriorityQueue<T> {
    heap: Vec<HeapNode<T>>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn push(&mut self, priority: f64, value: T) {
        let priority_bits = priority.to_bits();
        self.heap.push(HeapNode { priority: priority_bits, value });
        self.bubble_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let last_idx = self.heap.len() - 1;
        self.heap.swap(0, last_idx);
        let node = self.heap.pop()?;
        if !self.heap.is_empty() {
            self.bubble_down(0);
        }
        Some(node.value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first().map(|n| &n.value)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.heap[idx] > self.heap[parent] {
                self.heap.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        let len = self.heap.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right < len && self.heap[right] > self.heap[largest] {
                largest = right;
            }
            if largest != idx {
                self.heap.swap(idx, largest);
                idx = largest;
            } else {
                break;
            }
        }
    }
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}