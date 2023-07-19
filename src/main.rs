use std::borrow::{BorrowMut};

mod parser;

fn main() 
{   
    let mut tree = parser::parse::load("test.txt");
    let output = tree.borrow_mut().as_ref().borrow().print();

    println!("{}",output);
}
