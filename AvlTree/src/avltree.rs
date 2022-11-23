use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::cmp::max;



type TreeNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<TreeNode<T>>;

#[derive(Clone, PartialEq)]
pub struct Node<T: Ord+Display+Debug>{
    key: T,
    left : Tree<T>,
    right : Tree<T>,
    height : i8,
}


impl <T> Node<T>
where T: Debug+Ord+Display+Copy{
    pub fn new(key :T) -> Tree<T>{
        Some(Rc::new(RefCell::new(Node{key:key,left:None,right:None,height:1})))
    }

    pub fn height(&self) -> i8 { 
        self.height
    }
}

impl<T> fmt::Debug for Node<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
         .field("key", &self.key)
         .field("right", &self.right)
         .field("left", &self.left)
         .finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AvlTree<T:Ord + Display + Debug + Copy>{
    root : Tree<T>,
    count : usize,
}
trait _Tree<T>
where T: Ord+Display+Debug+Clone+Copy{
    fn new(key:T) -> Tree<T>;
    fn height(&self,tree_node:Tree<T>) -> i8;
    fn rotate_lr(&self,tree_node:Tree<T>) -> Tree<T>;
    fn rotate_rl(&self,tree_node:Tree<T>) -> Tree<T>;
    fn update_height(&self,tree_node:Tree<T>) -> Tree<T>;
    fn do_insert(&self, root:Tree<T>,val: T) -> Tree<T>;
    fn do_delete(&self,root:Tree<T>,val:T) -> Tree<T>;
    fn balance_factor(&self,tree_node:Tree<T>) -> i8;
    fn balance_tree(&self, tree_node:Tree<T>) -> Tree<T>;
    fn rotate_left(&self,tree_node:Tree<T>) -> Tree<T>;
    fn rotate_right(&self,tree_node:Tree<T>) ->Tree<T>;
    fn find_min(&self,tree_node:Tree<T>) -> Tree<T>;
}

