#![no_std]

use aoc2021::*;
use heapless::Vec;

fn most_common(data: &[&str]) -> Vec<char, 12> {
    let mut buf: Vec<u16, 12> = Vec::new();
    let len = data[0].len();
    let mut count = 0;

    for _ in 0..len {
        buf.push(0).unwrap();
    }

    for line in data {
        count += 1;
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                buf[i] += 1;
            }
        }
    }

    buf.into_iter()
        .map(|ones| {
            let zeroes = count - ones;
            if ones >= zeroes {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn part1(data: &[&str]) -> u64 {
    let common = most_common(data);
    let len = common.len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, x) in common.iter().enumerate() {
        gamma += if *x == '1' { 1 << (len - i - 1) } else { 0 };
        epsilon += if *x == '0' { 1 << (len - i - 1) } else { 0 };
    }

    gamma * epsilon
}

fn filter(mut data: Vec<&str, 1000>, most: bool) -> &str {
    for pos in 0..data[0].len() {
        let common = most_common(&data);
        let keep = if most {
            common[pos]
        } else if common[pos] == '1' {
            '0'
        } else {
            '1'
        };

        data = data
            .into_iter()
            .filter(|s| s.chars().nth(pos).unwrap() == keep)
            .collect();

        if data.len() == 1 {
            break;
        }
    }

    data[0]
}

fn part2(data: Vec<&str, 1000>) -> u64 {
    let oxygen = u64::from_str_radix(filter(data.clone(), true), 2).unwrap();
    let co2 = u64::from_str_radix(filter(data.clone(), false), 2).unwrap();

    oxygen * co2
}

#[test]
fn test() {
    let data: Vec<&str, 1000> = input!().lines().collect();
    assert_eq!(part1(&data), 198);
    assert_eq!(part2(data), 230);
}

fn main() {
    let data: Vec<&str, 1000> = input!().lines().collect();
    libc_println!("part 1: {}", part1(&data));
    libc_println!("part 2: {}", part2(data));
}
