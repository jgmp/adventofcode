use regex::Regex;
use std::io::{self, Read};

struct Password {
    min: u32,
    max: u32,
    letter: char,
    secret: String,
}

fn part1(password_database: &Vec<Password>) {
    let mut count_correct = 0;
    for password in password_database {
        let mut count: u32 = 0;
        for c in password.secret.chars() {
            if c == password.letter {
                count += 1;
            }
        }
        if count >= password.min && count <= password.max {
            count_correct += 1;
        }
    }
    println!("{}", count_correct);
}

fn part2(password_database: &Vec<Password>) {
    let mut count_correct = 0;
    for password in password_database {
        let secret: Vec<char> = password.secret.chars().collect();
        let min: usize = password.min as usize - 1;
        let max: usize = password.max as usize - 1;
        if (secret[min] == password.letter && secret[max] != password.letter)
            || (secret[min] != password.letter && secret[max] == password.letter)
        {
            count_correct += 1;
        }
    }
    println!("{}", count_correct);
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let password_database: Vec<Password> = input
        .lines()
        .map(|line| {
            let cap = re.captures(&line).unwrap();
            Password {
                min: cap[1].parse::<u32>().unwrap(),
                max: cap[2].parse::<u32>().unwrap(),
                letter: cap[3].chars().next().unwrap(),
                secret: cap[4].to_string(),
            }
        })
        .collect();

    part1(&password_database);
    part2(&password_database);
}
