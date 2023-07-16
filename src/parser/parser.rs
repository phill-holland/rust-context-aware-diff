use crate::parser::instruction::Instruction;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
//mod instruction;

//pub mod parser
//{
    //use std::fs;
    use std::io::Read;

    pub fn load(filename:&str) -> Instruction//Result<Instruction, Box<dyn std::error::Error>
    {
        let mut file = std::fs::File::open(filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        println!("{}",data);
        return parse(&data);
        //let data = fs::read_to_string(filename);
        //println!("{:?}",data);
        //return parse(data.);
    }

    pub fn parse(data:&str) -> Instruction
    {
        let mut result = Instruction::new();

        let mut stack:VecDeque<&mut Instruction> = VecDeque::new();
        stack.push_back(&mut result);
        //std::stack<diff::blocks::instruction*> stack;    
        //stack.push(&result);

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
                    //let mut child:Instruction = Instruction::new();            
                    //child.value = current.clone();
                    //current.clear();
    
                    let child = Rc::new(RefCell::new(Instruction::new()));
                    child.borrow_mut().value = current.clone();
                    current.clear();

                    stack.back_mut().unwrap().push(child);
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
                    let mut child:Instruction = Instruction::new();
                    child.value = current.clone();
                    //stack.top()->children.push_back(child);
                    current.clear();
                }
    
                stack.pop_back();
            }
            else if line_state == 1
            {
                if current.len() > 0
                {
                    let mut child:Instruction = Instruction::new();                    
                    child.value = current.clone();
                    //stack.top()->children.push_back(child);
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