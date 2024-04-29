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

impl<T: std::clone::Clone> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
       
        if self.search(value.clone()) {
            return; // 如果值已经存在于树中，则直接返回
        }

        if let Some(ref mut root) = self.root {
            // 如果根节点存在，则从根节点开始逐步查找合适的插入位置
            let mut current = root;

            loop {
                if value < current.value {
                    // 如果值小于当前节点的值，则插入到当前节点的左子树中
                    if let Some(ref mut left) = current.left {
                        current = left;
                    } else {
                        // 如果当前节点的左子树为空，则将新节点插入到当前节点的左子树中
                        current.left = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                } else {
                    // 如果值大于等于当前节点的值，则插入到当前节点的右子树中
                    if let Some(ref mut right) = current.right {
                        current = right;
                    } else {
                        // 如果当前节点的右子树为空，则将新节点插入到当前节点的右子树中
                        current.right = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                }
            }
        } else {
            // 如果根节点不存在，则创建一个新的根节点
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = self.root.as_ref();

        while let Some(node) = current {
            if value == node.value {
                return true; // 如果找到目标值，则返回true
            } else if value < node.value {
                current = node.left.as_ref(); // 如果目标值小于当前节点的值，则继续在左子树中搜索
            } else {
                current = node.right.as_ref(); // 如果目标值大于当前节点的值，则继续在右子树中搜索
            }
        }

        false // 如果未找到目标值，则返回false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


