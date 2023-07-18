use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq)]
pub struct Instruction
{
    pub value: Option<String>,
    pub children: VecDeque<Rc<RefCell<Instruction>>>
}

impl Instruction
{
    pub fn new() -> Instruction {
        Instruction 
        {
            value: None,
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