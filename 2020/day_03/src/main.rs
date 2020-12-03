use std::io::{self, Error, ErrorKind, Read};
use std::str::FromStr;

struct Grid {
    height: usize,
    width: usize,
    trees: Vec<Vec<bool>>,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let trees = string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        _ => Err(Error::new(
                            ErrorKind::Other,
                            format!("Unexpected character found: {}", c),
                        )),
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<bool>>, _>>()?;
        let height = trees.len();
        let width = trees[0].len();
        Ok(Grid {
            height: height,
            width: width,
            trees: trees,
        })
    }
}

fn nb_trees_on_slope(grid: &Grid, right: usize, down: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees_count = 0;

    while y < grid.height {
        if grid.trees[y][x] {
            trees_count += 1;
        }
        x = (x + right) % grid.width;
        y += down;
    }

    return trees_count;
}

fn part1(grid: &Grid) {
    println!("{}", nb_trees_on_slope(&grid, 3, 1))
}

fn part2(grid: &Grid) {
    let result: u64 = nb_trees_on_slope(&grid, 1, 1)
        * nb_trees_on_slope(&grid, 3, 1)
        * nb_trees_on_slope(&grid, 5, 1)
        * nb_trees_on_slope(&grid, 7, 1)
        * nb_trees_on_slope(&grid, 1, 2);

    println!("{}", result);
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let grid: Grid = input.parse()?;

    part1(&grid);
    part2(&grid);

    Ok(())
}
