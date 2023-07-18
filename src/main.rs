use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;

use std::cell::RefCell;
use std::rc::Rc;
//use std::collections::VecDeque;

mod parser;
mod tree;

fn main() 
{
    let mut bob = parser::TreeNode::load("test.txt");

    //let v = Some(bob.borrow_mut().as_ref().borrow_mut().children[0].borrow_mut().as_ref().borrow_mut().value);
    //let tree = tree::tree::init_tree(String::from("[1,2]"));
    //let bob = parser::parser::load("test.txt");
/*
    let mut owners:VecDeque<Rc<RefCell<tree::tree::TreeNode>>> = VecDeque::new();
    let mut stack:VecDeque<Rc<RefCell<tree::tree::TreeNode>>> = VecDeque::new();

    let root = tree::tree::TreeNode::new();
    let result: Rc<RefCell<tree::tree::TreeNode>> = Rc::new(RefCell::new(root));

    owners.push_back(result);
    //stack.push_back(Rc::clone(&owners[0]));
    stack.push_back(Rc::clone(&owners[owners.len()-1]));

    //stack[stack.len() - 1].borrow_mut().as_ref().children.push(result);

    //stack.top()->children.push_back(child);
    //stack.push(&stack.top()->children.back());

    //let w = owners[0].borrow_mut();//.as_ref();//.borrow_mut();
    //stack.push_back(w);
    //let m = Some(owners.back().as_ref()).borrow_mut();
    //let r = Some(owners.back_mut().borrow_mut()).;
    //stack.push_back(&result.borrow_mut());//&root.borrow_mut());
*/
    println!("{}","hello");
}
