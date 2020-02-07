pub mod game {
    # [derive(Debug)]
    pub enum Instruction {
        Look,
    }

    pub fn parse_instruction(instruction: &str) -> Result<Instruction, &str> {
        let instruction = instruction.trim();
        let tokens: Vec<&str> = instruction.split(" ").collect();

        if tokens.len() > 0 && tokens[0] == "look" {
            if tokens.len() > 1 {
                return Err("I can't find that")
            }

            return Ok(Instruction::Look)
        }

        Err("I don't understand")
    }
}
