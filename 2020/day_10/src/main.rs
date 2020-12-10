use std::io::{self, Read};

fn part1(adapters: &[u32]) {
    let mut diff: [u32; 3] = [0; 3];
    let mut prev = 0;
    for i in &adapters[1..] {
        diff[(i - prev - 1) as usize] += 1;
        prev = *i;
    }
    println!("part1: {}", diff[0] * diff[2]);
}

fn part2(adapters: &[u32]) {
    let mut arrangements: Vec<u128> = vec![0; adapters.len()];
    arrangements[0] = 1;
    for (i, adapter) in adapters.iter().enumerate() {
        if i > 0 && adapter - adapters[i - 1] <= 3 {
            arrangements[i] += arrangements[i - 1];
        }
        if i > 1 && adapter - adapters[i - 2] <= 3 {
            arrangements[i] += arrangements[i - 2];
        }
        if i > 2 && adapter - adapters[i - 3] <= 3 {
            arrangements[i] += arrangements[i - 3];
        }
    }
    println!("part2: {}", arrangements.last().unwrap());
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut adapters: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    part1(&adapters);
    part2(&adapters);
}
