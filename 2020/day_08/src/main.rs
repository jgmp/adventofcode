use std::io::{self, Error, ErrorKind, Read};
use std::iter::Iterator;

#[derive(Clone)]
enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn part1(program: &[Operation]) {
    match compute(program) {
        Err(acc) => println!("part1: {}", acc),
        _ => println!("no solution found"),
    }
}

fn compute(program: &[Operation]) -> Result<i32, Error> {
    let mut seen = vec![false; program.len()];
    let mut line = 0;
    let mut acc = 0;
    while line < program.len() {
        if seen[line] {
            return Err(Error::new(ErrorKind::Other, format!("{}", acc)));
        }
        seen[line] = true;
        match program[line] {
            Operation::Acc(value) => {
                line += 1;
                acc += value;
            }
            Operation::Jmp(value) => line = (line as i32 + value) as usize,
            Operation::Nop(_) => line += 1,
        }
    }
    Ok(acc)
}

fn mutate(program: &[Operation]) -> impl Iterator<Item = Vec<Operation>> {
    let mut line = 0;
    let program_orig = program.to_vec();
    std::iter::from_fn(move || {
        let mut mutation: Vec<Operation> = program_orig.clone();
        let mut found = false;
        for l in line..mutation.len() {
            found = match mutation[l] {
                Operation::Jmp(value) => {
                    mutation[l] = Operation::Nop(value);
                    true
                }
                Operation::Nop(value) => {
                    mutation[l] = Operation::Jmp(value);
                    true
                }
                _ => false,
            };
            if found {
                line = l + 1;
                break;
            }
        }
        if found {
            Some(mutation)
        } else {
            None
        }
    })
}

fn part2(program: &[Operation]) {
    let result;
    let mut program_iter = mutate(program);
    loop {
        let mutation = program_iter.next().unwrap();
        match compute(&mutation) {
            Ok(acc) => {
                result = acc;
                break;
            }
            _ => (),
        }
    }
    println!("part2: {}", result);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let program = input
        .lines()
        .map(|line| match &line[..3] {
            "acc" => Operation::Acc(line[4..].parse::<i32>().unwrap()),
            "jmp" => Operation::Jmp(line[4..].parse::<i32>().unwrap()),
            "nop" => Operation::Nop(line[4..].parse::<i32>().unwrap()),
            _ => Operation::Nop(0),
        })
        .collect::<Vec<Operation>>();

    part1(&program);
    part2(&program);
}
