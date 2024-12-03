advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let result = state_machine_parser(&input);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = state_machine_parser_with_do(&input);
    Some(result)
}

enum State {
    Init,
    Mul(char),
    LeftBracket,
    LeftValue(char),
    //Coma,
    RightValue(char),
    //RightBracket,
    // for part 2
    DoDont(char),
}

fn state_machine_parser(input: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut s = State::Init;
    let mut left_value = 0;
    let mut right_value = 0;
    let mut acc = 0;

    let empty_char = ' ';
    for c in input.chars() {
        s = match (s, c) {
            (State::Init, 'm') => State::Mul('m'),
            (State::Mul('m'), 'u') => State::Mul('u'),
            (State::Mul('u'), 'l') => State::LeftBracket,
            (State::LeftBracket, '(') => State::LeftValue(empty_char),
            (State::LeftValue(_cl), ci) => {
                if ci.is_ascii_digit() {
                    left_value = left_value * 10 + ci.to_digit(RADIX).unwrap();
                    State::LeftValue(ci)
                } else if ci == ',' {
                    State::RightValue(empty_char)
                } else {
                    (left_value, right_value) = (0, 0);
                    State::Init
                }
            }
            (State::RightValue(_cl), ci) => {
                if ci.is_ascii_digit() {
                    right_value = right_value * 10 + ci.to_digit(RADIX).unwrap();
                    State::RightValue(ci)
                } else if ci == ')' {
                    // do return value here, everything is ok
                    acc += left_value * right_value;
                    (left_value, right_value) = (0, 0);
                    State::Init
                } else {
                    (left_value, right_value) = (0, 0);
                    State::Init
                }
            }
            _ => {
                (left_value, right_value) = (0, 0);
                State::Init
            }
        }
    }
    acc
}

fn state_machine_parser_with_do(input: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut s = State::Init;
    let mut left_value = 0;
    let mut right_value = 0;
    let mut acc = 0;
    let mut active = true;

    // no prev char state label
    let empty_char = ' ';

    // [ is don't open bracket '(' state label
    let dont_open_bracket_char_state = '[';
    for c in input.chars() {
        s = match (s, c, active) {
            // begin part 2
            // do()
            (State::Init, 'd', _) => State::DoDont('d'),
            (State::DoDont('d'), 'o', _) => State::DoDont('o'),
            (State::DoDont('o'), '(', _) => State::DoDont('('),
            (State::DoDont('('), ')', _) => {
                active = true;
                State::Init
            }
            // don't()
            (State::DoDont('o'), 'n', _) => State::DoDont('n'),
            (State::DoDont('n'), '\'', _) => State::DoDont('\''),
            (State::DoDont('\''), 't', _) => State::DoDont('t'),
            (State::DoDont('t'), '(', _) => State::DoDont(dont_open_bracket_char_state),
            (State::DoDont(_dont_open_bracket_char_state), ')', _) => {
                active = false;
                State::Init
            }
            // end part 2
            // parse only if active
            (State::Init, 'm', true) => State::Mul('m'),
            (State::Mul('m'), 'u', true) => State::Mul('u'),
            (State::Mul('u'), 'l', true) => State::LeftBracket,
            (State::LeftBracket, '(', true) => State::LeftValue(empty_char),
            (State::LeftValue(_cl), ci, true) => {
                if ci.is_ascii_digit() {
                    left_value = left_value * 10 + ci.to_digit(RADIX).unwrap();
                    State::LeftValue(ci)
                } else if ci == ',' {
                    State::RightValue(empty_char)
                } else {
                    (left_value, right_value) = (0, 0);
                    State::Init
                }
            }
            (State::RightValue(_cl), ci, true) => {
                if ci.is_ascii_digit() {
                    right_value = right_value * 10 + ci.to_digit(RADIX).unwrap();
                    State::RightValue(ci)
                } else if ci == ')' {
                    // do return value here, everything is ok
                    acc += left_value * right_value;
                    (left_value, right_value) = (0, 0);
                    State::Init
                } else {
                    (left_value, right_value) = (0, 0);
                    State::Init
                }
            }
            _ => {
                (left_value, right_value) = (0, 0);
                State::Init
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_format_and_return() {
        let correct = "mul(5,5)";
        let result = state_machine_parser(&correct);
        assert_eq!(result, 25);

        let double_correct = "mul(5,5)mul(2,2)";
        let result = state_machine_parser(&double_correct);
        assert_eq!(result, 29);

        let double_correct_garbage_space = "mul(5,5)______________mul(2,2)";
        let result = state_machine_parser(&double_correct_garbage_space);
        assert_eq!(result, 29);

        let double_correct_garbage_sides = "_____mul(5,5)mul(2,2)______";
        let result = state_machine_parser(&double_correct_garbage_sides);
        assert_eq!(result, 29);

        let double_correct_garbage_space_unfinished = "mul(5,5)mulmul(2mul(2,mulmul(2,2)";
        let result = state_machine_parser(&double_correct_garbage_space_unfinished);
        assert_eq!(result, 29);

        let double_correct_garbage_sides_unfinished =
            "mulmul(2mul(2,mulmul(5,5)mul(2,2)mulmul(2mul(2,mul";
        let result = state_machine_parser(&double_correct_garbage_sides_unfinished);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_format_and_return_with_do() {
        let correct = "mul(5,5)";
        let result = state_machine_parser_with_do(&correct);
        assert_eq!(result, 25);

        let double_correct = "mul(5,5)mul(2,2)";
        let result = state_machine_parser_with_do(&double_correct);
        assert_eq!(result, 29);

        let double_correct_do_second = "don't()mul(5,5)do()mul(2,2)";
        let result = state_machine_parser_with_do(&double_correct_do_second);
        assert_eq!(result, 4);
    }
}
