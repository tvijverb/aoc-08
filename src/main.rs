use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use std::collections::HashMap;

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

pub fn lcm(vec: Vec<u64>) -> u64 {
    vec
        .iter()
        .fold(
            *vec.first().unwrap(),
            |ans, val| (val * ans) / (gcd(*val, ans))
        )
}

#[derive(Debug, Clone)]
pub enum Action {
    Left,
    Right
}

#[derive(Debug, Clone)]
pub struct ActionList {
    pub current_index: usize,
    pub actions: Vec<Action>
}

impl ActionList {
    pub fn new() -> ActionList {
        ActionList {
            current_index: 0,
            actions: Vec::new()
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn get_next_action(&mut self) -> &Action {
        if self.current_index >= self.actions.len() {
            self.current_index = 0;
        }
        let action = &self.actions[self.current_index];
        self.current_index += 1;
        action
    }
}

#[derive(Debug, Clone)]
pub struct StringMap {
    from: String,
    to_right: String,
    to_left: String,
}

impl StringMap {
    pub fn new(from: String, to_left: String, to_right: String) -> StringMap {
        StringMap {
            from,
            to_right,
            to_left
        }
    }
}

#[derive(Debug)]
pub struct StringMaps {
    pub maps: Vec<StringMap>
}

impl StringMaps {
    pub fn new() -> StringMaps {
        StringMaps {
            maps: Vec::new()
        }
    }

    pub fn add_map(&mut self, map: StringMap) {
        self.maps.push(map);
    }

    pub fn get_map(&self, from: &str) -> Option<&StringMap> {
        for map in &self.maps {
            if map.from == from {
                return Some(map);
            }
        }
        None
    }

    pub fn next_input(&self, input: &str, action: &Action) -> String {
        let map = self.get_map(input).unwrap();
        match action {
            Action::Left => map.to_left.clone(),
            Action::Right => map.to_right.clone()
        }
    }

    pub fn next_map(&self, input: &str, action: &Action) -> &StringMap {
        let map = self.get_map(input).unwrap();
        match action {
            Action::Left => self.get_map(map.to_left.as_str()).unwrap(),
            Action::Right => self.get_map(map.to_right.as_str()).unwrap()
        }
    }
}

fn parse_action_list(line: &str) -> ActionList {
    let mut action_list = ActionList::new();
    for c in line.chars() {
        match c {
            'L' => action_list.add_action(Action::Left),
            'R' => action_list.add_action(Action::Right),
            _ => panic!("Invalid action: {}", c)
        }
    }
    action_list
}

fn parse_string_map(line: &str) -> StringMap {
    // input example: "BRR = (LVC, FSJ)"
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap());
    let captures = RE.captures(line).unwrap();
    let from = captures.get(1).unwrap().as_str().to_string();
    let to_left = captures.get(2).unwrap().as_str().to_string();
    let to_right = captures.get(3).unwrap().as_str().to_string();
    StringMap::new(from, to_left, to_right)
}

fn get_start_strings(maps: &[StringMap]) -> Vec<&StringMap> {
    let mut start_strings = Vec::new();
    for map in maps {
        if map.from.ends_with("A") {
            start_strings.push(map);
        }
    }
    start_strings
}

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut action_list = ActionList::new();
    let mut string_maps = StringMaps::new();
    let mut output_string = String::from_str("AAA").unwrap();
    let mut iterations_part_1 = 0;
    let mut left_lookup = HashMap::new();
    let mut right_lookup = HashMap::new();
    let mut start_strings = Vec::new();


    for (idx, line) in reader.lines().enumerate() {
        let line = line?;

        if idx == 0 {
            action_list = parse_action_list(&line);
        }
        if idx > 1 {
            string_maps.add_map(parse_string_map(&line));
        }
    }

    while output_string.as_str() != "ZZZ" {
        let action = action_list.get_next_action();
        output_string = string_maps.next_input(output_string.as_str(), action);
        iterations_part_1 += 1;
    }
    println!("Iterations: {}", iterations_part_1);

    start_strings = get_start_strings(&string_maps.maps);

    for map in &string_maps.maps {
        left_lookup.insert(map.from.as_str(), map.to_left.as_str());
        right_lookup.insert(map.from.as_str(), map.to_right.as_str());
    }

    let steps_per_start_string = start_strings.iter().map(|map| {
        let mut steps: u64 = 0;
        let mut current_string = map.from.as_str();
        let mut action_list_copy = action_list.clone();
        while !current_string.ends_with("Z") {
            let action = action_list_copy.get_next_action();
            current_string = match action {
                Action::Left => left_lookup.get(current_string).unwrap(),
                Action::Right => right_lookup.get(current_string).unwrap()
            };
            steps += 1;
        }
        steps
    }).collect::<Vec<u64>>();

    println!("Steps per start string: {:?}", steps_per_start_string);
    println!("LCM: {}", lcm(steps_per_start_string));
    

    Ok(())
}