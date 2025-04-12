/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // 1、如果根节点为空，则创建一个新节点
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        // 2、根节点不为空
        let current = self.root.as_mut().unwrap();
        current.insert(value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let current = self.root.as_ref();
        fn sub_search<T: Ord>(current: Option<&Box<TreeNode<T>>>, value: T) -> bool {
            match current {
                Some(node) => {
                    if node.value == value {
                        return true;
                    }
                    if node.left.is_none() && node.right.is_none() {
                        return false;
                    }
                    // 如果当前节点值小于value，则继续查找左子树
                    if value < node.value {
                        sub_search(node.left.as_ref(), value)
                    } else {
                        sub_search(node.right.as_ref(), value)
                    }
                }
                None => {
                    return false;
                }
            }
        }
        sub_search(current, value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        fn sub_insert<T: Ord>(current: &mut TreeNode<T>, value: T) {
            if current.value == value {
                return;
            }
            // 2.1、如果值小于根节点，则插入到左子树
            if value < current.value {
                if current.left.is_none() {
                    current.left = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                sub_insert(current.left.as_mut().unwrap(), value);
            } else {
                // 2.2、如果值大于根节点，则插入到右子树
                if current.right.is_none() {
                    current.right = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                sub_insert(current.right.as_mut().unwrap(), value);
            }
        }
        sub_insert(self, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);
        println!("bst.root: {:?}", bst.search(1));
        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
