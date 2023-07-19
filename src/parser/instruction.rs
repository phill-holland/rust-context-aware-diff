use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

  pub fn print(&self) -> String 
  {
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

  pub fn compare(&self,source:&Rc<RefCell<Instruction>>) -> Rc<RefCell<Instruction>>
  {
    let root = Instruction::new();
    let result: Rc<RefCell<Instruction>> = Rc::new(RefCell::new(root));

    let mut idx = 0;
    let mut map: HashMap<String,i32> = HashMap::new();

    self.children
    .iter()
    .for_each(|tn| {
        if let Some(v) = tn.borrow().value.clone() {
            if !map.contains_key(&v) {
                map.insert(v,idx);
            }
            idx += 1;
        }
    });    

    idx = 0;
    source.borrow().children
    .iter()
    .for_each(|tn| {
        if let Some(v) = tn.borrow().value.clone() {
            if map.contains_key(&v) {
                if let Some(x) = map.get(&v) {
                    if x == &idx {
                        let child = self.children[idx as usize].borrow().compare(tn);
                        child.borrow_mut().value = Some(v);
                        result.borrow_mut().children.push(child);
                    }
                    else {
                        let mut child = Instruction::new();
                        child.value = Some(v.clone() + &String::from("\n"));
                        result.borrow_mut().children.push(Rc::new(RefCell::new(child)));                                            
                        map.remove(&v);
                    }
                }
            }            
            else {
                result.borrow_mut().children.push(tn.borrow_mut().prefix(&String::from("+ ")));
            }
            idx += 1;
        }

        for (_key, value) in &map {
            let tt = self.children[*value as usize].borrow_mut().prefix(&String::from("- "));
            result.borrow_mut().children.insert(*value as usize, tt);
        }
    });    

    return result;
  }

  pub fn prefix(&self,value:&String) -> Rc<RefCell<Instruction>>
  {
    let root = Instruction::new();
    let result: Rc<RefCell<Instruction>> = Rc::new(RefCell::new(root));

    if let Some(x) = self.value.clone() {
        result.borrow_mut().value = Some(String::from("") + value + &x);
    }

    self.children
    .iter()
    .for_each(|tn| {
        result.borrow_mut().children.push(tn.borrow().prefix(value));
    });    

    return result;
  }
}