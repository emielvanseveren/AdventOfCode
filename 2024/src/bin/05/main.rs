struct SafetyManual<'a> {
    rules: Vec<(&'a str, &'a str)>,
    pages: Vec<Vec<&'a str>>,
}

impl<'a> SafetyManual<'a> {
    fn new<'b>(input: &'b str) -> Self
    where
        'b: 'a,
    {
        let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
        let rules = rules_str
            .lines()
            .filter_map(|line| line.split_once('|'))
            .collect();

        let pages = updates_str
            .lines()
            .map(|line| line.split(',').collect())
            .collect();

        Self { rules, pages }
    }
    fn get_failed(&self) -> Vec<bool> {
        let len = self.pages.len();
        let mut failed = vec![false; len];

        for (before, after) in self.rules.iter() {
            for (i, page) in self.pages.iter().enumerate() {
                if failed[i] {
                    continue;
                }
                if let Some(s) = page.iter().position(|x| x == after) {
                    if page[s..].contains(before) {
                        failed[i] = true;
                    }
                }
            }
        }

        failed
    }
}

fn part_one(safety_manual: &SafetyManual) -> usize {
    let failed = safety_manual.get_failed();
    safety_manual
        .pages
        .iter()
        .enumerate()
        .filter_map(|(index, page)| {
            if failed[index] {
                return None;
            }
            let mid = page[page.len() / 2];
            Some(mid.parse::<usize>().unwrap())
        })
        .sum()
}

fn part_two(manual: &SafetyManual) -> usize {
    let failed = manual.get_failed();

    let failed_pages: Vec<_> = manual
        .pages
        .iter()
        .enumerate()
        .filter_map(|(index, page)| {
            if failed[index] {
                return Some(page.clone());
            }
            None
        })
        .collect();

    let mut sum = 0;
    for mut page in failed_pages {
        loop {
            let mut swapped = false;

            'outer: for &(first, second) in manual.rules.iter() {
                for i in 0..page.len() {
                    if page[i] == first {
                        continue 'outer;
                    }
                    if page[i] == second {
                        // look for first and swap
                        for j in i..page.len() {
                            if page[j] == first {
                                page.swap(i, j);
                                swapped = true;
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            if !swapped {
                sum += page[page.len() / 2]
                    .parse::<usize>()
                    .expect("it to be a number");
                break;
            }
        }
    }

    sum
}

fn main() -> Result<(), anyhow::Error> {
    let input = include_str!("./input.txt");
    let safety_manual = SafetyManual::new(input);
    println!("part one: {}", part_one(&safety_manual));
    println!("part two: {}", part_two(&safety_manual));
    Ok(())
}

const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

#[test]
fn example_1() {
    let safety_manual = SafetyManual::new(TEST_INPUT);
    assert_eq!(part_one(&safety_manual), 143);
}

#[test]
fn example_2() {
    let safety_manual = SafetyManual::new(TEST_INPUT);
    assert_eq!(part_one(&safety_manual), 123);
}
