use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut circuit = parse_circuit(input);

    circuit.exe();
    let result = circuit.output();

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    // thanks to https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3mrika/
    // and all other comments for general gate checking idea.

    // The main thing is the circuit is 44x44 -> 44 + cary adder
    // each a-b-c => z-c
    // and consists of two half adders
    /* art by https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3l9glk/

    x00──┬─────┐
     │    XOR──z00
    y00────┬───┘
     │ │
     │ └───┐
     │    AND──mgk
     └─────┘

    x01──┬─────┐
         │    XOR[rkf]┬────┐
    y01────┬───┘      │   XOR──────z01
         │ │          │    │
    mgk──────┬─────────────┘
         │ │ │        │
         │ │ │        │
         │ │ │       AND[kmj]──┐
         │ │ └────────┘        OR──rwp
         │ └──────────┐        │
         │           AND[nfw]──┘
         └────────────┘
    */

    // So we need each gate can only appear in certain configuration.
    // check_gates_correctness do all the checks, paying attention to
    // x00-y00 and z45 (last cary output).
    // Test showed* it is not 100 reliable, but seems to work with real task data.
    // * - sometimes I saw only one output when swapping outputs, and sometimes even inf cycle.
    // maybe only if swapping outputs that do not involve in z computations in any way.

    let circuit = parse_circuit(input);

    let result = circuit.check_gates_correctness();

    Some(result)
}

fn parse_circuit(input: &str) -> Circuit {
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
                gate,
                operand_1: first_key.to_string(),
                operand_2: second_key.to_string(),
            });
    }

    let circuit = Circuit {
        operands,
        operations,
    };

    #[allow(clippy::let_and_return)]
    circuit
}

struct Circuit {
    operands: HashMap<String, u8>,
    operations: HashMap<String, Operation>,
}

impl Circuit {
    fn exe(&mut self) {
        let mut to_compute = self.operations.clone();
        while !to_compute.is_empty() {
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
        }
        result
    }

    fn check_gates_correctness(&self) -> String {
        let mut incorrect_gates = vec![];

        for (c, operation) in &self.operations {
            let a = &operation.operand_1;
            let b = &operation.operand_2;
            let op = &operation.gate;

            if (c.starts_with("z") && *op != Gate::Xor && c != "z45")
                || (*op == Gate::Xor && [a, b, c].iter().all(|x| !x.starts_with(['x', 'y', 'z'])))
                || (*op == Gate::And
                    && a != "x00"
                    && b != "x00"
                    && self.operations.values().any(|operation| {
                        (*c == operation.operand_1 || *c == operation.operand_2)
                            && operation.gate != Gate::Or
                    }))
                || (*op == Gate::Xor
                    && self.operations.values().any(|operation| {
                        (*c == operation.operand_1 || *c == operation.operand_2)
                            && operation.gate == Gate::Or
                    }))
            {
                incorrect_gates.push(c);
            }
        }

        incorrect_gates.sort();
        incorrect_gates.iter().join(",")
    }
}

#[derive(Clone)]
struct Operation {
    gate: Gate,
    operand_1: String,
    operand_2: String,
}

#[derive(Clone, PartialEq)]
enum Gate {
    And,
    Or,
    Xor,
}

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
    fn test_part_two_1() {
        // correctly working
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_part_two_2() {
        // one swap
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some("ccc,z01".to_string()));
    }
}
