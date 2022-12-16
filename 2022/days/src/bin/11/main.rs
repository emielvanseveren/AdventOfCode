use std::collections::BinaryHeap;
use std::fs::read_to_string;

struct Monkey {
    items: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    div: usize,
    throw_true: usize,
    throw_false: usize,
    ins: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .into_iter()
        .map(|m| {
            let mut l = m.lines().map(|l| l.split(": ").last().unwrap()).skip(1);

            Monkey {
                items: l
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|n| n.parse().unwrap())
                    .collect(),

                op: {
                    let op: Vec<_> = l
                        .next()
                        .unwrap()
                        .rsplit_once("= ")
                        .unwrap()
                        .1
                        .split(' ')
                        .collect();

                    match op[2] {
                        // if second operand is old it is just old * old
                        "old" => Box::new(|old| old * old),

                        b => match (op[1], b.parse::<usize>().unwrap()) {
                            // operator
                            ("+", n) => Box::new(move |old| old + n),
                            ("*", n) => Box::new(move |old| old * n),
                            _ => panic!("unknown operation"),
                        },
                    }
                },

                div: l
                    .next()
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),

                throw_true: l
                    .next()
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),

                throw_false: l
                    .next()
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                ins: 0,
            }
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let mut monkeys = parse_input(input);

    let (mo, mut items): (usize, _) = (
        monkeys.iter().map(|m| m.div).product(),
        vec![vec![]; monkeys.len()],
    );

    for _ in 0..20 {
        monkeys.iter_mut().enumerate().for_each(|(i, m)| {
            m.items.append(&mut items[i]);
            m.items.drain(0..).for_each(|mut n| {
                n = (m.op)(n) / 3 % mo;
                items[if n % m.div == 0 {
                    m.throw_true
                } else {
                    m.throw_false
                }]
                .push(n);
                m.ins += 1;
            });
        });
    }

    let mut heap = monkeys.iter().map(|m| m.ins).collect::<BinaryHeap<_>>();

    heap.pop().unwrap() * heap.pop().unwrap()
}

fn part_2(input: &str) -> usize {
    let mut monkeys = parse_input(input);

    let (mo, mut items): (usize, _) = (
        monkeys.iter().map(|m| m.div).product(),
        vec![vec![]; monkeys.len()],
    );

    // no longer 20 rounds
    for _ in 0..10000 {
        monkeys.iter_mut().enumerate().for_each(|(i, m)| {
            m.items.append(&mut items[i]);
            m.items.drain(0..).for_each(|mut n| {
                // our worry level is no longer divided by 3
                n = (m.op)(n) % mo;
                items[if n % m.div == 0 {
                    m.throw_true
                } else {
                    m.throw_false
                }]
                .push(n);
                m.ins += 1;
            });
        });
    }

    let mut heap = monkeys.iter().map(|m| m.ins).collect::<BinaryHeap<_>>();
    heap.pop().unwrap() * heap.pop().unwrap()
}

fn main() {
    let input = read_to_string("input.txt").expect("file to exist");
    println!("part 1 {}", part_1(&input));
    println!("part 1 {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
        unimplemented!();
    }
";

    #[test]
    fn test_part_1_example() {
        assert_eq!(10605, part_1(INPUT));
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(2713310158, part_2(INPUT));
    }
}
