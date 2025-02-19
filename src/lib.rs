#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

pub const MODULO: usize = 1_000_000_007;

pub fn mod_pow(base: usize, exponent: usize, modulo: usize) -> usize {
    if modulo == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulo;
    let mut exp = exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        exp /= 2;
        base = (base * base) % modulo;
    }

    result
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
