use std::cmp::{max, min};
use std::fs::{File, read_to_string};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Race {
    time: u64,
    distance: u64
}

fn main() {
    let input_file: String = read_to_string(Path::new("input.txt"))
        .expect("Could not open file");
    let lines: Vec<String>  = input_file.split("\n").map(|s:&str|{String::from(s)}).collect();
    let way = part_one(&lines);
    println!("Race Win Possibilities {}", way);
    let fixed_way = part_two(&lines);
    println!("Fixed Win Possibilities {}", fixed_way);
}

fn part_one(lines: &Vec<String>) -> u64{
    let races: Vec<Race> = read_races(&lines);
    races.iter().map(|r: &Race| {
        get_winning_range(r)
    }).map( | r: Range<u64> | {
        get_num_options(r)
    }).product()
}

fn part_two(lines: &Vec<String>) -> u64 {
    let race = vec![read_race(lines)];
    race.iter().map(|r: &Race| {
        get_winning_range(r)
    }).map( | r: Range<u64> | {
        get_num_options(r)
    }).product()
}

fn read_races(lines: &Vec<String>) -> Vec<Race> {
    let times: Vec<&str> = lines[0]
        .split(" ")
        .map(|s|s.trim())
        .filter(|s| s.len() > 0)
        .collect();

    let distances: Vec<&str> = lines[1]
        .split(" ")
        .map(|s|s.trim())
        .filter(|s| s.len() > 0)
        .collect();


    let mut races: Vec<Race> = Vec::new();

    for index in 1..times.len() {
        let t = times[index].trim();
        let d = distances[index].trim();
        races.push(Race{time: u64::from_str(t).unwrap(), distance: u64::from_str(d).unwrap()})
    }

    races
}

fn read_race(lines: &Vec<String>) -> Race {
    let times: String = lines[0]
        .split(" ")
        .map(|s|s.trim())
        .filter(|s| !s.eq(&"Time:"))
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>().join("");

    let distances: String = lines[1]
        .split(" ")
        .map(|s|s.trim())
        .filter(|s| !s.eq(&"Distance:"))
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>().join("");

    let time = u64::from_str(&times).expect("Expected");
    let distance = u64::from_str(&distances).expect("Expected");
    Race{time, distance}
}

fn get_winning_range(race: &Race) -> Range<u64> {
    let target_dist = race.distance;
    let time = race.time;

    let mut starting_value = u64::MAX;
    let mut ending_value = u64::MIN;

    let minimum_speed = target_dist / time;
    for step in minimum_speed..time+1 {
        let dist = step * (time - step);
        if dist > target_dist {
            starting_value = min(starting_value, step);
            ending_value = max(ending_value, step);
        }
    };
    starting_value..ending_value+1
}

fn get_num_options(r :Range<u64>) -> u64 {
    r.end - r.start
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dataset() -> Vec<String> {
        Vec::from([
            String::from("Time:      7  15   30\n"),
            String::from("Distance:  9  40  200")
        ])
    }

    #[test]
    fn test_read_races() {
        let expected_races: Vec<Race> = Vec::from(
            [
                Race{time: 7, distance: 9},
                Race{time: 15, distance: 40},
                Race{time: 30, distance: 200}
            ]
        );
        let found_races: Vec<Race> = read_races(&get_dataset());
        assert_eq!(expected_races, found_races)
    }

    #[test]
    fn test_find_button_range() {
        let races: Vec<Race> = read_races(&get_dataset());

        assert_eq!(2..6, get_winning_range(&races[0]));
        assert_eq!(4..12, get_winning_range(&races[1]));
    }

    #[test]
    fn test_get_options() {
        let races: Vec<Race> = read_races(&get_dataset());

        assert_eq!(4, get_num_options(get_winning_range(&races[0])));
        assert_eq!(8, get_num_options(get_winning_range(&races[1])));
        assert_eq!(9, get_num_options(get_winning_range(&races[2])));
    }

    #[test]
    fn test_part_one() {
        let records = part_one(get_dataset());
        assert_eq!(288, records);
    }

    #[test]
    fn test_part_two_read() {
        let r: Race = read_race(&get_dataset());
        assert_eq!(71530, r.time);
        assert_eq!(940200, r.distance);
    }
}

