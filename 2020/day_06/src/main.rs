use std::collections::HashSet;
use std::io::{self, Read};

fn part1(input: &str) {
    let mut sum = 0;
    input.split("\n\n").for_each(|group| {
        let mut answers = HashSet::new();
        for c in group.chars() {
            if c == '\n' {
                continue;
            }
            answers.insert(c);
        }
        sum += answers.len();
    });
    println!("part1: {}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    input.trim().split("\n\n").for_each(|group| {
        let mut answers: [u32; 26] = [0; 26];
        let mut group_size = 0;
        group.split("\n").for_each(|form| {
            group_size += 1;
            for c in form.chars() {
                answers[c as usize - 'a' as usize] += 1;
            }
        });
        for answer in answers.iter() {
            if *answer == group_size {
                sum += 1;
            }
        }
    });
    println!("part2: {}", sum);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
