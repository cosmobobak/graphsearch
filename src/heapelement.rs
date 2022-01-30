

#[derive(Clone, Copy, Debug)]
pub struct HeapElement<T> {
    node: T,
    cost: i64
}

impl<T> HeapElement<T> {
    pub const fn new(node: T, cost: i64) -> Self {
        Self { node, cost }
    }

    pub const fn node(&self) -> &T {
        &self.node
    }
}

impl<T> PartialEq for HeapElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T> Eq for HeapElement<T> {}

impl<T> PartialOrd for HeapElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl<T> Ord for HeapElement<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}