impl <T> _Tree<T> for Tree<T>
where T: Ord+Display+Debug+Clone+Copy{
    fn new(key:T) -> Tree<T> {
        Node::new(key)
    }
    fn height(&self,tree_node:Tree<T>) -> i8{
        match tree_node{
            None => 0,
            Some(node) => node.clone().borrow().height,
        }
    }
    fn update_height(&self, tree_node:Tree<T>) ->Tree<T>{
        match tree_node {
            None => tree_node,
            Some(node) => {
                let left_height = self.height(node.borrow().clone().left);
                let right_height = self.height(node.borrow().clone().right);
                node.clone().borrow_mut().height = max(left_height,right_height) + 1;
                Some(node)
            }
        }
        
    }

    fn balance_factor(&self,tree_node:Tree<T>) -> i8 {
        match tree_node{
            Some(node) =>{
                let left_height = self.height(node.borrow().clone().left.clone());
                let right_height = self.height(node.borrow().clone().right.clone());
                if left_height >= right_height {
                    (left_height - right_height) as i8
                } else {
                    -((right_height - left_height) as i8)
                }
            }
            None => 0
        }
    }

    fn rotate_right(&self,tree_node:Tree<T>) -> Tree<T> {
        let final_tree: Tree<T>;
        match tree_node{
            None => unreachable!(),
            Some(node) =>{
                let left_node = node.borrow().left.clone();
                let left = left_node.clone();
                match left{
                    Some(left_tree) =>{
                        let left_right_tree = left_tree.borrow().right.clone();
                        final_tree = left_node.clone();
                        let new_right_tree = node.clone();
                        new_right_tree.borrow_mut().left = left_right_tree.clone();
                        let new_right_tree_updated = self.update_height(Some(new_right_tree.clone()));
                        final_tree.as_ref().unwrap().borrow_mut().right = new_right_tree_updated.clone();
                        let final_tree_updated = self.update_height(final_tree.clone());
                        final_tree_updated.clone()
                    }
                    None => unreachable!(),
                }
                
                
                
        }
    }
}

    fn rotate_left(&self,tree_node:Tree<T>) ->Tree<T> {
        let final_tree:Tree<T>; 
        match tree_node{
            None => unreachable!(),
            Some(node) =>{
                let right_node = node.borrow().right.clone();
                let right = right_node.clone();
                match right{
                    Some(right_tree) =>{
                        let right_left_tree = right_tree.borrow().left.clone();
                        final_tree = right_node.clone();
                        let new_left_tree = node.clone();
                        new_left_tree.borrow_mut().right = right_left_tree.clone();
                        let new_right_tree_updated = self.update_height(Some(new_left_tree.clone()));
                        final_tree.as_ref().unwrap().borrow_mut().left = new_right_tree_updated.clone();
                        let final_tree_updated = self.update_height(final_tree.clone());
                        final_tree_updated.clone()
                    }
                    None => unreachable!()
                }
            }
        }
    }
    fn rotate_lr(&self, tree_node:Tree<T>) -> Tree<T> {
        let rotated_tree = tree_node.clone();
        match tree_node {
            Some(root) => {
                let rotated_left_tree = self.rotate_left(root.borrow().left.clone());
                rotated_tree.as_ref().unwrap().borrow_mut().left = rotated_left_tree.clone();
                let lr_tree = self.rotate_right(rotated_tree.clone());
                lr_tree
            }
            None => unreachable!(),
        }
    }

    fn rotate_rl(&self,tree_node:Tree<T>) -> Tree<T>{
        let rotated_tree = tree_node.clone();
        match tree_node {
            Some(root) => {
                let rotated_right_tree = self.rotate_right(root.borrow().right.clone());
                rotated_tree.as_ref().unwrap().borrow_mut().right = rotated_right_tree.clone();
                let rl_tree = self.rotate_left(rotated_tree.clone());
                rl_tree
            }
            None => unreachable!(),
        }
    }

    fn do_insert(&self,tree:Tree<T>,key: T) -> Tree<T> {
        match tree {
            None => {
                let add_node = Self::new(key);
                add_node.clone()
            }
            Some(root) => {
                let clone_node = root.borrow().clone();
                let balanced_tree :Tree<T>;
                let updated_tree:Tree<T>;
                let sub_node:Tree<T>;
                if key == clone_node.key {
                    Some(root.clone())
                } 
                else if key < clone_node.key {
                    sub_node = root.borrow().left.clone();
                    let result = self.do_insert(sub_node,key);
                    let result_node = result;
                    root.borrow_mut().left = result_node;
                    let updated_tree = self.update_height(Some(root.clone()));
                    let balanced_tree = self.balance_tree(updated_tree);
                    balanced_tree.clone()
                }
                //进入右子树递归插入
                else {
                    sub_node = root.borrow().right.clone();
                    let result = self.do_insert(sub_node,key);
                    let result_node = result;
                    root.borrow_mut().right = result_node;
                    updated_tree = self.update_height(Some(root));
                    balanced_tree = self.balance_tree(updated_tree);
                    balanced_tree.clone()
                }
            }
        }
    }

    fn do_delete(&self,tree:Tree<T>,key:T) -> Tree<T>{
        let deleted_tree = tree.clone();
        let updated_tree:Tree<T>;
        let balanced_tree:Tree<T>;
        if tree.is_none(){
            return tree.clone();
        }
        else{
            let sub_node_left = tree.as_ref().unwrap().borrow().left.clone();
            let sub_node_right = tree.as_ref().unwrap().borrow().right.clone();
            if key < tree.as_ref().unwrap().borrow().key{
                deleted_tree.as_ref().unwrap().borrow_mut().left = self.do_delete(sub_node_left, key);
                updated_tree = self.update_height(deleted_tree.clone());
                balanced_tree = self.balance_tree(updated_tree);
                return balanced_tree.clone();
            }
            else if key > tree.as_ref().unwrap().borrow().key{
                deleted_tree.as_ref().unwrap().borrow_mut().right  = self.do_delete(sub_node_right, key);
                updated_tree = self.update_height(deleted_tree.clone());
                balanced_tree = self.balance_tree(updated_tree);
                return balanced_tree.clone();
            }
            else{
                if tree.as_ref().unwrap().borrow().left.is_none(){
                    let temp = tree.as_ref().unwrap().borrow().right.clone();
                    return temp.clone();
                }
                else if tree.as_ref().unwrap().borrow().right.is_none(){
                    let temp = tree.as_ref().unwrap().borrow().left.clone();
                    return temp.clone();
                }
                let temp = self.find_min(tree.as_ref().unwrap().borrow().right.clone());
                deleted_tree.as_ref().unwrap().borrow_mut().key  = temp.as_ref().unwrap().borrow().key;
                if deleted_tree.is_none(){
                    return deleted_tree.clone();
                }
                else{
                    deleted_tree.as_ref().unwrap().borrow_mut().right = self.do_delete(sub_node_right,temp.unwrap().borrow().key);
                    updated_tree = self.update_height(deleted_tree.clone());
                    balanced_tree = self.balance_tree(updated_tree);
                    return balanced_tree.clone();
                }
            }
        }
    }

    fn find_min(&self, tree: Tree<T>) -> Tree<T> {
        match tree {
            Some(sub_tree) => {
                let mut left = Some(sub_tree.clone());
                while left.as_ref().unwrap().borrow().left.clone().is_some() {
                    left = left.unwrap().borrow().left.clone();
                }
                left
            },
            None => {
                tree
            }
        }
    }

    fn balance_tree(&self, tree_node:Tree<T>) -> Tree<T>{
        let balance_factor = self.balance_factor(tree_node.clone());
        let balanced_tree :Tree<T>;
        if balance_factor > 1{
            let balance_factor_left = self.balance_factor(tree_node.as_ref().unwrap().borrow().left.clone());
            if balance_factor_left >= 0{
                balanced_tree = self.rotate_right(tree_node.clone());
                return balanced_tree.clone();
            }
            else{
                return self.rotate_lr(tree_node.clone());
            }
        } 
        
        if balance_factor < -1{
            let balance_factor_right = self.balance_factor(tree_node.as_ref().unwrap().borrow().right.clone());
            if balance_factor_right <= 0{
                return self.rotate_left(tree_node.clone());
            }
            else{
                return self.rotate_rl(tree_node.clone());
            }
        }
        tree_node
    }
}

