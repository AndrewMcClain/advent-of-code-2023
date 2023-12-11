use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use crate::hand::Hand;

mod hand;

fn main() {
    let lines = read_input_to_lines();
    println!("Total Winnings for part one: {}", part_one(&lines))
}

fn read_input_to_lines() -> Vec<String> {
    let input_file = read_to_string(Path::new("input.txt")).expect("Couldn't read input.txt");
    input_file.split("\n").map(|s|{String::from(s.trim())}).collect()
}

fn part_one(lines: &Vec<String>) -> u32{
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        hands.push(Hand::from_str(line).expect("Unable to read line"));
    };
    hands.sort();
    hands.reverse();
    hands.iter().enumerate().fold(0u32, |a, (i,h)| {
        a + (((i as u32) + 1) * h.bid)
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::hand::Hand;
    use crate::part_one;

    fn get_example_input() -> String {
        String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483")
    }

    #[test]
    fn test_sorting() {
        let input_string = get_example_input();
        let lines: Vec<&str> = input_string.split("\n").collect();

        let mut hands: Vec<Hand> = Vec::new();

        for line in lines {
            hands.push(Hand::from_str(line).expect("Unable to process input"));
        }
        hands.sort();
        hands.reverse();
        let mut total_winnings: u32 = 0u32;
        hands.iter().enumerate().for_each(|(i, h)| {
            total_winnings += h.bid * ((i as u32) + 1)
        });
        assert_eq!(6440, total_winnings)
    }

    #[test]
    fn test_part_one() {
        let input_string = get_example_input();
        let lines = input_string.split("\n").map(|s|{String::from(s.trim())}).collect();
        let example_score = part_one(&lines);
        assert_eq!(6440, example_score);
    }
}

