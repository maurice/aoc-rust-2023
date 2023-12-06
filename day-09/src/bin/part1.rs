fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn parse_input<'a>(input: &'a str) -> Vec<Vec<isize>> {
    input
        .trim()
        .lines()
        .map(|line| line.split(" ").filter_map(|s| s.parse().ok()).collect())
        .collect()
}

fn get_diffs(vec: &Vec<isize>) -> Vec<isize> {
    match vec.len() {
        1 => vec![0],
        _ => (1..vec.len())
            .map(|n| vec.get(n).unwrap() - vec.get(n - 1).unwrap())
            .collect(),
    }
}

fn get_next_in_sequence(vec: &Vec<isize>) -> isize {
    // if all the numbers are the same...
    let num = vec.get(0).unwrap();
    if vec.iter().skip(1).all(|other| num == other) {
        // then it's that number
        return *num;
    }

    // else crunch another set of deltas and add the result to the last number
    let diffs = get_diffs(vec);
    let to_add = get_next_in_sequence(&diffs);
    return vec.get(vec.len() - 1).unwrap() + to_add;
}

fn get_answer(input: &str) -> isize {
    let nums = parse_input(input);
    nums.iter().map(get_next_in_sequence).sum()
}

#[cfg(test)]
mod tests {
    use crate::get_answer;

    #[test]
    fn example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(get_answer(input), 114);
    }
}
