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

    println!("{}", sums.last().expect("No results"))
}
