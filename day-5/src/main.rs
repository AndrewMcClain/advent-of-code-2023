use std::cmp::min;
use std::fs::{File, read_to_string};
use std::i128;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

type Mapping = (Range<i64>, i64);

fn main() {
    let input_file: String = read_to_string(Path::new("input.txt"))
        .expect("Expected to read input.txt");
    let closest = part_one(&input_file);
    println!("Closest Location corresponding to a seed is {}", closest);
    let closest_p2 = part_two(&input_file);
    println!("Closes Location with seed ranges {}", closest_p2);
}

fn part_one(input: &str) -> i64 {
    let seeds = read_seeds(input);
    let seed_to_soil = read_seed_to_soil_from_source(input);
    let soil_to_fert = read_soil_to_fertilizer_from_source(input);
    let fert_to_water = read_fertilizer_to_water_from_source(input);
    let water_to_light = read_water_to_light_from_source(input);
    let light_to_temp = read_light_to_temperature_from_source(input);
    let temp_to_humid = read_temperature_to_humidity_from_source(input);
    let humid_to_loc = read_humidity_to_location_from_source(input);

    let mut closest_loc = i64::MAX;

    for seed in seeds {
        let soil = forward_map(seed, &seed_to_soil);
        let fert = forward_map(soil, &soil_to_fert);
        let water = forward_map(fert, &fert_to_water);
        let light = forward_map(water, &water_to_light);
        let temp = forward_map(light, &light_to_temp);
        let humid = forward_map(temp, &temp_to_humid);
        let loc = forward_map(humid, &humid_to_loc);

        closest_loc = min(loc, closest_loc);
    }

    return closest_loc;
}

fn part_two(input: &str) -> i64 {
    let seeds = read_seed_ranges(input);
    let mapping_layers = Vec::from(
        [
            read_seed_to_soil_from_source(input),
            read_soil_to_fertilizer_from_source(input),
            read_fertilizer_to_water_from_source(input),
            read_water_to_light_from_source(input),
            read_light_to_temperature_from_source(input),
            read_temperature_to_humidity_from_source(input),
            read_humidity_to_location_from_source(input)
        ]
    );

    map_all(seeds, &mapping_layers).iter().min_by( | r, s| {
        r.start.cmp(&s.start)
    }).unwrap().start
}

fn read_seeds(source_string: &str) -> Vec<i64> {
    let mut seed_str: &str = "";
    while let l = source_string.split("\n").next() {
        let line = l.unwrap_or_else( || {""});
        if line.starts_with("seeds") {
            seed_str = (line.split(":").collect::<Vec<_>>() as Vec<&str>)[1].trim();
            break;
        }
    }

    seed_str.split(" ").flat_map(|num: &str| {
        i64::from_str(num)
    }).collect()
}

fn read_seed_ranges(source_string: &str) -> Vec<Range<i64>> {
    let mut seed_str: &str = "";
    while let l = source_string.split("\n").next() {
        let line = l.unwrap_or_else( || {""});
        if line.starts_with("seeds") {
            seed_str = (line.split(":").collect::<Vec<_>>() as Vec<&str>)[1].trim();
            break;
        }
    }

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    let mut seed_iter = seed_str.split(" ");
    loop {
        let start_str = seed_iter.next();
        if start_str.is_none() {
            break;
        }
        let start = i64::from_str(start_str.unwrap()).unwrap();
        let length = i64::from_str(seed_iter.next().unwrap()).unwrap();
        seed_ranges.push(start..start+length);
    }
    return seed_ranges;
}

fn read_seed_to_soil_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "seed-to-soil")
}

fn read_soil_to_fertilizer_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "soil-to-fertilizer")
}

fn read_fertilizer_to_water_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "fertilizer-to-water")
}

fn read_water_to_light_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "water-to-light")
}

fn read_light_to_temperature_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "light-to-temperature")
}

fn read_temperature_to_humidity_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "temperature-to-humidity")
}

fn read_humidity_to_location_from_source(source_string: &str) -> Vec<Mapping> {
    read_block_from_source(source_string, "humidity-to-location")
}

fn read_block_from_source(source_string: &str, block_header: &str) -> Vec<Mapping> {
    let mut collect_line: bool = false;
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut line_iter = source_string.split("\n");
    while let l = line_iter.next() {
        if l.is_none() {
            break
        }
        let line = l.unwrap();
        if line.contains(block_header) {
            collect_line = true;
            continue
        }

        if collect_line {
            if !line.eq("") {
                mappings.push(read_range_mapping(line))
            } else {
                break
            }
        }
    }
    mappings
}

