use std::collections::BinaryHeap;

fn part_1() -> u32 {
    include_str!("input.txt").split("\n\n")
        .map(|e| e.lines().map(|s| s.parse::<u32>().unwrap()).sum())
        .max().unwrap()
}

fn part_2() -> u32 {
    //  ideally we should use partial sorting and slice the first 3 elements.
    //  Another performant option is to collect into a binary heap and take the 3 root nodes.
    let mut heap = include_str!("input.txt").split("\n\n")
        .map(|e| e.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<BinaryHeap<_>>();

    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

fn main() {
    println!("max: {}", part_1());
    println!("total max: {}", part_2());
}

// don't need tests too ez
