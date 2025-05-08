//problem B

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub type BtreeNodeLink = Option<Rc<RefCell<BTreeNode>>>;

#[derive(Debug)]  
pub struct BTreeNode {
    pub value: i32,
    pub children: HashMap<i32, BtreeNodeLink>,
}

impl BTreeNode {
    pub fn new(value: i32) -> Self {  
        BTreeNode {
            value,
            children: HashMap::new(),  
        }
    }

    // Fungsi untuk menambahkan nilai-nilai vektor ke tree
    pub fn insert(&mut self, digits: &[i32]) {
        let mut current = Rc::new(RefCell::new(self));  

        for &digit in digits {  // Mengganti 'for &digits' dengan 'for &digit'
            let mut node = current.borrow_mut();
            current = node.children
                .entry(digit)
                .or_insert_with(|| Some(Rc::new(RefCell::new(BTreeNode::new(digit)))))
                .as_ref()
                .unwrap()
                .clone();
        }
    }
}

// Fungsi pencarian sesuai request
pub fn lookup(node: BtreeNodeLink, keys: Vec<i32>) -> bool {
    let mut current = node;
    for key in keys {
        match current {
            Some(node_ref) => {
                let node = node_ref.borrow();
                match node.children.get(&key) {
                    Some(child) => {
                        current = child.clone();  
                    }
                    None => return false, 
                }
            }
            None => return false, 
        }
    }
    true 
}
