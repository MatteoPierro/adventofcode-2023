#[derive(Debug, Clone, Eq, PartialEq)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn build_from(line: &str) -> Self {
        let parts: Vec<_> = line.split(" ").map(|v| v.parse::<usize>().unwrap()).collect();
        Range { source: parts[1], destination: parts[0], length: parts[2] }
    }

    fn include(&self, n: usize) -> bool {
        n >= self.source && n < self.source + self.length
    }

    fn get(&self, n: usize) -> usize {
        if !self.include(n) {
            panic!("Number out of range!")
        }

        let step = n - self.source;
        self.destination + step
    }

    fn reverse(&self) -> Self {
        Self { destination: self.source, source: self.destination, length: self.length }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

impl Map {
    fn map_number(&self, number: usize) -> usize {
        if let Some(range) = self.ranges.iter().find(|r| r.include(number)) {
            range.get(number)
        } else {
            number
        }
    }

    fn reverse(&self) -> Map {
        let mut ranges = self.ranges.clone();
        ranges.reverse();
        let ranges = ranges.iter().map(|r| r.reverse()).collect();
        Map {
            destination: self.source.clone(),
            source: self.destination.clone(),
            ranges
        }
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn calculate_location(&self, seed: usize) -> usize {
        self.maps.iter()
            .fold(seed, |curr, map| map.map_number(curr))
    }

    fn lowest_location(&self) -> usize {
        self.seeds.iter()
            .map(|s| self.calculate_location(*s))
            .min()
            .unwrap_or(0)
    }
}

fn parse_almanac(input: Vec<String>) -> Almanac {
    let conversions = [
        ("seed", "soil"),
        ("soil", "fertilizer"),
        ("fertilizer", "water"),
        ("water", "light"),
        ("light", "temperature"),
        ("temperature", "humidity"),
        ("humidity", "location")
    ];

    let seeds: Vec<_> = input[0].split(": ").collect::<Vec<_>>()[1]
        .split(" ")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut maps: Vec<Map> = vec![];
    let mut index = 3;
    for conversion in conversions {
        let mut ranges = vec![];

        while index < input.len() && !input[index].is_empty() {
            ranges.push(Range::build_from(&input[index]));
            index += 1;
        }

        maps.push(Map {
            source: conversion.0.to_string(),
            destination: conversion.1.to_string(),
            ranges,
        });

        index += 2; // skip blank line and map line
    }

    Almanac { seeds, maps }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SeedRange {
    start: usize,
    end: usize
}

impl SeedRange {
    fn build_from(start: usize, length: usize) -> Self {
        SeedRange { start, end: start + length - 1 }
    }

    fn is_including(&self, seed: usize) -> bool{
        self.start <= seed && seed <= self.end
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day5::*;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_second_part() {
        let input = read_input_file("input_day05.txt");
        let almanac = parse_almanac(read_lines(&input));
        let ranges = [
            SeedRange::build_from(2880930400,17599561),
            SeedRange::build_from(549922357,200746426),
            SeedRange::build_from(1378552684,43534336),
            SeedRange::build_from(155057073,56546377),
            SeedRange::build_from(824205101,378503603),
            SeedRange::build_from(1678376802,130912435),
            SeedRange::build_from(2685513694,137778160),
            SeedRange::build_from(2492361384,188575752),
            SeedRange::build_from(3139914842,1092214826),
            SeedRange::build_from(2989476473,58874625)
        ];

        let min_location = calculate_min_location(ranges.to_vec(), almanac);
        println!("min location {:?}", min_location); // 78775051
    }

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day05.txt");

        let almanac = parse_almanac(read_lines(&input));

        assert_eq!(227653707, almanac.lowest_location());
    }

    #[test]
    fn it_calculates_reverse_map() {
        let input = indoc! {"
        seeds: 79 14 55 13

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
        56 93 4"};

        let ranges = [
            SeedRange::build_from(79,14),
            SeedRange::build_from(55,13)
        ];

        let almanac = parse_almanac(read_lines(input));

        let min_location = calculate_min_location(ranges.to_vec(), almanac);
        assert_eq!(46, min_location);
    }

    fn calculate_min_location(ranges: Vec<SeedRange>, almanac: Almanac) -> usize {
        let mut reversed = almanac.maps.clone();
        reversed.reverse();
        let reversed: Vec<_> = reversed.iter().map(|m| m.reverse()).collect();

        let location_map = reversed.first().unwrap();

        // println!("{:?}", reversed);
        // println!("{:?}", location_map);

        let mut lowest_location_range: Range = location_map.ranges.iter().min_by(|r1, r2| r1.source.cmp(&r2.source)).unwrap().clone();
        if lowest_location_range.source != 0 {
            lowest_location_range = Range { source: 0, destination: lowest_location_range.source - 1, length: lowest_location_range.source };
        }

        // println!("{:?}", lowest_location_range);
        let mut min_location: Option<usize> = None;
        for location in lowest_location_range.source..=lowest_location_range.destination {
            let seed = reversed.iter().fold(location, |curr, map| map.map_number(curr));
            if ranges.iter().any(|r| r.is_including(seed)) {
                min_location = Some(location);
                break;
            }
            // println!("{:?} location {:?}", seed, location);
        }
        min_location.unwrap()
    }

    #[test]
    fn it_calculates_location() {
        let input = indoc! {"
        seeds: 79 14 55 13

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
        56 93 4"};

        let almanac = parse_almanac(read_lines(input));

        assert_eq!(82, almanac.calculate_location(79));
        assert_eq!(35, almanac.lowest_location());
    }

    #[test]
    fn it_parses_input() {
        let input = indoc! {"
        seeds: 79 14 55 13

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
        56 93 4"};

        let almanac = parse_almanac(read_lines(input));

        assert_eq!(vec![79, 14, 55, 13], almanac.seeds);
        assert_eq!(Map {
            source: "seed".to_string(),
            destination: "soil".to_string(),
            ranges: vec![
                Range { source: 98, destination: 50, length: 2 },
                Range { source: 50, destination: 52, length: 48 },
            ],
        }, almanac.maps[0]);
        assert_eq!(Map {
            source: "humidity".to_string(),
            destination: "location".to_string(),
            ranges: vec![
                Range { source: 56, destination: 60, length: 37 },
                Range { source: 93, destination: 56, length: 4 },
            ],
        }, almanac.maps.last().unwrap().clone());
    }

    #[test]
    fn it_maps_a_value() {
        let map = Map {
            source: "seed".to_string(),
            destination: "soil".to_string(),
            ranges: vec![
                Range { source: 98, destination: 50, length: 2 },
                Range { source: 50, destination: 52, length: 48 },
            ],
        };

        assert_eq!(81, map.map_number(79));
        assert_eq!(14, map.map_number(14));
        assert_eq!(57, map.map_number(55));
        assert_eq!(13, map.map_number(13));
    }

    #[test]
    fn it_parses_a_range() {
        assert_eq!(
            Range { source: 98, destination: 50, length: 2 },
            Range::build_from("50 98 2")
        );
    }

    #[test]
    fn it_finds_if_number_in_range() {
        let range = Range { source: 98, destination: 50, length: 2 };
        assert_eq!(false, range.include(97));
        assert_eq!(true, range.include(98));
        assert_eq!(true, range.include(99));
        assert_eq!(false, range.include(100));
    }

    #[test]
    fn it_return_corresponding_number_if_in_range() {
        let range = Range { source: 98, destination: 50, length: 2 };
        assert_eq!(50, range.get(98));
        assert_eq!(51, range.get(99));
    }
}