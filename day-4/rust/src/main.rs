use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc::channel;
use futures::executor::ThreadPool;

fn main() {

    let input_file: File = File::open(Path::new("input.txt")).expect("Expected 'input.txt' to open");
    let reader = BufReader::new(input_file);

    let pool = ThreadPool::new().expect("Expected to create threadpool");

    let (tx, rx) = channel();

    reader.lines().for_each( | line_result: Result<String, std::io::Error> | {
        let line = line_result.expect("Expected a string");
        let txc = tx.clone();
        let worker = async move {
            txc.send(get_card_value(line)).expect("Expected to send card value along channel");
        };
        pool.spawn_ok(worker);
    });
    drop(tx);

    let card_sum = rx.iter().fold(0u32, |a, x| a + x);
    println!("Sum of card values is {}", card_sum);
}

fn get_card_value(line: String) -> u32 {
    let matches = get_card_matches(line);
    if matches > 0 {
        2u32.pow((matches as u32) - 1)
    } else {
        0u32
    }
}

fn get_card_matches(line: String) -> u8 {
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
        let value = get_card_value(line);
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
            sum += get_card_value(line);
        });

        assert_eq!(13, sum)
    }
}
