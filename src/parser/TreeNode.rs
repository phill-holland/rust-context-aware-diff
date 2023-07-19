//use std::borrow::{BorrowMut, Borrow };
//use std::borrow::{BorrowMut};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use std::io::Read;

#[derive(PartialEq)]
pub struct TreeNode {
  pub value: Option<String>,
  pub children: Vec<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
  pub fn new() -> TreeNode {
    return TreeNode {
      value: None,
      children: vec![]
    };
  }

/*
  pub fn print(&self) -> String {
    let mut result: String = String::new();

//result = self.children[0].as_ref().borrow_mut().value.unwrap();
    //for (_,i:Rc<RefCell<TreeNode>>) in self.children Vec<Rc<RefCell<TreeNode>>>//.borrow_mut()
    //for i in self.children.borrow_mut().as_
    //{
      //result = result + i.
    //}

    return result;
  }
  */

  pub fn print(&self) -> String {
    if let Some(value) = self.value.clone() {
      return value;
    }
    //} else {
      return String::from("[")
        + &self
          .children
          .iter()
          .map(|tn| tn.borrow().print())
          //.map(|tn| <Rc<RefCell<TreeNode>> as Borrow<std::borrow::Cow>>::borrow(tn).print())
          .collect::<Vec<String>>()
          .join(",")
        + "]";
    //}
  }
}
/*
  pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
    self.children.push(new_node);
  }
*/
  pub fn load(filename:&str) -> Rc<RefCell<TreeNode>>//Instruction>>//Result<Instruction, Box<dyn std::error::Error>
  {
      let mut file = std::fs::File::open(filename).unwrap();
      let mut data = String::new();
      file.read_to_string(&mut data).unwrap();
      println!("{}",data);
      
      return parse(&data); 
  }

  pub fn parse(data:&str) -> Rc<RefCell<TreeNode>>//Instruction>>
  {
      let root = TreeNode::new();
      let result: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(root));
      let mut stack:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    
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
                  let mut node = TreeNode::new();//{ value: Some(current.clone()), children: vec![] });//= TreeNode::new();
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
                  let mut node = TreeNode::new();
                  node.value = Some(current.clone());
                  current.clear();

                  let child = Rc::new(RefCell::new(node));

                  stack.push_back(Rc::clone(&child));

              }
  
              stack.pop_back();
          }
          else if line_state == 1
          {
              if current.len() > 0
              {
                let mut node = TreeNode::new();
                node.value = Some(current.clone());
                current.clear();

                let child = Rc::new(RefCell::new(node));

                stack.push_back(Rc::clone(&child));
                //stack.push_back(child);
              }
  
              current.push(ch);
              line_state = 0;
          }
          else { current.push(ch); }
  
          p = ch.clone();
      }

      return result;
  }
  

