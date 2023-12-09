use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc::channel;

fn main() {
    let input_file: File = File::open(Path::new("input.txt")).expect("Expected input.txt file");
    let mut line_reader = BufReader::new(input_file).lines();
    let all_lines: Vec<String> = line_reader.map( | l| {
        l.expect("Could not process line")
    }).collect();

    let running_total = process_all_lines(all_lines);

    println!("The ratio of the gears is {}", running_total);
}

#[derive(PartialEq, Eq, Hash)]
struct EnginePart {
    start: usize,
    end: usize,
    value: u32
}

fn process_all_lines(all_lines: Vec<String>) -> u32 {
    let mut all_lines_iter = all_lines.into_iter();
    let mut previous_line: Option<String> = None;
    let mut current_line: Option<String> = all_lines_iter.next();
    let mut next_line: Option<String> = all_lines_iter.next();
    let mut running_total = 0u32;

    while current_line.is_some() {
        running_total += get_part_sum(&previous_line, &current_line, &next_line);
        previous_line = current_line.take();
        current_line = next_line.take();

        let possible_next = all_lines_iter.next();
        if possible_next.is_some() {
            next_line = possible_next;
        } else {
            next_line = None;
        }
    }
    running_total
}

fn get_part_sum(previous_line: &Option<String>, current_line: &Option<String>, next_line: &Option<String>) -> u32 {
    let mut gear_ratio = 0;
    current_line.clone().expect("Current Line Expected to be some")
        .chars()
        .enumerate()
        .for_each(|(index, c)| {
            if c == '*' {
                let mut adjacent_parts: HashSet<EnginePart> = visit_eight_neighbors(index, &previous_line, &current_line, &next_line);

                if adjacent_parts.len() == 2 {
                    gear_ratio = adjacent_parts
                        .drain()
                        .fold(1u32, |a, x| a * x.value)
                }
            }
    });
    gear_ratio
}

fn visit_eight_neighbors(index: usize, prev_line: &Option<String>, current_line: &Option<String>, next_line: &Option<String>) -> HashSet<EnginePart> {
    let mut parts: HashSet<EnginePart> = HashSet::new();
    // Previous Line
    if prev_line.is_some() {
        let prev_str = prev_line.clone().expect("Expected Previous Line");
        let prev_chars: Vec<char> = prev_str.chars().collect();
        if check_neighbor(index-1, &prev_chars) {
            let opt_part = find_number_slice(index-1, &prev_str);
            parts.insert(opt_part.expect("Expected a part"));
        }
        if check_neighbor(index, &prev_chars) {
            let opt_part = find_number_slice(index, &prev_str);
            parts.insert(opt_part.expect("Expected a part"));
        }
        if check_neighbor(index+1, &prev_chars) {
            let opt_part = find_number_slice(index+1, &prev_str);
            parts.insert(opt_part.expect("Expected a part"));
        }
    }
    // Current Line

    let current_str = current_line.clone().expect("Expected a string");
    let current_chars: Vec<char> = current_str
        .chars()
        .collect();

    if check_neighbor(index-1, &current_chars) {
        let opt_part = find_number_slice(index-1, &current_str);
        parts.insert(opt_part.expect("Expected a part"));
    }
    if check_neighbor(index+1, &current_chars) {
        let opt_part = find_number_slice(index+1, &current_str);
        parts.insert(opt_part.expect("Expected a part"));
    }

    // Next Line

    if next_line.is_some() {
        let next_str = next_line.clone().expect("Expected next line");
        let next_chars: Vec<char> = next_str.chars().collect();
        if check_neighbor(index-1, &next_chars) {
            let opt_part = find_number_slice(index-1, &next_str);
            parts.insert(opt_part.expect("Expected a part"));
        }
        if check_neighbor(index, &next_chars) {
            let opt_part = find_number_slice(index, &next_str);
            parts.insert(opt_part.expect("Expected a part"));
        }
        if check_neighbor(index+1, &next_chars) {
            let opt_part = find_number_slice(index+1, &next_str);
            parts.insert(opt_part.expect("Expected a part"));
        }
    }

    return parts;
}

fn check_neighbor(index: usize, chars: &Vec<char>) -> bool {
    let c = chars.get(index);

    match c {
        Some(c) => c.is_digit(10),
        None => false
    }
}

fn find_number_slice(index: usize, line: &str) -> Option<EnginePart>{
    let mut start: usize = index;
    let mut end: usize = index;
    let chars: Vec<char> = line.chars().collect();

    if !chars[index].is_digit(10) {
        return None;
    }

    loop {
        let mut expanded: bool = false;
        if start > 0 {
            if chars[start - 1].is_digit(10) {
                start -= 1;
                expanded = true;
            }
        }
        if end < chars.len()-1 {
            if chars[end+1].is_digit(10) {
                end += 1;
                expanded = true;
            }
        }
        if expanded {
            expanded = false
        } else {
            break;
        }
    }

    let part_num = &line[start..end + 1];
    let part = EnginePart{
        start,
        end,
        value: u32::from_str(part_num).expect("Unable to parse line slice")
    };
    return Some(part);
}

fn is_symbol(c: char) -> bool{
    return (c >= '!' && c < '.') || (c == '/') || (c >= ':' && c <= '@') || (c >= '[' && c <= '`') || (c >= '{' && c <= '~')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_two() {
        let example: String =
            String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n");

        let example_string: Vec<String> = example.split("\n").map( | s: &str| {
            String::from(s)
        }).collect();


        let gear_ratio = process_all_lines(example_string);
        assert_eq!(467835, gear_ratio);
    }
}