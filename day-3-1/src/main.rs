const INPUT: &str = include_str!("./input.txt");

fn find_first_dupe(x: &str, y: &str) -> Option<char> {
    for ch in x.chars() {
        if y.contains(ch) {
            return Some(ch);
        }
    }
    None
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
    let rucksacks: Vec<(&str, &str)> = INPUT
        .split("\n")
        .map(|sack| {
            let len = sack.len();
            if ((len as f32) / 2f32) % 1f32 > 0f32 {
                panic!("{}", "a")
            }
            (&sack[0..len / 2], &sack[len / 2..len])
        })
        .collect();

    let mut item_priority_sum = 0;

    for (i, rucksack) in rucksacks.iter().enumerate() {
        let dupe = find_first_dupe(rucksack.0, rucksack.1)
            .expect(&format!("Couldn't find dupe in rucksack {}", i));

        let priority = calc_priority(dupe).expect(&format!(
            "Couldn't convert char {} into priority number",
            dupe
        ));

        item_priority_sum += priority
    }

    println!("{}", item_priority_sum)
}
