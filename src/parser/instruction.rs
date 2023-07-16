pub struct Instruction
{
    pub value: String
}

impl Instruction
{
    pub fn new() -> Instruction {
        Instruction 
        {
            value: "".to_string()
        }
    }
}