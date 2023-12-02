use std::fs::File;
use threadpool::ThreadPool;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};


static WORDS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];
fn line_sum(line: &String, tx: Sender<u16>) -> () {
    tx.send(sum_line_part_two(line)).expect("Channel unavailable");
}

fn main() {
    let input_file = File::open("input.txt").expect("File unable to be opened");
    let reader = BufReader::new(input_file);

    let pool: ThreadPool = ThreadPool::new(10);

    let (tx, rx) = channel();
    let mut line_count: u32 = 0;
    reader.lines().for_each(|line: Result<String, std::io::Error>| {
        line_count += 1;
        let txc: Sender<u16> = tx.clone();
        pool.execute(move || { line_sum(&line.unwrap(), txc) });
    });
    println!("Waiting for threads to finish...");
    pool.join();
    drop(tx);

    println!("Threads Finished!");
    let mut calibration :u32 = 0;
    rx.iter().for_each(|num: u16| {
        calibration += u32::from(num);

    });

    println!("Calibration Number is {}", calibration.to_string());
}

fn sum_line(line: &String) -> u16 {
    let mut first_num: char = 'a';
    let mut last_seen_num: char = 'a';

    line.chars().for_each(|f: char| {
        if f.is_digit(10) {
            if !first_num.is_digit(10) {
                first_num = f;
                last_seen_num = f;
            } else {
                last_seen_num = f;
            }
        }
    });
    let line_num = first_num.to_string() + &last_seen_num.to_string();
    u16::from_str(&line_num).unwrap()
}

fn sum_line_part_two(line: &String) -> u16 {
    let first_digit = find_first_digit(line);
    let first_word = find_first_word(line);

    let first_value = match (first_digit, first_word) {
        (Some((fd_pos, fd_char)), Some((fw_pos, fw_char))) => {
            if fd_pos < fw_pos {
                fd_char
            } else {
                fw_char
            }
        },
        (Some((_fd_pos, fd_char)), None) => fd_char,
        (None, Some((_fw_pos, fw_char))) => fw_char,
        _ => '0'
    };

    let last_digit = find_last_digit(line);
    let last_word = find_last_word(line);

    let last_value = match (last_digit, last_word) {
        (Some((ld_pos, ld_char)), Some((lw_pos, lw_char))) => {
            if ld_pos > lw_pos {
                ld_char
            } else {
                lw_char
            }
        },
        (Some((_ld_pos, ld_char)), None) => ld_char,
        (None, Some((_lw_pos, lw_char))) => lw_char,
        _ => '0'
    };

    let line_value = first_value.to_string()+&last_value.to_string();
    u16::from_str(&line_value).unwrap()
}

fn find_first_digit(line: &String) -> Option<(usize, char)> {
    let mut found: bool = false;
    let mut value = 'a';
    let mut pos: usize = usize::MAX;
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            pos = i;
            value = c;
            found = true;
            break;
        }
    };

    return if found {
        Some((pos, value))
    } else {
        None
    }
}

fn find_first_word(line: &String) -> Option<(usize, char)> {
    let mut found: bool = false;
    let mut value: u8 = 0;
    let mut pos: usize = usize::MAX;

    for (i, word) in WORDS.iter().enumerate() {
        let word_pos = line.find(word);
        match word_pos {
            Some(p) => {
                if p < pos {
                    value = (i as u8) + 1;
                    pos = p;
                    found = true;
                }
            }
            _ => ()
        }
    }

    return if found {
        Some((pos, char::from_digit(value as u32, 10)?))
    } else {
        None
    }
}

fn find_last_digit(line: &String) -> Option<(usize, char)> {
    let mut found: bool = false;
    let mut value: char = '1';
    let mut pos: usize = usize::MAX;
    let chars: Vec<char> = line.chars().collect();
    for (i, c) in chars.iter().enumerate().rev() {
        if c.is_digit(10) {
            pos = i;
            value = *c;
            found = true;
            break;
        }
    };

    return if found {
        Some((pos, value))
    } else {
        None
    }
}

fn find_last_word(line: &String) -> Option<(usize, char)> {
    let mut found: bool = false;
    let mut value: u8 = 0;
    let mut pos: usize = usize::MIN;

    for (i, word) in WORDS.iter().enumerate() {
        let word_pos = line.rfind(word);
        match word_pos {
            Some(p) => {
                if p > pos {
                    value = (i as u8) + 1;
                    pos = p;
                    found = true;
                }
            }
            _ => ()
        }
    }

    return if found {
        Some((pos, char::from_digit(value as u32, 10)?))
    } else {
        None
    }
}