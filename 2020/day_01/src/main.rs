use std::io::{self, Read};

fn part1(expense_report: &Vec<i32>) {
    let len = expense_report.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            if expense_report[i] + expense_report[j] == 2020 {
                println!("part1: {}", expense_report[i] * expense_report[j]);
                return;
            }
        }
    }
}

fn part2(expense_report: &Vec<i32>) {
    let len = expense_report.len();
    for i in 0..len - 2 {
        for j in i + 1..len - 1 {
            for k in j + 1..len {
                if expense_report[i] + expense_report[j] + expense_report[k] == 2020 {
                    println!(
                        "part2: {}",
                        expense_report[i] * expense_report[j] * expense_report[k]
                    );
                    return;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let expense_report: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    part1(&expense_report);
    part2(&expense_report);
}
