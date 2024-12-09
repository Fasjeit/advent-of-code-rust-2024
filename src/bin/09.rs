advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let compressed_form = parse_compressed_form(input);
    //dbg!(&compressed_form);

    let mut sparse_form = create_sparse_form(compressed_form);
    //dbg!(&sparse_form);

    compress(&mut sparse_form);
    //dbg!(&sparse_form);

    let checksum = checksum(&sparse_form);

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let compressed_form = parse_compressed_form(input);
    //dbg!(&compressed_form);

    let mut sparse_form = create_sparse_form(compressed_form);
    //dbg!(&sparse_form);

    compress_part2(&mut sparse_form);
    //dbg!(&sparse_form);

    let checksum = checksum(&sparse_form);

    Some(checksum)
}

fn parse_compressed_form(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("Expected digit only chars"))
        .collect()
}

fn create_sparse_form(compressed: Vec<u32>) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    for (index, data) in compressed.iter().enumerate() {
        if index % 2 == 0 {
            // data
            let id = index / 2;
            for _i in 0..*data {
                result.push(id as i64);
            }
        } else {
            for _i in 0..*data {
                result.push(-1);
            }
        }
    }
    result
}

fn compress(sparse_form: &mut Vec<i64>) {
    for index in 0..sparse_form.len() {
        // Skip until free space.

        if index >= sparse_form.len() {
            break;
        }

        if sparse_form[index] != -1 {
            continue;
        }

        // get last non-free element
        let mut last_element = -1;
        let mut pop_index = 0;
        while last_element == -1 {
            last_element = sparse_form.pop().unwrap();
            pop_index += 1;
        }

        if sparse_form.len() - pop_index <= index {
            sparse_form.push(last_element); // push back
            break;
        }

        sparse_form[index] = last_element;

        //dbg!(&sparse_form);
        //dbg!(&index);
    }
}

#[derive(Debug)]
struct File {
    data: i64,
    size: usize,
    starting_pos: usize,
}

fn pop_file(sparse_form: &mut [i64], right_index: usize) -> Option<File> {
    //dbg!(&sparse_form);
    let mut last_element = -1;
    let mut pop_index = right_index;

    // read last block
    while last_element == -1 {
        if pop_index > 0 {
            last_element = sparse_form[pop_index];
            pop_index -= 1;
        } else {
            return None;
        }
    }

    let mut file_size = 1;
    let data = last_element;

    // read all file blocks
    while last_element == data {
        if pop_index > 0 {
            last_element = sparse_form[pop_index];
            pop_index -= 1;
            file_size += 1;
        } else {
            return None;
        }
    }
    // return non file element back.
    pop_index += 2;
    file_size -= 1;

    let file = File {
        data,
        size: file_size,
        starting_pos: pop_index,
    };

    //dbg!(&file);

    Some(file)
}

#[allow(clippy::needless_range_loop)]
fn compress_part2(sparse_form: &mut [i64]) {
    let mut right_file_index = sparse_form.len() - 1;
    loop {
        if let Some(file) = pop_file(sparse_form, right_file_index) {
            //dbg!(right_file_index);
            right_file_index = file.starting_pos - 1;
            let mut free_space_counter = 0;
            for index in 0..sparse_form.len() {
                // Skip until free space.

                if index >= file.starting_pos {
                    break;
                }

                if sparse_form[index] != -1 {
                    free_space_counter = 0;
                    continue;
                }

                free_space_counter += 1;
                if free_space_counter == file.size {
                    // write file to the free space
                    for i in index - free_space_counter + 1..index + 1 {
                        sparse_form[i] = file.data;
                    }
                    // delete initial file
                    for i in file.starting_pos..file.starting_pos + file.size {
                        sparse_form[i] = -1;
                    }
                    break;
                }
            }
        } else {
            return;
        }
    }
}

fn checksum(sparse_form: &[i64]) -> u64 {
    let mut result = 0;
    //dbg!(sparse_form);
    for (index, data) in sparse_form.iter().enumerate() {
        if *data == -1 {
            continue;
        }
        result += (index as u64) * (*data as u64);
        //dbg!(index);
        //dbg!(*data);
        //dbg!((index as u64) * (*data as u64));
        //dbg!(result);

        // if (index > 100) {
        //     todo!()
        // }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_dbg() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two_dbg() {
        let data = "12343";
        let result = part_two(data);
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2858));
    }
}
