use crate::parser::instruction::Instruction;
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

        for ch in data.chars()
        {
        }

        return result;
    }
//}