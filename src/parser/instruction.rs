use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Instruction {
  pub value: Option<String>,
  pub children: Vec<Rc<RefCell<Instruction>>>
}

impl Instruction {
  pub fn new() -> Instruction {
    return Instruction {
      value: None,
      children: vec![]
    };
  }

  pub fn print(&self) -> String {
    let mut ss:String = String::from("");

    if let Some(value) = self.value.clone() {
        ss = value;
    }    

    return String::from("[") + &ss
    + &self
        .children
        .iter()
        .map(|tn| tn.borrow().print())
        .collect::<Vec<String>>()
        .join(",")
    + "]";
  }
}