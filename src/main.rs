use std::{collections::VecDeque, fmt::Display};

/// Prints a binary tree by level using a <tt>VecDeque</tt>.
/// For example, given the tree below:
///
/// <pre>
///       1
///     /   \
///    2     3
///   / \   / \
///  4   5 6   7
/// </pre>
///
/// The output should be:
///
/// <pre>
/// 1
/// 2 3
/// 4 5 6 7
/// </pre>
fn main() {
    TreeNode {
        value: 1,
        left: BinaryTree::NonEmpty(Box::new(TreeNode {
            value: 2,
            left: BinaryTree::NonEmpty(Box::new(TreeNode {
                value: 4,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
            right: BinaryTree::NonEmpty(Box::new(TreeNode {
                value: 5,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
        })),
        right: BinaryTree::NonEmpty(Box::new(TreeNode {
            value: 3,
            left: BinaryTree::NonEmpty(Box::new(TreeNode {
                value: 6,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
            right: BinaryTree::NonEmpty(Box::new(TreeNode {
                value: 7,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
        })),
    }
    .print_levels();
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Display> TreeNode<T> {
    fn print_levels(self) {
        let mut nodes = VecDeque::<TreeNode<T>>::new();
        nodes.push_back(self);
        while !nodes.is_empty() {
            let mut children = VecDeque::<TreeNode<T>>::new();
            while let Some(node) = nodes.pop_front() {
                print!("{} ", node.value);
                if let BinaryTree::NonEmpty(child) = node.left {
                    children.push_back(*child);
                }
                if let BinaryTree::NonEmpty(child) = node.right {
                    children.push_back(*child);
                }
            }
            println!();
            nodes.append(&mut children);
        }
    }
}
