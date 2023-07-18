use crate::parser::instruction::Instruction;
use crate::parser::TreeNode::TreeNode;
use std::borrow::{BorrowMut, Borrow};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

//mod instruction;

//pub mod parser
//{
    //use std::fs;
    use std::io::Read;

    pub fn load(filename:&str) -> Rc<RefCell<TreeNode>>//Instruction>>//Result<Instruction, Box<dyn std::error::Error>
    {
        let mut file = std::fs::File::open(filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        println!("{}",data);
        
        //let result = Instruction::new();
        return parse(&data);
        //return result;
        //let data = fs::read_to_string(filename);
        //println!("{:?}",data);
        //return parse(data.);
    }

    pub fn parse(data:&str) -> Rc<RefCell<TreeNode>>//Instruction>>
    {
        let result: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new()));
        let mut cur = Rc::clone(&result);
        let mut stack:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();//vec![];
        stack.push_back(Rc::clone(&result));
        //let mut result = Instruction::new();
        //let mut stack:VecDeque<&mut Instruction> = VecDeque::new();
        //stack.push_back(&mut result);
        
        //let result = Rc::new(RefCell::new(Instruction::new()));
        //let mut stack:VecDeque<Rc<RefCell<Instruction>>> = VecDeque::new();
        //stack.push_back(Rc::clone(&result));

        //std::stack<diff::blocks::instruction*> stack;    
        //stack.push(&result);

        //let mut result2 = Instruction::new();
        //let mut stack2:VecDeque<&mut RefCell<Instruction>> = VecDeque::new();
        //stack2.push_back(&mut RefCell::new(result2));

        //char p;
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
                    let child = Rc::new(RefCell::new(TreeNode::new()));
                    //cur.borrow_mut().children.push_back(Rc::clone(&child));
                    //let cc = Rc::clone(stack.back().borrow_mut());
                    //let m = stack.back().borrow_mut().;
                    //stack.top()->children.push_back(child);
                    //stack.push(&stack.top()->children.back());

                    //let mut child = Rc::new(RefCell::new(Instruction::new()));
                    //let mut mut_child = child.borrow_mut();
                    
                    //let mut mut_child = child.borrow_mut();
                    //mut_child.as_ref().value = current.clone();
                    //let mm = stack.back_mut().unwrap().borrow_mut().;
                    //let mut c = Rc::clone(&stack.back_mut());
                    //c.borrow_mut().
                    //let mut top  = stack.back_mut();
                    //let dd= top.borrow_mut();
                    
                    //stack.push_back(Rc::clone(&child));

                    //stack.top()->children.push_back(child);
                    //stack.push(&stack.top()->children.back());

                    /*let mut child:Instruction = Instruction::new();            
                    child.value = current.clone();
                    current.clear();
                    */
                    
                    //let w = stack.back_mut().unwrap().;
                    //parent.children.push_back(Rc::new(RefCell::new(child)));
                    //parent.push(Rc::new(RefCell::new(child)));
                    //stack.push_back(&mut child);
                    //stack.back_mut().unwrap().push(Rc::new(RefCell::new(child)));
                    //let rr = stack.back_mut().unwrap().children[0].borrow_mut();//.children.back_mut();//.borrow_mut().unwrap();
                    //stack.push_back(rr);
                    //let &ww = stack.back_mut().unwrap().children.back_mut().unwrap().borrow();

                    //let z = stack.back_mut().as_deref().unwrap();
                    //stack.push_back(z.children.back_mut());//.unwrap().borrow());
                    //let z = stack.back_mut().unwrap().children.back_mut().unwrap().as_ref().get_mut();
                    //let q = stack.back_mut().unwrap().children.borrow_mut().back_mut().unwrap().as_ref().borrow_mut();

                    //.borrow_mut().back_mut().as_ref();//.unwrap().as_ref();
                    
                    //let m = stack.back_mut().unwrap().children.back_mut().unwrap();
                    //let z = m.borrow();
                    //stack.push_back(&mut z);

                    //stack.top()->children.push_back(child);
                    //stack.push(&stack.top()->children.back());

                    // ***

                    //let rf = RefCell::new(Instruction { value: current, children: VecDeque::new() });
                    //let child = Rc::new(rf);
                    //current.clear();

                   // stack2.back_mut().unwrap().push(child);


                    //let child = Rc::new(RefCell::new(Instruction::new()));
                    //child.borrow_mut().value = current.clone();
                    //current.clear();

                    //stack.back_mut().unwrap().push(child);
                    //stack.push_back(stack.back_mut().unwrap().back().borrow_mut());

                    // ***
                    //let mut moo = Instruction::new();
                    //moo.value = current.clone();
                    //let cmoo = Rc::new(RefCell::new(moo));
                    
                    
                    //stack2.borrow_mut().back_mut().unwrap(
                   // let m = stack2.back().borrow_mut().unwrap();
                   // stack2.push_back(cmoo);//back_mut().unwrap().push(moo);
                    // ***

                    //let b = stack.back_mut().unwrap().children.back().unwrap();
                    //stack.push_back(&mut b.borrow_mut());

                    //stack.back_mut().unwrap().push(&mut child.borrow());//.children.back().unwrap());
                    //stack.push_back(&mut stack.back().unwrap().back().borrow_mut());//child.borrow_mut());//.borrow().children.back_mut().unwrap());                    
                    //stack.push_back(stack.back_mut().unwrap().back());
                    //if let Some(oo) = stack.back_mut() 
                    //{
                        //oo.children.push_back(child);
                    //}
                    //stack.back_mut().unwrap().children;//.push_back(child);
                    //stack.top()->children.push_back(child);
                    //stack.push(&stack.top()->children.back());
                    
                    //***stack.push_back(&stack.back());
    
                    line_state = 0;
                }
            }
            else if ch == '}'
            {
                if current.len() > 0
                {
                    //let child = Rc::new(RefCell::new(Instruction::new()));
                    //child.borrow_mut().value = current.clone();
                    //stack.back_mut().unwrap().push(child);
                    current.clear();
                }
    
                //stack.pop_back();
            }
            else if line_state == 1
            {
                if current.len() > 0
                {
                    //let child = Rc::new(RefCell::new(Instruction::new()));                    
                    //child.borrow_mut().value = current.clone();
                    //stack.back_mut().unwrap().push(child);
                    current.clear();
                }
    
                current.push(ch);
                line_state = 0;
            }
            else { current.push(ch); }
    
            p = ch.clone();
        }

        return result;
        //return stack.back_mut().unwrap().clone();
    }
