use std::io::{self, Error, ErrorKind, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
struct Seat {
    row: u8,
    column: u8,
    id: u16,
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = Pos { min: 0, max: 127 };
        let mut column = Pos { min: 0, max: 7 };

        for c in s.chars() {
            match c {
                'F' => row.lower_half(),
                'B' => row.upper_half(),
                'L' => column.lower_half(),
                'R' => column.upper_half(),
                _ => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Unexpected character found: {}", c),
                    ))
                }
            }
        }

        if row.min != row.max || column.min != column.max {
            return Err(Error::new(ErrorKind::Other, "Solution not unique"));
        }

        let id: u16 = row.min as u16 * 8 + column.min as u16;

        Ok(Seat {
            row: row.min,
            column: column.min,
            id: id,
        })
    }
}

struct Pos {
    min: u8,
    max: u8,
}

impl Pos {
    fn lower_half(&mut self) {
        self.max = (self.max - self.min) / 2 + self.min;
    }

    fn upper_half(&mut self) {
        self.min = (self.max - self.min + 1) / 2 + self.min;
    }
}

fn part1(seats: &[Seat]) {
    let mut max = 0;
    for seat in seats {
        if seat.id > max {
            max = seat.id
        }
    }
    println!("part1: {}", max);
}

fn part2(seats: &mut Vec<Seat>) {
    seats.sort_by(|a, b| a.id.cmp(&b.id));

    let mut prev_id = 0;
    for seat in seats {
        if prev_id != 0 && seat.id == prev_id + 2 {
            println!("part2: {}", seat.id - 1);
            break;
        }
        prev_id = seat.id;
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut seats = input
        .lines()
        .map(|line| line.parse::<Seat>())
        .collect::<Result<Vec<Seat>, _>>()?;

    part1(&seats);
    part2(&mut seats);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Seat {
                row: 70,
                column: 7,
                id: 567
            },
            "BFFFBBFRRR".parse::<Seat>().unwrap()
        );
        assert_eq!(
            Seat {
                row: 14,
                column: 7,
                id: 119
            },
            "FFFBBBFRRR".parse::<Seat>().unwrap()
        );
        assert_eq!(
            Seat {
                row: 102,
                column: 4,
                id: 820
            },
            "BBFFBBFRLL".parse::<Seat>().unwrap()
        );
    }
}
