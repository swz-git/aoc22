const INPUT: &str = include_str!("./input.txt");

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
        if (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1)
            || (pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1)
        {
            count += 1
        }
    }
    println!("{}", count)
}
