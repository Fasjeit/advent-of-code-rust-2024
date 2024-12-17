advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let machine_data = data[0];
    let commands_data = data[1];

    let mut machine = parse_machine_data(machine_data, commands_data);

    let mut results: Vec<u64> = Vec::new();
    loop {
        match machine.Execute() {
            ExecuteResult::None => (),
            ExecuteResult::Halt => break,
            ExecuteResult::Value(v) => results.push(v),
        }
    }

    let result = results
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    machine.print();
    println!("{}", &result);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // A involves only modulo 8.
    // At each step A = A / 8
    // So A can be brute forced byte by byte from the end.

    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let machine_data = data[0];
    let commands_data = data[1];

    let mut machine = parse_machine_data(machine_data, commands_data);

    let mut reg_a_candidate: Vec<u8> = vec![0; machine.memory.len()];

    for pos in 0..reg_a_candidate.len() - 1 {
        for position_candidate in 0..u8::MAX {
            reg_a_candidate[pos] = position_candidate;

            // run the machine
            machine.instruction_pointer = 0;
            machine.register_a = vec_to_u64(&reg_a_candidate);
            let mut results: Vec<u64> = Vec::new();
            loop {
                match machine.Execute() {
                    ExecuteResult::None => (),
                    ExecuteResult::Halt => break,
                    ExecuteResult::Value(v) => results.push(v),
                };
            }

            if results.len() - 1 < pos {
                continue;
            }

            if results[results.len() - 1 - pos] == machine.memory[machine.memory.len() - 1 - pos] {
                // dbg!(&reg_a_candidate);
                // dbg!(&results[pos]);
                // dbg!(&machine.memory[pos]);
                break;
            }
        }
    }

    //dbg!(reg_a_candidate);
    let result = vec_to_u64(&reg_a_candidate) * 8; // do not included first div before
    Some(result)
}

pub fn part_two_slow(input: &str) -> Option<u64> {
    // too slow.
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let machine_data = data[0];
    let commands_data = data[1];

    let mut machine = parse_machine_data(machine_data, commands_data);

    let mut reg_a_candidate = 0;

    loop {
        machine.instruction_pointer = 0;
        machine.register_a = reg_a_candidate;

        if reg_a_candidate % 100000 == 0 {
            println!(
                "Computing {} of {} [{:.1}%]",
                reg_a_candidate,
                u64::MAX,
                (reg_a_candidate as f32 / u64::MAX as f32) * 100_f32
            )
        }

        let mut results: Vec<u64> = Vec::new();
        loop {
            match machine.Execute() {
                ExecuteResult::None => (),
                ExecuteResult::Halt => break,
                ExecuteResult::Value(v) => results.push(v),
            };
            if results.len() > machine.memory.len() {
                break;
            }
            if !results.is_empty()
                && results[results.len() - 1] != machine.memory[results.len() - 1]
            {
                break;
            }
        }

        if results == machine.memory {
            break;
        }

        reg_a_candidate += 1;
    }

    Some(reg_a_candidate as u64)
}

fn vec_to_u64(data: &Vec<u8>) -> u64 {
    let mut result = 0;
    for i in (0..data.len()).rev() {
        result += 8_u64.pow((data.len() - 1 - i) as u32) * data[i] as u64;
    }
    result
}

enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc(ComboOperand),
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<(u64, u64)> for Instruction {
    fn from(value: (u64, u64)) -> Self {
        match value.0 {
            0 => Self::Adv(ComboOperand::from(value.1)),
            1 => Self::Bxl(LiteralOperand { value: value.1 }),
            2 => Self::Bst(ComboOperand::from(value.1)),
            3 => Self::Jnz(LiteralOperand { value: value.1 }),
            4 => Self::Bxc(ComboOperand::from(value.1)),
            5 => Self::Out(ComboOperand::from(value.1)),
            6 => Self::Bdv(ComboOperand::from(value.1)),
            7 => Self::Cdv(ComboOperand::from(value.1)),
            _ => panic!("Unknown instruction opcode!"),
        }
    }
}

#[derive(PartialEq)]
enum ExecuteResult {
    None,
    Halt,
    Value(u64),
}

struct Machine {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    memory: Vec<u64>,
    instruction_pointer: usize,
}