fn read_range_mapping(range: &str) -> Mapping {
    let range_def: Vec<i64> = range
        .trim()
        .split(" ")
        .map(|s: &str| { i64::from_str(s).expect("Expected Valid i64 Number") })
        .collect();

    (range_def[1]..range_def[1]+range_def[2], (range_def[0] - range_def[1]) as i64)
}

fn forward_map(input: i64, mappings: &Vec<Mapping>) -> i64 {
    for mapping in mappings {
        if mapping.0.contains(&input) {
            return input + mapping.1 as i64;
        }
    };
    return input;
}

fn map_all(seeds_ranges: Vec<Range<i64>>, mapping_vecs: &Vec<Vec<Mapping>>) -> Vec<Range<i64>>{
        let mut current = seeds_ranges.clone();

    for mapping_layer in mapping_vecs {
        current = map_layer(&current, mapping_layer);
    }

    return current;
}

fn map_layer(source_ranges: &Vec<Range<i64>>, layer: &Vec<Mapping>) -> Vec<Range<i64>> {
    let mut transformed_ranges: Vec<Range<i64>> = Vec::new();

    for source_range in source_ranges {
        let mut untransformed_ranges: Vec<Range<i64>> = Vec::new();
        untransformed_ranges.push(source_range.clone());
        for mapping in layer {
            while let opt_range = untransformed_ranges.pop() {
                if opt_range.is_none() {
                    break;
                }

                let candidate_range = opt_range.unwrap();

                if overlap(&candidate_range, &mapping.0) {
                    let (before, intersect, after) = destructive_intersect(&candidate_range, &mapping.0);
                    if before.is_some() {
                        untransformed_ranges.push(before.unwrap());
                    }
                    if intersect.is_some() {
                        let mut transformed_range = intersect.unwrap();
                        transformed_range.start += mapping.1;
                        transformed_range.end += mapping.1;
                        transformed_ranges.push(transformed_range);
                    }
                    if after.is_some() {
                        untransformed_ranges.push(after.unwrap());
                    }
                } else {
                    untransformed_ranges.push(candidate_range);
                    break;
                }
            }
        }
        for range in untransformed_ranges {
            transformed_ranges.push(range);
        }
    }

    transformed_ranges
}

fn overlap(r1: &Range<i64>, r2: &Range<i64>) -> bool {
    (r1.start < r2.end) && (r2.start < r1.end)
}

