use crate::parser::instruction::Instruction;
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;

pub fn load(filename:&str) -> Rc<RefCell<Instruction>>
{
    let mut file = std::fs::File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    return parse(&data); 
}

pub fn parse(data:&str) -> Rc<RefCell<Instruction>>
{
    let root = Instruction::new();
    let result: Rc<RefCell<Instruction>> = Rc::new(RefCell::new(root));
    let mut stack:VecDeque<Rc<RefCell<Instruction>>> = VecDeque::new();

    stack.push_back(Rc::clone(&result));

    let mut p:char = ' ';
    let mut current:String = String::from("");
    let mut quote_state = 0;
    let mut line_state = 0;

    for ch in data.chars()
    {
        if ch == 13 as char { }
        else if (ch == ' ') && ((p == ' ')||(p == 10 as char)) && (quote_state == 0) { }
        else if ch == 10 as char
        {
            if line_state == 0 { line_state += 1; }
        }
        else if ch == '"'
        {
            if quote_state == 0 { quote_state += 1; }
            else if quote_state == 1
            {
                if p != '\\' { quote_state = 0; }
            }
            current.push(ch);
        }        
        else if ch == '{'
        {           
            if line_state == 1 && current.len() > 0
            {
                let mut node = Instruction::new();
                node.value = Some(current.clone());
                current.clear();

                let child = Rc::new(RefCell::new(node));

                let n = stack.borrow_mut().len() - 1;
                stack.borrow_mut()[n].as_ref().borrow_mut().children.push(child);

                let m = stack.borrow_mut()[n].as_ref().borrow_mut().children.len() - 1;
                let z = Rc::clone(&stack.borrow_mut()[n].as_ref().borrow_mut().children[m]);

                stack.push_back(z);

                line_state = 0;
            }
        }
        else if ch == '}'
        {
            if current.len() > 0
            {
                let mut node = Instruction::new();
                node.value = Some(current.clone());
                current.clear();

                let child = Rc::new(RefCell::new(node));

                let n = stack.borrow_mut().len() - 1;
                stack.borrow_mut()[n].as_ref().borrow_mut().children.push(child);
            }

            stack.pop_back();
        }
        else if line_state == 1
        {
            if current.len() > 0
            {
                let mut node = Instruction::new();
                node.value = Some(current.clone());
                current.clear();

                let child = Rc::new(RefCell::new(node));

                let n = stack.borrow_mut().len() - 1;
                stack.borrow_mut()[n].as_ref().borrow_mut().children.push(child);
            }

            current.push(ch);
            line_state = 0;
        }
        else { current.push(ch); }

        p = ch.clone();
    }
    
    if current.len() > 0
    {
        let mut node = Instruction::new();
        node.value = Some(current.clone());
        current.clear();

        let child = Rc::new(RefCell::new(node));

        let n = stack.borrow_mut().len() - 1;
        stack.borrow_mut()[n].as_ref().borrow_mut().children.push(child);
    }

    return stack[0].clone();
}


