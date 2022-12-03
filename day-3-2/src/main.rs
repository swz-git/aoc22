use regex::Regex;

const INPUT: &str = include_str!("./input.txt");

fn check_dupes(x: &str, y: &str) -> Vec<char> {
    let mut dupes: Vec<char> = vec![];
    for ch in x.chars() {
        if y.contains(ch) {
            dupes.push(ch)
        }
    }
    return dupes;
}

fn calc_priority(ch: char) -> Option<u32> {
    if ch.is_lowercase() {
        // a-z -> 1-26
        return Some(ch as u32 - 96);
    }
    if ch.is_uppercase() {
        // A-Z -> 27-52
        return Some(ch as u32 - 64 + 26);
    }
    None
}

fn main() {
    let re = Regex::new(r"([a-zA-Z]+\n?){3}").unwrap();
    let groups: Vec<(&str, &str, &str)> = re
        .find_iter(INPUT)
        .map(|group| {
            let sacks: Vec<&str> = group.as_str().split("\n").collect();
            (sacks[0], sacks[1], sacks[2])
        })
        .collect();

    let mut item_priority_sum = 0;

    for group in groups {
        let dupe = check_dupes(
            &check_dupes(group.0, group.1)
                .iter()
                .cloned()
                .collect::<String>(),
            group.2,
        )[0];

        item_priority_sum += calc_priority(dupe).expect(&format!(
            "Couldn't convert char {} into priority number",
            dupe
        ))
    }

    println!("{}", item_priority_sum)
}
