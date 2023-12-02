use ndarray::Array3;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<(&str, u16, Vec<&str>)> {
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (([A-Z]{2}(, )?)+)",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let valve = caps.get(1).unwrap().as_str();
            (
                valve,
                caps.get(2).unwrap().as_str().parse::<u16>().unwrap(),
                caps.get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn part_1(input: &str) -> u16 {
    let mut valves = parse(input);

    // map valve names to index (so we can use it with a bitset)
    // to give them a logical index, we should sort them.
    valves.sort_by(|a, b| b.1.cmp(&a.1));

    let valve2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();

    let n = valves.len();

    // get flow and adj table based on idx
    let mut adj = vec![vec![0usize; 0]; n];
    let mut flow = vec![0u16; n];

    for v in valves.iter() {
        let i = valve2idx[&v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(valve2idx[*w]);
        }
    }

    // we should always start from AA
    let aa = valve2idx["AA"];

    // bitshift the amount of valves that have a flow > 0
    let mm = 1 << valves.iter().filter(|v| v.1 > 0).count();

    // calculate all possible solutions
    let mut opt = Array3::<u16>::zeros([30, n, mm]);

    for t in 1..30 {
        for i in 0..n {
            let ii = 1 << i;
            for x in 0..mm {
                let mut o = opt[(t, i, x)];
                if ii & x != 0 && t >= 2 {
                    o = o.max(opt[(t - 1, i, x - ii)] + flow[i] * t as u16);
                }
                for &j in adj[i].iter() {
                    o = o.max(opt[(t - 1, j, x)]);
                }
                opt[(t, i, x)] = o;
            }
        }
    }

    // since valve should not be opened, we immediately lose a minute by moving to another valve
    opt[(29, aa, mm - 1)]

    /* solution part 2
    // elephant and human open disjoint sets of valves
        let mut best = 0;
        for x in 0..mm {
            for y in 0..x {
                if (x & y) == 0 {
                    best = best.max(opt[(25, aa, x)] + opt[(25, aa, y)]);
                }
            }
        }
        println!("stage 2: {best}");
      */
}

fn part_2(input: &str) -> u16 {
    0
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(1651, part_1(input));
    }
}
