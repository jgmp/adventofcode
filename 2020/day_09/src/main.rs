use std::io::{self, Read};

fn part1_compute(numbers: &[i64], preamble: usize) -> i64 {
    for i in preamble + 1..numbers.len() {
        let mut found = false;
        for n in i - preamble..i - 1 {
            for x in n + 1..i {
                if numbers[i] == numbers[n] + numbers[x] {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return numbers[i];
        }
    }
    panic!("No solution found");
}

fn part1(numbers: &[i64], preamble: usize) {
    println!("part1: {}", part1_compute(numbers, preamble));
}

fn part2(numbers: &[i64], preamble: usize) {
    let target = part1_compute(numbers, preamble);
    let mut min: i64 = 0;
    let mut max: i64 = 0;
    for i in 0..numbers.len() {
        let mut sum = 0;
        min = numbers[i];
        for number in numbers[i..].into_iter() {
            if *number < min {
                min = *number;
            }
            if *number > max {
                max = *number;
            }
            sum += *number;
            if sum >= target {
                break;
            }
        }
        if sum == target {
            break;
        }
    }
    println!("part2: {} + {} = {}", min, max, min + max);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let preamble = 25;
    part1(&numbers, preamble);
    part2(&numbers, preamble);
}
