// THIS SCREAMS DIJKSTRAAAAA
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::read_to_string;

type PriorityQueue = BinaryHeap<Step>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Step {
    node_nr: usize,
    cost: usize,
    prev: isize,
    edge_nr: isize,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.prev.cmp(&other.prev))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    to_node: usize,
    cost: usize,
}

fn part_1(input: &str) -> u32 {
    shortest_path();
}
fn part_2(input: &str) -> u32 {}

struct ShortestPathStep {
    node_nr: isize,
    edge_nr: isize,
    dist: usize,
}

// adjacency list where each index corresponding to a node value, has a list of outgoing edges
// BFS Implementation of dijkstra
fn shortest_path(
    adj_list: &Vec<Vec<Edge>>,
    start: usize,
    dest: usize,
) -> VecDeque<ShortestPathStep> {
    // we can use a vecdeque instead because all paths have the same cost
    let temp = PriorityQueue::new();

    let permanent: HashMap<usize, Step> = HashMap::new();

    // add start node to the priority queue
    // we use -1 as a placeholder for the previous node (since the start node has no previous)
    temp.push(Step {
        cost: 0,
        prev: -1,
        node_nr: start,
        edge_nr: -1,
    });

    while !temp.is_empty() && !permanent.contains_key(&dest) {
        // is already permanent so we can skip
        while permanent.contains_key(&temp.peek().unwrap().node_nr) {
            temp.pop();
        }

        // make current node permanent
        let curr = temp.pop().unwrap();
        permanent[&curr.node_nr] = curr;

        // bfs (check all neighbours of current node
        for (idx, edge) in adj_list[curr.node_nr].iter().enumerate() {
            // if node is not permanent yet, update cost and add as option to temporary nodes
            // if new cost is lower than previous cost it will appear earlier
            if !permanent.contains_key(&edge.to_node) {
                temp.push( Step {
                    prev: curr.node_nr as isize,
                    node_nr: edge.to_node,
                    edge_nr: idx as isize,
                    cost: curr.cost + edge.cost /* todo: this should be the cost of the current edge */
                })
            }
        }
    }

    if temp.is_empty() && !permanent.contains_key(&dest) {
        panic!("graph is not connected.");
    }

    let result = VecDeque::new();
    let mut temp: isize = dest as isize;
    while temp != -1 {
        let curr = permanent[&(temp as usize)];
        result.push_front(ShortestPathStep {
            node_nr: curr.node_nr as isize,
            edge_nr: curr.edge_nr,
            dist: curr.cost,
        });
        temp = curr.prev;
    }

    // the permanent nodes now contain the steps from end to start

    result
}

fn part_2() {
    unimplemented!();
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let graph = println!("part 1: {}", part_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        unimplemented!();
    }

    fn part_1_example() {
        assert_eq!(2 + 2, 4);
    }
}
