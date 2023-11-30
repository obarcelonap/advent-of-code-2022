use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn main() {
    let mut fully_contained_assignments = 0;
    let mut range_overlaps = 0;

    for line in read_to_string("inputs/day04.txt").unwrap().lines() {
        let (range1, range2) = line.split_once(",")
            .map(|(r1, r2)| {
                (
                    RangeInclusive::parse(r1, "-").expect("First range is split by '-'"),
                    RangeInclusive::parse(r2, "-").expect("Second range is split by '-'"),
                )
            }).expect("Ranges are split by ','");

        fully_contained_assignments +=
            if range1.contains_range(&range2) || range2.contains_range(&range1) {
                1
            } else {
                0
            };

        range_overlaps +=
            if range1.overlaps(&range2) {
                1
            } else {
                0
            };
    }

    println!("Part1: fully contained assignments {}", fully_contained_assignments);
    println!("Part2: range overlaps {}", range_overlaps);
}

trait RangeExt<T> {
    fn parse(str: &str, delimiter: &str) -> Option<RangeInclusive<T>>;
    fn contains_range(&self, range: &RangeInclusive<T>) -> bool;
    fn overlaps(&self, range: &RangeInclusive<T>) -> bool;
}

impl RangeExt<i32> for RangeInclusive<i32> {
    fn parse(str: &str, delimiter: &str) -> Option<RangeInclusive<i32>> {
        str.split_once(delimiter)
            .map(|(start, end)| RangeInclusive::new(
                start.parse::<i32>().expect("Lower bound is an int"),
                end.parse::<i32>().expect("Upper bound is an int"),
            ))
    }
    fn contains_range(&self, range: &RangeInclusive<i32>) -> bool {
        self.start() <= range.start() && self.end() >= range.end()
    }

    fn overlaps(&self, range: &RangeInclusive<i32>) -> bool {
        self.start() <= range.end() && range.start() <= self.end()
    }
}
