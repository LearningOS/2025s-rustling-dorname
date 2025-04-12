/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.bubble_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 添加 bubble_up 方法
    fn bubble_up(&mut self, idx: usize) {
        let mut current = idx;
        // 当前节点不是根节点（索引>1）且不满足堆的性质时继续调整
        while current > 1 {
            let parent = self.parent_idx(current);
            // 如果父节点和当前节点满足堆的性质，则停止
            if (self.comparator)(&self.items[parent], &self.items[current]) {
                break;
            }
            // 否则交换父节点和当前节点
            self.items.swap(parent, current);
            current = parent;
        }
    }

    // 2. smallest_child_idx 方法（实际上是"最优"子节点的索引，取决于是最大堆还是最小堆）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果右子节点存在，比较左右子节点
        if right <= self.count {
            // 使用比较器选择"最优"子节点
            if (self.comparator)(&self.items[right], &self.items[left]) {
                right
            } else {
                left
            }
        } else {
            // 只有左子节点
            left
        }
    }

    // 添加 bubble_down 方法（供 next 方法使用）
    fn bubble_down(&mut self, idx: usize) {
        let mut current = idx;
        // 当还有子节点时继续调整
        while self.children_present(current) {
            let child = self.smallest_child_idx(current);
            // 如果当前节点和子节点满足堆的性质，则停止
            if (self.comparator)(&self.items[current], &self.items[child]) {
                break;
            }
            // 否则交换当前节点和子节点
            self.items.swap(current, child);
            current = child;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 保存堆顶元素
        let result = std::mem::replace(&mut self.items[1], T::default());

        // 将最后一个元素移到堆顶
        if self.count > 1 {
            self.items[1] = self.items[self.count].clone();
        }

        // 更新计数并删除最后一个元素
        self.count -= 1;
        self.items.pop();

        // 如果堆不为空，从堆顶开始向下调整
        if self.count > 0 {
            self.bubble_down(1);
        }

        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
