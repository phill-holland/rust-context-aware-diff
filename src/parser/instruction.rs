use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Instruction
{
    pub value: String,
    pub children: VecDeque<Rc<RefCell<Instruction>>>
}

//let mut stack:VecDeque<&Instruction> = VecDeque::new();

impl Instruction
{
    pub fn new() -> Instruction {
        Instruction 
        {
            value: "".to_string(),
            children: VecDeque::new()
        }
    }

    pub fn push(&mut self, child: Rc<RefCell<Instruction>>) 
    {
        self.children.push_back(child);
    }

    
    pub fn back(&mut self) -> &Rc<RefCell<Instruction>>
    {
        return self.children.back().unwrap();
    }
    
    /*
    pub fn push(&self, child: Instruction)
    {
        if let Some(moo) = &mut self.children
        {
            let ww = moo.unwrap();
            ww.push_back(child);
        }
        //self.children.push_back(child);
    }
    */
}