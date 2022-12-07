use std::fs::read_to_string;
use std::collections::HashSet;

fn first_start_of_packet(buffer : &str, offset: usize) -> Option<usize> {
    let v: Vec<u8> = buffer.bytes().collect();

    for (idx, window) in v.windows(offset).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == offset {
            return Some(idx + offset);
        }
    }
    None
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("part 1: {}", first_start_of_packet(&input, 4).unwrap());
    println!("part 2: {}", first_start_of_packet(&input, 14).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_start_of_packet_offset_4() {
        const OFFSET: usize = 4;
        assert_eq!(Some(7), first_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", OFFSET));
        assert_eq!(Some(5), first_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", OFFSET));
        assert_eq!(Some(6), first_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", OFFSET));
        assert_eq!(Some(10), first_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", OFFSET));
        assert_eq!(Some(11), first_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", OFFSET));
    }

    #[test]
    fn first_start_of_packet_offset_14() {
        const OFFSET: usize = 14;
        assert_eq!(Some(19), first_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", OFFSET));
        assert_eq!(Some(23), first_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", OFFSET));
        assert_eq!(Some(23), first_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", OFFSET));
        assert_eq!(Some(29), first_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", OFFSET));
        assert_eq!(Some(26), first_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", OFFSET));
    }
}
