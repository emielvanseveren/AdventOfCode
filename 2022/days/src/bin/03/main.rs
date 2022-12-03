pub fn priority(item: u8) -> u32 {
    (if item >= b'a' {
        item + 1 - b'a'
    } else {
        item + 27 - b'A'
    }) as u32
}

fn part_1(input: &[u8]) -> u32 {
           input
            .split(|b| *b == b'\n')
            .map(|l| {
                let mid = l.len() / 2;
                let left = &l[..mid];
                let right = &l[mid..];
                let dup = left.iter().find(|x| right.contains(x)).expect("to have char in common");
                u32::from((dup - 38) % 58)
            })
            .sum::<u32>()
            .into()
}

fn part_2() {
    unimplemented!();
}

fn main() {

    let input = include_bytes!("input.txt");
    println!("{}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
       let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part_1(s.as_bytes()), 157);
    }
}