fn destructive_intersect(r1: &Range<i64>, r2: &Range<i64>) -> (Option<Range<i64>>, Option<Range<i64>>, Option<Range<i64>>) {
    if overlap(r1, r2) {

        let before;
        let intersection;
        let after;

        if r2.contains(&r1.start) && !r2.contains(&(r1.end-1)) { // r1 to the right
            before = None;
            intersection = Some(r1.start..r2.end);
            after = Some(r2.end..r1.end);
        } else if r2.contains(&(r1.end-1)) && !r2.contains(&r1.start) { // r1 to the left
            before = Some(r1.start..r2.start);
            intersection = Some(r2.start..r1.end);
            after = None;
        } else if r2.contains(&r1.start) && r2.contains(&(r1.end-1)) { // r1 contained
            before = None;
            intersection = Some(r1.start..r1.end);
            after = None;
        } else if r1.contains(&r2.start) && r1.contains(&(r2.end-1)) {// r2 contained
            before = Some(r1.start..r2.start);
            intersection = Some(r2.start..r2.end);
            after = Some(r2.end..r1.end);
        } else {
            before = None;
            intersection = Some(r1.start..r2.start);
            after = None;
        }

        (before, intersection, after)
    } else {
        if r1.end <= r2.start {
            (Some(r1.clone()), None, None)
        } else {
            (None, None, Some(r1.clone()))
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

 seed-to-soil map:
 50 98 2
 52 50 48

 soil-to-fertilizer map:
 0 15 37
 37 52 2
 39 0 15

 fertilizer-to-water map:
 49 53 8
 0 11 42
 42 0 7
 57 7 4

 water-to-light map:
 88 18 7
 18 25 70

 light-to-temperature map:
 45 77 23
 81 45 19
 68 64 13

 temperature-to-humidity map:
 0 69 1
 1 0 69

 humidity-to-location map:
 60 56 37
 56 93 4";

    fn getDataSet() -> Vec<(Range<i64>, Range<i64>, bool, Option<Range<i64>>, Option<Range<i64>>, Option<Range<i64>>)> {
        Vec::from(
        [
            (0..2, 3..5, false, Some(0..2), None, None),
            (0..2, 2..4, false, Some(0..2), None, None),
            (0..2, 1..3, true, Some(0..1), Some(1..2), None),
            (0..2, 0..2, true, None, Some(0..2), None),
            (0..2, -1..1, true, None, Some(0..1), Some(1..2)),
            (-2..0, 0..2, false, Some(-2..0), None, None),
            (-3..-1, 0..2, false, Some(-3..-1), None, None),
            (3..5, 0..2, false, None, None, Some(3..5)),
            (2..4, 0..2, false, None, None, Some(2..4)),
            (1..3, 0..2, true, None, Some(1..2), Some(2..3)),
            (0..2, 0..2, true, None, Some(0..2), None),
            (-1..1, 0..2, true, Some(-1..0), Some(0..1), None),
            (0..2, -2..0, false, None, None, Some(0..2)),
            (0..2, -3..-1, false, None, None, Some(0..2)),
            (0..5, 1..4, true, Some(0..1), Some(1..4), Some(4..5)),
            (1..4, 0..5, true, None, Some(1..4), None)
        ]
    )
}

    #[test]
    fn read_file() {

        let seeds = read_seeds(EXAMPLE_INPUT);
        assert_eq!(vec![79i64, 14i64, 55i64, 13i64], seeds);
    }

    #[test]
    fn range_def_test() {
        let (range1, range2) = read_range_mapping("50 98 2");
        assert_eq!(98..100, range1);
        assert_eq!(-48, range2);
    }

    #[test]
    fn test_seed_to_soil() {
        let expected_mappings: Vec<Mapping> = Vec::from (
            [
                (98..98+2, -48),
                (50..50+48, 2)
            ]
        );
        let found_mappings = read_seed_to_soil_from_source(&EXAMPLE_INPUT);
        assert_eq!(expected_mappings, found_mappings);
        assert_eq!(50, forward_map(98, &found_mappings));
        assert_eq!(51, forward_map(99, &found_mappings));
    }

    #[test]
    fn test_soil_to_fertilizer() {
        let expected_mappings: Vec<Mapping> = Vec::from(
            [
                (15..15+37, -15),
                (52..52+2, -15),
                (0..15, 39)
            ]
        );
        let found_mappings = read_soil_to_fertilizer_from_source(&EXAMPLE_INPUT);
        assert_eq!(expected_mappings, found_mappings)
    }

    #[test]
    fn test_part_one() {
        let found_closest = part_one(EXAMPLE_INPUT);
        assert_eq!(35, found_closest);
    }

    #[test]
    fn test_read_ranges() {
        let expected_ranges: Vec<Range<i64>> = Vec::from(
            [
                79..79+14,
                55..55+13
            ]
        );
        let found_ranges = read_seed_ranges(EXAMPLE_INPUT);
        assert_eq!(expected_ranges, found_ranges);
    }

    #[test]
    fn test_overlap_ranges() {
        let range_overlap = getDataSet();
        for set in range_overlap {
            assert_eq!(set.2, overlap(&set.0, &set.1));
        }
    }

    #[test]
    fn test_destructive_intersection() {
        let range_overlap = getDataSet();
        for set in range_overlap {
            assert_eq!((set.3, set.4, set.5), destructive_intersect(&set.0, &set.1));
        }
    }

    #[test]
    fn test_map_app() {
        let seeds = read_seed_ranges(EXAMPLE_INPUT);
        let mapping_layers = Vec::from(
            [
                read_seed_to_soil_from_source(EXAMPLE_INPUT),
                read_soil_to_fertilizer_from_source(EXAMPLE_INPUT),
                read_fertilizer_to_water_from_source(EXAMPLE_INPUT),
                read_water_to_light_from_source(EXAMPLE_INPUT),
                read_light_to_temperature_from_source(EXAMPLE_INPUT),
                read_temperature_to_humidity_from_source(EXAMPLE_INPUT),
                read_humidity_to_location_from_source(EXAMPLE_INPUT)
            ]
        );

        let locations = map_all(seeds, &mapping_layers);
        let mut closest = i64::MAX;
        for location in locations {
            closest = min(closest, location.start);
        }
        assert_eq!(46, closest)
    }
}