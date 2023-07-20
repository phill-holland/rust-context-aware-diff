//use std::borrow::{BorrowMut};

mod parser;

fn main() 
{   
    let program1 = parser::parse::load("test.txt");
    //println!("{}",program1.borrow_mut().print());

    let program2 = parser::parse::load("test2.txt");
    //println!("{}",program2.borrow_mut().print());

    let result = program1.borrow_mut().compare(&program2);
    let output = result.borrow_mut().print();//.as_ref().borrow().print();

    println!("{}",output);   
}