//}


/*
    diff::blocks::instruction result;
    std::stack<diff::blocks::instruction*> stack;
    
    stack.push(&result);

    char p;
    std::string current;
    int quote_state = 0, line_state = 0;

    for(std::string::iterator it = data.begin(); it != data.end(); ++it)
    {        
        if(*it == 13) { }
        else if((*it == ' ') && ((p == ' ')||(p == 10)) && (quote_state == 0)) { }
        else if(*it == 10)
        {
            if(line_state == 0) ++line_state;
        }
        else if(*it == '"')
        {
            if(quote_state == 0) ++quote_state;
            else if (quote_state == 1)
            {
                if(p != '\\') quote_state = 0;
            }
            current.push_back(*it);
        }        
        else if (*it == '{')
        {           
            if((line_state == 1)&&(current.size() > 0))
            {
                diff::blocks::instruction child;            
                child.instr = current;                
                current.clear();

                stack.top()->children.push_back(child);
                stack.push(&stack.top()->children.back());

                line_state = 0;
            }
        }
        else if(*it == '}')
        {
            if(current.size() > 0)
            {
                diff::blocks::instruction child;
                child.instr = current;
                stack.top()->children.push_back(child);
                current.clear();
            }

            stack.pop();
        }
        else if(line_state == 1)
        {
            if(current.size() > 0)
            {
                diff::blocks::instruction child;
                child.instr = current;
                stack.top()->children.push_back(child);
                current.clear();
            }

            current.push_back(*it);
            line_state = 0;
        }
        else current.push_back(*it);

        p = *it;
    }
    
    return result;
    */