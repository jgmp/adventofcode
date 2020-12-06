use std::collections::HashMap;
use std::io::{self, Read};

fn contains_all_keys(passport: &HashMap<&str, &str>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn is_valid_number(n: &Option<&&str>, min: i32, max: i32) -> bool {
    match n.map(|x| x.parse::<i32>()) {
        Some(Ok(x)) if x >= min && x <= max => true,
        _ => false,
    }
}

fn is_valid_hgt(hgt: &Option<&&str>) -> bool {
    match hgt {
        Some(height) if height.len() == 5 && &height[height.len() - 2..] == "cm" => {
            is_valid_number(&Some(&&height[..height.len() - 2]), 150, 193)
        }
        Some(height) if height.len() == 4 && &height[height.len() - 2..] == "in" => {
            is_valid_number(&Some(&&height[..height.len() - 2]), 59, 76)
        }
        _ => false,
    }
}

fn is_valid_hcl(hcl: &Option<&&str>) -> bool {
    match hcl {
        Some(color)
            if color.len() == 7
                && color.starts_with("#")
                && i64::from_str_radix(&color[1..], 16).is_ok() =>
        {
            true
        }
        _ => false,
    }
}

fn is_valid_ecl(ecl: &Option<&&str>) -> bool {
    match ecl {
        Some(&"amb") | Some(&"blu") | Some(&"brn") | Some(&"gry") | Some(&"grn") | Some(&"hzl")
        | Some(&"oth") => true,
        _ => false,
    }
}

fn is_valid_pid(pid: &Option<&&str>) -> bool {
    match pid {
        Some(i) if i.len() == 9 && i.parse::<u32>().is_ok() => true,
        _ => false,
    }
}

fn is_valid_byr(byr: &Option<&&str>) -> bool {
    is_valid_number(&byr, 1920, 2002)
}

fn is_valid_iyr(iyr: &Option<&&str>) -> bool {
    is_valid_number(&iyr, 2010, 2020)
}

fn is_valid_eyr(eyr: &Option<&&str>) -> bool {
    is_valid_number(&eyr, 2020, 2030)
}

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    is_valid_byr(&passport.get(&"byr"))
        && is_valid_iyr(&passport.get(&"iyr"))
        && is_valid_eyr(&passport.get(&"eyr"))
        && is_valid_hgt(&passport.get(&"hgt"))
        && is_valid_hcl(&passport.get(&"hcl"))
        && is_valid_ecl(&passport.get(&"ecl"))
        && is_valid_pid(&passport.get(&"pid"))
}

fn part1(passports: &[HashMap<&str, &str>]) {
    let count = passports
        .iter()
        .fold(0, |acc, x| if contains_all_keys(x) { acc + 1 } else { acc });
    println!("part1: {}", count);
}

fn part2(passports: &[HashMap<&str, &str>]) {
    let count = passports
        .iter()
        .fold(0, |acc, x| if is_valid_passport(x) { acc + 1 } else { acc });
    println!("part2: {}", count);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let passports: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|group| {
            group
                .split(|c| c == ' ' || c == '\n')
                .filter(|i| i.len() > 3)
                .map(|i| (&i[..3], &i[4..]))
                .collect::<HashMap<&str, &str>>()
        })
        .collect();

    part1(&passports);
    part2(&passports);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr() {
        assert_eq!(true, is_valid_byr(&Some(&&"2002")));
        assert_eq!(false, is_valid_byr(&Some(&&"2003")));
        assert_eq!(false, is_valid_byr(&Some(&&"1919")));
        assert_eq!(true, is_valid_byr(&Some(&&"1920")));
        assert_eq!(false, is_valid_byr(&Some(&&"abcd")));
    }

    #[test]
    fn test_iyr() {
        assert_eq!(true, is_valid_iyr(&Some(&&"2020")));
        assert_eq!(false, is_valid_iyr(&Some(&&"2021")));
        assert_eq!(false, is_valid_iyr(&Some(&&"2009")));
        assert_eq!(true, is_valid_iyr(&Some(&&"2010")));
        assert_eq!(false, is_valid_iyr(&Some(&&"abcd")));
    }

    #[test]
    fn test_eyr() {
        assert_eq!(true, is_valid_eyr(&Some(&&"2030")));
        assert_eq!(false, is_valid_eyr(&Some(&&"2031")));
        assert_eq!(false, is_valid_eyr(&Some(&&"2019")));
        assert_eq!(true, is_valid_eyr(&Some(&&"2020")));
        assert_eq!(false, is_valid_eyr(&Some(&&"abcd")));
    }

    #[test]
    fn test_hgt() {
        assert_eq!(true, is_valid_hgt(&Some(&&"60in")));
        assert_eq!(true, is_valid_hgt(&Some(&&"190cm")));
        assert_eq!(false, is_valid_hgt(&Some(&&"190in")));
        assert_eq!(false, is_valid_hgt(&Some(&&"190")));
        assert_eq!(false, is_valid_hgt(&Some(&&"2100cm")));
    }

    #[test]
    fn test_hcl() {
        assert_eq!(true, is_valid_hcl(&Some(&&"#123abc")));
        assert_eq!(false, is_valid_hcl(&Some(&&"#123abz")));
        assert_eq!(false, is_valid_hcl(&Some(&&"123abc")));
        assert_eq!(false, is_valid_hcl(&Some(&&"#123abcd")));
        assert_eq!(false, is_valid_hcl(&Some(&&"123abcd")));
    }

    #[test]
    fn test_ecl() {
        assert_eq!(true, is_valid_ecl(&Some(&&"brn")));
        assert_eq!(false, is_valid_ecl(&Some(&&"wat")));
    }

    #[test]
    fn test_pid() {
        assert_eq!(true, is_valid_pid(&Some(&&"000000001")));
        assert_eq!(false, is_valid_pid(&Some(&&"00000000a")));
        assert_eq!(false, is_valid_pid(&Some(&&"0123456789")));
    }
}
