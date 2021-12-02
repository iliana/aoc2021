#![no_std]

use aoc2021::*;
pub use itertools::Itertools;

fn part1(data: impl Iterator<Item = u16>) -> usize {
    data.tuple_windows().filter(|(a, b)| a < b).count()
}

fn part2(data: impl Iterator<Item = u16>) -> usize {
    data.tuple_windows().filter(|(a, _, _, d)| a < d).count()
}

#[test]
fn test() {
    assert_eq!(part1(input!(u16)), 7);
    assert_eq!(part2(input!(u16)), 5);
}

fn main() {
    libc_println!("part 1: {}", part1(input!(u16)));
    libc_println!("part 2: {}", part2(input!(u16)));
}
