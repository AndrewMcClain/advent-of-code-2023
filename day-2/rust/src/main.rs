use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use futures::executor::ThreadPool;

fn main() {
    let input_file = File::open("input.txt").expect("Could not open the file");
    let reader = BufReader::new(input_file);
    let (tx, rx) = channel();

    let max_cubes = MaxCubes { red: 12, green: 13, blue: 14};

    let pool: ThreadPool = ThreadPool::new().expect("Couldn't create a threadpool");

    reader.lines().for_each( |line_result: Result<String, std::io::Error>| {

        let txc = tx.clone();
        let mcc = max_cubes.clone();
        let line = line_result.expect("Expected To read line");
        let future = async move {
            sum_powers(line, txc)
        };
        pool.spawn_ok(future);
    });
    drop(tx);

    let sum: u32 = rx.iter().fold(0u32, |acc, x| acc+ u32::from(x));
    println!("Sum of all games powers is : {}", sum);
}

#[derive(Copy, Clone)]
struct MaxCubes {
    red: u8,
    green: u8,
    blue: u8
}

fn sum_powers(line: String, tx: Sender<u16>) -> () {
    let mut max_cubes = MaxCubes{red: 0, green: 0, blue: 0};
    let line: Vec<&str> = line.trim().split(":").collect();

    line[1].trim().split(";").for_each( |round: &str| {
        round.trim().split(",").for_each( |pull: &str| {
            let split_pull: Vec<&str> = pull.trim().split(" ").collect();
            let count = u8::from_str(split_pull[0])
                .expect("Error parsing number of cubes pulled");
            match split_pull[1] {
                "red" => max_cubes.red = max(count, max_cubes.red),
                "green" => max_cubes.green = max(count, max_cubes.green),
                "blue" => max_cubes.blue = max(count, max_cubes.blue),
                _ => println!("Unrecognized color")
            }
        })
    });

    let power: u16 = (max_cubes.red as u16) * (max_cubes.green as u16) * (max_cubes.blue as u16);
    tx.send(power).expect("Unable to send message")

}

fn sum_ids(line: String, max_cubes: MaxCubes, tx: Sender<u8>) -> () {
    let line: Vec<&str> = line.trim().split(":").collect();
    let game_id: Vec<&str> = line[0].split(" ").collect();
    let id: u8 = u8::from_str(game_id[1]).expect("Invalid id");
    if valid_game(line[1].trim(), &max_cubes) {
        tx.send(id).expect("Channel Unavailable");
    }
}

fn valid_game(game: &str, max_cubes: &MaxCubes) -> bool {
    let mut valid = true;
    game.split(";").for_each(|round: &str| {
       if !valid_round(round, max_cubes) {
           valid = false;
       }
    });
    valid
}

fn valid_round(round: &str, max_cubes: &MaxCubes) -> bool {
    let mut valid = true;
    round.split(",").for_each(|cube_pull: &str| {
        let pull: Vec<&str> = cube_pull.trim().split(" ").collect();
        let count = u8::from_str(pull[0]).expect("Invalid cube count");
        match pull[1] {
            "red" => if count > max_cubes.red {valid = false;}
            "green" => if count > max_cubes.green {valid = false;}
            "blue" => if count > max_cubes.blue {valid = false;}
            _ => valid = false
        }
    });

    valid
}
