#![no_std]

use aoc2021::*;
use core::str::FromStr;
use itertools::Itertools;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Command, &'static str> {
        let (command, amount) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or("too many spaces")?;
        let amount = amount.parse().map_err(|_| "couldn't parse u8")?;
        Ok(match command {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => return Err("invalid command"),
        })
    }
}

fn part1(data: impl Iterator<Item = Command>) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in data {
        match command {
            Command::Forward(amount) => {
                horizontal += amount;
            }
            Command::Down(amount) => {
                depth += amount;
            }
            Command::Up(amount) => {
                depth -= amount;
            }
        }
    }

    horizontal * depth
}

fn part2(data: impl Iterator<Item = Command>) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in data {
        match command {
            Command::Down(amount) => {
                aim += amount;
            }
            Command::Up(amount) => {
                aim -= amount;
            }
            Command::Forward(amount) => {
                horizontal += amount;
                depth += aim * amount;
            }
        }
    }

    horizontal * depth
}

#[test]
fn test() {
    assert_eq!(part1(input!(Command)), 150);
    assert_eq!(part2(input!(Command)), 900);
}

fn main() {
    libc_println!("part 1: {}", part1(input!(Command)));
    libc_println!("part 2: {}", part2(input!(Command)));
}