impl <T> AvlTree<T>
where T: Ord+Display+Debug+Clone+Copy{
    pub fn new() -> Self{
        AvlTree { root: None ,count: 0}
    }

    pub fn is_empty(&self) -> bool{
        if self.root.is_none(){
            return true;
        }
        else {
            return false;
        }
    }

    pub fn leaves(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }
        let root = self.root.as_ref().unwrap().clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        stack.push(Some(root));

        let mut count = 0;
        while !stack.is_empty() {
            let node = stack.pop();
            let mut node_left = None;
            let mut node_right = None;

            if node.is_some() {
                node_left = node.as_ref().unwrap().as_ref().unwrap().borrow().clone().left.clone();
                node_right = node.as_ref().unwrap().as_ref().unwrap().borrow().clone().right.clone();
            }

            if node_left.is_some() {
                stack.push(node_left.clone());
            }

            if node_right.is_some() {
                stack.push(node_right.clone());
            }

            if node_left.is_none() && node_right.is_none() {
                count += 1;
            }
        }
        count
    }

    pub fn print_inorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        }
        let mut root = self.root.clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        while !stack.is_empty() || !root.is_none() {
            if root.is_some() {
                stack.push(root.clone());
                let p = root.as_ref().unwrap().borrow().left.clone();
                root = p.clone();
            } else {
                let pop = stack.pop().unwrap();
                println!("{:?}", pop.as_ref().unwrap().borrow().key.clone());
                root = pop.as_ref().unwrap().borrow().right.clone();
            }
        }
    }

    pub fn print_preorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        }
        let mut root = self.root.clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        stack.push(root);
        let mut cur: Tree<T>;
        while !stack.is_empty() {
            cur = stack.pop().unwrap();
            root = cur.clone();
            println!("{:?}", root.as_ref().unwrap().borrow().key.clone());
            let root_right = root.as_ref().unwrap().borrow().right.clone();
            let root_left = root.as_ref().unwrap().borrow().left.clone();
            if root_right.is_some() {
                stack.push(root_right.clone());
            }
            if root_left.is_some() {
                stack.push(root_left.clone());
            }
        }
    }

    pub fn insert(&mut self,key:T){
        let root_node = self.root.clone();
        let res_tree = self.root.do_insert(root_node,key);
        self.root = res_tree;
        self.count += 1;
    }

    
    pub fn delete(&mut self,key:T){
        let root_node = self.root.clone();
        let res_tree = self.root.do_delete(root_node.clone(),key);
        self.root = res_tree;
        self.count -= 1;
    }

    pub fn search(&self, key: T) -> Tree<T> {
        let dummy = Node::<T>::new(key).unwrap().borrow().clone();
        self.search_node(&self.root, &dummy)
    }

    fn search_node(&self, tree_node: &Tree<T>, node: &Node<T>) -> Tree<T> {
        match tree_node {
            Some(sub_tree) => {
                let sub_tree_clone = sub_tree.borrow().clone();
                if sub_tree_clone.key == node.key {
                    Some(sub_tree.clone())
                } else {
                    if sub_tree_clone.key > node.key {
                        self.search_node(&sub_tree_clone.left, node)
                    } else {
                        self.search_node(&sub_tree_clone.right, node)
                    }
                }
            },
            None => {None}
        }
    }
}

impl<T> fmt::Display for AvlTree<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AvlTree")
         .field("root", &self.root)
         .finish()
    }
}