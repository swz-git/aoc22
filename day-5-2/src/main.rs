const INPUT: &str = include_str!("./input.txt");

struct Instruction {
    crates: i32,
    from: i32,
    to: i32,
}

fn process_stacks(raw: &str) -> Vec<Vec<char>> {
    let mut line_split: Vec<&str> = raw.split("\n").collect();
    line_split.pop();
    let length = line_split[0].len();

    let mut char_list: Vec<Vec<char>> = vec![];
    for i in 0..(length + 1) / 4 {
        let mut chars: Vec<char> = vec![];
        for line in line_split.iter() {
            let char = line.chars().nth(i * 4 + 1).unwrap();
            if char != ' ' {
                chars.push(char)
            }
        }
        chars.reverse();
        char_list.push(chars)
    }

    char_list
}

fn main() {
    let split: Vec<&str> = INPUT.split("\n\n").collect();

    let mut stacks = process_stacks(split[0]);
    let instructions: Vec<Instruction> = split[1]
        .split("\n")
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            Instruction {
                crates: words[1].parse().unwrap(),
                from: words[3].parse().unwrap(),
                to: words[5].parse().unwrap(),
            }
        })
        .collect();

    for instruction in instructions {
        let mut popped: Vec<char> = vec![];
        for _ in 0..instruction.crates {
            popped.insert(0, stacks[(instruction.from - 1) as usize].pop().unwrap());
        }
        for pop in popped.iter() {
            stacks[(instruction.to - 1) as usize].push(*pop);
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
    println!()
}
