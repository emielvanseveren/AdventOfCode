use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file_to_string(filename: &str) -> String {
    let mut file = File::open(filename).expect("file not found");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Could not read file");
    buffer
}

fn parse_input(input: &str) -> (HashMap<(u8, u8), u8>, HashMap<(u8, u8), usize>) {
    let lines = input.lines().collect_vec();
    let rules: HashMap<(u8, u8), u8> = lines[2..]
        .iter()
        .filter_map(|l| l.split(" -> ").collect_tuple::<(&str, &str)>())
        .map(|(lhs, rhs)| ((lhs.as_bytes()[0], lhs.as_bytes()[1]), rhs.as_bytes()[0]))
        .collect();
    let template = format!("a{}a", &lines[0]).bytes().tuple_windows().counts();
    (rules, template)
}

fn polymer_grow(
    chain: HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut new_chain: HashMap<(u8, u8), usize> = HashMap::new();
    for ((c1, c2), count) in chain {
        if let Some(&c_mid) = rules.get(&(c1, c2)) {
            *new_chain.entry((c1, c_mid)).or_insert(0) += count;
            *new_chain.entry((c_mid, c2)).or_insert(0) += count;
        } else {
            *new_chain.entry((c1, c2)).or_insert(0) += count;
        }
    }
    new_chain
}

fn calculate(
    template: HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
    steps: u32,
) -> i128 {
    let final_chain = (0..steps).fold(template, |chain, _| polymer_grow(chain, rules));

    let mut letter_counts = HashMap::new();
    for ((c1, c2), count) in final_chain {
        *letter_counts.entry(c1).or_insert(0) += count;
        *letter_counts.entry(c2).or_insert(0) += count;
    }
    letter_counts.remove(&b'a');
    if let MinMaxResult::MinMax(min, max) = letter_counts.values().minmax() {
        ((*max - *min) / 2) as i128
    } else {
        0
    }
}

fn main() {
    let (rules, chain) = parse_input(&read_file_to_string("input.txt"));
    println!("Star one: {}", calculate(chain, &rules, 10));

    let (rules, chain) = parse_input(&read_file_to_string("input.txt"));
    println!("Star two: {}", calculate(chain, &rules, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let (rules, chain) = parse_input(&String::from(
            "NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C",
        ));
    }

    #[test]
    fn test_star_one() {
        let (rules, chain) = parse_input(&String::from(
            "NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C",
        ));
        assert_eq!(calculate(chain, &rules, 10), 1588);
    }
}