impl Machine {
    fn Execute(&mut self) -> ExecuteResult {
        if self.memory.len() < self.instruction_pointer + 2 {
            return ExecuteResult::Halt;
        }

        let opcode = self.memory[self.instruction_pointer];
        let operand = self.memory[self.instruction_pointer + 1];

        let instruction = Instruction::from((opcode, operand));

        let result = match instruction {
            Instruction::Adv(combo_operand) => {
                let numerator = self.register_a;
                let denominator = combo_operand.get_value(&self);
                let result = numerator / 2_u64.pow(denominator as u32) as u64;

                self.register_a = result;
                self.instruction_pointer += 2;
                ExecuteResult::None
            }
            Instruction::Bxl(literal_operand) => {
                let value = self.register_b;
                let operand = literal_operand.value;
                let result = value ^ operand;

                self.register_b = result;
                self.instruction_pointer += 2;
                ExecuteResult::None
            }
            Instruction::Bst(combo_operand) => {
                let operand = combo_operand.get_value(self);
                let result = operand % 8;

                self.register_b = result;
                self.instruction_pointer += 2;
                ExecuteResult::None
            }
            Instruction::Jnz(literal_operand) => {
                let a_value = self.register_a;
                if a_value == 0 {
                    // do nothing
                    self.instruction_pointer += 2;
                } else {
                    let operand = literal_operand.value;
                    self.instruction_pointer = operand as usize;
                    // do not increase instruction pointer
                }
                ExecuteResult::None
            }
            Instruction::Bxc(combo_operand) => {
                let reg_b = self.register_b;
                let reg_c = self.register_c;
                let result = reg_b ^ reg_c;

                self.register_b = result;
                self.instruction_pointer += 2;
                ExecuteResult::None
            }
            Instruction::Out(combo_operand) => {
                let operand = combo_operand.get_value(&self);
                let value = operand % 8;

                self.instruction_pointer += 2;
                ExecuteResult::Value(value)
            }
            Instruction::Bdv(combo_operand) => {
                let numerator = self.register_a;
                let denominator = combo_operand.get_value(&self);
                let result = numerator / 2_u64.pow(denominator as u32) as u64;

                self.register_b = result;
                self.instruction_pointer += 2;
                ExecuteResult::None
            }
            Instruction::Cdv(combo_operand) => {
                let numerator = self.register_a;
                let denominator = combo_operand.get_value(&self);
                let result = numerator / 2_u64.pow(denominator as u32) as u64;

                self.register_c = result;
                self.instruction_pointer += 2;
                ExecuteResult::None
            }
        };

        result
    }

    fn print(&self) {
        println!("Register A: {}", self.register_a);
        println!("Register B: {}", self.register_b);
        println!("Register C: {}", self.register_c);
        println!("Instruction pointer C: {}", self.instruction_pointer);
        println!("Memory: {:?}", self.memory);
        println!();
    }
}

struct LiteralOperand {
    value: u64,
}

enum ComboOperand {
    Literal(LiteralOperand),
    RegA,
    RegB,
    RegC,
    Reserved,
}

impl From<u64> for ComboOperand {
    fn from(value: u64) -> Self {
        match value {
            0..=3 => ComboOperand::Literal(LiteralOperand { value }),
            4 => ComboOperand::RegA,
            5 => ComboOperand::RegB,
            6 => ComboOperand::RegC,
            7 => ComboOperand::Reserved,
            _ => panic!("Unsupported operand!"),
        }
    }
}

impl ComboOperand {
    fn get_value(&self, machine: &Machine) -> u64 {
        match self {
            ComboOperand::Literal(literal_operand) => literal_operand.value,
            ComboOperand::RegA | ComboOperand::RegB | ComboOperand::RegC => {
                self.get_reg_value(machine)
            }
            _ => panic!("Not reg or literal operand!"),
        }
    }

    fn get_reg_value(&self, machine: &Machine) -> u64 {
        match self {
            ComboOperand::RegA => machine.register_a,
            ComboOperand::RegB => machine.register_b,
            ComboOperand::RegC => machine.register_c,
            _ => panic!("Not reg operand!"),
        }
    }

    fn set_reg_value(&self, value: u64, machine: &mut Machine) {
        match self {
            ComboOperand::RegA => machine.register_b = value,
            ComboOperand::RegB => machine.register_b = value,
            ComboOperand::RegC => machine.register_b = value,
            _ => panic!("Not reg operand!"),
        }
    }
}

fn parse_machine_data(machine_data: &str, commands_data: &str) -> Machine {
    let mut iterator = machine_data.lines();
    let a_reg_value: u64 = iterator.next().expect("Error parsing data")[12..]
        .parse()
        .expect("Error parsing u64 value");
    let b_reg_value: u64 = iterator.next().expect("Error parsing data")[12..]
        .parse()
        .expect("Error parsing u64 value");
    let c_reg_value: u64 = iterator.next().expect("Error parsing data")[12..]
        .parse()
        .expect("Error parsing u64 value");

    let mut commands: Vec<u64> = Vec::new();
    commands_data[9..]
        .split(',')
        .for_each(|c| commands.push(c.parse().expect("Error parsing u64 value")));

    let result = Machine {
        register_a: a_reg_value,
        register_b: b_reg_value,
        register_c: c_reg_value,
        memory: commands,
        instruction_pointer: 0,
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("0,3,5,4,3,0".to_string()));
    }

    #[test]
    fn test_part_two_2_slow() {
        let result = part_two_slow(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }

    #[test]
    fn test_vec_to_u64() {
        let result = vec_to_u64(&vec![1]);
        assert_eq!(result, 1);

        let result = vec_to_u64(&vec![4]);
        assert_eq!(result, 4);

        let result = vec_to_u64(&vec![1, 1]);
        assert_eq!(result, 9);

        let result = vec_to_u64(&vec![2, 3]);
        assert_eq!(result, 19);
    }
}
