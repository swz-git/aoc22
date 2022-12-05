const INPUT: &str = include_str!("./input.txt");

fn check_dupes(x: Vec<i32>, y: Vec<i32>) -> bool {
    for ch in x {
        if y.contains(&ch) {
            return true;
        }
    }
    false
}

fn main() {
    let pairs: Vec<((i32, i32), (i32, i32))> = INPUT
        .split("\n")
        .map(|pair| {
            let ranges: Vec<(i32, i32)> = pair
                .split(",")
                .map(|range| {
                    let range: Vec<&str> = range.split("-").collect();
                    (
                        range[0].parse::<i32>().unwrap(),
                        range[1].parse::<i32>().unwrap(),
                    )
                })
                .collect();
            (ranges[0], ranges[1])
        })
        .collect();

    let mut count = 0;
    for pair in pairs {
        // fuck it, i'm doing this the brute-force way
        let pair_ranges: (Vec<i32>, Vec<i32>) = (
            (pair.0 .0..pair.0 .1 + 1).collect(),
            (pair.1 .0..pair.1 .1 + 1).collect(),
        );
        if check_dupes(pair_ranges.0, pair_ranges.1) {
            count += 1
        }
    }
    println!("{}", count)
}
