use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc::channel;
use futures::executor::ThreadPool;
use futures::StreamExt;

fn main() {

    let input_file: File = File::open(Path::new("input.txt")).expect("Expected 'input.txt' to open");
    let value = parallel_card_value_calc(input_file);
    println!("Total Card Value : {}", value);
    let input_file_part_2: File = File::open(Path::new("input.txt")).expect("Expected 'input.txt' to open");
    let count = part_two(input_file_part_2);
    println!("Total Card Count : {}", count);
}

fn part_two(input_file: File) -> u64 {
    let reader = BufReader::new(input_file);
    let all_lines: Vec<String> = reader.lines().map( | s: Result<String, Error> | {
        s.unwrap()
    }).collect();
    card_count_calc(all_lines)
}

fn card_count_calc(all_lines: Vec<String>) -> u64 {

    let mut card_quantity: VecDeque<u64> = VecDeque::new();
    let mut total_card_count = 0u64;

    all_lines.iter().for_each( |line: &String | {
        let quantity: Option<u64> = card_quantity.pop_front();

        let concrete_quantity: u64;
        if quantity.is_some() {
            concrete_quantity = quantity.unwrap()+1;
        } else {
            concrete_quantity = 1u64;
        }

        total_card_count += concrete_quantity;
        let matches = get_card_matches(line);
        if matches > 0 {
            for m in 0..matches {
                let index = usize::from(m);
                let current_count = card_quantity.remove(index);

                println!("Concrete quantity {}", concrete_quantity);
                println!("Current count: {} ", current_count.unwrap_or_else( || 0u64 ));
                card_quantity.insert(index, concrete_quantity + current_count.unwrap_or_else(|| 0u64));
            }
        }
    });
    total_card_count
}

fn parallel_card_value_calc(input_file: File) -> u32 {
    let reader = BufReader::new(input_file);

    let pool = ThreadPool::new().expect("Expected to create threadpool");

    let (tx, rx) = channel();

    reader.lines().for_each( | line_result: Result<String, std::io::Error> | {
        let line = line_result.expect("Expected a string");
        let txc = tx.clone();
        let worker = async move {
            txc.send(get_card_value(&line)).expect("Expected to send card value along channel");
        };
        pool.spawn_ok(worker);
    });
    drop(tx);

    rx.iter().fold(0u32, |a, x| a + x)
}

fn get_card_value(line: &str) -> u32 {
    let matches = get_card_matches(line);
    if matches > 0 {
        2u32.pow((matches as u32) - 1)
    } else {
        0u32
    }
}

fn get_card_matches(line: &str) -> u8 {
    let _card_label = line.trim()[0..line.find(":").expect("Expected to find ':'")].trim();
    let winning_numbers_str = line.trim()[
        line.find(":").expect("Expected to find ':'")+1
            ..
        line.find("|").expect("Expected to find '|'")].trim();
    let our_card_numbers_str = line.trim()[line.find("|").expect("Expected to find '|'")+1..].trim();

    let winning_nums = card_str_to_u8(winning_numbers_str);
    let card_nums = card_str_to_u8(our_card_numbers_str);

    card_matches(&winning_nums, &card_nums)
}

fn card_str_to_u8(nums_str: &str) -> Vec<u8> {
    let mut nums: Vec<u8> = Vec::new();

    nums_str.trim().split(" ").for_each( |num_str: &str| {
        let num_trim = num_str.trim();
        if num_trim.len() > 0 {
            nums.push(u8::from_str(num_trim).expect("Expected valid number"));
        }
    });

    nums
}

fn card_matches(winning_nums: &Vec<u8>, card_nums: &Vec<u8>) -> u8 {
    let mut hits: u8 = 0;

    winning_nums.iter().for_each( | winning_num | {
        card_nums.iter().for_each( | card_num | {
           if winning_num == card_num {
               hits += 1;
           }
        });
    });
    hits
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example_part_one() {
        let line = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let value = get_card_value(&line);
        assert_eq!(8, value);
    }

    #[test]
    fn example_part_one_lines() {
        let mut example_lines: Vec<String> = vec!{
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        };
        let mut sum = 0;

        example_lines.drain(..).for_each( | line: String | {
            sum += get_card_value(&line);
        });

        assert_eq!(13, sum)
    }

    #[test]
    fn example_part_two_lines() {
        let example_lines: Vec<String> = vec!{
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        };
        let count = card_count_calc(example_lines);

        assert_eq!(30, count)
    }
}
