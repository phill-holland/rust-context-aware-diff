mod parser;

use std::env;

fn main() 
{   
    let args:Vec<String> = env::args().collect();

    let filename1 = &args[1];
    let filename2 = &args[2];

    let program1 = parser::parse::load(filename1);
    let program2 = parser::parse::load(filename2);
    
    let result = program1.borrow_mut().compare(&program2);
    let output = result.borrow_mut().print();

    println!("{}",output);   
}
