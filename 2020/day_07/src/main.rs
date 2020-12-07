use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Error, Read};
use std::str::FromStr;

struct Content {
    color: String,
    count: u32,
}

struct Rule {
    color: String,
    content: Vec<Content>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_BAG: Regex = Regex::new(r"^(.+?) bags contain").unwrap();
            static ref RE_CONTENT: Regex = Regex::new(r"(\d+) (.+?) bag").unwrap();
        }

        let bag_color = &RE_BAG.captures(&s).unwrap()[1];
        let mut content = Vec::new();
        for cap in RE_CONTENT.captures_iter(&s) {
            content.push(Content {
                count: cap[1].parse::<u32>().unwrap(),
                color: cap[2].to_string(),
            });
        }

        Ok(Rule {
            color: bag_color.to_string(),
            content: content,
        })
    }
}

fn search_count(target: &str, rules: &[Rule], seen: &mut Vec<String>) -> u32 {
    let mut count = 0;

    for rule in rules {
        for content in &rule.content {
            if content.color == target && !seen.contains(&rule.color) {
                seen.push(rule.color.clone());
                count += 1;
                count += search_count(&rule.color, rules, seen);
            }
        }
    }

    count
}

fn part1(rules: &[Rule]) {
    let target = "shiny gold";
    let mut seen = Vec::new();
    println!("part1: {}", search_count(&target, rules, &mut seen));
}

fn find_rule<'a, 'b>(target: &'b str, rules: &'a [Rule]) -> &'a Rule {
    rules.iter().find(|rule| rule.color == target).unwrap()
}

fn count_bags(target: &str, rules: &[Rule]) -> u32 {
    let mut count = 0;
    let bag = find_rule(target, rules);

    for content in &bag.content {
        count += count_bags(&content.color, rules) * content.count + content.count;
    }

    count
}

fn part2(rules: &[Rule]) {
    println!("part2: {}", count_bags("shiny gold", rules));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let rules: Vec<Rule> = input
        .lines()
        .map(|line| line.parse::<Rule>())
        .collect::<Result<Vec<Rule>, _>>()
        .unwrap();

    part1(&rules);
    part2(&rules);
}
