const INPUT: &str = include_str!("./input.txt");

fn main() {
    let mut sums = INPUT
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| y.parse::<i32>().expect("Invalid input, non valid int"))
                .reduce(|a, b| a + b)
                .expect("Couldn't add upp ints")
        })
        .collect::<Vec<i32>>();

    sums.sort();
    sums.reverse();

    println!(
        "{}",
        sums[0..3]
            .iter()
            .map(|x| x.clone())
            .reduce(|a, b| a + b)
            .expect("Couldn't add upp ints")
    )
}
