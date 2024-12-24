use std::{
    collections::{HashMap, HashSet},
    io::LineWriter,
};

use itertools::Itertools;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut data: Vec<&str> = input.split("\r\n\r\n").collect();
    if data.len() < 2 {
        // Actual data split.
        data = input.split("\n\n").collect();
    }
    let operand_data = data[0];
    let operation_data = data[1];

    let mut operands: HashMap<String, u8> = HashMap::new();
    let mut operations: HashMap<String, Operation> = HashMap::new();

    for line in operand_data.lines() {
        let mut iter = line.split(": ");
        let key = iter.next().unwrap();
        let value = iter.next().unwrap().parse().unwrap();

        operands.entry(key.to_string()).or_insert(value);
    }

    for line in operation_data.lines() {
        let mut iter = line.split(" ");
        let first_key = iter.next().unwrap();
        let operation = iter.next().unwrap();
        let second_key = iter.next().unwrap();
        let _ = iter.next().unwrap();
        let result_key = iter.next().unwrap();

        let gate = match operation {
            "AND" => Gate::And,
            "XOR" => Gate::Xor,
            "OR" => Gate::Or,
            _ => panic!(),
        };

        operations
            .entry(result_key.to_string())
            .or_insert(Operation {
                gate: gate,
                operand_1: first_key.to_string(),
                operand_2: second_key.to_string(),
            });
    }

    //let inverse_opratetion: HashMap<(&'a str, &'a str), Operation> = HashMap::new();

    let mut circuit = Circuit {
        operands,
        operations,
        //inverse_opratetion,
    };

    circuit.exe();
    let result = circuit.output();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

struct Circuit {
    operands: HashMap<String, u8>,
    operations: HashMap<String, Operation>,
    //inverse_opratetion: HashMap<(&'a str, &'a str), Operation>,
}

impl Circuit {
    fn exe(&mut self) {
        let mut to_compute = self.operations.clone();
        while to_compute.len() != 0 {
            let mut not_computed = HashMap::new();
            for (value, operation) in to_compute {
                if self.operands.contains_key(&operation.operand_1)
                    && self.operands.contains_key(&operation.operand_2)
                {
                    let first = self.operands[&operation.operand_1];
                    let second = self.operands[&operation.operand_2];

                    self.operands.entry(value).or_insert(match operation.gate {
                        Gate::And => first & second,
                        Gate::Or => first | second,
                        Gate::Xor => first ^ second,
                    });
                } else {
                    not_computed.entry(value).or_insert(operation);
                }
            }

            to_compute = not_computed;
        }
    }

    fn output(&self) -> u64 {
        let mut index = 0;
        let mut result: u64 = 0;

        let mut output_indexes: Vec<&String> = self
            .operands
            .keys()
            .filter(|k| k.starts_with("z"))
            .collect();

        output_indexes.sort();
        output_indexes.reverse();

        for key in output_indexes {
            result <<= 1;
            result += self.operands[key] as u64;
            index += 1;
        }
        result
    }

    // fn inverse_operation_find(&self, first: &str, second: &str) -> Option<Operation> {
    //     if self.inverse_opratetion.contains_key(&(first, second)) {
    //         return Some(self.inverse_opratetion[&(first, second)].clone());
    //     } else if self.inverse_opratetion.contains_key(&(second, first)) {
    //         return Some(self.inverse_opratetion[&(first, second)].clone());
    //     }
    //     None
    // }

    // fn check_connections(&self) {
    //     for i in 0..45 {
    //         let index = if i < 10 {
    //             format!("0{}", i)
    //         } else {
    //             i.to_string()
    //         };

    //         let x_index = format!("x{}", index);
    //         let y_index = format!("y{}", index);
    //         let z_index = format!("z{}", index);

    //         // first xor
    //         let xor_1 = self.inverse_operation_find(&x_index, &y_index);
    //         let mut xor_1_result = "".to_string();
    //         if let Some(x) = xor_1 {
    //             match x.gate {
    //                 Gate::Xor => xor_1_result = x.result,
    //                 _ => panic!(),
    //             }
    //         }

    //         // first and
    //         let and_1 = self.inverse_operation_find(&x_index, &y_index
    //     }
    // }

    //     fn outputv2(&self) -> u64 {
    //         let mut output_indexes: Vec<&String> = self
    //             .operands
    //             .keys()
    //             .filter(|k| k.starts_with("z"))
    //             .collect();

    //         output_indexes.sort();

    //         let mut carry = 0;
    //         for key in output_indexes {
    //             let c_result = self.operands[key] as u64;

    //             let x_index = format!("x{}", &key[1..]);
    //             let y_index = format!("y{}", &key[1..]);

    //             let true_result = self.operands[&x_index] ^ self.operands[&y_index] ^ carry;
    //             let next_carry = (self.operands[&x_index] & self.operands[&y_index])
    //                 | (carry & (self.operands[&x_index] ^ self.operands[&y_index]));

    //             if self.operands.contains_key(&x_index) {
    //                 println!(
    //                     "{}: {} + {} = {} + ?",
    //                     &key[1..],
    //                     self.operands[&x_index],
    //                     self.operands[&y_index],
    //                     self.operands[key]
    //                 );

    //                 if self.operands[key] != true_result {
    //                     print!("ERROR HERE!");
    //                     print!("carry is {}", carry);
    //                 }
    //                 println!();

    //                 carry = next_carry;
    //             }
    //         }
    //         1337
    //     }
}

#[derive(Clone)]
struct Operation {
    gate: Gate,
    operand_1: String,
    operand_2: String,
    // result: String,
}

#[derive(Clone)]
enum Gate {
    And,
    Or,
    Xor,
}

// #[derive(Clone)]
// struct Operand {
//     name: String,
//     value: Option<u8>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
